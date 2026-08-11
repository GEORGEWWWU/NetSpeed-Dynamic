use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use std::{
    process::Command,
    sync::{atomic::{AtomicBool, Ordering}, Mutex},
    time::{Duration, Instant},
};
use tauri::{command, AppHandle, Emitter};
use tokio::{net::TcpListener, time::timeout};
use tokio_tungstenite::{accept_async, connect_async, tungstenite::Message};

#[cfg(target_os = "windows")]
use std::{collections::HashSet, mem::zeroed};

#[cfg(target_os = "windows")]
fn send_windows_media_key(action: &str) -> bool {
    use winapi::um::winuser::keybd_event;

    let virtual_key = match action {
        "next" => 0xB0,
        "prev" => 0xB1,
        "play_pause" => 0xB3,
        _ => return false,
    };

    unsafe {
        keybd_event(virtual_key, 0, 0, 0);
        keybd_event(virtual_key, 0, 0x0002, 0);
    }
    true
}

// --- 引入 SMTC 需要的模块 ---
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession, GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus,
};

// 全局记录当前选中的平台（默认空，由前端传来）
static TARGET_PLAYER: Mutex<String> = Mutex::new(String::new());
static MUSIC_CONTROLLER_ENABLED: AtomicBool = AtomicBool::new(false);
static KUGOU_TIMELINE_REPORTED: AtomicBool = AtomicBool::new(false);

#[derive(Clone)]
struct NeteaseBridgeState {
    song: String,
    artist: String,
    playing: bool,
    position_ms: i64,
    duration_ms: i64,
    updated_at: Instant,
}

#[derive(Deserialize)]
struct NeteaseBridgePayload {
    #[serde(rename = "type")]
    message_type: String,
    source: String,
    song: String,
    #[serde(default)]
    artist: String,
    playing: bool,
    position: i64,
    duration: i64,
}

lazy_static::lazy_static! {
    static ref NETEASE_BRIDGE_STATE: Mutex<Option<NeteaseBridgeState>> = Mutex::new(None);
}

fn get_fresh_netease_bridge_state() -> Option<NeteaseBridgeState> {
    let state = NETEASE_BRIDGE_STATE.lock().ok()?.clone()?;
    (state.updated_at.elapsed() <= Duration::from_secs(3)).then_some(state)
}

