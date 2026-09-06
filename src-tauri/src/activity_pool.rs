// 活动池模块：外部服务通过 47300 端口的 HTTP API 创建/更新/删除"活动"，
// Rust 侧维护活动池状态，并以 30Hz 节流将快照定向推送给 widget 灵动岛窗口。
//
// HTTP API（均绑定 127.0.0.1:47300）：
//   POST   /api/activities               创建活动（仅创建：同 id 已存在则 409）
//   PATCH  /api/activities/{id}          部分更新（字段缺失=不改，null/空串=清除）
//   DELETE /api/activities/{id}          删除单个活动
//   DELETE /api/activities               清空活动池
//   GET    /api/activities               获取当前快照（调试用）
//
//   约束：extra 字段大小上限 EXTRA_MAX_BYTES（16KB），超限请求返回 400。
//
// 事件推送（30Hz 节流，定向 emit 到 "widget" 窗口）：
//   event: "activity-pool"
//   payload: { "ts": ..., "activities": [Activity...] } 按 (priority, updated) 排序

use axum::extract::State as AxState;
use axum::routing::{patch, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;

/// 活动池 HTTP 服务端口
const ACTIVITY_HTTP_PORT: u16 = 47300;
/// 前端窗口 label
const WIDGET_LABEL: &str = "widget";
/// 节流推送间隔（30Hz）
const TICK_INTERVAL: Duration = Duration::from_millis(33);
/// extra 大小上限（序列化后的字节数）。每帧全量快照序列化会携带每个 extra，
/// 无上限的超大 extra 会白白拖累 CPU，故设硬上限拒绝超限写入。
const EXTRA_MAX_BYTES: usize = 16 * 1024;

// ---------- 内部数据模型 ----------

/// 池内活动（含内部时间戳，不直接外发）
#[derive(Debug, Clone)]
struct Activity {
    id: String,
    title: String,
    subtitle: String,
    kind: String,
    icon: String,
    color: String,
    /// Some(v)=确定进度 0-100；None=不确定进度(indeterminate)
    progress: Option<u8>,
    /// false=隐藏进度条；true=显示（默认）
    show_progress: bool,
    /// 越大越优先展示（同优先级按更新时间倒序）
    priority: i32,
    updated_ms: u64,
    /// Some = 绝对过期时间戳(ms)，到期自动从池中移除
    expires_at: Option<u64>,
    /// 任意扩展字段，前端可自由消费
    extra: Option<serde_json::Value>,
}

impl Activity {
    /// 是否没有任何可展示内容（池里留着它没有意义）
    fn is_blank(&self) -> bool {
        self.title.is_empty()
            && self.subtitle.is_empty()
            && self.kind.is_empty()
            && self.icon.is_empty()
            && self.color.is_empty()
            && self.progress.is_none()
            && self.extra.is_none()
    }
}

/// 外发给前端的一条活动
#[derive(Debug, Clone, Serialize)]
struct ActivityOut {
    id: String,
    title: String,
    subtitle: String,
    kind: String,
    /// 空串 = 无图标（前端用 kind 兜底）
    icon: String,
    color: String,
    progress: Option<u8>,
    /// false=隐藏进度条；true=显示（默认）
    show_progress: bool,
    priority: i32,
    /// 剩余存活毫秒；null = 永不过期
    remaining_ms: Option<u64>,
    extra: Option<serde_json::Value>,
}

impl From<&Activity> for ActivityOut {
    fn from(a: &Activity) -> Self {
        let remaining_ms = a.expires_at.map(|e| e.saturating_sub(now_ms()));
        Self {
            id: a.id.clone(),
            title: a.title.clone(),
            subtitle: a.subtitle.clone(),
            kind: a.kind.clone(),
            icon: a.icon.clone(),
            color: a.color.clone(),
            progress: a.progress,
            show_progress: a.show_progress,
            priority: a.priority,
            remaining_ms,
            extra: a.extra.clone(),
        }
    }
}

// ---------- HTTP 请求体 ----------

/// POST 创建活动请求体
#[derive(Debug, Clone, Default, Deserialize)]
struct CreateActivityReq {
    id: String,
    title: Option<String>,
    subtitle: Option<String>,
    kind: Option<String>,
    icon: Option<String>,
    color: Option<String>,
    progress: Option<u8>,
    /// 0=隐藏进度条；1=显示（默认）；null/missing=显示
    show_progress: Option<u8>,
    priority: Option<i32>,
    /// 相对存活时长；已有活动不传则不重置过期
    ttl_ms: Option<u64>,
    extra: Option<serde_json::Value>,
}

/// PATCH 部分更新：外层 Option = 本次是否修改；内层值 = 具体值
#[derive(Debug, Clone, Default, Deserialize)]
struct PatchActivityReq {
    title: Option<Option<String>>,
    subtitle: Option<Option<String>>,
    kind: Option<Option<String>>,
    icon: Option<Option<String>>,
    color: Option<Option<String>>,
    progress: Option<Option<u8>>,
    /// 0=隐藏进度条；1=显示；null/missing=不改
    show_progress: Option<u8>,
    priority: Option<i32>,
    ttl_ms: Option<u64>,
    extra: Option<Option<serde_json::Value>>,
}

// ---------- 活动池 ----------

struct PoolInner {
    items: HashMap<String, Activity>,
    /// 有改动待推送（节流后合并推送）
    dirty: bool,
}

#[derive(Clone)]
struct ServerState {
    pool: Arc<Mutex<PoolInner>>,
    app: AppHandle,
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn clamp_progress(v: u8) -> u8 {
    v.min(100)
}

/// extra 值是否超过大小上限（按真实序列化字节数统计，与推送时消耗一致）
fn extra_exceeds_limit(v: &serde_json::Value) -> bool {
    serde_json::to_vec(v)
        .map(|vec| vec.len() > EXTRA_MAX_BYTES)
        // Value 通常总能序列化；万一失败按超限保守拒绝
        .unwrap_or(true)
}

fn extra_limit_err() -> (axum::http::StatusCode, String) {
    (
        axum::http::StatusCode::BAD_REQUEST,
        format!("extra 超过 {} 字节上限", EXTRA_MAX_BYTES),
    )
}

/// 快照：清理过期 + 排序 + 转外发结构（调用方需持有锁）
fn build_snapshot(inner: &PoolInner) -> Vec<ActivityOut> {
    let now = now_ms();
    let mut list: Vec<&Activity> = inner
        .items
        .values()
        // 过滤过期项与无展示内容的项
        .filter(|a| a.expires_at.map_or(true, |e| e > now))
        .filter(|a| !a.is_blank())
        .collect();
    // 优先级降序，其次更新时间降序（越新越靠前）
    list.sort_by(|a, b| {
        b.priority
            .cmp(&a.priority)
            .then_with(|| b.updated_ms.cmp(&a.updated_ms))
            .then_with(|| a.id.cmp(&b.id))
    });
    list.into_iter().map(ActivityOut::from).collect()
}

/// 从池中移除已过期活动，返回是否有移除
fn purge_expired(inner: &mut PoolInner) -> bool {
    let now = now_ms();
    let before = inner.items.len();
    inner.items.retain(|_, a| a.expires_at.map_or(true, |e| e > now));
    inner.items.len() != before
}

// ---------- HTTP handlers ----------

async fn create_activity(
    AxState(state): AxState<ServerState>,
    Json(req): Json<CreateActivityReq>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    if req.id.trim().is_empty() {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            "id 不能为空".into(),
        ));
    }
    // 大小校验无需持锁，放最前
    if req.extra.as_ref().is_some_and(extra_exceeds_limit) {
        return Err(extra_limit_err());
    }

    let mut inner = state.pool.lock().await;
    // 仅创建：id 已被占用时返回冲突（更新请走 PATCH，重建请先 DELETE）
    if inner.items.contains_key(&req.id) {
        return Err((
            axum::http::StatusCode::CONFLICT,
            format!("活动 {} 已存在", req.id),
        ));
    }

    let now = now_ms();
    let activity = Activity {
        id: req.id.clone(),
        title: req.title.unwrap_or_default(),
        subtitle: req.subtitle.unwrap_or_default(),
        kind: req.kind.unwrap_or_default(),
        icon: req.icon.unwrap_or_default(),
        color: req.color.unwrap_or_default(),
        progress: req.progress.map(clamp_progress),
        show_progress: req.show_progress.map(|v| v != 0).unwrap_or(true),
        priority: req.priority.unwrap_or_default(),
        updated_ms: now,
        expires_at: req.ttl_ms.map(|ttl| now.saturating_add(ttl)),
        extra: req.extra,
    };
    inner.items.insert(req.id.clone(), activity);

    inner.dirty = true;
    drop(inner);

    Ok(Json(serde_json::json!({ "ok": true, "id": req.id })))
}

