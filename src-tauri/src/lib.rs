use std::sync::Mutex;
use tauri::{State, Manager};
use sysinfo::{Networks, System};
use std::net::{SocketAddr, TcpStream};
use std::time::{Duration, Instant};
use tauri_plugin_autostart::MacosLauncher;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent, MouseButton};
use winapi::um::winuser::{EnumWindows, GetWindowTextW, IsWindowVisible, GetClassNameW};
use winapi::shared::windef::HWND;
use std::sync::atomic::{AtomicU32, AtomicBool, Ordering};
static LAST_NOTIFICATION_ID: AtomicU32 = AtomicU32::new(0);
static IS_NOTIF_INIT: AtomicBool = AtomicBool::new(false);

// 结构体：用于在窗口枚举中传递和存储找到的歌词/音乐信息
struct MusicInfo {
    title: String,
}

#[derive(serde::Serialize, Clone)]
pub struct ToastData {
    pub app_name: String,
    pub title: String,
    pub body: String,
    pub aumid: String,
}

// 外部枚举的回调函数
unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: winapi::shared::minwindef::LPARAM) -> winapi::shared::minwindef::BOOL {
    if IsWindowVisible(hwnd) == 0 {
        return 1; // 继续寻找
    }

    // 获取类名
    let mut class_name = [0u16; 256];
    GetClassNameW(hwnd, class_name.as_mut_ptr(), class_name.len() as i32);
    let class_str = String::from_utf16_lossy(&class_name);

    // 网易云音乐的主窗口类名通常包含 "Orpheus" 或由其内核派生
    // 同时也支持直接匹配标题，双重保险
    if class_str.contains("Orpheus") || class_str.contains("CloudMusic") {
        let mut title = [0u16; 512];
        GetWindowTextW(hwnd, title.as_mut_ptr(), title.len() as i32);
        let title_str = String::from_utf16_lossy(&title);
        let clean_title = title_str.trim_matches('\0').trim().to_string();

        // 排除掉未播放状态或者辅助窗口
        if !clean_title.is_empty() && clean_title != "网易云音乐" && clean_title != "DesktopLyric" {
            let info = &mut *(lparam as *mut MusicInfo);
            info.title = clean_title;
            return 0; // 找到目标，停止枚举
        }
    }
    1 // 继续枚举
}

#[tauri::command]
async fn get_random_cover_url(song_name: String, artist_name: String) -> Result<String, String> {
    // 构建带超时的 Client，防止请求卡死导致前端一直等待
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;

    let query = format!("{} {}", song_name, artist_name);
    let encoded_query = urlencoding::encode(&query);

    // 公共 User-Agent，网易云会校验 UA
    let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36";

    // ========== 【数据源 1】网易云搜索接口 (POST 方式，修正后最精准) ==========
    // 关键修正：此接口必须用 POST + 表单提交，GET 方式经常返回空结果！
    let netease_search_url = "https://music.163.com/api/search/get/web";

    if let Ok(resp) = client.post(netease_search_url)
        .header("Referer", "https://music.163.com")
        .header("User-Agent", ua)
        .form(&[
            ("s", query.as_str()),
            ("type", "1"),       // 1=歌曲
            ("limit", "1"),
            ("offset", "0"),
        ])
        .send().await
    {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            if let Some(pic) = json.pointer("/result/songs/0/al/picUrl").and_then(|v| v.as_str()) {
                if !pic.is_empty() && pic != "http://p4.music.126.net/UeTuwE7pvjBpypWLudqukQ==/3135032972947607.jpg" {
                    // 排除网易云默认的无封面占位图
                    return Ok(pic.replace("http://", "https://") + "?param=300y300");
                }
            }
        }
    }

    // ========== 【数据源 2】Deezer API (极其稳定，中文歌支持优秀) ==========
    // Deezer 公开搜索接口，无需鉴权，返回专辑封面 cover_medium (250x250) / cover_big (500x500)
    let deezer_url = format!(
        "https://api.deezer.com/search?q=track:\"{}\" artist:\"{}\"&limit=1",
        urlencoding::encode(&song_name),
        urlencoding::encode(&artist_name)
    );

    if let Ok(resp) = client.get(&deezer_url)
        .header("User-Agent", ua)
        .send().await
    {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            // 优先取 cover_medium (250x250)，清晰度足够灵动岛使用
            if let Some(cover) = json.pointer("/data/0/album/cover_medium").and_then(|v| v.as_str()) {
                if !cover.is_empty() {
                    return Ok(cover.to_string());
                }
            }
            // 备选：取 cover_big (500x500)
            if let Some(cover) = json.pointer("/data/0/album/cover_big").and_then(|v| v.as_str()) {
                if !cover.is_empty() {
                    return Ok(cover.to_string());
                }
            }
        }
    }

    // ========== 【数据源 3】iTunes Search API (稳定备用方案) ==========
    let itunes_url = format!(
        "https://itunes.apple.com/search?term={}&media=music&limit=1",
        encoded_query
    );

    if let Ok(resp) = client.get(&itunes_url).send().await {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            if let Some(artwork) = json.pointer("/results/0/artworkUrl100").and_then(|v| v.as_str()) {
                // iTunes 返回的是 100x100，替换字符串获取更高清的版本
                return Ok(artwork.replace("100x100bb", "300x300bb"));
            }
        }
    }

    // ========== 【数据源 4】终极保底：返回纯色渐变 SVG (永不失败) ==========
    // 不再依赖外部图片服务（Unsplash 在国内可能加载慢或被墙）
    // 使用内联 Base64 SVG 作为绝对兜底，保证永远有图显示
    Ok("data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTUwIiBoZWlnaHQ9IjE1MCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48ZGVmcz48bGluZWFyR3JhZGllbnQgaWQ9ImciIHgxPSIwJSIgeTE9IjAlIiB4Mj0iMTAwJSIgeTI9IjEwMCUiPjxzdG9wIG9mZnNldD0iMCUiIHN0b3AtY29sb3I9IiNhOGVkZWEiLz48c3RvcCBvZmZzZXQ9IjEwMCUiIHN0b3AtY29sb3I9IiNmZWQ2ZTMiLz48L2xpbmVhckdyYWRpZW50PjwvZGVmcz48cmVjdCB3aWR0aD0iMTUwIiBoZWlnaHQ9IjE1MCIgcng9Ijc1IiBmaWxsPSJ1cmwoI2cpIi8+PC9zdmc+".to_string())
}