/// 可选的本机 Chromium 调试桥接；默认不启动，仅用于兼容性诊断。
#[allow(dead_code)]
pub async fn run_netease_cdp_bridge(app: AppHandle) {
    const TARGETS_URL: &str = "http://127.0.0.1:47393/json";
    const PLAYER_EXPRESSION: &str = r#"(()=>{const ranges=Array.from(document.querySelectorAll('input[type=range]'));const slider=ranges.find(e=>Number(e.max)>30&&e.closest('[aria-label]')?.querySelector('.thumb'));const song=document.querySelector('.cmd-space.info-wrapper .main-title,.cmd-space.info-wrapper .vinly-title')?.innerText?.trim()??'';const artist=document.querySelector('.cmd-space.info-wrapper .author')?.innerText?.trim()??'';const position=Number(slider?.value);const duration=Number(slider?.max);return JSON.stringify({type:'player-state',source:'netease',song,artist,playing:Boolean(document.querySelector('[data-testid=tid_playbar_play_btn] .cmd-icon-pause,#btn_pc_minibar_play .cmd-icon-pause')),position:Math.max(0,Math.round(position*1000)),duration:Math.max(0,Math.round(duration*1000))})})()"#;

    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
    {
        Ok(client) => client,
        Err(error) => {
            eprintln!("[网易云同步] 无法创建本地调试客户端: {error}");
            return;
        }
    };

    loop {
        let debugger_url = async {
            let targets = client
                .get(TARGETS_URL)
                .send()
                .await
                .ok()?
                .json::<serde_json::Value>()
                .await
                .ok()?;
            targets
                .as_array()?
                .iter()
                .find(|target| target.get("type").and_then(|value| value.as_str()) == Some("page"))?
                .get("webSocketDebuggerUrl")?
                .as_str()
                .map(str::to_string)
        }
        .await;

        let Some(debugger_url) = debugger_url else {
            tokio::time::sleep(Duration::from_secs(2)).await;
            continue;
        };

        let Ok((mut websocket, _)) = connect_async(&debugger_url).await else {
            tokio::time::sleep(Duration::from_secs(2)).await;
            continue;
        };

        println!("[网易云同步] 已连接网易云真实播放器页面");
        let mut request_id: u64 = 1;
        let mut reported_first_state = false;
        loop {
            let command = serde_json::json!({
                "id": request_id,
                "method": "Runtime.evaluate",
                "params": {
                    "expression": PLAYER_EXPRESSION,
                    "returnByValue": true
                }
            });
            if websocket
                .send(Message::Text(command.to_string()))
                .await
                .is_err()
            {
                break;
            }

            let response = loop {
                match timeout(Duration::from_secs(2), websocket.next()).await {
                    Ok(Some(Ok(message))) => {
                        let Ok(text) = message.to_text() else { continue };
                        let Ok(value) = serde_json::from_str::<serde_json::Value>(text) else {
                            continue;
                        };
                        if value.get("id").and_then(|value| value.as_u64()) == Some(request_id) {
                            break Some(value);
                        }
                    }
                    _ => break None,
                }
            };

            let Some(response) = response else { break };
            let snapshot_json = response
                .pointer("/result/result/value")
                .and_then(|value| value.as_str());
            let Some(snapshot_json) = snapshot_json else {
                tokio::time::sleep(Duration::from_millis(250)).await;
                request_id += 1;
                continue;
            };
            let Ok(payload) = serde_json::from_str::<NeteaseBridgePayload>(snapshot_json) else {
                tokio::time::sleep(Duration::from_millis(250)).await;
                request_id += 1;
                continue;
            };

            if payload.message_type == "player-state"
                && payload.source == "netease"
                && !payload.song.trim().is_empty()
                && payload.position >= 0
                && payload.duration > 0
                && payload.position <= payload.duration + 30_000
            {
                let state = NeteaseBridgeState {
                    song: payload.song.trim().to_string(),
                    artist: payload.artist.trim().to_string(),
                    playing: payload.playing,
                    position_ms: payload.position,
                    duration_ms: payload.duration,
                    updated_at: Instant::now(),
                };
                if !reported_first_state {
                    println!(
                        "[网易云同步] 已读取真实播放进度: {} - {}，{} ms / {} ms",
                        state.song, state.artist, state.position_ms, state.duration_ms
                    );
                    reported_first_state = true;
                }
                if let Ok(mut cached) = NETEASE_BRIDGE_STATE.lock() {
                    *cached = Some(state.clone());
                }
                let _ = app.emit(
                    "websocket-lyrics",
                    serde_json::json!({ "type": "progress", "position": state.position_ms }),
                );
                let _ = app.emit(
                    "websocket-lyrics",
                    serde_json::json!({
                        "type": "playback",
                        "status": if state.playing { "playing" } else { "paused" }
                    }),
                );
            }

            request_id += 1;
            tokio::time::sleep(Duration::from_millis(250)).await;
        }

        println!("[网易云同步] 播放器页面连接中断，等待自动重连");
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

/// 网易云 3.x 主动连接这个仅限本机的桥接端口，把内部真实进度转换成统一播放器时间轴。
pub fn start_netease_bridge(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let listener = match TcpListener::bind("127.0.0.1:47391").await {
            Ok(listener) => listener,
            Err(error) => {
                eprintln!("[网易云同步] 无法监听 127.0.0.1:47391: {error}");
                return;
            }
        };

        println!("[本地播放器桥接] 进度桥接已启动: ws://127.0.0.1:47391");
        loop {
            let (stream, peer) = match listener.accept().await {
                Ok(connection) => connection,
                Err(error) => {
                    eprintln!("[网易云同步] 接收连接失败: {error}");
                    continue;
                }
            };

            if !peer.ip().is_loopback() {
                continue;
            }

            let mut websocket = match accept_async(stream).await {
                Ok(websocket) => websocket,
                Err(error) => {
                    eprintln!("[网易云同步] WebSocket 握手失败: {error}");
                    continue;
                }
            };

            println!("[网易云同步] 网易云进度插件已连接");
            let mut reported_first_state = false;
            let mut reported_nonzero_state = false;
            let mut reported_diagnostic = false;
            while let Ok(Some(message)) = timeout(Duration::from_secs(15), websocket.next()).await {
                let Ok(message) = message else { break };
                let Ok(text) = message.to_text() else { continue };
                let Ok(value) = serde_json::from_str::<serde_json::Value>(text) else {
                    continue;
                };
                if value.get("type").and_then(|value| value.as_str()) == Some("bridge-heartbeat") {
                    if !reported_diagnostic {
                        println!("[网易云同步] 插件诊断: {value}");
                        reported_diagnostic = true;
                    }
                    continue;
                }
                let Ok(payload) = serde_json::from_value::<NeteaseBridgePayload>(value) else {
                    continue;
                };

                if payload.message_type != "player-state"
                    || payload.source != "netease"
                    || payload.song.trim().is_empty()
                    || payload.position < 0
                    || payload.duration < 0
                    || (payload.duration > 0 && payload.position > payload.duration + 30_000)
                {
                    continue;
                }

                let state = NeteaseBridgeState {
                    song: payload.song.trim().to_string(),
                    artist: payload.artist.trim().to_string(),
                    playing: payload.playing,
                    position_ms: payload.position,
                    duration_ms: payload.duration,
                    updated_at: Instant::now(),
                };
                if !reported_first_state {
                    println!(
                        "[网易云同步] 已收到真实播放器状态: {} - {}，位置 {} ms / {} ms，播放={}",
                        state.song,
                        state.artist,
                        state.position_ms,
                        state.duration_ms,
                        state.playing
                    );
                    reported_first_state = true;
                }
                if state.position_ms > 0 && !reported_nonzero_state {
                    println!(
                        "[网易云同步] 进度已开始推进: {} ms / {} ms",
                        state.position_ms, state.duration_ms
                    );
                    reported_nonzero_state = true;
                }
                if let Ok(mut cached) = NETEASE_BRIDGE_STATE.lock() {
                    *cached = Some(state.clone());
                }

                let _ = app.emit(
                    "websocket-lyrics",
                    serde_json::json!({ "type": "progress", "position": state.position_ms }),
                );
                let _ = app.emit(
                    "websocket-lyrics",
                    serde_json::json!({
                        "type": "playback",
                        "status": if state.playing { "playing" } else { "paused" }
                    }),
                );
            }

            println!("[网易云同步] 网易云进度插件已断开，等待自动重连");
            if let Ok(mut cached) = NETEASE_BRIDGE_STATE.lock() {
                *cached = None;
            }
        }
    });
}

