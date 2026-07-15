mod audio_spectrum;
mod music_controller;
mod notification;
mod system_events;

use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};
use tauri::{State, Manager, Emitter};
use sysinfo::{Networks};
use std::net::{SocketAddr, TcpStream};
use std::time::{Duration, Instant};
use tauri_plugin_autostart::MacosLauncher;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent, MouseButton};
use winapi::shared::windef::RECT;

// 全功能灵动岛智能双模动画锁
static ANIMATION_ID: AtomicU32 = AtomicU32::new(0);

// 将分散的坐标合并为一个结构体，并附带所有权 ID 防止误删
struct AnchorState {
    center_x: i32,
    origin_y: i32,
    left_x: i32,
    bottom_y: i32,
    active_id: u32,
}
static ANIMATION_ANCHOR: Mutex<Option<AnchorState>> = Mutex::new(None);

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

#[tauri::command]
async fn start_island_animation(
    window: tauri::WebviewWindow,
    start_width: f64,
    start_height: f64,
    target_width: f64,
    target_height: f64,
    is_pinned: bool,
) -> Result<(), String> {
    let id = ANIMATION_ID.fetch_add(1, Ordering::SeqCst) + 1;
    let scale_factor = window.scale_factor().unwrap_or(1.0);

    #[cfg(target_os = "windows")]
    {
        if let Ok(hwnd) = window.hwnd() {
            use winapi::um::winuser::{GetWindowRect, SetWindowPos};
            use winapi::shared::windef::RECT;

            let mut rect: RECT = unsafe { std::mem::zeroed() };
            unsafe { GetWindowRect(hwnd.0 as _, &mut rect); }

            // 核心修复 1：在主线程安全地获取并克隆锚点值，避免进入子线程后再 unwrap
            let (anchor_cx, anchor_cy, anchor_lx, anchor_by) = {
                // 使用 unwrap_or_else 防止之前的 Panic 导致锁中毒
                let mut anchor_guard = ANIMATION_ANCHOR.lock().unwrap_or_else(|e| e.into_inner());
                
                if let Some(anchor) = anchor_guard.as_mut() {
                    // 已经有动画锚点，说明正在连续打断动画，继承坐标并刷新所有权 ID
                    anchor.active_id = id;
                    (anchor.center_x, anchor.origin_y, anchor.left_x, anchor.bottom_y)
                } else {
                    // 首次触发，设定新的物理锚点
                    let cx = rect.left + (rect.right - rect.left) / 2;
                    let cy = rect.top;
                    let lx = rect.left;
                    let by = rect.bottom;
                    *anchor_guard = Some(AnchorState {
                        center_x: cx,
                        origin_y: cy,
                        left_x: lx,
                        bottom_y: by,
                        active_id: id,
                    });
                    (cx, cy, lx, by)
                }
            };

            let window_clone = window.clone();
            let hwnd_raw = hwnd.0 as isize;

            std::thread::spawn(move || {
                let start_time = std::time::Instant::now();
                let duration = std::time::Duration::from_millis(400);
                let freq = 2.4;
                let decay = 12.0;

                while start_time.elapsed() < duration {
                    std::thread::sleep(std::time::Duration::from_millis(8));

                    if ANIMATION_ID.load(Ordering::SeqCst) != id {
                        return;
                    }

                    let elapsed = start_time.elapsed().as_secs_f64();
                    let progress = elapsed / 0.4;
                    if progress >= 1.0 { break; }

                    let spring = 1.0 - (freq * elapsed * 2.0 * std::f64::consts::PI).cos() * (-decay * elapsed).exp();
                    let current_w = start_width + (target_width - start_width) * spring;
                    let current_h = start_height + (target_height - start_height) * spring;

                    let phys_window_w = (current_w * scale_factor).round() as i32;
                    let phys_window_h = (current_h * scale_factor).round() as i32;

                    // 核心修复 2：直接使用预先拷贝进来的局部变量，完全告别 unwrap
                    let (final_x, final_y) = if is_pinned {
                        (anchor_lx, anchor_by - phys_window_h)
                    } else {
                        (anchor_cx - phys_window_w / 2, anchor_cy)
                    };

                    unsafe {
                        SetWindowPos(hwnd_raw as _, std::ptr::null_mut(), final_x, final_y, phys_window_w, phys_window_h, 0x0014);
                    }
                }

                if ANIMATION_ID.load(Ordering::SeqCst) == id {
                    let phys_target_w = (target_width * scale_factor).round() as i32;
                    let phys_target_h = (target_height * scale_factor).round() as i32;

                    let (final_x, final_y) = if is_pinned {
                        (anchor_lx, anchor_by - phys_target_h)
                    } else {
                        (anchor_cx - phys_target_w / 2, anchor_cy)
                    };

                    unsafe {
                        SetWindowPos(hwnd_raw as _, std::ptr::null_mut(), final_x, final_y, phys_target_w, phys_target_h, 0x0014);
                    }
                    let _ = window_clone.emit("island-resize", vec![target_width, target_height]);

                    // 核心修复 3：仅当当前动画仍是持有者时才清理锚点，防止误删新一轮动画的锁
                    if let Ok(mut guard) = ANIMATION_ANCHOR.lock() {
                        if let Some(anchor) = guard.as_ref() {
                            if anchor.active_id == id {
                                *guard = None;
                            }
                        }
                    }
                }
            });
        }
    }
    Ok(())
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

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--autostart"])))
        .manage(AppState { networks: Mutex::new(networks) })
        .invoke_handler(tauri::generate_handler![
            get_network_stats,
            is_widget_visible,
            get_network_latency,
            notification::fetch_latest_notification,
            force_window_topmost,
            set_window_bounds,
            start_island_animation,
            audio_spectrum::get_audio_spectrum,
            music_controller::set_target_player,
            music_controller::fetch_netease_music_info,
            music_controller::control_system_media,
            music_controller::get_random_cover_url,
        ])
        .setup(|app| {
            audio_spectrum::start_monitor();
            system_events::start_monitor(app.handle().clone());

            // 全屏应用检测线程
            let app_handle_for_fs = app.handle().clone();
            std::thread::spawn(move || {
                unsafe { let _ = windows::Win32::System::Com::CoInitializeEx(None, windows::Win32::System::Com::COINIT_MULTITHREADED); }
                
                let mut was_fullscreen = false;
                loop {
                    std::thread::sleep(std::time::Duration::from_millis(600));
                    
                    #[cfg(target_os = "windows")]
                    {
                        unsafe {
                            let mut is_fullscreen = false;
                            let fg_hwnd = winapi::um::winuser::GetForegroundWindow();
                            let shell_hwnd = winapi::um::winuser::GetShellWindow(); // 系统的根：explorer.exe
                            
                            // 1. 过滤掉无焦点窗口、桌面根节点
                            if !fg_hwnd.is_null() 
                                && fg_hwnd != winapi::um::winuser::GetDesktopWindow() 
                                && fg_hwnd != shell_hwnd 
                            {
                                // 【终极杀手锏】：获取系统外壳 (explorer.exe) 的进程 ID
                                let mut shell_pid = 0;
                                if !shell_hwnd.is_null() {
                                    winapi::um::winuser::GetWindowThreadProcessId(shell_hwnd, &mut shell_pid);
                                }

                                // 获取当前前景窗口的进程 ID
                                let mut fg_pid = 0;
                                winapi::um::winuser::GetWindowThreadProcessId(fg_hwnd, &mut fg_pid);

                                // 核心判定：如果抢占焦点的窗口 PID 和任务栏/桌面是一家人
                                // 说明这绝对是任务栏悬浮窗、音量面板或透明防误触层，直接忽略！
                                if shell_pid != 0 && fg_pid == shell_pid {
                                    // 属于系统外壳组件，当做无事发生
                                } else {
                                    // 2. 进一步排除子窗口 (WS_CHILD) 和 鼠标穿透层 (WS_EX_TRANSPARENT)
                                    let style = winapi::um::winuser::GetWindowLongPtrW(fg_hwnd, winapi::um::winuser::GWL_STYLE) as u32;
                                    let ex_style = winapi::um::winuser::GetWindowLongPtrW(fg_hwnd, winapi::um::winuser::GWL_EXSTYLE) as u32;
                                    
                                    if (style & winapi::um::winuser::WS_CHILD) == 0 && (ex_style & winapi::um::winuser::WS_EX_TRANSPARENT) == 0 {
                                        
                                        let mut class_name = [0u16; 256];
                                        let len = winapi::um::winuser::GetClassNameW(fg_hwnd, class_name.as_mut_ptr(), class_name.len() as i32);
                                        let class_str = String::from_utf16_lossy(&class_name[..len as usize]);
                                        
                                        // 3. 保底黑名单（防一手那些不在 explorer.exe 里的新版 UWP 系统层）
                                        let is_blacklisted = class_str.contains("Windows.UI.Core.CoreWindow") 
                                            || class_str.contains("Xaml_WindowedPopupClass")
                                            || class_str.contains("SearchApp")
                                            || class_str.contains("NotifyIconOverflowWindow");

                                        if !is_blacklisted {
                                            // 4. 几何判定：真正判断它是否铺满了屏幕
                                            let mut rect: winapi::shared::windef::RECT = std::mem::zeroed();
                                            winapi::um::winuser::GetWindowRect(fg_hwnd, &mut rect);

                                            let monitor = winapi::um::winuser::MonitorFromWindow(fg_hwnd, winapi::um::winuser::MONITOR_DEFAULTTONEAREST);
                                            let mut mi: winapi::um::winuser::MONITORINFO = std::mem::zeroed();
                                            mi.cbSize = std::mem::size_of::<winapi::um::winuser::MONITORINFO>() as u32;
                                            winapi::um::winuser::GetMonitorInfoW(monitor, &mut mi);

                                            if rect.left <= mi.rcMonitor.left 
                                                && rect.top <= mi.rcMonitor.top 
                                                && rect.right >= mi.rcMonitor.right 
                                                && rect.bottom >= mi.rcMonitor.bottom 
                                            {
                                                is_fullscreen = true;
                                            }
                                        }
                                    }
                                }
                            }

                            // 状态翻转时发送信号
                            if is_fullscreen != was_fullscreen {
                                let _ = app_handle_for_fs.emit("fullscreen-changed", is_fullscreen);
                                was_fullscreen = is_fullscreen;
                            }
                        }
                    }
                }
            });

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