#[tauri::command]
fn fetch_netease_music_info() -> Result<Option<(String, String, bool)>, String> {
    let mut music_info = MusicInfo { title: String::new() };
    
    unsafe {
        EnumWindows(Some(enum_windows_proc), &mut music_info as *mut _ as winapi::shared::minwindef::LPARAM);
    }

    if music_info.title.is_empty() {
        return Ok(None); // 没有检测到正在播放的网易云
    }

    // 格式通常是: "歌名 - 歌手"
    let parts: Vec<&str> = music_info.title.splitn(2, " - ").collect();
    if parts.len() == 2 {
        let song_name = parts[0].trim().to_string();
        let artist_name = parts[1].trim().to_string();
        Ok(Some((song_name, artist_name, true))) // 返回 歌名, 歌手, 是否在播放
    } else {
        // 有可能某些特殊歌名没有 - 分隔
        Ok(Some((music_info.title, "未知歌手".to_string(), true)))
    }
}

// 模拟多媒体控制指令（发送全局系统媒体按键：最稳定、免去绑定特定进程）
#[tauri::command]
fn control_system_media(action: String) {
    use winapi::um::winuser::{keybd_event, VK_MEDIA_NEXT_TRACK, VK_MEDIA_PLAY_PAUSE, VK_MEDIA_PREV_TRACK};
    unsafe {
        let vk = match action.as_str() {
            "play_pause" => VK_MEDIA_PLAY_PAUSE,
            "next" => VK_MEDIA_NEXT_TRACK,
            "prev" => VK_MEDIA_PREV_TRACK,
            _ => return,
        };
        keybd_event(vk as u8, 0, 0, 0);
        keybd_event(vk as u8, 0, 2, 0); // key up
    }
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

    if max_id == 0 {
        return Ok(None);
    }

    let last_processed_id = LAST_NOTIFICATION_ID.load(Ordering::SeqCst);

    if !IS_NOTIF_INIT.load(Ordering::SeqCst) {
        LAST_NOTIFICATION_ID.store(max_id, Ordering::SeqCst);
        IS_NOTIF_INIT.store(true, Ordering::SeqCst);
        return Ok(None);
    }

    if max_id > last_processed_id {
        LAST_NOTIFICATION_ID.store(max_id, Ordering::SeqCst);

        if let Some(notif) = latest_notif {
            // 1. 【核心修复】通过 AppInfo 获取精准的程序名称
            let app_name = notif.AppInfo()
                .and_then(|info| info.DisplayInfo())
                .and_then(|dinfo| dinfo.DisplayName())
                .map(|name| name.to_string())
                .unwrap_or_else(|_| "系统通知".to_string());

            // 👇新增：获取程序的 AUMID
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
                        // 2. 分离标题和内容
                        let title = text_list.first().cloned().unwrap_or_default();
                        let body = if text_list.len() > 1 {
                            text_list[1..].join(" ")
                        } else {
                            String::new()
                        };

                        // 3. 过滤微信（保留你原有的逻辑）
                        if title.contains("微信")
                            || title.contains("WeChat")
                            || body.contains("微信")
                            || body.contains("WeChat")
                        {
                            return Ok(None);
                        }

                        // 4. 返回打包好的数据
                        return Ok(Some(ToastData {
                            app_name,
                            title,
                            body,
                            aumid,
                        }));
                    }
                }
            }
        }
    }

    Ok(None)
}

