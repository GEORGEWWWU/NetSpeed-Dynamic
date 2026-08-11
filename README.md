<div align="center">

<img src="./src/assets/logo.png" alt="音乐控制器" width="200" />

<h1>NetSpeed Dynamic Pro · Wei_XingYU Edition</h1>
<p>专为 Windows 而生的灵动岛</p>

[![Tauri](https://img.shields.io/badge/Tauri-2.x-blue?logo=tauri)](https://tauri.app)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust)](https://rust-lang.org)
[![Vue 3](https://img.shields.io/badge/Vue-3.x-green?logo=vue.js)](https://vuejs.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.x-blue?logo=typescript)](https://www.typescriptlang.org)
[![Vite](https://img.shields.io/badge/Vite-6.x-yellow?logo=vite)](https://vite.dev)
[![ECharts](https://img.shields.io/badge/ECharts-6.x-purple?logo=apache-echarts)](https://echarts.apache.org)

[简体中文](./README.md) &nbsp; | [English](./README.en.md) &nbsp; | [直接下载安装包](https://github.com/wei-xingyu/NetSpeed-Dynamic/releases/latest/download/NetSpeed%20Dynamic%20Pro%20-%20Wei_XingYU%20Edition_2.5.0_x64-setup.exe)

</div>

![音乐控制器](./src/assets/screenshot2.png)
![灵动岛通知](./src/assets/screenshot4.png)
![音乐控制器 2.0](./src/assets/screenshot.gif)
![Wei_XingYU Edition 2.5.0](./src/assets/screenshot3.png)

---

NetSpeed Dynamic Pro · Wei_XingYU Edition（NSD）是一个基于 Tauri 2、Vue 3、TypeScript 与 Rust 构建的 Windows 桌面应用。它将“实时网速监控、系统资源观察、音乐控制、系统通知、任务栏插件与个性化配置”封装在一个悬浮式 Dynamic Island 中。

## Wei_XingYU Edition 2.5.0：基于上游新增内容

本项目基于 [GEORGEWWWU/NetSpeed-Dynamic](https://github.com/GEORGEWWWU/NetSpeed-Dynamic) 继续开发，保留上游 MIT 许可证和原作者署名。本版本新增或完善：

- 展开态设备状态卡片：网络名称与信号、音频输出设备、蓝牙连接数、系统音量和日历。
- 收起态/展开态分别配置媒体、资源、FPS、网速等显示内容。
- CPU、内存、FPS 与网速卡片的紧凑布局和图标显示优化。
- 灵动岛位置自定义、全屏隐藏和媒体控制展开体验优化。
- 任务栏插件与 FPS 插件随安装包一并提供；安装后无需手动复制组件。
- 更新检查、下载入口和问题提示均指向本仓库的 GitHub Releases。

## 项目亮点

- 实时展示上传/下载网速，并提供本地流量统计、月度累计与趋势图
- 使用悬浮式 Dynamic Island 展示网络、音乐、消息、CPU/RAM 资源和系统状态
- 支持多平台音乐控制，兼容 Windows SMTC 生态与多种播放器与媒体会话
- 拦截并展示系统 Toast 通知，并支持静默模式、优先级处理与点击唤醒控制台
- 提供亮色、暗色、沉浸模式、透明度、圆角、全局缩放、歌词延迟、流光边框等设置
- 支持开机自启、托盘图标、任务栏插件、全屏自动隐藏、位置锁定与置顶等桌面增强能力

## 核心功能

### 1. 网络监控

- 每秒刷新上传/下载速度，并自动换算单位
- 显示网络状态灯：正常 / 高延迟 / 断网
- 提供本地累计流量统计与按月统计图表
- 支持在控制台中切换柱状图与折线图视图
- 在高流量波动下进行更稳妥的断网判断，减少误报

### 2. 多平台音乐控制

- 通过 Windows SMTC API 进行上一首 / 播放暂停 / 下一首控制
- 支持网易云音乐、Spotify、Apple Music、QQ 音乐、酷狗音乐、Echo Music、LX Music 等媒体源
- 实时展示歌曲名、歌手与封面，播放时封面可旋转
- 优先读取系统媒体会话中的本地封面，兼容多源封面兜底
- 支持歌词显示、歌词队列与同步调校、彩虹流光边框等视觉表现

### 3. 系统通知与事件

- 接收系统 Toast 通知，并在 Dynamic Island 中呈现消息卡片
- 支持消息通知优先级队列、静默模式、点击打开应用等交互
- 监听系统音量变化、电源插拔、锁屏/解锁、低电量等事件
- 根据事件类型切换独立图标、颜色与通知样式

### 4. 任务栏插件与桌面集成

- 提供任务栏插件能力，将实时网速、歌词、消息与资源信息同步到任务栏侧边组件
- 支持通过托盘图标快速打开或关闭界面
- 支持全屏自动隐藏，避免游戏或视频观看时干扰
- 支持动态岛位置锁定、重置、流光边框开关与始终置顶

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
| 媒体控制 | Windows SMTC API |
| 音频处理 | cpal + rustfft |
| 系统事件 | Windows COM / WinAPI |
| 存储 | localStorage |

## 项目结构

```text
NetSpeed-Dynamic/
├── src/                      # 前端源码
│   ├── main.ts               # 应用入口
│   ├── router/index.ts       # 路由设置
│   ├── i18n.ts               # 中文/英文国际化
│   ├── views/
│   │   ├── MainPanel.vue     # 主控制台界面
│   │   └── WidgetIsland.vue  # 灵动岛悬浮窗
│   ├── components/
│   │   └── DynamicSet.vue    # 个性化中心
│   └── assets/               # 图标、截图与静态资源
├── src-tauri/                # Tauri Rust 后端
│   ├── src/
│   │   ├── lib.rs            # 核心逻辑、窗口、动画与托盘
│   │   ├── music_controller.rs  # 媒体控制与封面/歌词
│   │   ├── notification.rs   # 系统通知捕获
│   │   ├── system_events.rs  # 音量、电源、锁屏等事件
│   │   └── audio_spectrum.rs # 音频频谱分析
│   ├── Cargo.toml           # Rust 依赖
│   └── tauri.conf.json      # Tauri 配置
├── package.json              # 前端依赖与脚本
└── README.md                 # 中文说明
```

## 开发环境

### 依赖要求

- Windows 10/11
- Node.js 18+
- Rust 1.70+
- Tauri 2 CLI

### 安装与运行

```bash
git clone https://github.com/wei-xingyu/NetSpeed-Dynamic.git
cd NetSpeed-Dynamic
npm install
npm run tauri dev
```

### 构建发布

```bash
npm run tauri build
```

构建产物会输出到 `src-tauri/target/release/bundle/`；任务栏插件和 FPS 插件会自动纳入安装包。

## 直接下载与安装（普通用户）

[点击直接下载 Windows 安装包](https://github.com/wei-xingyu/NetSpeed-Dynamic/releases/latest/download/NetSpeed%20Dynamic%20Pro%20-%20Wei_XingYU%20Edition_2.5.0_x64-setup.exe)

下载后双击安装即可使用，不需要安装 Node.js、Rust，也无需手动复制任何插件。首次运行可在控制台开启“开机自启动”。

> Windows 如显示“未知发布者”提示，是因为当前安装包尚未做代码签名；请仅从本仓库的 Release 下载。

## 使用方式

1. 启动应用后，主控制台会弹出实时网速与设置入口。
2. 打开“Widget”开关后，屏幕顶部会显示可拖拽的 Dynamic Island 悬浮窗。
3. 左键拖拽移动，右键菜单可进行位置锁定、重置、关闭、流光边框开关与置顶设置。
4. 在控制台中配置音乐平台、消息通知、主题、透明度、自动启动与任务栏插件。
5. 进入“个性化中心”后，可调整物理动效、外观、尺寸与全局缩放比例。

> 说明：当前项目针对 Windows 平台深度适配，部分能力依赖系统 SMTC、WinAPI、COM 与通知管理接口。

## 许可证

MIT License

Upstream Copyright (c) 2026 Ryen (GEORGEWU)

Wei_XingYU Edition modifications Copyright (c) 2026 Wei_XingYU
