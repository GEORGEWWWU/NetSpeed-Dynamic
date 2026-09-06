mod activity_pool;
mod audio_spectrum;
mod music_controller;
mod notification;
mod system_events;

use std::net::{IpAddr, Ipv4Addr};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};
use sysinfo::Networks;
use tauri::menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};
use tauri::{Emitter, Manager, State};
use tauri_plugin_autostart::MacosLauncher;
use tokio::sync::Mutex as TokioMutex;

use futures_util::{SinkExt, StreamExt};
use surge_ping::{Client, Config, PingIdentifier, PingSequence};
use std::path::PathBuf;
use std::sync::OnceLock;
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::Message;

use std::process::{Child, Command};

// 维护一个全局变量，持有挂件的子进程，方便随时掐死
static TASKBAR_PLUGIN_PROCESS: Mutex<Option<Child>> = Mutex::new(None);

// FPS 插件的全局进程控制器
static FPS_PLUGIN_PROCESS: Mutex<Option<Child>> = Mutex::new(None);

// 用于向任务栏挂件广播数据的通道
static TASKBAR_WS_SENDER: OnceLock<broadcast::Sender<String>> = OnceLock::new();

// 智能获取插件路径（全方位无死角兼容开发与生产环境）
fn get_plugin_path(exe_name: &str) -> Result<PathBuf, String> {
    // 👈 增加参数
    // 1. 生产环境：优先尝试与主程序相同的绝对目录
    if let Ok(mut exe_path) = std::env::current_exe() {
        exe_path.pop();
        exe_path.push(exe_name);
        if exe_path.exists() {
            return Ok(exe_path);
        }
    }

    // 2. 开发环境：暴力穷举所有可能的工作目录
    if let Ok(cwd) = std::env::current_dir() {
        let paths_to_try = vec![
            cwd.join("src-tauri").join(exe_name),
            cwd.join(exe_name),
            cwd.join("..").join(exe_name),
        ];

        for path in paths_to_try {
            if path.exists() {
                return Ok(path);
            }
        }
    }

    Err(format!(
        "未能找到 {}, 请确保已将其放入 src-tauri 目录或打包根目录。",
        exe_name
    ))
}

// 启动 47291 端口的 WebSocket 广播服务器 (修复 Tokio 崩溃问题)
fn init_taskbar_ws_server() {
    let (tx, _rx) = broadcast::channel(16);
    TASKBAR_WS_SENDER.set(tx).unwrap();

    // 核心修复：使用 tauri::async_runtime::spawn 代替原生的 tokio::spawn
    // 这样任务就会安全地挂载到 Tauri 已经初始化好的异步运行时上
    tauri::async_runtime::spawn(async move {
        // 监听全新端口 47291
        if let Ok(listener) = tokio::net::TcpListener::bind("127.0.0.1:47291").await {
            while let Ok((stream, _)) = listener.accept().await {
                let tx = TASKBAR_WS_SENDER.get().unwrap().clone();
                tauri::async_runtime::spawn(async move {
                    if let Ok(ws_stream) = tokio_tungstenite::accept_async(stream).await {
                        let (mut ws_tx, _) = ws_stream.split();
                        let mut rx = tx.subscribe();

                        // 持续将 Vue 传来的数据广播给 WPF 插件
                        while let Ok(msg) = rx.recv().await {
                            if ws_tx.send(Message::Text(msg)).await.is_err() {
                                break;
                            }
                        }
                    }
                });
            }
        }
    });
}

#[tauri::command]
fn toggle_taskbar_plugin(enable: bool) -> Result<bool, String> {
    let mut process_guard = TASKBAR_PLUGIN_PROCESS.lock().unwrap();

    if enable {
        if process_guard.is_none() {
            let exe_path = get_plugin_path("NSD_Taskbar_Plugin.exe")?;
            // 启动插件
            match Command::new(exe_path).spawn() {
                Ok(child) => {
                    *process_guard = Some(child);
                    return Ok(true);
                }
                Err(e) => {
                    return Err(format!("启动插件失败: {}", e));
                }
            }
        }
    } else {
        // 关闭挂件
        if let Some(mut child) = process_guard.take() {
            let _ = child.kill();
        }
    }
    Ok(true)
}