#[cfg(target_os = "windows")]
struct NeteaseWindowSearch {
    process_ids: HashSet<u32>,
    title: Option<String>,
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn find_netease_window(
    hwnd: winapi::shared::windef::HWND,
    lparam: winapi::shared::minwindef::LPARAM,
) -> winapi::shared::minwindef::BOOL {
    use std::os::windows::ffi::OsStringExt;
    use winapi::um::winuser::{GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId, IsWindowVisible};

    let search = &mut *(lparam as *mut NeteaseWindowSearch);
    if IsWindowVisible(hwnd) == 0 {
        return 1;
    }

    let mut process_id = 0;
    GetWindowThreadProcessId(hwnd, &mut process_id);
    if !search.process_ids.contains(&process_id) {
        return 1;
    }

    let text_len = GetWindowTextLengthW(hwnd);
    if text_len <= 0 {
        return 1;
    }

    let mut buffer = vec![0u16; text_len as usize + 1];
    let copied = GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32);
    if copied > 0 {
        let title = std::ffi::OsString::from_wide(&buffer[..copied as usize])
            .to_string_lossy()
            .trim()
            .to_string();
        if title.contains(" - ") {
            search.title = Some(title);
            return 0;
        }
    }

    1
}

#[cfg(target_os = "windows")]
fn get_netease_window_info() -> Option<(String, String)> {
    use winapi::um::{handleapi::CloseHandle, tlhelp32::{CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS}};

    let mut process_ids = HashSet::new();
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        let mut entry: PROCESSENTRY32W = zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

        if Process32FirstW(snapshot, &mut entry) != 0 {
            loop {
                let name_end = entry.szExeFile.iter().position(|&c| c == 0).unwrap_or(entry.szExeFile.len());
                let process_name = String::from_utf16_lossy(&entry.szExeFile[..name_end]);
                if process_name.eq_ignore_ascii_case("cloudmusic.exe") {
                    process_ids.insert(entry.th32ProcessID);
                }
                if Process32NextW(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }
        CloseHandle(snapshot);
    }

    if process_ids.is_empty() {
        return None;
    }

    let mut search = NeteaseWindowSearch { process_ids, title: None };
    unsafe {
        winapi::um::winuser::EnumWindows(Some(find_netease_window), &mut search as *mut _ as isize);
    }
    let title = search.title?;
    let (song, artist) = title.split_once(" - ")?;
    let song = song.trim().to_string();
    let artist = artist.trim().to_string();
    (!song.is_empty()).then_some((song, artist))
}

// 给前端调用的切换接口
#[command]
pub fn set_target_player(player: String) {
    if let Ok(mut target) = TARGET_PLAYER.lock() {
        *target = player;
    }
}

#[command]
pub fn set_music_controller_enabled(enabled: bool) {
    MUSIC_CONTROLLER_ENABLED.store(enabled, Ordering::SeqCst);
}

#[command]
pub fn is_music_controller_enabled() -> bool {
    MUSIC_CONTROLLER_ENABLED.load(Ordering::SeqCst)
}

// 自动匹配你选择的软件
fn get_target_media_session() -> Option<GlobalSystemMediaTransportControlsSession> {
    let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
        .ok()?
        .get()
        .ok()?;

    let sessions = manager.GetSessions().ok()?;

    // 获取当前的目标（前端如果还没传，默认用 netease）
    let target = {
        let guard = TARGET_PLAYER.lock().unwrap_or_else(|e| e.into_inner()); // 加入防中毒
        if guard.is_empty() {
            "netease".to_string()
        } else {
            guard.clone()
        }
    };

    // 通用模式优先匹配 JustSolo 逻辑
    if target == "other" {
        // 第一轮遍历：优先寻找 JustSolo.JustSolo
        for session in manager.GetSessions().ok()? {
            if let Ok(app_id) = session.SourceAppUserModelId() {
                if app_id.to_string().to_lowercase().contains("justsolo") {
                    return Some(session);
                }
            }
        }
        // 第二轮遍历：如果没有找到 JustSolo，回退到原逻辑，直接返回第一个有效媒体会话
        for session in manager.GetSessions().ok()? {
            return Some(session);
        }
        return None;
    }

    // 其他平台逻辑
    for session in sessions {
        if let Ok(app_id) = session.SourceAppUserModelId() {
            let app_id_str = app_id.to_string().to_lowercase();
            // 网易云特殊一点，包名可能叫 cloudmusic 或 netease
            if target == "netease"
                && (app_id_str.contains("cloudmusic") || app_id_str.contains("netease"))
            {
                return Some(session);
            }
            // 其他软件直接用名字去系统进程列表里撞
            else if target != "netease" && app_id_str.contains(&target) {
                return Some(session);
            }
        }
    }
    None
}

/// 酷狗的 SMTC 会话不提供 TimelineProperties。它的底栏却始终展示真实的“已播放/总时长”；
/// 使用 Windows 内置 OCR 读取该小区域，专门处理拖动进度条后的实时定位。
#[cfg(target_os = "windows")]
fn get_kugou_screen_position() -> Option<i64> {
    const SCRIPT: &str = include_str!("../../integrations/kugou/read-position.ps1");
    let utf16le: Vec<u8> = SCRIPT
        .encode_utf16()
        .flat_map(|unit| unit.to_le_bytes())
        .collect();
    let encoded_script = inline_base64_encode(&utf16le);
    let output = Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-WindowStyle",
            "Hidden",
            "-EncodedCommand",
            &encoded_script,
        ])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<i64>()
        .ok()
        .filter(|position| *position >= 0)
}

