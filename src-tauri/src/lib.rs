use std::sync::Mutex;
use tauri::{State, Manager};
use sysinfo::{Networks, System};
use std::net::{SocketAddr, TcpStream};
use std::time::{Duration, Instant};
use tauri_plugin_autostart::MacosLauncher;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent, MouseButton};
use std::sync::atomic::{AtomicU32, AtomicBool, Ordering};

// --- WinAPI Imports ---
use winapi::um::winuser::{
    EnumWindows, GetClassNameW, GetWindowTextW, GetWindowThreadProcessId, IsWindowVisible,
    keybd_event, VK_MEDIA_NEXT_TRACK, VK_MEDIA_PLAY_PAUSE, VK_MEDIA_PREV_TRACK,
    VK_MENU, KEYEVENTF_KEYUP, SW_SHOWNORMAL,
};
use winapi::shared::windef::{HWND, RECT};
use winapi::shared::minwindef::{BOOL, LPARAM, TRUE, FALSE};
use std::os::windows::ffi::OsStrExt;
use winapi::um::shellapi::ShellExecuteW;

// --- Windows Crate Imports ---
use windows::Win32::Media::Audio::{
    eMultimedia, eRender, AudioSessionStateActive, IAudioSessionControl2, IAudioSessionManager2,
    IMMDeviceEnumerator, MMDeviceEnumerator, 
};
use windows::Win32::Media::Audio::Endpoints::IAudioMeterInformation;
use windows::Win32::System::Com::{CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED};
use windows::core::Interface;

static LAST_NOTIFICATION_ID: AtomicU32 = AtomicU32::new(0);
static IS_NOTIF_INIT: AtomicBool = AtomicBool::new(false);
// 👈 新增：记录应用最后一次发出声音的绝对时间
static LAST_AUDIO_TIME: Mutex<Option<Instant>> = Mutex::new(None); 

#[derive(serde::Serialize, Clone)]
pub struct ToastData {
    pub app_name: String,
    pub title: String,
    pub body: String,
    pub aumid: String,
}

struct MusicInfo {
    title: String,
    pid: u32,
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    if IsWindowVisible(hwnd) == 0 {
        return TRUE;
    }

    let mut class_name = [0u16; 256];
    let len = GetClassNameW(hwnd, class_name.as_mut_ptr(), class_name.len() as i32);
    let class_str = String::from_utf16_lossy(&class_name[..len as usize]);

    if class_str.contains("Orpheus") || class_str.contains("CloudMusic") {
        let mut title = [0u16; 512];
        let len = GetWindowTextW(hwnd, title.as_mut_ptr(), title.len() as i32);
        let title_str = String::from_utf16_lossy(&title[..len as usize]);
        let clean_title = title_str.trim_matches('\0').trim().to_string();

        if !clean_title.is_empty() && clean_title != "网易云音乐" && clean_title != "DesktopLyric" {
            let mut pid = 0;
            GetWindowThreadProcessId(hwnd, &mut pid);

            let info = &mut *(lparam as *mut MusicInfo);
            info.title = clean_title;
            info.pid = pid; 
            return FALSE; 
        }
    }
    TRUE 
}