use std::os::windows::ffi::OsStrExt;
use winapi::um::shellapi::ShellExecuteW;
use winapi::um::winuser::{keybd_event, VK_MENU, KEYEVENTF_KEYUP, SW_SHOWNORMAL};

#[tauri::command]
fn open_app_by_aumid(aumid: String, app_name: String) {
    println!("👉 收到点击！准备唤醒程序: app_name='{}', aumid='{}'", app_name, aumid);

    let app_lower = app_name.to_lowercase();
    
    // 【核心黑魔法：底层破解 Windows 焦点防劫持】
    // 极速模拟按一下 Alt 键并松开（0 延迟，0 闪烁）。
    // 这能骗过系统，让它以为用户正在敲击键盘，直接下发“允许前台弹窗”的权限，根治任务栏闪烁不弹窗的问题。
    unsafe {
        keybd_event(VK_MENU as u8, 0, 0, 0);
        keybd_event(VK_MENU as u8, 0, KEYEVENTF_KEYUP, 0);
    }
    
    // 封装一个干净、底层的协议执行器，彻底告别 CMD 和 PowerShell
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
        // 让 QQ 自己的协议管理器去平滑唤醒托盘进程，不再用 Win32 强拉避免黑屏卡死
        execute_protocol("tencent://message/");
    } else if app_lower.contains("微信") || app_lower.contains("wechat") {
        execute_protocol("weixin://");
    } else if app_lower.contains("钉钉") || app_lower.contains("dingtalk") {
        execute_protocol("dingtalk://");
    } else if !aumid.is_empty() {
        // UWP 应用也直接走底层的 ShellExecute，绝不生成任何控制台黑窗
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
            // 1. 优雅避让：检测当前是否存在右键菜单 或 【全屏游戏】
            let fg_hwnd = winapi::um::winuser::GetForegroundWindow();
            if !fg_hwnd.is_null() {
                let mut class_name = [0u16; 256];
                let len = winapi::um::winuser::GetClassNameW(fg_hwnd, class_name.as_mut_ptr(), class_name.len() as i32);
                let class_str = String::from_utf16_lossy(&class_name[..len as usize]);
                
                // #32768 是 Windows 系统标准右键菜单的底层类名
                if class_str == "#32768" {
                    return; // 如果发现前台正在展示右键菜单，立刻收手不置顶，保护菜单不被遮挡！
                }

                // ================= 新增核心修复：全屏防闪退检测 =================
                // 获取当前最前台窗口的坐标和尺寸
                let mut rect: winapi::shared::windef::RECT = std::mem::zeroed();
                winapi::um::winuser::GetWindowRect(fg_hwnd, &mut rect);

                // 获取该窗口所在的物理显示器尺寸
                let monitor = winapi::um::winuser::MonitorFromWindow(fg_hwnd, winapi::um::winuser::MONITOR_DEFAULTTONEAREST);
                let mut mi: winapi::um::winuser::MONITORINFO = std::mem::zeroed();
                mi.cbSize = std::mem::size_of::<winapi::um::winuser::MONITORINFO>() as u32;
                winapi::um::winuser::GetMonitorInfoW(monitor, &mut mi);

                // 判断前台窗口的尺寸是否和显示器物理尺寸完全一致（即：是不是全屏）
                if rect.left == mi.rcMonitor.left &&
                   rect.top == mi.rcMonitor.top &&
                   rect.right == mi.rcMonitor.right &&
                   rect.bottom == mi.rcMonitor.bottom {
                    
                    // 排除 Windows 自己的桌面壁纸层（桌面本身也是全屏，但此时需要置顶）
                    if class_str != "Progman" && class_str != "WorkerW" {
                        // 判定为正在运行全屏游戏或全屏视频！立刻终止置顶操作，放过焦点！
                        return; 
                    }
                }
                // ===============================================================
            }

            // 2. 静默置顶：彻底解决抢占焦点的问题
            if let Some(win) = app.get_webview_window("widget") {
                if let Ok(hwnd) = win.hwnd() {
                    winapi::um::winuser::SetWindowPos(
                        hwnd.0 as _,
                        -1isize as _, 
                        0, 0, 0, 0,
                        // 19 = SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOSIZE
                        19, 
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
    sys.refresh_cpu_usage(); // 刷新CPU，获取精准增量
    sys.refresh_memory();    // 刷新内存
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
        Ok(_) => {
            let elapsed = start.elapsed().as_millis();
            Ok(elapsed)
        }
        Err(_) => Err("Timeout or disconnected".to_string()),
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
        // 1. 【唯一实例保证】初始化单实例插件。如果重复启动，直接退出进程
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--autostart"]),
        ))
        .manage(AppState {
            networks: Mutex::new(networks),
            system: Mutex::new(system),
        })
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
        ])
        .setup(|app| {
            // --- 新增：处理静默启动逻辑 ---
            // 获取应用的启动命令行参数
            let args: Vec<String> = std::env::args().collect();
            let is_autostart = args.iter().any(|arg| arg == "--autostart");

            // 获取主窗口
            if let Some(main_window) = app.get_webview_window("main") {
                if !is_autostart {
                    // 如果不是开机自启（即用户手动双击快捷方式），则主动显示主窗口
                    let _ = main_window.show();
                    let _ = main_window.set_focus();
                }
                // 如果是开机自启，什么都不做，窗口会保持在 tauri.conf.json 中设置的隐藏状态
            }

            // 2. 【系统托盘右键菜单】仅创建一个 "强制退出" 按钮
            let quit_item = MenuItem::with_id(app, "quit", "强制退出", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&quit_item])?;

            // 3. 【构建系统托盘】
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone()) // 使用默认图标
                .tooltip("NetSpeed Dynamic Pro")
                .menu(&tray_menu)
                .on_menu_event(move |_app_handle, event| {
                    if event.id == "quit" {
                        // 点击"强制退出"时：强杀进程完全退出
                        std::process::exit(0);
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    // 当点击系统托盘图标时
                    if let TrayIconEvent::Click { button: MouseButton::Left, .. } = event {
                        let app_handle = tray.app_handle();
                        // 寻找主控制台窗口
                        if let Some(main_window) = app_handle.get_webview_window("main") {
                            let _ = main_window.show();     // 显示窗口
                            let _ = main_window.unminimize(); // 取消最小化
                            let _ = main_window.set_focus();  // 聚焦窗口
                        }
                    }
                })
                .build(app)?;

            // 4. 【拦截控制台关闭事件】使其点击关闭时隐藏而不是真的退出
            if let Some(main_window) = app.get_webview_window("main") {
                let w_clone = main_window.clone();
                main_window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        // 阻止窗口真正关闭
                        api.prevent_close();
                        // 使用克隆的窗口句柄来将其隐藏
                        let _ = w_clone.hide();
                    }
                });
            }

            // 5. 灵动岛(Widget)的原有 Windows 样式裁剪逻辑
            if let Some(widget_window) = app.get_webview_window("widget") {
                #[cfg(target_os = "windows")]
                {
                    use windows_sys::Win32::Graphics::Dwm::{
                        DwmSetWindowAttribute,
                        DWMWA_WINDOW_CORNER_PREFERENCE,
                        DWMWA_BORDER_COLOR,
                        DWMWCP_DONOTROUND,
                    };
                    use windows_sys::Win32::UI::WindowsAndMessaging::{
                        SetWindowLongPtrW,
                        GWL_STYLE,
                        WS_CAPTION,
                    };
                    use windows_sys::Win32::Foundation::HWND;

                    if let Ok(hwnd) = widget_window.hwnd() {
                        let hwnd_raw = hwnd.0 as HWND;
                        unsafe {
                            let current_style = windows_sys::Win32::UI::WindowsAndMessaging::GetWindowLongPtrW(hwnd_raw, GWL_STYLE);
                            SetWindowLongPtrW(hwnd_raw, GWL_STYLE, current_style & !(WS_CAPTION as isize));

                            let border_color: u32 = 0xFFFFFFFE;
                            let _ = DwmSetWindowAttribute(
                                hwnd_raw,
                                DWMWA_BORDER_COLOR as u32,
                                &border_color as *const _ as *const _,
                                std::mem::size_of::<u32>() as u32,
                            );

                            let corner_preference = DWMWCP_DONOTROUND;
                            let _ = DwmSetWindowAttribute(
                                hwnd_raw,
                                DWMWA_WINDOW_CORNER_PREFERENCE as u32,
                                &corner_preference as *const _ as *const _,
                                std::mem::size_of::<i32>() as u32,
                            );
                        }
                    }
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}