async fn patch_activity(
    AxState(state): AxState<ServerState>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(req): Json<PatchActivityReq>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    // PATCH 传 null 表示清除，不校验；只有写入实际值才查大小。校验无需持锁，放最前。
    if req.extra.as_ref().and_then(|o| o.as_ref()).is_some_and(extra_exceeds_limit) {
        return Err(extra_limit_err());
    }

    let mut inner = state.pool.lock().await;

    let entry = match inner.items.get_mut(&id) {
        Some(e) => e,
        None => {
            return Err((
                axum::http::StatusCode::NOT_FOUND,
                format!("活动 {} 不存在", id),
            ))
        }
    };

    let now = now_ms();
    // 文本/图标/颜色/kind：Some(Some(s)) 覆盖；Some(None) 清除
    let apply_opt = |cur: &mut String, v: &Option<Option<String>>| {
        if let Some(inner_v) = v {
            *cur = inner_v.clone().unwrap_or_default();
        }
    };
    apply_opt(&mut entry.title, &req.title);
    apply_opt(&mut entry.subtitle, &req.subtitle);
    apply_opt(&mut entry.kind, &req.kind);
    apply_opt(&mut entry.icon, &req.icon);
    apply_opt(&mut entry.color, &req.color);

    // progress：Some(Some(v)) 覆盖；Some(None) 转为不确定进度
    if let Some(p) = req.progress {
        entry.progress = p.map(clamp_progress);
    }
    // show_progress：0=隐藏，1=显示，null/missing=不改
    if let Some(v) = req.show_progress {
        entry.show_progress = v != 0;
    }
    if let Some(p) = req.priority {
        entry.priority = p;
    }
    if let Some(ttl) = req.ttl_ms {
        entry.expires_at = Some(now.saturating_add(ttl));
    }
    // extra：Some(Some(v)) 覆盖；Some(None) 清除
    if let Some(v) = req.extra {
        entry.extra = v;
    }

    entry.updated_ms = now;
    inner.dirty = true;
    drop(inner);

    Ok(Json(serde_json::json!({ "ok": true, "id": id })))
}