#[command]
pub async fn fetch_netease_music_info() -> Result<Option<(String, String, bool, i64, i64)>, String>
{
    let target = TARGET_PLAYER
        .lock()
        .map(|value| value.clone())
        .unwrap_or_default();
    if target.is_empty() || target == "netease" {
        if let Some(state) = get_fresh_netease_bridge_state() {
            return Ok(Some((
                state.song,
                state.artist,
                state.playing,
                state.position_ms,
                state.duration_ms,
            )));
        }
    }

    let session = match get_target_media_session() {
        Some(s) => s,
        None => {
            if target == "netease" {
                if let Some((song, artist)) = get_netease_window_info() {
                    return Ok(Some((song, artist, true, 0, 0)));
                }
            }
            return Ok(None);
        }
    };

    let is_playing = if let Ok(playback_info) = session.GetPlaybackInfo() {
        if let Ok(status) = playback_info.PlaybackStatus() {
            status == GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing
        } else {
            false
        }
    } else {
        false
    };

    let properties = session
        .TryGetMediaPropertiesAsync()
        .map_err(|e| e.to_string())?
        .get()
        .map_err(|e| e.to_string())?;

    let title = properties.Title().unwrap_or_default().to_string();
    let artist = properties.Artist().unwrap_or_default().to_string();

    if title.is_empty() {
        return Ok(None);
    }

    let mut position_ms: i64 = 0;
    let mut duration_ms: i64 = 0; // 新增：用于记录歌曲总时长

    if let Ok(timeline) = session.GetTimelineProperties() {
        if let Ok(pos) = timeline.Position() {
            position_ms = pos.Duration / 10000;

            // 提取准确的歌曲总时长
            if let Ok(end) = timeline.EndTime() {
                duration_ms = end.Duration / 10000;
            }

            // 补偿算法保持不变
            if is_playing {
                if let Ok(last_updated) = timeline.LastUpdatedTime() {
                    if let Ok(now) =
                        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
                    {
                        let current_100ns = (now.as_nanos() / 100) as i64 + 116_444_736_000_000_000;
                        let diff_100ns = current_100ns - last_updated.UniversalTime;
                        let diff_ms = diff_100ns / 10000;

                        if diff_ms > 0 && diff_ms < 86400000 {
                            position_ms += diff_ms;
                        }
                    }
                }
            }
        }
    }

    if target == "kugou" && position_ms == 0 && duration_ms == 0 {
        if let Some(screen_position) = get_kugou_screen_position() {
            position_ms = screen_position;
            if !KUGOU_TIMELINE_REPORTED.swap(true, Ordering::SeqCst) {
                println!("[酷狗同步] 已启用播放器界面真实进度读取");
            }
        }
    }

    // 返回值增加了一个 duration_ms 参数
    Ok(Some((title, artist, is_playing, position_ms, duration_ms)))
}

