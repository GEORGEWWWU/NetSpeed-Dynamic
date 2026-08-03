use serde::Serialize;
use std::time::Duration;
use sysinfo::System;
use tauri::{AppHandle, Emitter};
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::Win32::Media::Audio::{eConsole, eRender, IMMDeviceEnumerator, MMDeviceEnumerator};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED,
};
use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};

// 定义符合 Vue 前端期待的 JSON 结构体
#[derive(Clone, Serialize)]
struct BatteryPayload {
    state: String,
    percent: u8,
}
#[derive(Clone, Serialize)]
struct ResourcePayload {
    cpu: u8,
    ram: u8,
}

pub fn start_monitor(app: AppHandle) {
    std::thread::spawn(move || {
        // 初始化 COM 接口以获取音频
        unsafe {
            let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
        }

        // 初始化 System 监控
        let mut sys = System::new_all();
        sys.refresh_cpu_usage(); // 必须先刷新一次，否则第一次读取全是 0

        let mut last_volume = get_system_volume().unwrap_or(-1.0);
        let mut last_power_state = 255; // 255 = 未知状态
        let mut last_battery_percent = 255; // 255 = 未知电量

        // 启动时先拉取一次真实状态打底
        if let Some((ac_status, battery_percent)) = get_power_status() {
            last_power_state = ac_status;
            last_battery_percent = battery_percent;
        }

        loop {
            std::thread::sleep(Duration::from_millis(800)); // 每 800ms 检查一次

            // 读取并广播 CPU 与 RAM
            sys.refresh_cpu_usage();
            sys.refresh_memory();

            let cpu_usage = sys.global_cpu_info().cpu_usage().round() as u8;
            let total_mem = sys.total_memory();
            let used_mem = sys.used_memory();
            let ram_percent = if total_mem > 0 {
                ((used_mem as f64 / total_mem as f64) * 100.0).round() as u8
            } else {
                0
            };

            let _ = app.emit(
                "resource-event",
                ResourcePayload {
                    cpu: cpu_usage,
                    ram: ram_percent,
                },
            );

            // 检查音量变化
            if let Some(current_volume) = get_system_volume() {
                if (current_volume - last_volume).abs() > 0.01 && last_volume != -1.0 {
                    let vol_percent = (current_volume * 100.0).round() as i32;
                    let _ = app.emit("system-event", format!("当前系统音量 {}%", vol_percent));
                }
                last_volume = current_volume;
            }

            // 检查电源状态与电量变化
            if let Some((current_power, current_percent)) = get_power_status() {
                // 【情况 A】电源插入/拔出状态发生了改变
                if current_power != last_power_state && last_power_state != 255 {
                    if current_power == 1 {
                        // 插入电源：发送专属电池事件（触发灵动岛绿色充电 SVG）
                        let _ = app.emit(
                            "battery-event",
                            BatteryPayload {
                                state: "charging".to_string(),
                                percent: current_percent,
                            },
                        );
                    } else if current_power == 0 {
                        // 拔出电源：发送普通系统文本（触发原本的普通黑白系统通知）
                        let _ = app.emit("system-event", "正在使用电池供电");
                    }
                }

                // 【情况 B】正在使用电池（未插电），且电量正在下降
                if current_power == 0 && current_percent < last_battery_percent {
                    // 低电量防抖机制：仅在跌破这几个关键节点时，触发红色警告
                    if current_percent <= 20 && [20, 15, 10, 5].contains(&current_percent) {
                        let _ = app.emit(
                            "battery-event",
                            BatteryPayload {
                                state: "discharging".to_string(),
                                percent: current_percent,
                            },
                        );
                    }
                }

                last_power_state = current_power;
                last_battery_percent = current_percent;
            }
        }
    });
}

// 辅助函数：获取 Windows 系统音量 (0.0 到 1.0)
fn get_system_volume() -> Option<f32> {
    unsafe {
        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).ok()?;
        let device = enumerator.GetDefaultAudioEndpoint(eRender, eConsole).ok()?;
        let endpoint_volume: IAudioEndpointVolume = device.Activate(CLSCTX_ALL, None).ok()?;

        let volume = endpoint_volume.GetMasterVolumeLevelScalar().ok()?;
        Some(volume)
    }
}

// 辅助函数：同时获取电源插入状态和剩余电量
fn get_power_status() -> Option<(u8, u8)> {
    unsafe {
        let mut status: SYSTEM_POWER_STATUS = std::mem::zeroed();
        if GetSystemPowerStatus(&mut status).is_ok() {
            // 返回元组: (ACLineStatus, BatteryLifePercent)
            // ACLineStatus: 0 = 使用电池, 1 = 插入电源
            // BatteryLifePercent: 0 ~ 100
            Some((status.ACLineStatus, status.BatteryLifePercent))
        } else {
            None
        }
    }
}
