// ============================================================================
// main.rs - 应用入口，整合所有模块和状态
// ============================================================================

mod audio_spectrum;
mod music_controller;
mod notification;
mod system_events;

use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};
use tauri::{State, Manager, Emitter};
use sysinfo::Networks;
use std::net::{SocketAddr, TcpStream};
use std::time::{Duration, Instant};
use tauri_plugin_autostart::MacosLauncher;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent, MouseButton};
use winapi::shared::windef::RECT;
use tokio::sync::Mutex as TokioMutex; // 用于异步互斥锁

// ----------------------------------------------------------------------------
// 全局动画控制
// ----------------------------------------------------------------------------
static ANIMATION_ID: AtomicU32 = AtomicU32::new(0);

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
                let len = winapi::um::winuser::GetClassNameW(
                    fg_hwnd,
                    class_name.as_mut_ptr(),
                    class_name.len() as i32,
                );
                let class_str = String::from_utf16_lossy(&class_name[..len as usize]);

                if class_str == "#32768" {
                    return;
                }

                let mut rect: RECT = std::mem::zeroed();
                winapi::um::winuser::GetWindowRect(fg_hwnd, &mut rect);

                let monitor = winapi::um::winuser::MonitorFromWindow(
                    fg_hwnd,
                    winapi::um::winuser::MONITOR_DEFAULTTONEAREST,
                );
                let mut mi: winapi::um::winuser::MONITORINFO = std::mem::zeroed();
                mi.cbSize = std::mem::size_of::<winapi::um::winuser::MONITORINFO>() as u32;
                winapi::um::winuser::GetMonitorInfoW(monitor, &mut mi);

                if rect.left == mi.rcMonitor.left
                    && rect.top == mi.rcMonitor.top
                    && rect.right == mi.rcMonitor.right
                    && rect.bottom == mi.rcMonitor.bottom
                {
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

#[tauri::command]
fn set_window_bounds(app: tauri::AppHandle, x: i32, y: i32, width: i32, height: i32) {
    #[cfg(target_os = "windows")]
    {
        if let Some(win) = app.get_webview_window("widget") {
            if let Ok(hwnd) = win.hwnd() {
                unsafe {
                    winapi::um::winuser::SetWindowPos(
                        hwnd.0 as _,
                        std::ptr::null_mut(),
                        x,
                        y,
                        width,
                        height,
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
    spring_style: String,
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

            let (anchor_cx, anchor_cy, anchor_lx, anchor_by) = {
                let mut anchor_guard = ANIMATION_ANCHOR.lock().unwrap_or_else(|e| e.into_inner());

                if let Some(anchor) = anchor_guard.as_mut() {
                    anchor.active_id = id;
                    (anchor.center_x, anchor.origin_y, anchor.left_x, anchor.bottom_y)
                } else {
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

                let (freq, decay, duration_ms) = if spring_style == "stiff" {
                    (3.8, 22.0, 250)
                } else {
                    (2.4, 12.0, 400)
                };

                let duration = std::time::Duration::from_millis(duration_ms);

                while start_time.elapsed() < duration {
                    std::thread::sleep(std::time::Duration::from_millis(8));

                    if ANIMATION_ID.load(Ordering::SeqCst) != id {
                        return;
                    }

                    let elapsed = start_time.elapsed().as_secs_f64();
                    let progress = elapsed / (duration_ms as f64 / 1000.0);
                    if progress >= 1.0 {
                        break;
                    }

                    let spring =
                        1.0 - (freq * elapsed * 2.0 * std::f64::consts::PI).cos() * (-decay * elapsed).exp();
                    let current_w = start_width + (target_width - start_width) * spring;
                    let current_h = start_height + (target_height - start_height) * spring;

                    let phys_window_w = (current_w * scale_factor).round() as i32;
                    let phys_window_h = (current_h * scale_factor).round() as i32;

                    let (final_x, final_y) = if is_pinned {
                        (anchor_lx, anchor_by - phys_window_w)
                    } else {
                        (anchor_cx - phys_window_w / 2, anchor_cy)
                    };

                    unsafe {
                        SetWindowPos(
                            hwnd_raw as _,
                            std::ptr::null_mut(),
                            final_x,
                            final_y,
                            phys_window_w,
                            phys_window_h,
                            0x0014,
                        );
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
                        SetWindowPos(
                            hwnd_raw as _,
                            std::ptr::null_mut(),
                            final_x,
                            final_y,
                            phys_target_w,
                            phys_target_h,
                            0x0014,
                        );
                    }
                    let _ = window_clone.emit("island-resize", vec![target_width, target_height]);

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

// ----------------------------------------------------------------------------
// 应用状态（包含网络统计和 WebSocket 任务句柄）
// 必须 pub 以便其他模块（如 music_controller）可以访问
// ----------------------------------------------------------------------------
pub struct AppState {
    pub networks: Mutex<Networks>,
    pub ws_task: TokioMutex<Option<tokio::task::JoinHandle<()>>>,
}

// ----------------------------------------------------------------------------
// 网络相关命令
// ----------------------------------------------------------------------------
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

// ----------------------------------------------------------------------------
// 主入口
// ----------------------------------------------------------------------------
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let networks = Networks::new_with_refreshed_list();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--autostart"]),
        ))
        // 初始化 AppState（注意这里补上了逗号）
        .manage(AppState {
            networks: Mutex::new(networks),
            ws_task: TokioMutex::new(None),
        })
        // 注册所有命令（包含新加的 WebSocket 命令）
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
            music_controller::fetch_netease_lyrics,
            // 👇 新增 WebSocket 歌词命令
            music_controller::start_websocket_lyrics,
            music_controller::stop_websocket_lyrics,
        ])
        .setup(|app| {
            audio_spectrum::start_monitor();
            system_events::start_monitor(app.handle().clone());

            // ---- 全屏检测线程（保持不变） ----
            let app_handle_for_fs = app.handle().clone();
            std::thread::spawn(move || {
                unsafe {
                    let _ = windows::Win32::System::Com::CoInitializeEx(
                        None,
                        windows::Win32::System::Com::COINIT_MULTITHREADED,
                    );
                }

                let mut was_fullscreen = false;
                loop {
                    std::thread::sleep(std::time::Duration::from_millis(600));

                    #[cfg(target_os = "windows")]
                    {
                        unsafe {
                            let mut is_fullscreen = false;
                            let fg_hwnd = winapi::um::winuser::GetForegroundWindow();
                            let shell_hwnd = winapi::um::winuser::GetShellWindow();

                            if !fg_hwnd.is_null()
                                && fg_hwnd != winapi::um::winuser::GetDesktopWindow()
                                && fg_hwnd != shell_hwnd
                            {
                                let mut shell_pid = 0;
                                if !shell_hwnd.is_null() {
                                    winapi::um::winuser::GetWindowThreadProcessId(shell_hwnd, &mut shell_pid);
                                }

                                let mut fg_pid = 0;
                                winapi::um::winuser::GetWindowThreadProcessId(fg_hwnd, &mut fg_pid);

                                if shell_pid != 0 && fg_pid == shell_pid {
                                    // 系统组件，忽略
                                } else {
                                    let style = winapi::um::winuser::GetWindowLongPtrW(
                                        fg_hwnd,
                                        winapi::um::winuser::GWL_STYLE,
                                    ) as u32;
                                    let ex_style = winapi::um::winuser::GetWindowLongPtrW(
                                        fg_hwnd,
                                        winapi::um::winuser::GWL_EXSTYLE,
                                    ) as u32;

                                    if (style & winapi::um::winuser::WS_CHILD) == 0
                                        && (ex_style & winapi::um::winuser::WS_EX_TRANSPARENT) == 0
                                    {
                                        let mut class_name = [0u16; 256];
                                        let len = winapi::um::winuser::GetClassNameW(
                                            fg_hwnd,
                                            class_name.as_mut_ptr(),
                                            class_name.len() as i32,
                                        );
                                        let class_str = String::from_utf16_lossy(&class_name[..len as usize]);

                                        let is_blacklisted = class_str.contains("Windows.UI.Core.CoreWindow")
                                            || class_str.contains("Xaml_WindowedPopupClass")
                                            || class_str.contains("SearchApp")
                                            || class_str.contains("NotifyIconOverflowWindow");

                                        if !is_blacklisted {
                                            let mut rect: winapi::shared::windef::RECT =
                                                std::mem::zeroed();
                                            winapi::um::winuser::GetWindowRect(fg_hwnd, &mut rect);

                                            let monitor = winapi::um::winuser::MonitorFromWindow(
                                                fg_hwnd,
                                                winapi::um::winuser::MONITOR_DEFAULTTONEAREST,
                                            );
                                            let mut mi: winapi::um::winuser::MONITORINFO =
                                                std::mem::zeroed();
                                            mi.cbSize = std::mem::size_of::<
                                                winapi::um::winuser::MONITORINFO,
                                            >() as u32;
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

                            if is_fullscreen != was_fullscreen {
                                let _ = app_handle_for_fs.emit("fullscreen-changed", is_fullscreen);
                                was_fullscreen = is_fullscreen;
                            }
                        }
                    }
                }
            });

            // ---- 启动参数处理 ----
            let args: Vec<String> = std::env::args().collect();
            let is_autostart = args.iter().any(|arg| arg == "--autostart");

            if let Some(main_window) = app.get_webview_window("main") {
                if !is_autostart {
                    let _ = main_window.show();
                    let _ = main_window.set_focus();
                }
            }

            // ---- 系统托盘 ----
            let quit_item = MenuItem::with_id(app, "quit", "强制退出", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&quit_item])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("NetSpeed Dynamic Pro")
                .menu(&tray_menu)
                .on_menu_event(move |_app_handle, event| {
                    if event.id == "quit" {
                        std::process::exit(0);
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        ..
                    } = event
                    {
                        if let Some(main_window) = tray.app_handle().get_webview_window("main") {
                            let _ = main_window.show();
                            let _ = main_window.unminimize();
                            let _ = main_window.set_focus();
                        }
                    }
                })
                .build(app)?;

            // ---- 主窗口关闭时隐藏而非退出 ----
            if let Some(main_window) = app.get_webview_window("main") {
                let w_clone = main_window.clone();
                main_window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = w_clone.hide();
                    }
                });
            }

            // ---- 悬浮窗（widget）去边框和圆角 ----
            if let Some(widget_window) = app.get_webview_window("widget") {
                #[cfg(target_os = "windows")]
                {
                    use windows_sys::Win32::Graphics::Dwm::{
                        DwmSetWindowAttribute, DWMWA_WINDOW_CORNER_PREFERENCE, DWMWA_BORDER_COLOR,
                        DWMWCP_DONOTROUND,
                    };
                    use windows_sys::Win32::UI::WindowsAndMessaging::{
                        SetWindowLongPtrW, GWL_STYLE, WS_CAPTION,
                    };
                    use windows_sys::Win32::Foundation::HWND;

                    if let Ok(hwnd) = widget_window.hwnd() {
                        let hwnd_raw = hwnd.0 as HWND;
                        unsafe {
                            let current_style =
                                windows_sys::Win32::UI::WindowsAndMessaging::GetWindowLongPtrW(
                                    hwnd_raw,
                                    GWL_STYLE,
                                );
                            SetWindowLongPtrW(
                                hwnd_raw,
                                GWL_STYLE,
                                current_style & !(WS_CAPTION as isize),
                            );

                            let border_color: u32 = 0xFFFFFFFE;
                            let _ = DwmSetWindowAttribute(
                                hwnd_raw,
                                DWMWA_BORDER_COLOR as u32,
                                &border_color as *const _ as *const _,
                                4,
                            );

                            let corner_preference = DWMWCP_DONOTROUND;
                            let _ = DwmSetWindowAttribute(
                                hwnd_raw,
                                DWMWA_WINDOW_CORNER_PREFERENCE as u32,
                                &corner_preference as *const _ as *const _,
                                4,
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