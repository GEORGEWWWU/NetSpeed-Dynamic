use std::sync::Mutex;
use tauri::command;

// --- 引入 SMTC 需要的模块 ---
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus,
    GlobalSystemMediaTransportControlsSession,
};

// 全局记录当前选中的平台（默认空，由前端传来）
static TARGET_PLAYER: Mutex<String> = Mutex::new(String::new());

// 给前端调用的切换接口
#[command]
pub fn set_target_player(player: String) {
    if let Ok(mut target) = TARGET_PLAYER.lock() {
        *target = player;
    }
}

// 自动匹配你选择的软件
fn get_target_media_session() -> Option<GlobalSystemMediaTransportControlsSession> {
    let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
        .ok()?.get().ok()?;
    
    let sessions = manager.GetSessions().ok()?;

    // 获取当前的目标（前端如果还没传，默认用 netease）
    let target = {
        let guard = TARGET_PLAYER.lock().unwrap_or_else(|e| e.into_inner()); // 加入防中毒
        if guard.is_empty() { "netease".to_string() } else { guard.clone() }
    };

    for session in sessions {
        if target == "smtc" {
            // SMTC 模式：返回第一个有活跃媒体的会话（不限制来源）
            if let Ok(playback) = session.GetPlaybackInfo() {
                if let Ok(status) = playback.PlaybackStatus() {
                    if status != GlobalSystemMediaTransportControlsSessionPlaybackStatus::Stopped {
                        return Some(session);
                    }
                }
            }
            continue;
        }

        if let Ok(app_id) = session.SourceAppUserModelId() {
            let app_id_str = app_id.to_string().to_lowercase();
            
            // 网易云特殊一点，包名可能叫 cloudmusic 或 netease
            if target == "netease" && (app_id_str.contains("cloudmusic") || app_id_str.contains("netease")) {
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

#[command]
pub async fn fetch_netease_music_info() -> Result<Option<(String, String, bool)>, String> {
    let session = match get_target_media_session() {
        Some(s) => s,
        None => return Ok(None),
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

    let properties = session.TryGetMediaPropertiesAsync()
        .map_err(|e| e.to_string())?
        .get()
        .map_err(|e| e.to_string())?;

    let title = properties.Title().unwrap_or_default().to_string();
    let artist = properties.Artist().unwrap_or_default().to_string();

    if title.is_empty() {
        return Ok(None);
    }

    Ok(Some((title, artist, is_playing)))
}

#[command]
pub async fn control_system_media(action: String) -> Result<(), String> {
    if let Some(session) = get_target_media_session() {
        match action.as_str() {
            "play_pause" => { let _ = session.TryTogglePlayPauseAsync(); },
            "next" => { let _ = session.TrySkipNextAsync(); },
            "prev" => { let _ = session.TrySkipPreviousAsync(); },
            _ => {}
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
                result.push(CHARSET[(((chunk[1] & 0x0F) << 2)) as usize] as char);
                result.push('=');
            }
            1 => {
                result.push(CHARSET[(chunk[0] >> 2) as usize] as char);
                result.push(CHARSET[(((chunk[0] & 0x03) << 4)) as usize] as char);
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
    use windows::Storage::Streams::{Buffer, InputStreamOptions, DataReader};

    let session = get_target_media_session()?;
    let properties = session.TryGetMediaPropertiesAsync().ok()?.get().ok()?;
    let thumbnail_ref = properties.Thumbnail().ok()?;
    let stream = thumbnail_ref.OpenReadAsync().ok()?.get().ok()?;
    let size = stream.Size().ok()? as u32;
    if size == 0 { return None; }

    let buffer = Buffer::Create(size).ok()?;
    stream.ReadAsync(&buffer, size, InputStreamOptions::None).ok()?.get().ok()?;
    let reader = DataReader::FromBuffer(&buffer).ok()?;
    let mut bytes = vec![0u8; size as usize];
    reader.ReadBytes(&mut bytes).ok()?;

    Some(format!("data:image/jpeg;base64,{}", inline_base64_encode(&bytes)))
}

#[command]
pub async fn get_random_cover_url(song_name: String, artist_name: String) -> Result<String, String> {
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
        let itunes_url = format!("https://itunes.apple.com/search?term={}&media=music&limit=1", encoded_query);
        if let Ok(resp) = client_itunes.get(&itunes_url).send().await {
            if let Ok(json) = resp.json::<serde_json::Value>().await {
                if let Some(artwork) = json.pointer("/results/0/artworkUrl100").and_then(|v| v.as_str()) {
                    let _ = tx_itunes.send(artwork.replace("100x100bb", "300x300bb")).await;
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
        if let Ok(resp) = client_netease.post("https://music.163.com/api/search/get/web")
            .header("Referer", "https://music.163.com")
            .header("User-Agent", ua)
            .form(&[("s", query.as_str()), ("type", "1"), ("limit", "1"), ("offset", "0")])
            .send().await
        {
            if let Ok(json) = resp.json::<serde_json::Value>().await {
                if let Some(pic) = json.pointer("/result/songs/0/al/picUrl").and_then(|v| v.as_str()) {
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
        if let Ok(resp) = client_deezer.get(&deezer_url).header("User-Agent", ua).send().await {
            if let Ok(json) = resp.json::<serde_json::Value>().await {
                if let Some(cover) = json.pointer("/data/0/album/cover_medium").and_then(|v| v.as_str()) {
                    if !cover.is_empty() { let _ = tx_deezer.send(cover.to_string()).await; }
                } else if let Some(cover) = json.pointer("/data/0/album/cover_big").and_then(|v| v.as_str()) {
                    if !cover.is_empty() { let _ = tx_deezer.send(cover.to_string()).await; }
                }
            }
        }
    });

    match tokio::time::timeout(std::time::Duration::from_secs(3), rx.recv()).await {
        Ok(Some(url)) => Ok(url),
        _ => Ok("data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTUwIiBoZWlnaHQ9IjE1MCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48ZGVmcz48bGluZWFyR3JhZGllbnQgaWQ9ImciIHgxPSIwJSIgeTE9IjAlIiB4Mj0iMTAwJSIgeTI9IjEwMCUiPjxzdG9wIG9mZnNldD0iMCUiIHN0b3AtY29sb3I9IiNhOGVkZWEiLz48c3RvcCBvZmZzZXQ9IjEwMCUiIHN0b3AtY29sb3I9IiNmZWQ2ZTMiLz48L2xpbmVhckdyYWRpZW50PjwvZGVmcz48cmVjdCB3aWR0aD0iMTUwIiBoZWlnaHQ9IjE1MCIgcng9Ijc1IiBmaWxsPSJ1cmwoI2cpIi8+PC9zdmc+".to_string()),
    }
}