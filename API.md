# 活动池（Activity Pool）API 文档

> NetSpeed Dynamic Pro 的「活动池」子系统：外部服务（本地程序 / 脚本 / AI Agent）通过
> 本机 HTTP 接口创建、更新、删除"活动"，灵动岛 widget 以消息弹窗形式实时展示。
>
> 适用场景：下载/上传进度、文件同步、后台任务、构建与转码进度、实时状态提示等
> 需要在灵动岛上持续可见的活动。
>
> **通道划分（先记住这一句）**：链路分两段——外部与本应用之间是 **HTTP**（`127.0.0.1:47300`，即第 4 节全部端点，外部可调用）；数据上岛则靠 **Rust → widget 的进程内事件**（非 HTTP、无端口，外部不可见，见第 5 节）。

---

## 目录

1. [架构总览](#1-架构总览)
2. [端口与服务约定](#2-端口与服务约定)
3. [数据模型](#3-数据模型)
   - 3.1 POST 请求字段详解
   - 3.2 重复创建规则（409）
   - 3.3 PATCH 部分更新语义
   - 3.4 快照对象字段
   - 3.5 排序与过滤规则
   - 3.6 活动生命周期
4. [HTTP 接口详解](#4-http-接口详解)
5. [前端事件契约（内部通道，非 HTTP）](#5-前端事件契约内部通道非-http)
6. [前端展示行为](#6-前端展示行为)
7. [状态码与错误格式](#7-状态码与错误格式)
8. [调用示例](#8-调用示例)
9. [完整任务生命周期示例](#9-完整任务生命周期示例)
10. [边界与注意事项](#10-边界与注意事项)
11. [常见问题排查](#11-常见问题排查)

---

## 1. 架构总览

```
┌─────────────┐   HTTP (127.0.0.1:47300)   ┌──────────────────────────────┐
│  外部服务    │ ─ POST / PATCH / DELETE ─▶ │  Rust 后端（Tauri，axum 0.8）  │
│ 脚本 / 程序  │                            │  ┌────────────────────────┐  │
│ AI Agent    │                            │  │   活动池（内存状态）      │  │
└─────────────┘                            │  │ · 增/删/改              │  │
                                           │  │ （409 防重 / 改走 PATCH）│  │
                                           │  │ · ttl 自动过期清理       │  │
                                           │  │ · 优先级/更新时间排序     │  │
                                           │  └────────────────────────┘  │
                                           │            │ 30Hz 节流快照    │
                                           │            ▼                 │
                                           │   emit_to("widget")          │
                                           └─────────────┬────────────────┘
                                                         │ 事件: activity-pool
                                                         ▼
                                             ┌──────────────────────────┐
                                             │   widget 灵动岛窗口        │
                                             │  消息弹窗卡片 + 进度条动画   │
                                             └──────────────────────────┘
```

**上图是两段相互独立的通道，别混为一谈：**

| 段 | 传输方式 | 传输范围 | 谁能收发 | 对应章节 |
|---|---|---|---|---|
| ① 入站通道 | HTTP/1.1（axum 服务，绑定 `127.0.0.1:47300`） | 进程外：外部服务 → 本应用 | **任何本机进程**（唯一对外暴露面） | [第 4 节](#4-http-接口详解) |
| ② 出站通道 | Tauri 事件 `emit_to("widget")`，事件名 `activity-pool` | **进程内**：Rust 后端 → widget 窗口 | 仅本应用内代码，**外部无法连接/订阅** | [第 5 节](#5-前端事件契约内部通道非-http) |

> 一句话判断：出现 `127.0.0.1:47300` 的是 **HTTP**（你能调）；出现 `activity-pool` 事件的是**进程内通道**（外部够不着，只是文档解释给前端维护者听的）。

**核心设计决策**

| 决策点 | 方案 | 理由 |
|---|---|---|
| 入站通道 | 本机 HTTP（axum） | 任意语言/工具可调用；低频创建/删除 + 高频刷进度都够用 |
| 内部推送 | Rust → 前端定向事件 `emit_to("widget")` | 前端从不直连外部服务，所有状态经 Rust 中转，单一事实来源 |
| 推送频率 | **30Hz 节流**（33ms 固定打点） | 33ms 内的 N 次 HTTP 改动合并成 1 次快照推送，高刷进度时前端也只收到 30 帧/秒 |
| 过期机制 | `ttl_ms` 由服务端计时 | 外部忘记删除也能自动消失，不会残留占岛 |
| 监听地址 | `127.0.0.1:47300` | 仅本机可访问；避开 47290（WS 歌词）/ 47291（任务栏 WS）/ 47292（FPS UDP） |

---

## 2. 端口与服务约定

| 项 | 值 |
|---|---|
| Base URL | `http://127.0.0.1:47300` |
| 协议 | HTTP/1.1，请求体统一 `application/json`，字符编码 **UTF-8** |
| 生命周期 | 随应用启动自动拉起，应用退出即停止；**数据仅存内存，重启清空** |
| 鉴权 | 无（仅绑定回环地址，局域网/外网不可达；本机其它进程可调用，属有意设计） |
| 并发 | 多线程安全；同一活动并发写以最后到达者为准（覆盖式合并） |

---

## 3. 数据模型

一个「活动」抽象为一条可展示的任务状态。字段分三类：**标识**（id）、**展示**（title/subtitle/kind/icon/color/progress/extra）、**调度**（priority/ttl_ms）。

### 3.1 POST 请求字段详解

```jsonc
// POST /api/activities 请求体完整示例
{
  "id": "dl-20260905-1",        // 必填：活动唯一标识
  "title": "正在下载",           // 主标题（第 1 行，加粗）
  "subtitle": "NetSpeed-Setup.exe", // 副标题（第 2 行，较小）
  "kind": "下载",                // 类型徽标（title 右侧的小标签）
  "icon": "https://example.com/dl.png", // 图标 URL
  "color": "#00C853",            // 强调色（图标底色 + 进度条颜色）
  "progress": 42,                // 进度 0~100；null 表示不确定进度
  "priority": 10,                // 优先级，越大越先展示
  "ttl_ms": 3600000,             // 存活时长，到期自动移除
  "extra": { "url": "https://...", "speed": "3.2MB/s" } // 任意扩展
}
```

| 字段 | 类型 | 必填 | 默认 | 说明 |
|---|---|---|---|---|
| `id` | string | **是** | — | 唯一标识。**同 id 重复 POST 返回 `409`**（见 3.2）。建议语义化命名，如 `dl-20260905-1`、`sync-dropbox` |
| `title` | string | 否 | `""` | 主标题。空串时前端显示兜底文案「任务进行中」 |
| `subtitle` | string | 否 | `""` | 副标题，用于文件名、进度明细、URL 等 |
| `kind` | string | 否 | `""` | 类型徽标，显示在标题右侧（如 `下载`/`上传`/`转码`/`同步`）。空则不显示徽标 |
| `icon` | string | 否 | `""` | 图标地址。推荐 **http(s) URL 或 data: URI**（`<img src>` 直载，不受 CORS 限制）。本地文件需先经 Tauri `convertFileSrc` 转成 asset 协议再传入。空串显示默认活动图标（心跳图形） |
| `color` | string | 否 | `""` | 强调色，**任意 CSS 颜色值**（`#00C853`、`rgb(...)`、`hsl(...)`）。作用于：头像图标底色、进度条填充色、kind 徽标底色 |
| `progress` | number\|null | 否 | `null` | 进度百分比 **0–100**。超出自动截断。`null` = 不确定进度（前端显示流动动画，适合"处理中/等待中"） |
| `priority` | number | 否 | `0` | 可正可负。多活动并存时数字大的优先上岛（见 3.5 排序） |
| `ttl_ms` | number | 否 | 永不过期 | 相对**服务端收到时刻**的存活毫秒，到期自动移除。已有活动不传 = 保留原过期时间（不会误刷新倒计时） |
| `extra` | object\|null | 否 | `null` | 任意 JSON 扩展字段，服务端原样存储、原样透传，前端可自由消费。**大小上限 16KB**（序列化字节数），超限请求返回 `400`（见 7） |

### 3.2 重复创建规则（POST 仅创建）

`POST` 是**仅创建**语义（**非 upsert**）：同 id 已存在时**不会覆盖**原活动，返回 `409 Conflict`，body：`活动 {id} 已存在`。

```text
第 1 次 POST: { id: "a", title: "下载中", progress: 10 }
   → 200，创建成功: title="下载中", progress=10

第 2 次 POST: { id: "a", progress: 50 }        ← 重复创建
   → 409：活动 a 已存在，原内容保持不变
```

⚠️ 因此：
- 更新已存在活动用 [PATCH](#33-patch-部分更新语义)（清空文本/图标/extra 传 `null`）。
- 需要整体重建时先 `DELETE` 再 `POST`。
- 重复 POST 的作用是**暴露 id 冲突**（如误用同名任务），不会被悄悄覆盖。

### 3.3 PATCH 部分更新语义

```jsonc
// PATCH /api/activities/{id} 请求体
{
  "progress": 66,        // 传数字 → 覆盖进度
  "subtitle": null,      // 传 null  → 清空为 ""
  "extra": null          // 传 null  → 清除 extra
}
```

| 字段 | 传值 | 传 `null` | 缺失 |
|---|---|---|---|
| `title`/`subtitle`/`kind`/`icon`/`color` | 覆盖为字符串 | 清空为 `""` | 不改 |
| `progress` | 覆盖为确定进度 | 转为**不确定进度**（`null`） | 不改 |
| `priority`/`ttl_ms` | 覆盖 | 不支持置空 | 不改 |
| `extra` | 整体替换 | 清除 | 不改 |

> PATCH 只允许修改已存在的活动；id 不存在返回 `404`。典型用途就是高频刷 `progress`。

### 3.4 快照对象字段

GET 与事件推送中的每个活动对象：

```json
{
  "id": "dl-1",
  "title": "正在下载",
  "subtitle": "NetSpeed-Setup.exe",
  "kind": "下载",
  "icon": "https://example.com/dl.png",
  "color": "#00C853",
  "progress": 66,
  "priority": 10,
  "remaining_ms": 12400,
  "extra": { "speed": "3.2MB/s" }
}
```

| 字段 | 说明 |
|---|---|
| `remaining_ms` | 距自动过期的剩余毫秒；`null` = 永不过期。每帧随快照刷新，前端可做倒计时/到期动画 |
| 其余字段 | 与 3.1 语义一致；`icon`/`color` 空串表示"无"，前端走默认样式 |

### 3.5 排序与过滤规则

快照输出前统一处理：

1. **过滤过期项**：`expires_at <= now` 的移除（即 `remaining_ms` 已到 0）；
2. **过滤空内容项**：title/subtitle/kind/icon/color/progress/extra 全空的活动不进入快照；
3. **排序**：`priority` 降序 → 最近更新者优先 → `id` 字典序兜底。

**排序结果中第 1 个（`activities[0]`）就是当前灵动岛应展示的活动。**

### 3.6 活动生命周期

```text
创建 ──▶ 展示中 ──▶ (PATCH 更新 / 刷进度)  ──▶ 结束
  │            │                             │
  │            ├─ 过期（ttl_ms 到点） 自动移除 ─┘
  │            └─ 外部显式 DELETE 移除 ────────┘
  └─ 应用退出 → 全部清空（内存存储）
```

---

## 4. HTTP 接口详解

> **这是外部服务唯一需要看、且唯一能调用的一节。** 以下端点全部走 HTTP/1.1、
> 全部挂在 `http://127.0.0.1:47300`，与 [第 5 节](#5-前端事件契约内部通道非-http) 的进程内事件无关。

### 4.1 `POST /api/activities` — 创建活动

```http
POST http://127.0.0.1:47300/api/activities
Content-Type: application/json

{ "id": "dl-1", "title": "正在下载", "progress": 15, "ttl_ms": 60000 }
```

**成功响应 `200 OK`**

```json
{ "ok": true, "id": "dl-1" }
```

**失败响应**

| 场景 | 状态码 | 响应体（text/plain） |
|---|---|---|
| body 不是合法 JSON / 字段类型错误 | `400` | 解析错误描述 |
| `id` 缺失或为空字符串 | `400` | `id 不能为空` |
| `extra` 超过 16KB 上限 | `400` | `extra 超过 16384 字节上限` |
| `id` 已存在（重复创建） | `409` | `活动 dl-1 已存在` |

### 4.2 `PATCH /api/activities/{id}` — 部分更新

```http
PATCH http://127.0.0.1:47300/api/activities/dl-1
Content-Type: application/json

{ "progress": 66 }
```

**成功响应 `200 OK`**

```json
{ "ok": true, "id": "dl-1" }
```

**失败响应**：id 不存在 → `404 Not Found`，body：`活动 dl-1 不存在`；`extra` 传值超过 16KB → `400`（同 [4.1](#41-post-apiactivities--创建活动) 的错误表）

### 4.3 `DELETE /api/activities/{id}` — 删除单个活动

```http
DELETE http://127.0.0.1:47300/api/activities/dl-1
```

**成功响应 `200 OK`**

```json
{ "ok": true, "id": "dl-1" }
```

> 删除不存在的 id 不报错：返回 `{ "ok": false, "id": "..." }`（幂等）。

### 4.4 `DELETE /api/activities` — 清空活动池

```http
DELETE http://127.0.0.1:47300/api/activities
```

**成功响应 `200 OK`**

```json
{ "ok": true, "count": 0 }
```

> 清空后下一帧（≤33ms）前端即收起活动卡片。调试利器。

### 4.5 `GET /api/activities` — 查看当前快照

```http
GET http://127.0.0.1:47300/api/activities
```

**成功响应 `200 OK`**

```json
{
  "activities": [
    { "id": "dl-1", "title": "正在下载", "...": "同 3.4" }
  ],
  "ts": 1788619734360
}
```

> `ts` 为服务端当前 epoch 毫秒，便于调试同步。此接口不触发前端推送，仅查询。

---

## 5. 前端事件契约（内部通道，非 HTTP）

> ⚠️ **本节不是 HTTP API，也不是给外部调用方用的。**
> 该事件由 Rust 在**同一进程内**推送给 widget 窗口：无端口、不走网络、无鉴权概念。
> 外部服务能调用的只有 [第 4 节](#4-http-接口详解) 的 HTTP 接口（`127.0.0.1:47300`）；
> 本节仅作实现说明，供前端维护者理解数据是如何上岛的。

| 项 | 值 |
|---|---|
| 传输方式 | Tauri 进程内事件（`event.emit_to`），**非 HTTP** |
| 事件名 | `activity-pool` |
| 推送目标 | 仅 `widget` 窗口（`emit_to` 定向，不会广播到 main 控制台窗口） |
| 推送频率 | 30Hz（33ms 固定打点；**状态无变化时静默**，空池不空推） |
| Payload | `{ "ts": <epoch_ms>, "activities": [ <活动对象，同 3.4> ] }` |

前端示例（TypeScript）：

```ts
import { listen } from '@tauri-apps/api/event';

interface ActivityData {
    id: string;
    title: string;
    subtitle: string;
    kind: string;
    icon: string;
    color: string;
    progress: number | null;
    priority: number;
    remaining_ms: number | null;
    extra: unknown;
}

const stop = await listen<{ ts: number, activities: ActivityData[] }>('activity-pool', (event) => {
    const list = event.payload.activities;
    const current = list[0]; // 当前应展示的活动
    // ...更新 UI
});
```

---

## 6. 前端展示行为

活动池接入灵动岛后的实际行为（widget 窗口）：

| 场景 | 表现 |
|---|---|
| 池由空 → 非空 | 岛体**展开**到活动卡片宽度（≈max(设置的消息展开宽, 320px)×70px），顶掉正在显示的系统消息/音乐展开态 |
| 持续刷新 | 进度条与百分比随 30Hz 快照实时更新（`transition: width 0.12s` 平滑过渡）；换活动（新 id 上岛）自动切换内容 |
| `progress: null` | 进度条显示**流动动画**（不确定进度） |
| 池变空 | 岛体**自动收起**，回落到底部基础显示（网速 / 音乐 / 自定义内容等） |
| 与消息通知冲突 | 活动展示优先级高于系统消息；活动结束后，消息轮询自动恢复 |
| 岛处于隐藏状态（静默模式） | 遵循应用现有静默策略，活动**不强制唤醒**岛体 |

卡片视觉：左侧圆形图标（`color` 着色，缺省用默认图标）→ 右侧「标题 + kind 徽标」/ 副标题 / 底部进度条 + 百分比。

---

## 7. 状态码与错误格式

| 状态码 | 含义 | 常见触发 |
|---|---|---|
| `200 OK` | 成功（所有端点） | — |
| `400 Bad Request` | 请求体不是合法 JSON、字段类型错误、`id` 为空、`extra` 超过 16KB 上限 | 客户端编码错误 / 传了数组 / 塞入超大 extra |
| `409 Conflict` | POST 创建时 `id` 已被占用 | 重复创建同名活动 |
| `404 Not Found` | PATCH/DELETE 单个活动时 id 不存在（PATCH 报错，DELETE 幂等返回 ok:false） | id 拼错 / 已过期删除 |

错误响应体为**纯文本**（非 JSON），如：

```text
id 不能为空
活动 dl-1 不存在
```

---

## 8. 调用示例

前置：应用已启动，HTTP 已监听 `127.0.0.1:47300`（应用日志出现 `[activity-pool] HTTP 服务已启动`）。

### 8.1 Node.js（推荐，天然 UTF-8）

```js
const BASE = 'http://127.0.0.1:47300/api/activities';

// 创建
await fetch(BASE, {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    id: 'dl-1', title: '正在下载', subtitle: 'NetSpeed-Setup.exe',
    kind: '下载', color: '#00C853', progress: 15, priority: 10, ttl_ms: 60000
  })
});

// 刷进度（高频随便刷，后端 30Hz 自动合并）
for (let i = 16; i <= 100; i += 4) {
  await fetch(`${BASE}/dl-1`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ progress: i })
  });
  await new Promise(r => setTimeout(r, 50));
}

// 删除
await fetch(`${BASE}/dl-1`, { method: 'DELETE' });

// 查快照
const snap = await (await fetch(BASE)).json();
console.log(snap.activities);
```

### 8.2 curl（bash / macOS / Linux / Git Bash）

```bash
curl -X POST http://127.0.0.1:47300/api/activities \
  -H "Content-Type: application/json" \
  -d '{"id":"dl-1","title":"正在下载","progress":15,"ttl_ms":60000}'

curl -X PATCH http://127.0.0.1:47300/api/activities/dl-1 \
  -H "Content-Type: application/json" -d '{"progress":66}'

curl -X DELETE http://127.0.0.1:47300/api/activities/dl-1
curl -s http://127.0.0.1:47300/api/activities
```

### 8.3 Windows PowerShell / cmd

> **两个大坑**：
> 1. PowerShell 里 `curl` 是 `Invoke-WebRequest` 的**别名**，没有 `-X/-H/-d` 参数 —— 用 `curl.exe`；
> 2. PowerShell 5.1 把单引号里的 JSON 传给原生程序时会**剥掉内部双引号**、直接把字符串当 body 发还会按默认编码把中文变 `?` —— 最稳的方式是 **JSON 写文件 + UTF-8 字节**：

```powershell
# 方式 A：Invoke-RestMethod + UTF-8 字节（最稳）
$json  = @{ id='dl-1'; title='正在下载'; subtitle='NetSpeed-Setup.exe'; kind='下载'; progress=15; ttl_ms=60000 } | ConvertTo-Json
$bytes = [System.Text.Encoding]::UTF8.GetBytes($json)
Invoke-RestMethod -Uri 'http://127.0.0.1:47300/api/activities' -Method Post `
  -ContentType 'application/json; charset=utf-8' -Body $bytes

# 刷进度
$json2  = @{ progress = 66 } | ConvertTo-Json
$bytes2 = [System.Text.Encoding]::UTF8.GetBytes($json2)
Invoke-RestMethod -Uri 'http://127.0.0.1:47300/api/activities/dl-1' -Method Patch `
  -ContentType 'application/json; charset=utf-8' -Body $bytes2

# 方式 B：curl.exe + JSON 文件
# 先用任意编辑器把 JSON 存为 UTF-8 无 BOM 文件 act.json
curl.exe -X POST http://127.0.0.1:47300/api/activities `
  -H "Content-Type: application/json" --data-binary "@act.json"
```

### 8.4 Python（标准库，无第三方依赖）

```python
import json, urllib.request

BASE = 'http://127.0.0.1:47300/api/activities'

def req(method, path='', body=None):
    data = json.dumps(body, ensure_ascii=False).encode('utf-8') if body is not None else None
    r = urllib.request.Request(BASE + path, data=data, method=method,
                               headers={'Content-Type': 'application/json'})
    with urllib.request.urlopen(r) as resp:
        return json.loads(resp.read())

req('POST', body={'id': 'dl-1', 'title': '正在下载', 'progress': 15, 'ttl_ms': 60000})
req('PATCH', '/dl-1', {'progress': 66})
req('DELETE', '/dl-1')
print(req('GET'))
```

### 8.5 项目自带演示脚本 `test-activity.mjs`

位于项目根目录，Node 18+ 直接运行：

```bash
node test-activity.mjs create            # 创建 "正在下载 15%"（60s 自动过期）
node test-activity.mjs p dl-demo 66      # 刷进度到 66%
node test-activity.mjs p dl-demo 99      # 再刷到 99%
node test-activity.mjs list              # 查看池内容
node test-activity.mjs delete dl-demo    # 删除单个
node test-activity.mjs clear             # 清空活动池
```

---

## 9. 完整任务生命周期示例

以「下载 + 自动过期兜底」为例：

```js
// 1) 任务开始：创建活动，设 1 小时兜底过期
POST { "id": "dl-20260905-1", "title": "正在下载",
       "subtitle": "NetSpeed-Setup.exe", "kind": "下载",
       "color": "#00C853", "priority": 10, "ttl_ms": 3600000 }

// 2) 下载中：高频刷进度（每 100ms 一次；服务端 30Hz 节流推送）
PATCH /api/activities/dl-20260905-1  { "progress": 8 }
PATCH /api/activities/dl-20260905-1  { "progress": 35 }
PATCH /api/activities/dl-20260905-1  { "progress": 87 }

// 3a) 正常完成：DELETE 立即下岛
DELETE /api/activities/dl-20260905-1

// 3b) 异常中断（进程崩了没发 DELETE）：ttl 1 小时后自动移除，不会残留
```

多活动并存时的行为：

```js
// 低优先级通知（download A，priority 0）
POST { "id": "a", "title": "备份中", "progress": null, "priority": 0 }
// 高优先级任务（转码 B，priority 100）→ 立即顶替 A 上岛
POST { "id": "b", "title": "正在转码", "subtitle": "clip.mov", "priority": 100 }
// B 完成删除 → A 自动上岛（无需重推，快照实时重排）
DELETE /api/activities/b
```

---

## 10. 边界与注意事项

1. **编码**：请求体必须 UTF-8。任何把中文转成 GBK / Latin-1 / ASCII 的发送方都会导致乱码或 `?`。
2. **`progress` 越界**：>100 自动截断为 100，<0 截断为 0。
3. **并发写**：PATCH 对同一活动并发更新无锁冲突（内部互斥锁），最终值为最后到达者的覆盖结果；并发 POST 同一 id 只会有一个成功（其余 `409`）。
4. **幂等性**：DELETE 可安全重试（不存在的 id 返回 `ok:false` 而非报错）；**POST 不可重试**——同 id 重复创建返回 `409`，需先 DELETE 或改用 PATCH。
5. **端口占用**：47300 被占用时应用日志打印 `[activity-pool] 绑定 ... 失败`，HTTP 不可用，其余功能不受影响。
6. **内存数据**：活动池不落盘，应用重启即空。需要持久化请由外部服务自己恢复重建。
7. **icon 加载**：仅推荐 http(s) / data URI。图片加载失败不影响卡片，会回退到默认图标。
8. **extra 大小**：**硬上限 16KB（序列化字节数），超出返回 `400`**。即便在限内也应保持精简 —— 每帧 30 次全量快照序列化，超大 extra 会白白消耗 CPU。
9. **安全边界**：无鉴权 + 仅回环。本机恶意进程可调用，请勿在活动内容中注入 HTML/脚本
   （前端按纯文本渲染，无注入面，但请保持 title/subtitle 为纯文本习惯）。

---

## 11. 常见问题排查

| 症状 | 原因 | 处理 |
|---|---|---|
| 请求报 `Failed to parse the request body as JSON` | 双引号被吞 / 编码被破坏 | 改用 JSON 文件 `--data-binary @file` 或 Node/Python 示例 |
| 中文显示 `?` 或乱码 | 发送侧非 UTF-8 | 见 8.3（UTF-8 字节）与 8.1/8.4（天然 UTF-8） |
| 岛没有弹出活动 | 应用未运行 / 端口占用 / 静默模式下岛隐藏 | 检查日志是否有 `[activity-pool] HTTP 服务已启动`；`curl http://127.0.0.1:47300/api/activities` 验证；开启岛常显 |
| 重复 POST 同 id 报 409 `活动 xx 已存在` | POST 是仅创建语义，同 id 已存在即冲突 | 预期行为。要更新用 PATCH；要重建先 DELETE 再 POST |
| 活动不消失 | 未设 `ttl_ms` 且外部没 DELETE | DELETE 或调大服务端侧外部清理逻辑 |
| 同时创建多个活动只显示一个 | 设计如此 | 岛单卡片制，只展示排序后第 1 个（优先级最高者） |
| 卡顿 / 高 CPU | 外部超高频推送（>30Hz 无意义） | 后端已 30Hz 合并；前端无需改动。检查是否每帧带超大 extra |
