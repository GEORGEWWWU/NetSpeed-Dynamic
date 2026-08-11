use std::sync::atomic::{AtomicU32, AtomicBool, Ordering};

// 静态状态迁移至此
static LAST_NOTIFICATION_ID: AtomicU32 = AtomicU32::new(0);
static IS_NOTIF_INIT: AtomicBool = AtomicBool::new(false);

#[derive(serde::Serialize, Clone)]
pub struct ToastData {
    pub app_name: String,
    pub title: String,
    pub body: String,
    pub aumid: String,
}

#[tauri::command]
pub async fn fetch_latest_notification() -> Result<Option<ToastData>, String> {
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
            let mut app_name = notif.AppInfo()
                .and_then(|info| info.DisplayInfo())
                .and_then(|dinfo| dinfo.DisplayName())
                .map(|name| name.to_string())
                .unwrap_or_else(|_| "系统通知".to_string());

            let aumid = notif.AppInfo()
                .and_then(|info| info.AppUserModelId())
                .map(|id| id.to_string())
                .unwrap_or_default();

            // 手机连接同步的微信通知以 com.tencent.mm 标识；按微信本身展示图标和名称。
            if aumid.to_lowercase().contains("com.tencent.mm") {
                app_name = "微信".to_string();
            }

            if let Ok(notification) = notif.Notification() {
                if let Ok(visual) = notification.Visual() {
                    // QQ 使用 ToastGeneric，微信可能使用其他 Windows 通知模板；统一读取全部模板。
                    if let Ok(bindings) = visual.Bindings() {
                        for toast_binding in bindings {
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

                                    return Ok(Some(ToastData { app_name, title, body, aumid }));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(None)
}
