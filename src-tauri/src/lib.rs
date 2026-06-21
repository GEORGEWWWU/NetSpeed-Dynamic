use std::sync::Mutex;
use tauri::{State, Manager};
use sysinfo::Networks;
use std::net::{SocketAddr, TcpStream};
use std::time::{Duration, Instant};

// 引入 autostart 所需的 MacosLauncher (即使只在 Windows 上运行，这也是标准的初始化参数)
use tauri_plugin_autostart::MacosLauncher;

// 引入托盘相关组件
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent, MouseButton};

use winapi::um::winuser::{EnumWindows, GetWindowTextW, IsWindowVisible, GetClassNameW};
use winapi::shared::windef::HWND;
use std::os::windows::ffi::OsStringExt;

// 结构体：用于在窗口枚举中传递和存储找到的歌词/音乐信息
struct MusicInfo {
    title: String,
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

struct AppState {
    networks: Mutex<Networks>,
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
        })
        .invoke_handler(tauri::generate_handler![
            get_network_stats,
            is_widget_visible,
            get_network_latency,
            fetch_netease_music_info,
            control_system_media
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