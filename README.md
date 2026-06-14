# NetSpeed Dynamic (NSD)

一款基于 **Tauri 2 + Vue 3** 构建的桌面动态网速监控工具，提供灵动岛风格的悬浮窗实时显示网络速度，并支持竞技游戏服务器延迟测试。

![NetSpeed Dynamic 效果图](./src/assets/screenshot.png)

## 功能特性

### 网速监控

- **实时网速监控** — 每秒刷新上传/下载速度，支持 B/s、KB/s、MB/s 自动单位切换
- **灵动岛悬浮窗** — 仿 macOS Dynamic Island 设计的透明悬浮窗，支持拖拽移动、弹簧动画进出场
- **网络状态指示灯** — 绿色（延迟 <150ms）/ 黄色（高延迟或大流量遮挡）/ 红色（断网），带智能防抖逻辑避免误判
- **流量高亮提示** — 当流量超过 1MB/s 时，悬浮窗箭头自动高亮提醒
- **速度趋势图表** — 控制台内置 ECharts 迷你折线图，展示最近 15 秒下载速度走势

### 游戏模式 (GameMode)

- **ICMP Ping 延迟测试** — 使用原生 ICMP 协议对多台竞技游戏服务器进行真实网络延迟测量
- **多服务器并行测试** — 同时 Ping 6 台热门游戏服务器，一键获取全部结果
- **内置热门游戏服务器列表**：
  - 无畏契约 (国服腾讯广东节点)
  - 反恐精英 2 / CS2 (国服完美世界北京)
  - 英雄联盟 (国服电信一区艾欧尼亚)
  - 绝地求生 PUBG (亚洲区韩国首尔 AWS)
  - APEX 英雄 (亚洲区中国香港 Cloudflare)
  - 永劫无间 (国服网易杭州机房)
- **延迟分级显示** — 极佳(≤35ms 绿色) / 良好(≤70ms 黄色) / 较差(红色) / 超时/错误标识
- **手动触发测试** — 支持随时点击按钮重新测试所有服务器

### 设置与系统集成

- **主题切换** — 支持浅色模式 / 深色模式 / 跟随系统，全局 CSS 变量驱动，设置持久化到 localStorage
- **悬浮窗透明度可调** — 滑块调节灵动岛背景不透明度（0%~100%）
- **开机自动启动** — 通过 `tauri-plugin-autostart` 实现跟随系统启动 NSD
- **系统托盘集成** — 托盘左键唤起主控制台，右键菜单强制退出；关闭主窗口时隐藏至托盘而非退出
- **单实例保证** — 防止重复启动多个进程
- **检查更新** — 通过 GitHub Releases API 检测新版本并引导下载
- **Windows 原生优化** — DWM 圆角裁剪去除、边框隐藏、无边框拖拽区域

## 技术栈