// 核心重构：不仅查激活状态，更查“音轨分贝”！
unsafe fn is_process_playing_audio(target_pid: u32) -> bool {
    let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

    let enumerator: IMMDeviceEnumerator = match windows::Win32::System::Com::CoCreateInstance(
        &MMDeviceEnumerator, None, CLSCTX_ALL,
    ) {
        Ok(e) => e, Err(_) => return false,
    };

    let device = match enumerator.GetDefaultAudioEndpoint(eRender, eMultimedia) {
        Ok(d) => d, Err(_) => return false,
    };

    let manager: IAudioSessionManager2 = match device.Activate(CLSCTX_ALL, None) {
        Ok(m) => m, Err(_) => return false,
    };

    let session_enum = match manager.GetSessionEnumerator() {
        Ok(s) => s, Err(_) => return false,
    };

    let count = session_enum.GetCount().unwrap_or(0);
    let mut is_making_sound = false;

    for i in 0..count {
        if let Ok(session) = session_enum.GetSession(i) {
            if let Ok(session2) = session.cast::<IAudioSessionControl2>() {
                if let Ok(pid) = session2.GetProcessId() {
                    if pid == target_pid {
                        if let Ok(state) = session2.GetState() {
                            if state == AudioSessionStateActive {
                                // 终极杀招：直接把音频会话强制转换为“声卡分贝仪”
                                if let Ok(meter) = session.cast::<IAudioMeterInformation>() {
                                    // 修复：完美适配 windows 0.58.0 版本的返回值签名
                                    if let Ok(peak) = meter.GetPeakValue() {
                                        // 哪怕只有极其微弱的声音 (振幅 > 0.0001)，也说明在发声
                                        if peak > 0.0001 {
                                            is_making_sound = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if is_making_sound {
        // 更新最后一次发声时间
        if let Ok(mut last_time) = LAST_AUDIO_TIME.lock() {
            *last_time = Some(Instant::now());
        }
        return true;
    } else {
        // 如果此刻振幅跌至 0，给 2 秒的“静音宽限期”（防止歌曲本身的安静片段导致图标跳跃）
        if let Ok(last_time) = LAST_AUDIO_TIME.lock() {
            if let Some(time) = *last_time {
                if time.elapsed().as_secs() < 2 {
                    return true;
                }
            }
        }
    }
    false
}

#[tauri::command]
async fn fetch_netease_music_info() -> Result<Option<(String, String, bool)>, String> {
    let mut info = MusicInfo { title: String::new(), pid: 0 };
    
    unsafe {
        EnumWindows(Some(enum_windows_proc), &mut info as *mut _ as LPARAM);
    }

    if info.title.is_empty() {
        return Ok(None);
    }

    // 调用终极测音轨函数
    let is_playing = unsafe { is_process_playing_audio(info.pid) };

    let parts: Vec<&str> = info.title.splitn(2, " - ").collect();
    let song_name = parts[0].to_string();
    let artist_name = if parts.len() > 1 { parts[1].to_string() } else { "未知歌手".to_string() };

    Ok(Some((song_name, artist_name, is_playing)))
}

// ============== 下面的代码跟你原来一模一样，原封不动 ==============

#[tauri::command]
async fn control_system_media(action: String) -> Result<(), String> {
    unsafe {
        let vk = match action.as_str() {
            "play_pause" => VK_MEDIA_PLAY_PAUSE,
            "next" => VK_MEDIA_NEXT_TRACK,
            "prev" => VK_MEDIA_PREV_TRACK,
            _ => return Ok(()),
        };
        keybd_event(vk as u8, 0, 0, 0);
        keybd_event(vk as u8, 0, KEYEVENTF_KEYUP, 0);
    }
    Ok(())
}

#[tauri::command]
async fn get_random_cover_url(song_name: String, artist_name: String) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;

    let query = format!("{} {}", song_name, artist_name);
    let encoded_query = urlencoding::encode(&query);
    let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36";

    let netease_search_url = "https://music.163.com/api/search/get/web";
    if let Ok(resp) = client.post(netease_search_url)
        .header("Referer", "https://music.163.com")
        .header("User-Agent", ua)
        .form(&[
            ("s", query.as_str()),
            ("type", "1"),
            ("limit", "1"),
            ("offset", "0"),
        ])
        .send().await
    {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            if let Some(pic) = json.pointer("/result/songs/0/al/picUrl").and_then(|v| v.as_str()) {
                if !pic.is_empty() && pic != "http://p4.music.126.net/UeTuwE7pvjBpypWLudqukQ==/3135032972947607.jpg" {
                    return Ok(pic.replace("http://", "https://") + "?param=300y300");
                }
            }
        }
    }

    let deezer_url = format!(
        "https://api.deezer.com/search?q=track:\"{}\" artist:\"{}\"&limit=1",
        urlencoding::encode(&song_name),
        urlencoding::encode(&artist_name)
    );

    if let Ok(resp) = client.get(&deezer_url).header("User-Agent", ua).send().await {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            if let Some(cover) = json.pointer("/data/0/album/cover_medium").and_then(|v| v.as_str()) {
                if !cover.is_empty() { return Ok(cover.to_string()); }
            }
            if let Some(cover) = json.pointer("/data/0/album/cover_big").and_then(|v| v.as_str()) {
                if !cover.is_empty() { return Ok(cover.to_string()); }
            }
        }
    }

    let itunes_url = format!("https://itunes.apple.com/search?term={}&media=music&limit=1", encoded_query);
    if let Ok(resp) = client.get(&itunes_url).send().await {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            if let Some(artwork) = json.pointer("/results/0/artworkUrl100").and_then(|v| v.as_str()) {
                return Ok(artwork.replace("100x100bb", "300x300bb"));
            }
        }
    }

    Ok("data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTUwIiBoZWlnaHQ9IjE1MCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48ZGVmcz48bGluZWFyR3JhZGllbnQgaWQ9ImciIHgxPSIwJSIgeTE9IjAlIiB4Mj0iMTAwJSIgeTI9IjEwMCUiPjxzdG9wIG9mZnNldD0iMCUiIHN0b3AtY29sb3I9IiNhOGVkZWEiLz48c3RvcCBvZmZzZXQ9IjEwMCUiIHN0b3AtY29sb3I9IiNmZWQ2ZTMiLz48L2xpbmVhckdyYWRpZW50PjwvZGVmcz48cmVjdCB3aWR0aD0iMTUwIiBoZWlnaHQ9IjE1MCIgcng9Ijc1IiBmaWxsPSJ1cmwoI2cpIi8+PC9zdmc+".to_string())
}

#[tauri::command]
async fn fetch_latest_notification() -> Result<Option<ToastData>, String> {
    use windows::UI::Notifications::Management::UserNotificationListener;
    use windows::UI::Notifications::NotificationKinds;

    let listener = match UserNotificationListener::Current() {
        Ok(l) => l,
        Err(_) => return Ok(None),
    };

    let _ = listener.RequestAccessAsync();

    let notifications = match listener.GetNotificationsAsync(NotificationKinds::Toast) {
        Ok(op) => match op.get() {
            Ok(ns) => ns,
            Err(_) => return Ok(None),
        },
        Err(_) => return Ok(None),
    };

    let mut latest_notif = None;
    let mut max_id = 0u32;

    for notif in notifications {
        if let Ok(id) = notif.Id() {
            if id > max_id {
                max_id = id;
                latest_notif = Some(notif);
            }
        }
    }

    if max_id == 0 { return Ok(None); }

    let last_processed_id = LAST_NOTIFICATION_ID.load(Ordering::SeqCst);

    if !IS_NOTIF_INIT.load(Ordering::SeqCst) {
        LAST_NOTIFICATION_ID.store(max_id, Ordering::SeqCst);
        IS_NOTIF_INIT.store(true, Ordering::SeqCst);
        return Ok(None);
    }

    if max_id > last_processed_id {
        LAST_NOTIFICATION_ID.store(max_id, Ordering::SeqCst);

        if let Some(notif) = latest_notif {
            let app_name = notif.AppInfo()
                .and_then(|info| info.DisplayInfo())
                .and_then(|dinfo| dinfo.DisplayName())
                .map(|name| name.to_string())
                .unwrap_or_else(|_| "系统通知".to_string());

            let aumid = notif.AppInfo()
                .and_then(|info| info.AppUserModelId())
                .map(|id| id.to_string())
                .unwrap_or_default();

            if let Ok(toast_binding) = notif
                .Notification()
                .and_then(|n| n.Visual())
                .and_then(|v| v.GetBinding(&windows::core::HSTRING::from("ToastGeneric")))
            {
                if let Ok(text_elements) = toast_binding.GetTextElements() {
                    let mut text_list = Vec::new();
                    for elem in text_elements {
                        if let Ok(text) = elem.Text() {
                            text_list.push(text.to_string());
                        }
                    }

                    if !text_list.is_empty() {
                        let title = text_list.first().cloned().unwrap_or_default();
                        let body = if text_list.len() > 1 {
                            text_list[1..].join(" ")
                        } else {
                            String::new()
                        };

                        if title.contains("微信") || title.contains("WeChat") || body.contains("微信") || body.contains("WeChat") {
                            return Ok(None);
                        }

                        return Ok(Some(ToastData { app_name, title, body, aumid }));
                    }
                }
            }
        }
    }

    Ok(None)
}

#[tauri::command]
fn open_app_by_aumid(aumid: String, app_name: String) {
    let app_lower = app_name.to_lowercase();
    
    unsafe {
        keybd_event(VK_MENU as u8, 0, 0, 0);
        keybd_event(VK_MENU as u8, 0, KEYEVENTF_KEYUP, 0);
    }
    
    let execute_protocol = |protocol: &str| {
        unsafe {
            let op = std::ffi::OsStr::new("open").encode_wide().chain(Some(0)).collect::<Vec<u16>>();
            let file = std::ffi::OsStr::new(protocol).encode_wide().chain(Some(0)).collect::<Vec<u16>>();
            ShellExecuteW(
                std::ptr::null_mut(),
                op.as_ptr(),
                file.as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
                SW_SHOWNORMAL,
            );
        }
    };

    if app_lower.contains("qq") {
        execute_protocol("tencent://message/");
    } else if app_lower.contains("微信") || app_lower.contains("wechat") {
        execute_protocol("weixin://");
    } else if app_lower.contains("钉钉") || app_lower.contains("dingtalk") {
        execute_protocol("dingtalk://");
    } else if !aumid.is_empty() {
        unsafe {
            let op = std::ffi::OsStr::new("open").encode_wide().chain(Some(0)).collect::<Vec<u16>>();
            let file = std::ffi::OsStr::new("explorer.exe").encode_wide().chain(Some(0)).collect::<Vec<u16>>();
            let params = std::ffi::OsStr::new(&format!("shell:AppsFolder\\{}", aumid)).encode_wide().chain(Some(0)).collect::<Vec<u16>>();
            ShellExecuteW(
                std::ptr::null_mut(),
                op.as_ptr(),
                file.as_ptr(),
                params.as_ptr(),
                std::ptr::null(),
                SW_SHOWNORMAL,
            );
        }
    }
}

#[tauri::command]
fn force_window_topmost(app: tauri::AppHandle) {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            let fg_hwnd = winapi::um::winuser::GetForegroundWindow();
            if !fg_hwnd.is_null() {
                let mut class_name = [0u16; 256];
                let len = winapi::um::winuser::GetClassNameW(fg_hwnd, class_name.as_mut_ptr(), class_name.len() as i32);
                let class_str = String::from_utf16_lossy(&class_name[..len as usize]);
                
                if class_str == "#32768" { return; }

                let mut rect: RECT = std::mem::zeroed();
                winapi::um::winuser::GetWindowRect(fg_hwnd, &mut rect);

                let monitor = winapi::um::winuser::MonitorFromWindow(fg_hwnd, winapi::um::winuser::MONITOR_DEFAULTTONEAREST);
                let mut mi: winapi::um::winuser::MONITORINFO = std::mem::zeroed();
                mi.cbSize = std::mem::size_of::<winapi::um::winuser::MONITORINFO>() as u32;
                winapi::um::winuser::GetMonitorInfoW(monitor, &mut mi);

                if rect.left == mi.rcMonitor.left && rect.top == mi.rcMonitor.top && rect.right == mi.rcMonitor.right && rect.bottom == mi.rcMonitor.bottom {
                    if class_str != "Progman" && class_str != "WorkerW" {
                        return; 
                    }
                }
            }

            if let Some(win) = app.get_webview_window("widget") {
                if let Ok(hwnd) = win.hwnd() {
                    winapi::um::winuser::SetWindowPos(hwnd.0 as _, -1isize as _, 0, 0, 0, 0, 19);
                }
            }
        }
    }
}

// 新增：底层原子化窗口调整指令，彻底消除位移闪烁
#[tauri::command]
fn set_window_bounds(app: tauri::AppHandle, x: i32, y: i32, width: i32, height: i32) {
    #[cfg(target_os = "windows")]
    {
        if let Some(win) = app.get_webview_window("widget") {
            if let Ok(hwnd) = win.hwnd() {
                unsafe {
                    // 0x0014 = SWP_NOACTIVATE (0x0010) | SWP_NOZORDER (0x0004)
                    // 确保同时修改坐标和尺寸时，不抢占用户焦点，不打乱窗口层级
                    winapi::um::winuser::SetWindowPos(
                        hwnd.0 as _,
                        std::ptr::null_mut(),
                        x, y, width, height,
                        0x0014,
                    );
                }
            }
        }
    }
}

struct AppState {
    networks: Mutex<Networks>,
    system: Mutex<System>,
}

#[tauri::command]
fn get_hardware_stats(state: State<'_, AppState>) -> (f32, u64, u64) {
    let mut sys = state.system.lock().unwrap();
    sys.refresh_cpu_usage(); 
    sys.refresh_memory();    
    (sys.global_cpu_info().cpu_usage(), sys.used_memory(), sys.total_memory())
}

#[tauri::command]
fn get_network_stats(state: State<'_, AppState>) -> (u64, u64) {
    let mut networks = state.networks.lock().unwrap();
    networks.refresh_list();

    let mut total_rx = 0;
    let mut total_tx = 0;

    for (_interface_name, data) in networks.iter() {
        total_rx += data.total_received();
        total_tx += data.total_transmitted();
    }

    (total_rx, total_tx)
}

#[tauri::command]
fn get_network_latency() -> Result<u128, String> {
    let addr: SocketAddr = "223.5.5.5:53".parse().unwrap();
    let timeout = Duration::from_millis(1500);

    let start = Instant::now();
    match TcpStream::connect_timeout(&addr, timeout) {
        Ok(_) => Ok(start.elapsed().as_millis()),
        Err(_) => Err("Timeout".to_string()),
    }
}

#[tauri::command]
fn is_widget_visible(app: tauri::AppHandle) -> bool {
    match app.get_webview_window("widget") {
        Some(win) => win.is_visible().unwrap_or(false),
        None => false,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let networks = Networks::new_with_refreshed_list();
    let mut system = System::new_all();
    system.refresh_cpu_usage();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--autostart"])))
        .manage(AppState { networks: Mutex::new(networks), system: Mutex::new(system) })
        .invoke_handler(tauri::generate_handler![
            get_network_stats,
            is_widget_visible,
            get_network_latency,
            fetch_netease_music_info,
            control_system_media,
            get_random_cover_url,
            fetch_latest_notification,
            get_hardware_stats,
            open_app_by_aumid,
            force_window_topmost,
            set_window_bounds,
        ])
        .setup(|app| {
            let args: Vec<String> = std::env::args().collect();
            let is_autostart = args.iter().any(|arg| arg == "--autostart");

            if let Some(main_window) = app.get_webview_window("main") {
                if !is_autostart {
                    let _ = main_window.show();
                    let _ = main_window.set_focus();
                }
            }

            let quit_item = MenuItem::with_id(app, "quit", "强制退出", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&quit_item])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("NetSpeed Dynamic Pro")
                .menu(&tray_menu)
                .on_menu_event(move |_app_handle, event| {
                    if event.id == "quit" { std::process::exit(0); }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button: MouseButton::Left, .. } = event {
                        if let Some(main_window) = tray.app_handle().get_webview_window("main") {
                            let _ = main_window.show();     
                            let _ = main_window.unminimize(); 
                            let _ = main_window.set_focus();  
                        }
                    }
                })
                .build(app)?;

            if let Some(main_window) = app.get_webview_window("main") {
                let w_clone = main_window.clone();
                main_window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = w_clone.hide();
                    }
                });
            }

            if let Some(widget_window) = app.get_webview_window("widget") {
                #[cfg(target_os = "windows")]
                {
                    use windows_sys::Win32::Graphics::Dwm::{
                        DwmSetWindowAttribute, DWMWA_WINDOW_CORNER_PREFERENCE, DWMWA_BORDER_COLOR, DWMWCP_DONOTROUND,
                    };
                    use windows_sys::Win32::UI::WindowsAndMessaging::{SetWindowLongPtrW, GWL_STYLE, WS_CAPTION};
                    use windows_sys::Win32::Foundation::HWND;

                    if let Ok(hwnd) = widget_window.hwnd() {
                        let hwnd_raw = hwnd.0 as HWND;
                        unsafe {
                            let current_style = windows_sys::Win32::UI::WindowsAndMessaging::GetWindowLongPtrW(hwnd_raw, GWL_STYLE);
                            SetWindowLongPtrW(hwnd_raw, GWL_STYLE, current_style & !(WS_CAPTION as isize));

                            let border_color: u32 = 0xFFFFFFFE;
                            let _ = DwmSetWindowAttribute(hwnd_raw, DWMWA_BORDER_COLOR as u32, &border_color as *const _ as *const _, 4);

                            let corner_preference = DWMWCP_DONOTROUND;
                            let _ = DwmSetWindowAttribute(hwnd_raw, DWMWA_WINDOW_CORNER_PREFERENCE as u32, &corner_preference as *const _ as *const _, 4);
                        }
                    }
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}