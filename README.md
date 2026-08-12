<div align="center">

<img src="./src/assets/logo.png" alt="音乐控制器" width="200" />

<h1>NetSpeed Dynamic Pro</h1>
<p>专为 Windows 而生的灵动岛</p>

[![Tauri](https://img.shields.io/badge/Tauri-2.x-blue?logo=tauri)](https://tauri.app)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust)](https://rust-lang.org)
[![Vue 3](https://img.shields.io/badge/Vue-3.x-green?logo=vue.js)](https://vuejs.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.x-blue?logo=typescript)](https://www.typescriptlang.org)
[![Vite](https://img.shields.io/badge/Vite-6.x-yellow?logo=vite)](https://vite.dev)
[![ECharts](https://img.shields.io/badge/ECharts-6.x-purple?logo=apache-echarts)](https://echarts.apache.org)

[简体中文](./README.md) &nbsp; | [English](./README.en.md) &nbsp; | [下载地址](https://github.com/GEORGEWWWU/NetSpeed-Dynamic/releases/latest) &nbsp; | [官方网站](https://nsd.georgewu.top/) &nbsp; | [QQ群：1080730621](https://qm.qq.com/cgi-bin/qm/qr?k=i70z7rbl-VWpejQugvlXeARDUjwP7sIW&jump_from=webapi&authKey=b6Pj6zLuuCINDhafPJRttePdy3D45vvtWzcZ109LWoWYXkcKo8bNWI7fMhr+yV87)

</div>

![音乐控制器](./src/assets/screenshot2.png)
![灵动岛通知](./src/assets/screenshot4.png)
![音乐控制器 2.0](./src/assets/screenshot.gif)
![2.4.2](./src/assets/screenshot3.png)

---

NetSpeed Dynamic Pro（NSD）是一个基于 Tauri 2、Vue 3、TypeScript 与 Rust 构建的 Windows 桌面应用。它将 “实时网速监控、系统资源观察、音乐控制、系统通知、任务栏插件与个性化配置” 封装在一个悬浮式 Dynamic Island 中，旨在为桌面环境提供更轻量、更顺手的状态展示与交互体验。

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
git clone https://github.com/GEORGEWWWU/NetSpeed-Dynamic.git
cd NetSpeed-Dynamic
npm install
npm run tauri dev
```

### 构建发布

```bash
npm run tauri build
```

构建产物会输出到 `src-tauri/target/release/bundle/`。

## 使用方式

1. 启动应用后，主控制台会弹出实时网速与设置入口。
2. 打开“Widget”开关后，屏幕顶部会显示可拖拽的 Dynamic Island 悬浮窗。
3. 左键拖拽移动，右键菜单可进行位置锁定、重置、关闭、流光边框开关与置顶设置。
4. 在控制台中配置音乐平台、消息通知、主题、透明度、自动启动与任务栏插件。
5. 进入“个性化中心”后，可调整物理动效、外观、尺寸与全局缩放比例。

> 说明：当前项目针对 Windows 平台深度适配，部分能力依赖系统 SMTC、WinAPI、COM 与通知管理接口。

## 许可证

MIT License

Copyright (c) 2026 Ryen (GEORGEWU)

## 贡献者与 Star 历史

感谢所有为本项目做出贡献的开发者！

<div align="left">
  <a href="https://github.com/GEORGEWWWU/NetSpeed-Dynamic/graphs/contributors">
    <img src="https://contrib.rocks/image?repo=GEORGEWWWU/NetSpeed-Dynamic" alt="Contributors" />
  </a>
</div>

### Star 历史趋势

<div align="center">
  <a href="https://star-history.com/#GEORGEWWWU/NetSpeed-Dynamic&Date">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=GEORGEWWWU/NetSpeed-Dynamic&type=Date&theme=dark" />
      <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=GEORGEWWWU/NetSpeed-Dynamic&type=Date" />
      <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=GEORGEWWWU/NetSpeed-Dynamic&type=Date" />
    </picture>
  </a>
</div>

## 支持与捐赠

如果这个项目对你有帮助，欢迎支持作者：

| 方式 | 信息 |
|------|------|
| 微信支付 | [微信](./src/assets/wechat-pay.png) |
| 支付宝 | [支付宝](./src/assets/alipay.jpg) |
| GitHub Sponsors | [前往支持](https://github.com/sponsors/GEORGEWWWU) |

---

> 感谢每一位支持者与使用者！