| 层级 | 技术 |
|------|------|
| 桌面框架 | [Tauri 2](https://tauri.app/) (Rust) |
| 前端框架 | [Vue 3](https://vuejs.org/) + TypeScript |
| 构建工具 | [Vite 6](https://vite.dev/) |
| 路由 | [Vue Router 5](https://router.vuejs.org/) |
| 图表 | [ECharts 6](https://echarts.apache.org/) |
| 图标 | [Lucide Vue Next](https://lucide.dev/) |
| 网络监控 | [sysinfo](https://docs.rs/sysinfo) (Rust) |
| ICMP Ping | [pinger](https://docs.rs/pinger) (Rust) |
| 异步运行时 | [Tokio](https://tokio.rs/) (Rust) |
| Windows API | windows-sys (DWM / GDI / Messaging) |

## 项目结构

```
NetSpeed-Dynamic/
├── src/                          # 前端源码
│   ├── main.ts                   # 应用入口，挂载 Vue + Router
│   ├── App.vue                   # 根组件（router-view）
│   ├── router/index.ts           # 路由配置：/ → MainPanel, /widget → WidgetIsland
│   ├── views/
│   │   ├── MainPanel.vue         # 主控制台面板（网速仪表盘 + 设置 + 图表 + 游戏模式）
│   │   └── WidgetIsland.vue      # 灵动岛悬浮窗组件（网速显示 + 动画 + 右键菜单）
│   └── assets/
│       └── logo.png              # 应用 Logo
├── src-tauri/                    # Tauri 后端（Rust）
│   ├── src/
│   │   ├── main.rs               # Rust 入口
│   │   └── lib.rs                # 核心逻辑：Tauri 命令、托盘、窗口管理、Windows DWM 样式
│   ├── Cargo.toml                # Rust 依赖声明
│   ├── tauri.conf.json           # Tauri 配置（双窗口：main + widget）
│   └── icons/                    # 应用图标（全平台）
├── index.html                    # HTML 入口（含主题预加载脚本）
├── vite.config.ts                # Vite 配置（Tauri 开发模式适配）
├── package.json                  # 前端依赖与脚本
└── tsconfig.json                 # TypeScript 配置
```

## 架构说明

### 双窗口架构

应用采用 **双窗口** 设计，通过 Tauri 多窗口 + Vue Router 分发：

| 窗口 | 标签 | 尺寸 | 用途 |
|------|------|------|------|
| 主控制台 | `main` | 700×550px，不可调整大小 | 网速总览、设置面板、图表展示、游戏模式 |
| 灵动岛 Widget | `widget` | 210×36px，无边框透明、置顶、不在任务栏显示 | 实时网速悬浮条 |

两个窗口通过 **Tauri Event** 进行通信：
- `control-island-visibility` — 控制台 → 灵动岛显隐指令
- `control-island-opacity` — 控制台 → 灵动岛透明度同步
- `island-status-sync` — 灵动岛 → 控制台状态回传

### 后端命令（Tauri Commands）

| 命令 | 说明 |
|------|------|
| `get_network_stats` | 通过 `sysinfo::Networks` 获取所有网卡累计收发字节数，前端做差分计算瞬时速度 |
| `get_network_latency` | TCP 连接 `223.5.5.5:53`（阿里 DNS）测量网络延迟，超时 1.5s |
| `ping_game_host` | 使用 ICMP 协议 Ping 指定主机地址，返回往返延迟（ms），超时 1.5s；基于 `pinger` + `tokio` 异步实现 |
| `is_widget_visible` | 查询灵动岛窗口当前可见性 |

### 灵动岛动画

入场/退场动画使用 JavaScript `requestAnimationFrame` 实现，公式源自 After Effects 弹簧表达式转换：

```
scale = 1 - cos(2πft) × e^(-dt)    // f=2.0, d=10.5, duration=600ms
```

退场动画完成后才触发 Tauri 窗口隐藏，确保视觉连贯。

## 开发环境准备

### 前置依赖

- **Node.js** >= 18
- **Rust** >= 1.70（[安装指南](https://www.rust-lang.org/tools/install)）
- **Tauri 2 CLI**

### 安装与运行

```bash
# 1. 克隆项目
git clone https://github.com/GEORGEWWWU/NetSpeed-Dynamic.git
cd NetSpeed-Dynamic

# 2. 安装前端依赖
npm install

# 3. 启动开发模式（同时运行 Vite dev server 和 Tauri 窗口）
npm run tauri dev
```

### 构建发布版本

```bash
# 构建前端 + 编译 Rust + 打包安装包
npm run tauri build
```

产物位于 `src-tauri/target/release/bundle/`。

## 使用方式

1. 启动应用后，默认仅显示**主控制台**窗口
2. 点击右上角开关或系统托盘图标打开**主控制台**
3. 在控制台中开启 **Widget 开关**，屏幕顶部中央出现灵动岛悬浮窗
4. 灵动岛支持：
   - **左键拖拽** — 自由移动位置
   - **右键菜单** — 重置位置 / 关闭悬浮窗
5. 在控制台中可调整**主题**和悬浮窗**透明度**
6. 点击顶部 **GAMEMODE 按钮** 切换至游戏模式：
   - 自动对 6 台热门竞技游戏服务器执行 ICMP Ping 延迟测试
   - 延迟结果以颜色分级展示（绿/黄/红）
   - 可随时点击「手动测试」重新检测

## 开源协议

本项目基于 [MIT License](./LICENSE) 开源。

Copyright (c) 2026 Ryen (GEORGEWU)