#[command]
pub async fn control_system_media(action: String) -> Result<(), String> {
    if let Some(session) = get_target_media_session() {
        match action.as_str() {
            "play_pause" => {
                let _ = session.TryTogglePlayPauseAsync();
            }
            "next" => {
                let _ = session.TrySkipNextAsync();
            }
            "prev" => {
                let _ = session.TrySkipPreviousAsync();
            }
            _ => {}
        }
        return Ok(());
    }

    // 网易云 3.x 主动关闭了 Windows SMTC；使用同一组标准媒体键作为控制适配层。
    // 这与键盘上的上一首/播放暂停/下一首完全等价，不依赖界面坐标或按钮选择器。
    let target = TARGET_PLAYER
        .lock()
        .map(|value| value.clone())
        .unwrap_or_default();
    if target.is_empty() || target == "netease" {
        #[cfg(target_os = "windows")]
        if send_windows_media_key(&action) {
            return Ok(());
        }
    }
    Ok(())
}

// 纯手工轻量 Base64 编码器
fn inline_base64_encode(input: &[u8]) -> String {
    const CHARSET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity((input.len() + 2) / 3 * 4);
    for chunk in input.chunks(3) {
        match chunk.len() {
            3 => {
                result.push(CHARSET[(chunk[0] >> 2) as usize] as char);
                result.push(CHARSET[(((chunk[0] & 0x03) << 4) | (chunk[1] >> 4)) as usize] as char);
                result.push(CHARSET[(((chunk[1] & 0x0F) << 2) | (chunk[2] >> 6)) as usize] as char);
                result.push(CHARSET[(chunk[2] & 0x3F) as usize] as char);
            }
            2 => {
                result.push(CHARSET[(chunk[0] >> 2) as usize] as char);
                result.push(CHARSET[(((chunk[0] & 0x03) << 4) | (chunk[1] >> 4)) as usize] as char);
                result.push(CHARSET[((chunk[1] & 0x0F) << 2) as usize] as char);
                result.push('=');
            }
            1 => {
                result.push(CHARSET[(chunk[0] >> 2) as usize] as char);
                result.push(CHARSET[((chunk[0] & 0x03) << 4) as usize] as char);
                result.push('=');
                result.push('=');
            }
            _ => {}
        }
    }
    result
}

// 利用微软官方 SMTC API 直接把网易云的本地封面榨出来
fn get_smtc_thumbnail() -> Option<String> {
    use windows::Storage::Streams::{Buffer, DataReader, InputStreamOptions};

    let session = get_target_media_session()?;
    let properties = session.TryGetMediaPropertiesAsync().ok()?.get().ok()?;
    let thumbnail_ref = properties.Thumbnail().ok()?;
    let stream = thumbnail_ref.OpenReadAsync().ok()?.get().ok()?;
    let size = stream.Size().ok()? as u32;
    if size == 0 {
        return None;
    }

    let buffer = Buffer::Create(size).ok()?;
    stream
        .ReadAsync(&buffer, size, InputStreamOptions::None)
        .ok()?
        .get()
        .ok()?;
    let reader = DataReader::FromBuffer(&buffer).ok()?;
    let mut bytes = vec![0u8; size as usize];
    reader.ReadBytes(&mut bytes).ok()?;

    Some(format!(
        "data:image/jpeg;base64,{}",
        inline_base64_encode(&bytes)
    ))
}