async fn delete_activity(
    AxState(state): AxState<ServerState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Json<serde_json::Value> {
    let mut inner = state.pool.lock().await;
    let existed = inner.items.remove(&id).is_some();
    if existed {
        inner.dirty = true;
    }
    drop(inner);
    Json(serde_json::json!({ "ok": existed, "id": id }))
}

async fn clear_activities(AxState(state): AxState<ServerState>) -> Json<serde_json::Value> {
    let mut inner = state.pool.lock().await;
    let had = !inner.items.is_empty();
    inner.items.clear();
    if had {
        inner.dirty = true;
    }
    drop(inner);
    Json(serde_json::json!({ "ok": true, "count": 0 }))
}

async fn list_activities(AxState(state): AxState<ServerState>) -> Json<serde_json::Value> {
    let inner = state.pool.lock().await;
    let snap = build_snapshot(&inner);
    drop(inner);
    Json(serde_json::json!({ "activities": snap, "ts": now_ms() }))
}

// ---------- 启动 ----------

/// 在 Tauri 运行时内启动：1) 47300 HTTP 服务；2) 30Hz 节流推送任务
pub fn start(app: AppHandle) {
    let state = ServerState {
        pool: Arc::new(Mutex::new(PoolInner {
            items: HashMap::new(),
            dirty: false,
        })),
        app: app.clone(),
    };

    // HTTP 服务
    let router = Router::new()
        .route(
            "/api/activities",
            post(create_activity)
                .get(list_activities)
                .delete(clear_activities),
        )
        .route(
            "/api/activities/{id}",
            patch(patch_activity).delete(delete_activity),
        )
        .with_state(state.clone());

    tauri::async_runtime::spawn(async move {
        let addr = format!("127.0.0.1:{}", ACTIVITY_HTTP_PORT);
        match tokio::net::TcpListener::bind(&addr).await {
            Ok(listener) => {
                eprintln!("[activity-pool] HTTP 服务已启动: http://{}", addr);
                let _ = axum::serve(listener, router).await;
            }
            Err(e) => {
                eprintln!("[activity-pool] 绑定 {} 失败: {}", addr, e);
            }
        }
    });

    // 30Hz 节流推送（Tauri 事件本身是高层的 JSON IPC，瓶颈在推送频率，
    // 用固定打点把 33ms 内的 N 次改动合并成 1 次快照推送）
    tauri::async_runtime::spawn(async move {
        let mut ticker = tokio::time::interval(TICK_INTERVAL);
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        // 记录上次是否推过非空快照：只有"非空 → 空"才补推一次空快照通知前端收起岛体；
        // 持续为空则完全静默（空池不空推），避免清空池 / 空活动残留时反复空推 []。
        let mut had_content = false;
        loop {
            ticker.tick().await;
            let mut inner = state.pool.lock().await;
            // 处于"池空且已收起"稳定态：没有活动、没有过期可清、没有内容可推，直接静默
            if inner.items.is_empty() && !had_content {
                continue;
            }
            // 顺带清理过期活动（可能产生脏标记）
            let purged = purge_expired(&mut inner);
            if !inner.dirty && !purged {
                continue;
            }
            inner.dirty = false;
            let snapshot = build_snapshot(&inner);
            drop(inner);

            if snapshot.is_empty() {
                // 空快照仅在"刚由非空变空"时推一次
                if !had_content {
                    continue;
                }
                had_content = false;
            } else {
                had_content = true;
            }

            let payload = serde_json::json!({
                "ts": now_ms(),
                "activities": snapshot,
            });
            let _ = state.app.emit_to(WIDGET_LABEL, "activity-pool", payload);
        }
    });
}