#[tauri::command]
fn toggle_fps_plugin(enable: bool) -> Result<bool, String> {
    let mut process_guard = FPS_PLUGIN_PROCESS.lock().unwrap();

    if enable {
        if process_guard.is_none() {
            let exe_path = get_plugin_path("NSD_Fps_Plugin.exe")?;

            match Command::new(exe_path).spawn() {
                Ok(child) => {
                    *process_guard = Some(child);
                    return Ok(true);
                }
                Err(e) => return Err(format!("启动 FPS 插件失败: {}", e)),
            }
        }
    } else {
        if let Some(mut child) = process_guard.take() {
            let _ = child.kill();
        }
    }
    Ok(true)
}

// 供 Vue 调用的同步数据接口
#[tauri::command]
fn sync_to_taskbar(
    up: String,
    down: String,
    lyric: String,
    mode: String,
    is_playing: bool,
    cover: String,
    msg_title: String,
    msg_body: String,
    msg_icon: String,
    cpu: u8,
    ram: u8,
) {
    if let Some(tx) = TASKBAR_WS_SENDER.get() {
        // 使用 serde_json 防止特殊字符破坏 JSON 格式
        let json_str = serde_json::json!({
            "up": up,
            "down": down,
            "lyric": lyric,
            "mode": mode,
            "is_playing": is_playing,
            "cover": cover,
            "msg_title": msg_title,
            "msg_body": msg_body,
            "msg_icon": msg_icon,
            "cpu": cpu,
            "ram": ram
        })
        .to_string();

        let _ = tx.send(json_str);
    }
}

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
fn show_window_no_activate(app: tauri::AppHandle, label: String) {
    if let Some(win) = app.get_webview_window(&label) {
        #[cfg(target_os = "windows")]
        {
            if let Ok(hwnd) = win.hwnd() {
                unsafe {
                    // SW_SHOWNOACTIVATE = 4，显示窗口但不抢占当前应用的焦点
                    winapi::um::winuser::ShowWindow(hwnd.0 as _, 4);
                    // 强制回到置顶带最顶端：防止被其他置顶窗口（全屏应用/视频播放器悬浮窗等）盖住
                    winapi::um::winuser::SetWindowPos(
                        hwnd.0 as _,
                        winapi::um::winuser::HWND_TOPMOST,
                        0,
                        0,
                        0,
                        0,
                        // SWP_NOSIZE(0x0001) | SWP_NOMOVE(0x0002) | SWP_NOACTIVATE(0x0010)
                        0x0001 | 0x0002 | 0x0010,
                    );
                }
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = win.show();
        }
    }
}

// 动态切换窗口的“不可激活”扩展样式 (WS_EX_NOACTIVATE)。
// 全屏悬停唤起期间开启：点击灵动岛不再抢占焦点/激活，前台始终留给全屏应用，
// 从而避免 Windows 因前台变成非全屏小窗而弹出任务栏；退出全屏后恢复可激活。
#[tauri::command]
fn set_window_no_activate(app: tauri::AppHandle, label: String, enabled: bool) {
    if let Some(win) = app.get_webview_window(&label) {
        #[cfg(target_os = "windows")]
        {
            if let Ok(hwnd) = win.hwnd() {
                unsafe {
                    // GetWindowLongPtrW 返回 LONG_PTR，全程按其位宽运算，避免 64 位下截断
                    let ex_style = winapi::um::winuser::GetWindowLongPtrW(
                        hwnd.0 as _,
                        winapi::um::winuser::GWL_EXSTYLE,
                    );
                    let no_activate = winapi::um::winuser::WS_EX_NOACTIVATE
                        as winapi::shared::basetsd::LONG_PTR;
                    let new_style = if enabled {
                        ex_style | no_activate
                    } else {
                        ex_style & !no_activate
                    };
                    winapi::um::winuser::SetWindowLongPtrW(
                        hwnd.0 as _,
                        winapi::um::winuser::GWL_EXSTYLE,
                        new_style,
                    );
                }
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = (label, enabled);
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
    spring_style: String,
) -> Result<(), String> {
    let id = ANIMATION_ID.fetch_add(1, Ordering::SeqCst) + 1;
    let scale_factor = window.scale_factor().unwrap_or(1.0);

    #[cfg(target_os = "windows")]
    {
        if let Ok(hwnd) = window.hwnd() {
            use winapi::shared::windef::RECT;
            use winapi::um::winuser::{GetWindowRect, SetWindowPos};

            let mut rect: RECT = unsafe { std::mem::zeroed() };
            unsafe {
                GetWindowRect(hwnd.0 as _, &mut rect);
            }

            let (anchor_cx, anchor_cy, _anchor_lx, _anchor_by) = {
                let mut anchor_guard = ANIMATION_ANCHOR.lock().unwrap_or_else(|e| e.into_inner());

                if let Some(anchor) = anchor_guard.as_mut() {
                    anchor.active_id = id;
                    (
                        anchor.center_x,
                        anchor.origin_y,
                        anchor.left_x,
                        anchor.bottom_y,
                    )
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

                // 2. 👈 根据参数动态匹配弹性物理常数
                // Stiff (克制): 提高频率，大幅拉高阻尼，使其快准狠
                // Bouncy (Q弹): 保持原本欢快的震喜感
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

                    let spring = 1.0
                        - (freq * elapsed * 2.0 * std::f64::consts::PI).cos()
                            * (-decay * elapsed).exp();
                    let current_w = start_width + (target_width - start_width) * spring;
                    let current_h = start_height + (target_height - start_height) * spring;

                    // 1. 保留这俩变量，SetWindowPos 必须用到它们作为宽高参数
                    let phys_window_w = (current_w * scale_factor).round() as i32;
                    let phys_window_h = (current_h * scale_factor).round() as i32;

                    // 2. 坐标计算：直接用浮点数除以 2，避免 i32 除法丢失 0.5 像素导致漂移
                    let final_x = (anchor_cx as f64 - (current_w * scale_factor) / 2.0).round() as i32;
                    let final_y = anchor_cy;

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

                // 动画结束定格的那一帧，也要使用相同的浮点计算逻辑
                if ANIMATION_ID.load(Ordering::SeqCst) == id {
                    let phys_target_w = (target_width * scale_factor).round() as i32;
                    let phys_target_h = (target_height * scale_factor).round() as i32;

                    let final_x = (anchor_cx as f64 - (target_width * scale_factor) / 2.0).round() as i32;
                    let final_y = anchor_cy;

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

                if ANIMATION_ID.load(Ordering::SeqCst) == id {
                    let phys_target_w = (target_width * scale_factor).round() as i32;
                    let phys_target_h = (target_height * scale_factor).round() as i32;

                    let final_x = anchor_cx - phys_target_w / 2;
                    let final_y = anchor_cy;

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

pub struct AppState {
    pub networks: Mutex<Networks>,
    pub ws_task: TokioMutex<Option<tokio::task::JoinHandle<()>>>,
    // 换成专业的原生复选菜单项引用
    pub tray_items: Mutex<
        Option<(
            CheckMenuItem<tauri::Wry>,
            CheckMenuItem<tauri::Wry>,
            CheckMenuItem<tauri::Wry>,
            CheckMenuItem<tauri::Wry>,
        )>,
    >,
}

#[tauri::command]
fn sync_tray_menu(
    state: State<'_, AppState>,
    island: Option<bool>,
    quiet: Option<bool>,
    glow: Option<bool>,
    lock: Option<bool>,
) {
    // 接收到 Vue 发来的状态，直接调用原生 API 改变打勾状态，不再拼接野路子字符串
    if let Some((island_item, quiet_item, glow_item, lock_item)) = &*state.tray_items.lock().unwrap()
    {
        if let Some(v) = island {
            let _ = island_item.set_checked(v);
        }
        if let Some(v) = quiet {
            let _ = quiet_item.set_checked(v);
        }
        if let Some(v) = glow {
            let _ = glow_item.set_checked(v);
        }
        if let Some(v) = lock {
            let _ = lock_item.set_checked(v);
        }
    }
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
async fn get_network_latency() -> Result<u128, String> {
    let timeout = Duration::from_millis(1500);
    let client = Client::new(&Config::default())
        .map_err(|e| format!("Failed to create ping client: {}", e))?;

    // 多目标 ICMP ping：避免单一 IP 被网络环境拦截导致误判断网
    let targets = [
        IpAddr::V4(Ipv4Addr::new(223, 5, 5, 5)),
        IpAddr::V4(Ipv4Addr::new(223, 6, 6, 6)),
        IpAddr::V4(Ipv4Addr::new(119, 29, 29, 29)),
        IpAddr::V4(Ipv4Addr::new(1, 0, 0, 1)),
    ];

    for ip in targets {
        let mut pinger = client.pinger(ip, PingIdentifier(rand::random::<u16>())).await;
        pinger.timeout(timeout);
        let attempt_start = Instant::now();
        if pinger.ping(PingSequence(0), &[0u8; 16]).await.is_ok() {
            return Ok(attempt_start.elapsed().as_millis());
        }
    }
    Err("Timeout".to_string())
}

#[tauri::command]
fn is_widget_visible(app: tauri::AppHandle) -> bool {
    match app.get_webview_window("widget") {
        Some(win) => win.is_visible().unwrap_or(false),
        None => false,
    }
}

/// 读取系统剪贴板文本（Windows 专用），供灵动岛检测复制到链接
#[cfg(target_os = "windows")]
#[tauri::command]
fn get_clipboard_text() -> Result<String, String> {
    unsafe {
        use winapi::um::winbase::{GlobalLock, GlobalUnlock};
        use winapi::um::winuser::{CF_UNICODETEXT, CloseClipboard, GetClipboardData, OpenClipboard};

        if OpenClipboard(std::ptr::null_mut()) == 0 {
            return Err("无法打开剪贴板".to_string());
        }

        // 若剪贴板为空或格式不含文本，全局句柄为 NULL
        let handle = GetClipboardData(CF_UNICODETEXT as u32);
        if handle.is_null() {
            CloseClipboard();
            return Ok(String::new());
        }

        let ptr = GlobalLock(handle) as *const u16;
        if ptr.is_null() {
            CloseClipboard();
            return Ok(String::new());
        }

        // 统计字符串长度直到遇到结尾空字符
        let mut len = 0usize;
        while *ptr.add(len) != 0 {
            len += 1;
        }
        let text = String::from_utf16_lossy(std::slice::from_raw_parts(ptr, len));

        GlobalUnlock(handle);
        CloseClipboard();
        Ok(text)
    }
}

// 非 Windows 平台空实现，避免编译报错
#[cfg(not(target_os = "windows"))]
#[tauri::command]
fn get_clipboard_text() -> Result<String, String> {
    Ok(String::new())
}

/// 获取浏览器实时活动标签页（当前窗口标题，即活动标签标题）
/// Windows 上通过 Win32 API（EnumWindows）枚举 msedge/chrome 进程的可见顶层窗口标题实现，
/// 替代原来的 PowerShell 轮询方案：不再每 2s 冷启动一个 PowerShell 子进程，开销更低、响应更快，
/// 且 GetWindowTextW 直接返回 UTF-16，中文标题天然正确，无需再处理控制台编码。
/// 注意：必须 async + spawn_blocking 放到阻塞线程池执行——Tauri v2 里不带 async 的同步命令
/// 会直接跑在主线程，阻塞 UI（频谱/动画掉帧）。
#[cfg(target_os = "windows")]
#[tauri::command]
async fn get_active_browser_tabs() -> Result<Vec<String>, String> {
    tauri::async_runtime::spawn_blocking(|| {
        use winapi::shared::minwindef::{BOOL, FALSE, LPARAM, TRUE};
        use winapi::shared::windef::HWND;
        use winapi::um::handleapi::CloseHandle;
        use winapi::um::processthreadsapi::OpenProcess;
        use winapi::um::winbase::QueryFullProcessImageNameW;
        use winapi::um::winnt::PROCESS_QUERY_LIMITED_INFORMATION;
        use winapi::um::winuser::{EnumWindows, GetWindowTextW, GetWindowThreadProcessId, IsWindowVisible};

        // EnumWindows 回调：lParam 携带结果 Vec 的指针，收集所有可见的 msedge/chrome 窗口标题
        unsafe extern "system" fn enum_browser_windows(hwnd: HWND, lparam: LPARAM) -> BOOL {
            let titles = lparam as *mut Vec<String>;
            // 与旧 PowerShell 的 MainWindowTitle 语义一致：只处理可见窗口
            if IsWindowVisible(hwnd) == FALSE {
                return TRUE;
            }
            // 通过窗口所属进程判断是否为 msedge/chrome（与旧 PowerShell 按进程名匹配一致）
            let mut pid: u32 = 0;
            GetWindowThreadProcessId(hwnd, &mut pid);
            if pid == 0 {
                return TRUE;
            }
            let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, FALSE, pid);
            if process.is_null() {
                return TRUE; // 权限不足（如浏览器以管理员运行）时跳过，与 PowerShell 读不到标题时行为一致
            }
            let mut path_buf = [0u16; 1024];
            let mut path_len = path_buf.len() as u32;
            let ok = QueryFullProcessImageNameW(process, 0, path_buf.as_mut_ptr(), &mut path_len);
            CloseHandle(process);
            if ok == FALSE {
                return TRUE;
            }
            let path = String::from_utf16_lossy(&path_buf[..path_len as usize]).to_lowercase();
            let is_browser = path.ends_with("msedge.exe") || path.ends_with("chrome.exe");
            if !is_browser {
                return TRUE;
            }
            // 读取窗口标题（UTF-16，天然支持中文，无需编码转换）
            let mut title_buf = [0u16; 512];
            let n = GetWindowTextW(hwnd, title_buf.as_mut_ptr(), title_buf.len() as i32);
            if n > 0 {
                let title = String::from_utf16_lossy(&title_buf[..n as usize]);
                let title = title.trim().to_string();
                if !title.is_empty() {
                    (*titles).push(title);
                }
            }
            TRUE
        }

        let mut titles: Vec<String> = Vec::new();
        unsafe {
            EnumWindows(Some(enum_browser_windows), &mut titles as *mut Vec<String> as isize);
        }
        // 去重：同一标题可能来自多个窗口/进程，保持首次出现顺序
        let mut seen = std::collections::HashSet::new();
        titles.retain(|t| seen.insert(t.clone()));

        Ok(titles)
    })
    .await
    .map_err(|e| format!("获取浏览器标签页任务失败: {}", e))?
}

// 非 Windows 平台空实现，避免编译报错
#[cfg(not(target_os = "windows"))]
#[tauri::command]
fn get_active_browser_tabs() -> Result<Vec<String>, String> {
    Ok(Vec::new())
}

// 缓存 AppHandle 供剪贴板监听线程的窗口过程使用
#[cfg(target_os = "windows")]
static CLIPBOARD_EMITTER: OnceLock<tauri::AppHandle> = OnceLock::new();

// 剪贴板监听窗口过程：一旦检测到剪贴板内容变化（WM_CLIPBOARDUPDATE），推送事件给前端
#[cfg(target_os = "windows")]
extern "system" fn clipboard_wndproc(
    hwnd: winapi::shared::windef::HWND,
    msg: winapi::shared::minwindef::UINT,
    wparam: winapi::shared::minwindef::WPARAM,
    lparam: winapi::shared::minwindef::LPARAM,
) -> winapi::shared::minwindef::LRESULT {
    use winapi::um::winuser::{
        DefWindowProcW, PostQuitMessage, WM_CLIPBOARDUPDATE, WM_DESTROY,
    };
    unsafe {
        if msg == WM_CLIPBOARDUPDATE {
            if let Some(app) = CLIPBOARD_EMITTER.get() {
                let _ = Emitter::emit(app, "clipboard-changed", ());
            }
            return 0;
        }
        if msg == WM_DESTROY {
            PostQuitMessage(0);
            return 0;
        }
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}

// 启动剪贴板变更监听（事件驱动，无需轮询）：通过隐藏窗口 + AddClipboardFormatListener
#[cfg(target_os = "windows")]
fn start_clipboard_monitor(app: tauri::AppHandle) {
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winapi::um::winuser::{
        AddClipboardFormatListener, CreateWindowExW, DispatchMessageW, GetMessageW,
        RegisterClassW, TranslateMessage, MSG, WNDCLASSW, WS_OVERLAPPEDWINDOW,
    };

    // 缓存 AppHandle 供窗口过程回调使用
    let _ = CLIPBOARD_EMITTER.set(app);

    std::thread::spawn(|| unsafe {
        let class_name = "NetSpeedClipboardListener"
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<u16>>();
        let hinstance = GetModuleHandleW(std::ptr::null());

        let wc = WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(clipboard_wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance as _,
            hIcon: std::ptr::null_mut(),
            hCursor: std::ptr::null_mut(),
            hbrBackground: std::ptr::null_mut(),
            lpszMenuName: std::ptr::null(),
            lpszClassName: class_name.as_ptr(),
        };
        // 注册窗口失败则直接退出监听线程
        if RegisterClassW(&wc) == 0 {
            return;
        }

        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            class_name.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            0,
            0,
            0,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            hinstance as _,
            std::ptr::null_mut(),
        );
        if hwnd.is_null() {
            return;
        }

        // 注册为剪贴板格式监听者，剪贴板变化时收到 WM_CLIPBOARDUPDATE
        AddClipboardFormatListener(hwnd);

        // 线程消息循环
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let networks = Networks::new_with_refreshed_list();

    // 启动为任务栏挂件服务的 WS 服务器
    init_taskbar_ws_server();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--autostart"]),
        ))
        .manage(AppState {
            networks: Mutex::new(networks),
            ws_task: TokioMutex::new(None),
            tray_items: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            get_network_stats,
            is_widget_visible,
            get_network_latency,
            notification::fetch_latest_notification,
            set_window_bounds,
            start_island_animation,
            show_window_no_activate,
            set_window_no_activate,
            toggle_taskbar_plugin,
            sync_to_taskbar,
            audio_spectrum::get_audio_spectrum,
            music_controller::set_target_player,
            music_controller::fetch_netease_music_info,
            music_controller::control_system_media,
            music_controller::get_random_cover_url,
            music_controller::get_smtc_cover,
            music_controller::fetch_netease_lyrics,
            music_controller::fetch_song_meta,
            music_controller::start_websocket_lyrics,
            music_controller::stop_websocket_lyrics,
            toggle_fps_plugin,
            sync_tray_menu,
            get_clipboard_text,
            get_active_browser_tabs,
        ])
        .setup(|app| {
            // 活动池：47300 HTTP 入站 + 30Hz 节流推送（需尽早启动，供外部随时调用）
            activity_pool::start(app.handle().clone());

            audio_spectrum::start_monitor();
            system_events::start_monitor(app.handle().clone());
            // 启动剪贴板变更监听（事件驱动，复制时实时推送）
            #[cfg(target_os = "windows")]
            start_clipboard_monitor(app.handle().clone());

            // 启动超轻量 UDP 监听器，专用于接收 FPS 数据 (监听 47292 端口)
            let app_handle_for_fps = app.handle().clone();
            std::thread::spawn(move || {
                // 监听 47292 端口，接收 C# 用 UDP 砸过来的帧率数据
                if let Ok(socket) = std::net::UdpSocket::bind("127.0.0.1:47292") {
                    let mut buf = [0; 16];
                    loop {
                        if let Ok((amt, _)) = socket.recv_from(&mut buf) {
                            if let Ok(text) = std::str::from_utf8(&buf[..amt]) {
                                if let Ok(fps) = text.trim().parse::<u32>() {
                                    // 收到后瞬间抛给 Vue
                                    let _ = app_handle_for_fps
                                        .emit("fps-event", serde_json::json!({ "fps": fps }));
                                }
                            }
                        }
                    }
                }
            });

            // 全屏应用检测线程
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
                            let shell_hwnd = winapi::um::winuser::GetShellWindow(); // 系统的根：explorer.exe

                            // 过滤掉无焦点窗口、桌面根节点
                            if !fg_hwnd.is_null()
                                && fg_hwnd != winapi::um::winuser::GetDesktopWindow()
                                && fg_hwnd != shell_hwnd
                            {
                                // 获取系统外壳 (explorer.exe) 的进程 ID
                                let mut shell_pid = 0;
                                if !shell_hwnd.is_null() {
                                    winapi::um::winuser::GetWindowThreadProcessId(
                                        shell_hwnd,
                                        &mut shell_pid,
                                    );
                                }

                                // 获取当前前景窗口的进程 ID
                                let mut fg_pid = 0;
                                winapi::um::winuser::GetWindowThreadProcessId(fg_hwnd, &mut fg_pid);

                                // 核心判定：如果抢占焦点的窗口 PID 和任务栏/桌面是一家人
                                // 说明这绝对是任务栏悬浮窗、音量面板或透明防误触层，直接忽略！
                                if shell_pid != 0 && fg_pid == shell_pid {
                                    // 属于系统外壳组件，当做无事发生
                                } else {
                                    // 进一步排除子窗口 (WS_CHILD) 和 鼠标穿透层 (WS_EX_TRANSPARENT)
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
                                        let class_str =
                                            String::from_utf16_lossy(&class_name[..len as usize]);

                                        // 保底黑名单（防一手那些不在 explorer.exe 里的新版 UWP 系统层）
                                        let is_blacklisted = class_str
                                            .contains("Windows.UI.Core.CoreWindow")
                                            || class_str.contains("Xaml_WindowedPopupClass")
                                            || class_str.contains("SearchApp")
                                            || class_str.contains("NotifyIconOverflowWindow");

                                        if !is_blacklisted {
                                            // 几何判定：真正判断它是否铺满了屏幕
                                            let mut rect: winapi::shared::windef::RECT =
                                                std::mem::zeroed();
                                            winapi::um::winuser::GetWindowRect(fg_hwnd, &mut rect);

                                            let monitor = winapi::um::winuser::MonitorFromWindow(
                                                fg_hwnd,
                                                winapi::um::winuser::MONITOR_DEFAULTTONEAREST,
                                            );

                                            // 跨屏判定：只有全屏发生在灵动岛所在的那块显示器上才触发隐藏。
                                            // 否则副屏全屏看片/游戏时，主屏的岛也会被误藏。
                                            // 岛窗口取不到时（启动瞬间/已销毁）回退为旧的“任意屏全屏”行为。
                                            let mut same_monitor_as_island = true;
                                            if let Some(island_win) =
                                                app_handle_for_fs.get_webview_window("widget")
                                            {
                                                if let Ok(island_hwnd) = island_win.hwnd() {
                                                    let island_monitor =
                                                        winapi::um::winuser::MonitorFromWindow(
                                                            island_hwnd.0 as _,
                                                            winapi::um::winuser::MONITOR_DEFAULTTONEAREST,
                                                        );
                                                    same_monitor_as_island =
                                                        island_monitor == monitor;
                                                }
                                            }
                                            if !same_monitor_as_island {
                                                // 全屏在另一块屏上：与本应用无关，维持可见
                                            } else {
                                                let mut mi: winapi::um::winuser::MONITORINFO =
                                                    std::mem::zeroed();
                                                mi.cbSize = std::mem::size_of::<
                                                    winapi::um::winuser::MONITORINFO,
                                                >(
                                                )
                                                    as u32;
                                                winapi::um::winuser::GetMonitorInfoW(
                                                    monitor,
                                                    &mut mi,
                                                );

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

            // 1. 构建全新结构的托盘菜单
            let version = app.package_info().version.to_string();
            let title_item = MenuItem::with_id(
                app,
                "title",
                format!("NSDPRO v{}", version),
                false,
                None::<&str>,
            )?;
            let sep1 = PredefinedMenuItem::separator(app)?;

            // 【核心专业改法】：使用原生的 CheckMenuItem
            // 操作系统会自动把勾选框放在左侧的隐藏图标列，右侧文本天然完美对齐
            let island_item =
                CheckMenuItem::with_id(app, "toggle_island", "灵动岛", true, true, None::<&str>)?;
            let quiet_item =
                CheckMenuItem::with_id(app, "toggle_quiet", "静默模式", true, true, None::<&str>)?;
            let glow_item =
                CheckMenuItem::with_id(app, "toggle_glow", "流光边框", true, true, None::<&str>)?;
            let lock_item =
                CheckMenuItem::with_id(app, "toggle_lock", "锁定位置", true, false, None::<&str>)?;

            let sep2 = PredefinedMenuItem::separator(app)?;
            // 下方常规按钮（文字会自动与上方的“灵动岛”对齐）
            let console_item =
                MenuItem::with_id(app, "open_console", "打开控制台", true, None::<&str>)?;
            let reset_item =
                MenuItem::with_id(app, "reset_pos", "重置灵动岛位置", true, None::<&str>)?;
            let sep3 = PredefinedMenuItem::separator(app)?;
            let quit_item = MenuItem::with_id(app, "quit", "✕ 强制退出", true, None::<&str>)?;

            // 2. 存入引用
            {
                *app.state::<AppState>().tray_items.lock().unwrap() = Some((
                    island_item.clone(),
                    quiet_item.clone(),
                    glow_item.clone(),
                    lock_item.clone(),
                ));
            }

            // 3. 构建菜单
            let tray_menu = Menu::with_items(
                app,
                &[
                    &title_item,
                    &sep1,
                    &island_item,
                    &quiet_item,
                    &glow_item,
                    &lock_item,
                    &sep2,
                    &console_item,
                    &reset_item,
                    &sep3,
                    &quit_item,
                ],
            )?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("NetSpeed Dynamic Pro")
                .menu(&tray_menu)
                .on_menu_event(move |app_handle, event| {
                    match event.id().as_ref() {
                        "quit" => {
                            if let Ok(mut process_guard) = TASKBAR_PLUGIN_PROCESS.lock() {
                                if let Some(mut child) = process_guard.take() {
                                    let _ = child.kill();
                                }
                            }
                            if let Ok(mut process_guard) = FPS_PLUGIN_PROCESS.lock() {
                                if let Some(mut child) = process_guard.take() {
                                    let _ = child.kill();
                                }
                            }
                            app_handle.exit(0);
                        }
                        "open_console" => {
                            if let Some(main_window) = app_handle.get_webview_window("main") {
                                let _ = main_window.show();
                                let _ = main_window.unminimize();
                                let _ = main_window.set_focus();
                            }
                        }
                        // 抛出事件给前端
                        "toggle_island" => {
                            let _ = app_handle.emit("tray-toggle-island", ());
                        }
                        "toggle_quiet" => {
                            let _ = app_handle.emit("tray-toggle-quiet", ());
                        }
                        "toggle_glow" => {
                            let _ = app_handle.emit("tray-toggle-glow", ());
                        }
                        "toggle_lock" => {
                            let _ = app_handle.emit("tray-toggle-lock", ());
                        }
                        "reset_pos" => {
                            let _ = app_handle.emit("tray-reset-pos", ());
                        }
                        _ => {}
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

            if let Some(main_window) = app.get_webview_window("main") {
                let w_clone = main_window.clone();
                main_window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = w_clone.hide();
                    }
                });
            }

            Ok(())
        })
        // 拆分 build 和 run，拦截全局退出事件
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| {
            if let tauri::RunEvent::Exit = event {
                // 程序主循环生命周期结束时，确保掐死任务栏子进程
                if let Ok(mut process_guard) = TASKBAR_PLUGIN_PROCESS.lock() {
                    if let Some(mut child) = process_guard.take() {
                        let _ = child.kill();
                    }
                }
                if let Ok(mut process_guard) = FPS_PLUGIN_PROCESS.lock() {
                    if let Some(mut child) = process_guard.take() {
                        let _ = child.kill();
                    }
                }
            }
        });
}