#[command]
pub async fn get_random_cover_url(
    song_name: String,
    artist_name: String,
) -> Result<String, String> {
    if let Some(base64_cover) = get_smtc_thumbnail() {
        return Ok(base64_cover);
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .map_err(|e| e.to_string())?;

    let (tx, mut rx) = tokio::sync::mpsc::channel(3);

    // 1号赛道：Apple Music
    let tx_itunes = tx.clone();
    let client_itunes = client.clone();
    let query_itunes = format!("{} {}", song_name, artist_name);
    tokio::spawn(async move {
        let encoded_query = urlencoding::encode(&query_itunes).into_owned();
        let itunes_url = format!(
            "https://itunes.apple.com/search?term={}&media=music&limit=1",
            encoded_query
        );
        if let Ok(resp) = client_itunes.get(&itunes_url).send().await {
            if let Ok(json) = resp.json::<serde_json::Value>().await {
                if let Some(artwork) = json
                    .pointer("/results/0/artworkUrl100")
                    .and_then(|v| v.as_str())
                {
                    let _ = tx_itunes
                        .send(artwork.replace("100x100bb", "300x300bb"))
                        .await;
                }
            }
        }
    });

    // 2号赛道：网易云 API
    let tx_netease = tx.clone();
    let client_netease = client.clone();
    let song_netease = song_name.clone();
    let artist_netease = artist_name.clone();
    tokio::spawn(async move {
        let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36";
        let query = format!("{} {}", song_netease, artist_netease);
        if let Ok(resp) = client_netease
            .post("https://music.163.com/api/search/get/web")
            .header("Referer", "https://music.163.com")
            .header("User-Agent", ua)
            .form(&[
                ("s", query.as_str()),
                ("type", "1"),
                ("limit", "1"),
                ("offset", "0"),
            ])
            .send()
            .await
        {
            if let Ok(json) = resp.json::<serde_json::Value>().await {
                if let Some(pic) = json
                    .pointer("/result/songs/0/al/picUrl")
                    .and_then(|v| v.as_str())
                {
                    if !pic.is_empty() && pic != "http://p4.music.126.net/UeTuwE7pvjBpypWLudqukQ==/3135032972947607.jpg" {
                        let _ = tx_netease.send(pic.replace("http://", "https://") + "?param=300y300").await;
                    }
                }
            }
        }
    });

    // 3号赛道：Deezer API
    let tx_deezer = tx.clone();
    let client_deezer = client.clone();
    let song_deezer = song_name.clone();
    let artist_deezer = artist_name.clone();
    tokio::spawn(async move {
        let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36";
        let deezer_url = format!(
            "https://api.deezer.com/search?q=track:\"{}\" artist:\"{}\"&limit=1",
            urlencoding::encode(&song_deezer).into_owned(),
            urlencoding::encode(&artist_deezer).into_owned()
        );
        if let Ok(resp) = client_deezer
            .get(&deezer_url)
            .header("User-Agent", ua)
            .send()
            .await
        {
            if let Ok(json) = resp.json::<serde_json::Value>().await {
                if let Some(cover) = json
                    .pointer("/data/0/album/cover_medium")
                    .and_then(|v| v.as_str())
                {
                    if !cover.is_empty() {
                        let _ = tx_deezer.send(cover.to_string()).await;
                    }
                } else if let Some(cover) = json
                    .pointer("/data/0/album/cover_big")
                    .and_then(|v| v.as_str())
                {
                    if !cover.is_empty() {
                        let _ = tx_deezer.send(cover.to_string()).await;
                    }
                }
            }
        }
    });

    match tokio::time::timeout(std::time::Duration::from_secs(3), rx.recv()).await {
        Ok(Some(url)) => Ok(url),
        _ => Ok("data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTUwIiBoZWlnaHQ9IjE1MCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48ZGVmcz48bGluZWFyR3JhZGllbnQgaWQ9ImciIHgxPSIwJSIgeTE9IjAlIiB4Mj0iMTAwJSIgeTI9IjEwMCUiPjxzdG9wIG9mZnNldD0iMCUiIHN0b3AtY29sb3I9IiNhOGVkZWEiLz48c3RvcCBvZmZzZXQ9IjEwMCUiIHN0b3AtY29sb3I9IiNmZWQ2ZTMiLz48L2xpbmVhckdyYWRpZW50PjwvZGVmcz48cmVjdCB3aWR0aD0iMTUwIiBoZWlnaHQ9IjE1MCIgcng9Ijc1IiBmaWxsPSJ1cmwoI2cpIi8+PC9zdmc+".to_string()),
    }
}

#[command]
pub async fn fetch_netease_lyrics(
    song_name: String,
    artist_name: String,
    duration_ms: i64,
) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(4))
        .build()
        .map_err(|e| e.to_string())?;

    // ENGINE 1: LRCLIB (精确匹配，自带极高校验度)
    let duration_sec = duration_ms / 1000;
    if duration_sec > 0 {
        let lrclib_url = format!(
            "https://lrclib.net/api/get?track_name={}&artist_name={}&duration={}",
            urlencoding::encode(&song_name),
            urlencoding::encode(&artist_name),
            duration_sec
        );

        if let Ok(resp) = client.get(&lrclib_url).send().await {
            if let Ok(json) = resp.json::<serde_json::Value>().await {
                if let Some(synced_lyrics) = json.pointer("/syncedLyrics").and_then(|v| v.as_str())
                {
                    if !synced_lyrics.is_empty() {
                        println!("[网络歌词调试] 命中引擎 1: LRCLIB API 精确匹配");
                        return Ok(synced_lyrics.to_string());
                    }
                }
            }
        }
    }

    let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36";
    let query = format!("{} {}", song_name, artist_name);
    let query_name_lower = song_name.to_lowercase();
    let query_artist_lower = artist_name.to_lowercase(); // 新增：歌手小写比对

    // ENGINE 2: NETEASE FALLBACK (网易云兜底)
    let fake_ip = {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        format!(
            "{}.{}.{}.{}",
            rng.gen_range(11..250),
            rng.gen_range(11..250),
            rng.gen_range(11..250),
            rng.gen_range(11..250)
        )
    };

    if let Ok(resp) = client
        .post("https://music.163.com/api/search/get/web")
        .header("Referer", "https://music.163.com")
        .header("User-Agent", ua)
        .header("X-Real-IP", &fake_ip)
        .form(&[
            ("s", query.as_str()),
            ("type", "1"),
            ("limit", "8"),
            ("offset", "0"),
        ])
        .send()
        .await
    {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            if let Some(songs) = json.pointer("/result/songs").and_then(|v| v.as_array()) {
                let mut best_song_id = None;
                let mut min_diff = i64::MAX;

                for song in songs {
                    let song_duration = song
                        .get("duration")
                        .or(song.get("dt"))
                        .and_then(|v| v.as_i64());
                    let id = song.get("id").and_then(|v| v.as_i64());
                    let name = song
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_lowercase();

                    // 提取网易云歌手名进行比对
                    let mut singer_name = String::new();
                    if let Some(artists) = song
                        .get("artists")
                        .or(song.get("ar"))
                        .and_then(|v| v.as_array())
                    {
                        for a in artists {
                            if let Some(aname) = a.get("name").and_then(|v| v.as_str()) {
                                singer_name.push_str(&aname.to_lowercase());
                            }
                        }
                    }

                    let name_match =
                        name.contains(&query_name_lower) || query_name_lower.contains(&name);
                    let artist_match = singer_name.contains(&query_artist_lower)
                        || query_artist_lower.contains(&singer_name)
                        || query_artist_lower.is_empty();

                    if let (Some(id), Some(song_dur)) = (id, song_duration) {
                        if duration_ms > 0 {
                            let diff = (song_dur - duration_ms).abs();
                            // 核心修复：必须名字匹配，且 (歌手匹配 或 时间误差极小) 才算及格！
                            if name_match && (artist_match || diff <= 3000) {
                                if diff < min_diff {
                                    min_diff = diff;
                                    best_song_id = Some(id);
                                }
                            }
                        } else if name_match && artist_match {
                            best_song_id = Some(id);
                            break;
                        }
                    }
                }

                if let Some(song_id) = best_song_id {
                    let lyric_url = format!(
                        "https://music.163.com/api/song/lyric?id={}&lv=-1&kv=-1&tv=-1",
                        song_id
                    );
                    if let Ok(lyric_resp) = client
                        .get(&lyric_url)
                        .header("User-Agent", ua)
                        .header("X-Real-IP", &fake_ip)
                        .send()
                        .await
                    {
                        if let Ok(lyric_json) = lyric_resp.json::<serde_json::Value>().await {
                            if let Some(lyric_text) =
                                lyric_json.pointer("/lrc/lyric").and_then(|v| v.as_str())
                            {
                                println!("[网络歌词调试] 命中引擎 2: 网易云 API 兜底 (已通过完美双重校验)");
                                return Ok(lyric_text.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    // ENGINE 3: QQ MUSIC (极速国内优选源)
    let qq_search_url = format!(
        "https://c.y.qq.com/soso/fcgi-bin/client_search_cp?w={}&n=5&format=json",
        urlencoding::encode(&query)
    );

    if let Ok(resp) = client
        .get(&qq_search_url)
        .header("User-Agent", ua)
        .send()
        .await
    {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            if let Some(songs) = json.pointer("/data/song/list").and_then(|v| v.as_array()) {
                let mut best_songmid = None;

                for song in songs {
                    let songmid = song.get("songmid").and_then(|v| v.as_str());
                    let interval = song.get("interval").and_then(|v| v.as_i64()).unwrap_or(0);
                    let name = song
                        .get("songname")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_lowercase();

                    // 提取 QQ 音乐歌手名
                    let mut singer_name = String::new();
                    if let Some(singers) = song.get("singer").and_then(|v| v.as_array()) {
                        for s in singers {
                            if let Some(sname) = s.get("name").and_then(|v| v.as_str()) {
                                singer_name.push_str(&sname.to_lowercase());
                            }
                        }
                    }

                    let name_match =
                        name.contains(&query_name_lower) || query_name_lower.contains(&name);
                    let artist_match = singer_name.contains(&query_artist_lower)
                        || query_artist_lower.contains(&singer_name)
                        || query_artist_lower.is_empty();

                    if let Some(mid) = songmid {
                        if duration_ms > 0 {
                            let diff = (interval * 1000 - duration_ms).abs();
                            // 核心修复：必须名字匹配，且 (歌手匹配 或 时间误差极小)
                            if name_match && (artist_match || diff <= 3000) {
                                best_songmid = Some(mid.to_string());
                                break;
                            }
                        } else if name_match && artist_match {
                            best_songmid = Some(mid.to_string());
                            break;
                        }
                    }
                }

                if let Some(songmid) = best_songmid {
                    let qq_lyric_url = format!("https://c.y.qq.com/lyric/fcgi-bin/fcg_query_lyric_new.fcg?songmid={}&format=json&nobase64=1", songmid);
                    if let Ok(lyric_resp) = client
                        .get(&qq_lyric_url)
                        .header("Referer", "https://y.qq.com/")
                        .header("User-Agent", ua)
                        .send()
                        .await
                    {
                        if let Ok(lyric_json) = lyric_resp.json::<serde_json::Value>().await {
                            if let Some(lyric_text) =
                                lyric_json.get("lyric").and_then(|v| v.as_str())
                            {
                                let decoded = lyric_text
                                    .replace("&#10;", "\n")
                                    .replace("&#13;", "\r")
                                    .replace("&#32;", " ")
                                    .replace("&#45;", "-")
                                    .replace("&#40;", "(")
                                    .replace("&#41;", ")");
                                if !decoded.is_empty() {
                                    println!("[网络歌词调试] 命中引擎 3: QQ音乐 API (已通过完美双重校验)");
                                    return Ok(decoded);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("[网络歌词调试] 失败：所有网络接口均未找到匹配歌词，或未通过双重校验");
    Ok("".to_string())
}

// WebSocket 实时歌词推送
async fn run_websocket_lyrics(url: String, app: AppHandle) -> Result<(), String> {
    let (ws_stream, _) = connect_async(&url)
        .await
        .map_err(|e| format!("WebSocket 连接失败: {}", e))?;

    println!("[WebSocket 调试] 连接成功！开始实时接收歌词推送...");
    let _ = app.emit("websocket-status", true);

    let (_sender, mut receiver) = ws_stream.split();

    while let Some(Ok(msg)) = receiver.next().await {
        if let Ok(text) = msg.to_text() {
            // 如果解析 JSON 成功，正常发给前端；
            // 如果解析失败（比如 JustSolo 发送的是纯文本格式），绝对不能丢弃！直接把原始文本发给前端！
            if let Ok(payload) = serde_json::from_str::<serde_json::Value>(text) {
                let _ = app.emit("websocket-lyrics", &payload);
            } else {
                let _ = app.emit("websocket-lyrics", text);
            }
        }
    }

    let _ = app.emit("websocket-status", false);
    Ok(())
}

#[command]
pub async fn start_websocket_lyrics(
    app: tauri::AppHandle,
    state: tauri::State<'_, crate::AppState>,
    url: Option<String>,
) -> Result<(), String> {
    let ws_url = url.unwrap_or_else(|| "ws://127.0.0.1:47290/".to_string());

    println!("[WebSocket 调试] 正在连接歌词服务器: {}", ws_url);

    let mut task_guard = state.ws_task.lock().await;
    if let Some(handle) = task_guard.take() {
        handle.abort();
    }

    let app_clone = app.clone();
    let app_err = app.clone();
    let handle = tokio::spawn(async move {
        if let Err(e) = run_websocket_lyrics(ws_url, app_clone).await {
            // JustSolo 服务没有启动时属于正常降级，不在终端制造“任务出错”噪音。
            // 其他连接错误才通知前端，方便真正的外部歌词服务排障。
            if !e.contains("os error 10061") {
                eprintln!("WebSocket 歌词服务异常: {}", e);
                let _ = app_err.emit("websocket-error", e);
            }
            let _ = app_err.emit("websocket-status", false);
        }
    });

    *task_guard = Some(handle);
    Ok(())
}

#[command]
pub async fn stop_websocket_lyrics(state: tauri::State<'_, crate::AppState>) -> Result<(), String> {
    let mut task_guard = state.ws_task.lock().await;
    if let Some(handle) = task_guard.take() {
        handle.abort();
    }
    Ok(())
}
