# NetSpeed Dynamic Pro (NSD)

<div align="center">

<img src="./src/assets/logo.png" alt="Music Controller" width="200" />

<h1>NetSpeed Dynamic Pro</h1>
<p>Dynamic Island for Windows</p>

[![Tauri](https://img.shields.io/badge/Tauri-2.x-blue?logo=tauri)](https://tauri.app)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust)](https://rust-lang.org)
[![Vue 3](https://img.shields.io/badge/Vue-3.x-green?logo=vue.js)](https://vuejs.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.x-blue?logo=typescript)](https://www.typescriptlang.org)
[![Vite](https://img.shields.io/badge/Vite-6.x-yellow?logo=vite)](https://vite.dev)
[![ECharts](https://img.shields.io/badge/ECharts-6.x-purple?logo=apache-echarts)](https://echarts.apache.org)

[简体中文](./README.md) &nbsp; | [English](./README.en.md) &nbsp; | [Download](https://github.com/GEORGEWWWU/NetSpeed-Dynamic/releases/latest) &nbsp; | [Website](https://nsd.georgewu.top/) &nbsp; | [QQ Group：1080730621](https://qm.qq.com/cgi-bin/qm/qr?k=i70z7rbl-VWpejQugvlXeARDUjwP7sIW&jump_from=webapi&authKey=b6Pj6zLuuCINDhafPJRttePdy3D45vvtWzcZ109LWoWYXkcKo8bNWI7fMhr+yV87)

</div>

![Music Controller](./src/assets/screenshot2.png)
![Dynamic Island Notification](./src/assets/screenshot4.png)
![Music Controller 2.0](./src/assets/screenshot.gif)
![2.4.2](./src/assets/screenshot3.png)

---

NetSpeed Dynamic Pro (NSD) is a Windows desktop application built with Tauri 2, Vue 3, TypeScript, and Rust. It packages real-time network monitoring, system resource visibility, music control, toast notifications, taskbar plugin support, and personalization into a floating Dynamic Island interface.

## Highlights

- Monitor upload and download activity in real time with live traffic stats, monthly totals, and trend charts
- Show network, music, message, CPU, RAM, and system status in a floating Dynamic Island UI
- Support cross-platform media control through the Windows SMTC ecosystem and system media sessions
- Capture incoming Toast notifications and render them as on-screen notification cards with priority handling
- Offer theme switching, transparency, corner style, scaling, lyric timing, glow border, and animation tuning
- Support startup launch, tray icon usage, taskbar plugin sync, fullscreen auto-hide, and position locking

## Core Features

### 1. Network Monitoring

- Refresh upload and download speed every second with auto-scaling units
- Display network health indicators for normal, high-latency, and disconnected states
- Provide local traffic totals and monthly cumulative traffic analytics
- Switch between bar and line charts inside the console
- Make disconnection decisions more conservatively during heavy traffic fluctuations

### 2. Music and Media Control

- Use the Windows SMTC API to control previous / play-pause / next
- Support NetEase Cloud Music, Spotify, Apple Music, QQ Music, Kugou Music, Echo Music, and LX Music
- Present current song title, artist, and cover art in real time
- Prefer local cover art from the system media session and fall back gracefully to other sources
- Support synchronized lyrics, lyric queue logic, and glowing visual borders

### 3. Notifications and System Events

- Receive system Toast notifications and display them directly in the Dynamic Island
- Support silent mode, a message priority queue, and click-to-open actions
- Watch for volume changes, power connection/disconnection, lock/unlock, and low battery events
- Use event-specific icons and visual styles for different kinds of system updates

### 4. Taskbar Plugin and Desktop Integration

- Expose a taskbar plugin mode that mirrors live speed, lyrics, resource info, and messages into a taskbar companion component
- Keep the app accessible through the tray icon
- Auto-hide the island during fullscreen gaming or video playback
- Support position reset, lock, glow border switching, and always-on-top behavior

## Tech Stack

| Layer | Technology |
|-------|------------|
| Desktop Framework | Tauri 2 (Rust) |
| Frontend Framework | Vue 3 + TypeScript |
| Build Tool | Vite 6 |
| Router | Vue Router 5 |
| Charts | ECharts 6 |
| Icons | Lucide Vue Next |
| Network Monitoring | sysinfo (Rust) |
| Async Runtime | Tokio (Rust) |
| HTTP Client | reqwest (Rust) |
| Media Control | Windows SMTC API |
| Audio Processing | cpal + rustfft |
| System Events | Windows COM / WinAPI |
| Storage | localStorage |

## Project Structure

```text
NetSpeed-Dynamic/
├── src/                      # Frontend source
│   ├── main.ts               # App entry
│   ├── router/index.ts       # Router setup
│   ├── i18n.ts               # Chinese/English localization
│   ├── views/
│   │   ├── MainPanel.vue     # Main console UI
│   │   └── WidgetIsland.vue  # Floating Dynamic Island
│   ├── components/
│   │   └── DynamicSet.vue    # Personalization center
│   └── assets/               # Icons, screenshots, and static assets
├── src-tauri/                # Tauri Rust backend
│   ├── src/
│   │   ├── lib.rs            # Core logic, windows, animation, tray
│   │   ├── music_controller.rs  # Media control and cover/lyric logic
│   │   ├── notification.rs   # System notification capture
│   │   ├── system_events.rs  # Volume, power, and lock events
│   │   └── audio_spectrum.rs # Audio spectrum analysis
│   ├── Cargo.toml           # Rust dependencies
│   └── tauri.conf.json      # Tauri configuration
├── package.json              # Frontend dependencies and scripts
└── README.en.md             # English documentation
```

## Development Environment

### Prerequisites

- Windows 10/11
- Node.js 18+
- Rust 1.70+
- Tauri 2 CLI

### Install and Run

```bash
git clone https://github.com/GEORGEWWWU/NetSpeed-Dynamic.git
cd NetSpeed-Dynamic
npm install
npm run tauri dev
```

### Build for Release

```bash
npm run tauri build
```

The packaged output is written to `src-tauri/target/release/bundle/`.

## Usage

1. Launch the app to open the main console.
2. Enable the Widget switch to show the floating Dynamic Island.
3. Drag it with the mouse and use the right-click menu to lock, reset, close, toggle the glow border, or pin it on top.
4. Configure music platform selection, notification preferences, theme, opacity, auto-start, and taskbar plugin settings.
5. Open the Personalization Center to adjust animation behavior, appearance, size, and scaling.

> Note: This project is deeply adapted for Windows and relies on system SMTC, WinAPI, COM, and notification-management features.

## License

MIT License

Copyright (c) 2026 Ryen (GEORGEWU)

## Support and Donation

If this project helps you, feel free to support the author:

| Method | Information |
|--------|-------------|
| WeChat Pay | [WeChat](./src/assets/wechat-pay.png) |
| Alipay | [Alipay](./src/assets/alipay.jpg) |
| GitHub Sponsors | [Support Here](https://github.com/sponsors/GEORGEWWWU) |

---

> Thank you to every supporter and user!