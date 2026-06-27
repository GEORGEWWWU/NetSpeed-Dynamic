# NetSpeed Dynamic Pro (NSD)

<div align="center">

<img src="./src/assets/logo.png" alt="音乐控制器" width="350" />

**NetSpeed Dynamic Pro** —— 专为 Windows 而生的灵动岛

[![Tauri](https://img.shields.io/badge/Tauri-2.x-blue?logo=tauri)](https://tauri.app)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust)](https://rust-lang.org)
[![Vue 3](https://img.shields.io/badge/Vue-3.x-green?logo=vue.js)](https://vuejs.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.x-blue?logo=typescript)](https://www.typescriptlang.org)
[![Vite](https://img.shields.io/badge/Vite-6.x-yellow?logo=vite)](https://vite.dev)
[![ECharts](https://img.shields.io/badge/ECharts-6.x-purple?logo=apache-echarts)](https://echarts.apache.org)

</div>

![音乐控制器](./src/assets/screenshot2.png)
![灵动岛通知](./src/assets/screenshot4.jpg)
![系统资源监控](./src/assets/screenshot5.png)
![标准网速显示](./src/assets/screenshot.png)
![置于任务栏](./src/assets/screenshot7.png)

---

这是一个基于 **Tauri 2 + Rust + Vue 3** 的桌面动态网速监控工具，灵动岛悬浮窗实时显示网络速度，支持流量统计、网易云音乐控制、系统通知接收与硬件监控，支持置于任务栏左下角。

## 功能

### 网速监控

- **实时网速**：每秒刷新上传/下载速度，自动切换单位
- **灵动岛悬浮窗**：支持拖拽移动、弹簧动画进出场
- **网络状态指示灯**：绿色（正常）/黄色（高延迟）/红色（断网）
- **流量高亮**：超过 1MB/s 时箭头自动高亮提醒
- **速度趋势图表**：控制台内置迷你折线图展示最近 15 秒下载速度

### 网易云音乐控制

- **播放控制**：上一首 / 播放暂停 / 下一首
- **歌曲信息**：实时显示歌名、歌手和专辑封面
- **封面旋转**：播放时封面自动旋转，暂停时停止
- **多源封面获取**：优先网易云，依次降级至 Deezer、iTunes，SVG 渐变兜底
- **彩虹流光边框**：8 色渐变旋转边框，可独立开关
- **智能交互**：悬停显示控件，离开后自动切换为歌曲信息

### 系统消息通知

- **实时捕获**：接收系统 Toast 通知并在灵动岛展示
- **动态扩展**：收到通知时灵动岛自动放大展示应用图标、标题和内容
- **智能过滤**：自动过滤微信通知避免干扰
- **点击唤醒**：点击通知区域直接打开对应应用

### 系统硬件监控

- **CPU/内存/GPU**：实时显示占用率
- **高占用预警**：≥90% 时自动红色警示
- **主题自适应**：支持暗色/亮色主题

### 设置与系统集成

- **主题切换**：浅色/深色/跟随系统
- **透明度调节**：0%~100% 实时同步至悬浮窗
- **开机自启**：跟随系统启动，静默启动时主窗口隐藏
- **系统托盘**：左键唤起控制台，右键强制退出
- **置于任务栏**：锁定至屏幕左下角，禁止拖拽，自动置顶
- **全屏游戏避让**：自动检测全屏窗口，避免抢占焦点
- **检查更新**：静默检测新版本并提示下载

## 技术栈

| 层级 | 技术 |
|------|------|
| 桌面框架 | Tauri 2 (Rust) |
| 前端框架 | Vue 3 + TypeScript |
| 构建工具 | Vite 6 |
| 路由 | Vue Router 5 |
| 图表 | ECharts 6 |
| 图标 | Lucide Vue Next |
| 网络监控 | sysinfo (Rust) |
| 异步运行时 | Tokio (Rust) |
| HTTP 客户端 | reqwest (Rust) |
| Windows API | windows-sys + winapi |

## 项目结构

```
NetSpeed-Dynamic/
├── src/                    # 前端源码
│   ├── main.ts             # 应用入口
│   ├── router/index.ts     # 路由配置
│   ├── views/
│   │   ├── MainPanel.vue   # 主控制台
│   │   └── WidgetIsland.vue # 灵动岛悬浮窗
│   └── assets/             # 静态资源
├── src-tauri/              # Tauri 后端
│   ├── src/
│   │   ├── main.rs         # Rust 入口
│   │   └── lib.rs          # 核心逻辑
│   ├── Cargo.toml          # Rust 依赖
│   └── tauri.conf.json     # Tauri 配置
└── package.json            # 前端依赖
```

## 开发环境

### 前置依赖

- Node.js >= 18
- Rust >= 1.70
- Tauri 2 CLI

### 安装与运行

```bash
git clone https://github.com/GEORGEWWWU/NetSpeed-Dynamic.git
cd NetSpeed-Dynamic
npm install
npm run tauri dev
```

### 构建发布

```bash
npm run tauri build
```

产物位于 `src-tauri/target/release/bundle/`。

## 使用方式

1. 启动后显示主控制台，点击系统托盘可随时唤起
2. 开启 Widget 开关，屏幕顶部出现灵动岛悬浮窗
3. 左键拖拽移动，右键菜单可重置位置、开关流光边框或关闭
4. 在"灵动岛设置"中开启音乐控制、消息通知、硬件监控或置于任务栏
5. 控制台右侧可切换常规设置与数据统计面板

## 开源协议

MIT License

Copyright (c) 2026 Ryen (GEORGEWU)

## 捐赠

如果 NSD 对你有帮助，欢迎请作者喝杯咖啡！

| 方式 | 信息 |
|------|------|
| 微信支付 | ![微信](./src/assets/wechat-pay.png) |
| 支付宝 | ![支付宝](./src/assets/alipay.jpg) |
| GitHub Sponsors | [前往支持](https://github.com/sponsors/GEORGEWWWU) |

---

> 感谢每一位支持者！