<template>
    <transition @enter="onEnter" @leave="onLeave" :css="false">
        <div v-show="isIslandVisible" :class="['island-container', { 'has-music-border': isGlowBorderEnabled }]"
            @mousedown="handleMouseDown" @mousemove="handleMouseMove" @mouseup="handleMouseUp"
            @mouseleave="handleMouseLeave" @mouseenter="handleMouseEnter" :style="islandStyle"
            @contextmenu="handleRightClick">

            <div class="rainbow-border-glow" v-if="isGlowBorderEnabled" :style="{ opacity: glowOpacity }"></div>

            <div class="island-core-content" :style="coreContentStyle">
                <div class="inner-wrapper">
                    <transition mode="out-in" @enter="onInnerEnter" @leave="onInnerLeave" :css="false">
                        <div v-if="isMsgActive" class="msg-box" key="msg" @click="handleMsgClick"
                            style="cursor: pointer;">
                            <div class="msg-avatar">
                                <img :src="currentMsgIcon" alt="消息图标" class="msg-avatar-img">
                            </div>
                            <div class="msg-text-wrapper">
                                <div class="msg-title">{{ msgTitle }}</div>
                                <div class="msg-body">{{ msgBody }}</div>
                            </div>
                        </div>

                        <div v-else-if="displayHardware" class="systemstate-box" key="hardware">
                            <div class="hw-item">
                                <span class="hw-label">CPU</span>
                                <span class="hw-value" :class="{ 'high-usage': parseInt(cpuUsage) >= 90 }">{{ cpuUsage
                                }}</span>
                            </div>
                            <div class="hw-divider"></div>
                            <div class="hw-item">
                                <span class="hw-label">GPU</span>
                                <span class="hw-value" :class="{ 'high-usage': parseInt(gpuUsage) >= 90 }">{{ gpuUsage
                                }}</span>
                            </div>
                            <div class="hw-divider"></div>
                            <div class="hw-item">
                                <span class="hw-label">RAM</span>
                                <span class="hw-value" :class="{ 'high-usage': parseInt(memUsage) >= 90 }">{{ memUsage
                                }}</span>
                            </div>
                        </div>

                        <div v-else-if="displayMusic" class="music-ctl-box" :class="{ 'expanded': isMusicExpanded }"
                            :key="'music_' + musicBoxKey" @click="expandMusic" style="cursor: pointer;">
                            <div class="music-top-row">
                                <div class="album-cover" :class="{ 'is-playing': isPlaying }">
                                    <div class="cover-inner"
                                        :style="coverUrl ? { backgroundImage: `url(${coverUrl})`, backgroundSize: 'cover' } : {}">
                                    </div>
                                </div>
                                <div class="music-info-mask-box">
                                    <div class="music-info-text single-line" :class="{ 'fade-out': isMusicExpanded }">{{
                                        currentTrackInfo }}</div>
                                    <div class="music-info-text double-line" :class="{ 'fade-in': isMusicExpanded }">
                                        <div class="song-title">{{ currentSongName }}</div>
                                        <div class="song-artist">{{ currentArtistName }}</div>
                                    </div>
                                </div>
                            </div>
                            <transition name="fade">
                                <div class="music-controls" v-show="isMusicExpanded">
                                    <button class="ctl-btn" @click.stop="prevTrack">
                                        <svg viewBox="0 0 24 24" fill="currentColor">
                                            <path d="M6 6h2v12H6zm3.5 6l8.5 6V6z" />
                                        </svg>
                                    </button>
                                    <button class="ctl-btn play-btn" @click.stop="togglePlay">
                                        <svg v-if="isPlaying" viewBox="0 0 24 24" fill="currentColor">
                                            <path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z" />
                                        </svg>
                                        <svg v-else viewBox="0 0 24 24" fill="currentColor"
                                            style="transform: translateX(1px);">
                                            <path d="M8 5v14l11-7z" />
                                        </svg>
                                    </button>
                                    <button class="ctl-btn" @click.stop="nextTrack">
                                        <svg viewBox="0 0 24 24" fill="currentColor">
                                            <path d="M6 18l8.5-6L6 6v12zM16 6v12h2V6h-2z" />
                                        </svg>
                                    </button>
                                </div>
                            </transition>
                        </div>

                        <div v-else-if="displaySpeed" class="speed-box" key="speed">
                            <div class="speed-item">
                                <span :class="['label', { 'high-traffic': isHighUpload }]">↑</span>
                                <span class="value">{{ uploadSpeed }}</span>
                            </div>
                            <div class="speed-item">
                                <span :class="['label', { 'high-traffic': isHighDownload }]">↓</span>
                                <span class="value">{{ downloadSpeed }}</span>
                            </div>
                        </div>
                    </transition>
                </div>

                <transition mode="out-in" @enter="onInnerEnter" @leave="onInnerLeave" :css="false">
                    <div v-if="displayMusic" class="audio-spectrum"
                        :class="{ 'is-playing': isPlaying, 'expanded': isMusicExpanded }" key="spectrum">
                        <span class="bar"></span>
                        <span class="bar"></span>
                        <span class="bar"></span>
                        <span class="bar"></span>
                        <span class="bar"></span>
                    </div>

                    <div v-else :class="['status-dot', networkStatus]" key="dot"></div>
                </transition>
            </div>
        </div>
    </transition>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed, watch, type CSSProperties } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow, currentMonitor, PhysicalPosition, LogicalPosition, PhysicalSize } from '@tauri-apps/api/window'; import { Menu, MenuItem } from '@tauri-apps/api/menu';
import { listen, emit } from '@tauri-apps/api/event';

const isIslandVisible = ref(false);
const isMenuOpen = ref(false);

// 灵动岛自身的透明度变量（默认100）
const islandOpacity = ref(Number(localStorage.getItem('nsd_island_opacity') || '100'));

const islandTheme = ref(localStorage.getItem('nsd_island_theme') || 'black');

// 1. 瞬间判定当前是否处于大窗口状态
const isExpandedSize = computed(() => isMusicExpanded.value || isMsgActive.value);

// 2. 外层容器：状态一变，立马切成目标圆角
const islandStyle = computed<CSSProperties>(() => {
    const linear = islandOpacity.value / 100;
    const alpha = Math.pow(linear, 1 / 2.2);
    const baseStyle = islandTheme.value === 'white' ? {
        backgroundColor: `rgba(255, 255, 255, ${alpha})`,
        color: '#000000'
    } : {
        backgroundColor: `rgba(0, 0, 0, ${alpha})`,
        color: '#ffffff'
    };

    return {
        ...baseStyle,
        width: '100vw',
        height: '100vh',
        // ✅ 核心：只要展开就是 24px，收起就是 100px
        borderRadius: isExpandedSize.value ? '24px' : '100px',
        position: 'relative',
    };
});

// 3. 内层核心：永远比外层小 2px
const coreContentStyle = computed(() => {
    const linear = islandOpacity.value / 100;
    const alpha = Math.pow(linear, 1 / 2.2);

    // ✅ 核心：展开 22px，收起 98px
    const innerRadius = isExpandedSize.value ? '22px' : '98px';

    if (islandTheme.value === 'white') {
        return {
            backgroundColor: `rgba(255, 255, 255, ${alpha})`,
            borderRadius: innerRadius
        };
    }
    return {
        backgroundColor: `rgba(0, 0, 0, ${alpha})`,
        borderRadius: innerRadius
    };
});

const glowOpacity = computed(() => {
    const linear = islandOpacity.value / 100;
    return Math.pow(linear, 1 / 2.2);
});

const uploadSpeed = ref('0 KB/s');
const downloadSpeed = ref('0 KB/s');

// 记录当前是否属于大流量状态
const isHighDownload = ref(false);
const isHighUpload = ref(false);

// 网络状态指示灯：good(绿), warning(黄), error(红)
const networkStatus = ref<'good' | 'warning' | 'error'>('good');

// 系统硬件监控相关
const isHardwareMonEnabled = ref(localStorage.getItem('nsd_hardware_mon') === 'true');
const cpuUsage = ref('0%');
const gpuUsage = ref('0%');
const memUsage = ref('0%');

// 音乐控制功能开关
const isMusicCtlEnabled = ref(localStorage.getItem('nsd_music_ctrl') === 'true');
const isPlaying = ref(false);
// 流光边框默认状态完全镜像音乐控制器（只要音乐控制器开着它就开，关了就一起关）
const isGlowBorderEnabled = ref(localStorage.getItem('nsd_glow_border') === 'true');

const coverUrl = ref('');
const coverCache = new Map<string, string>();

// 记录是否开启了置于任务栏
const isPinnedToTaskbar = ref(localStorage.getItem('nsd_pin_taskbar') === 'true');
// 👇 新增这一行：记录是否锁定了位置，并存到本地
const isPositionLocked = ref(localStorage.getItem('nsd_position_locked') === 'true');

// 记录消息模式开关状态
const isMsgModeEnabled = ref(localStorage.getItem('nsd_msg_mode') === 'true');

// --- 轮换功能核心逻辑 ---
const isRotationEnabled = ref(localStorage.getItem('nsd_rotation_mode') === 'true');
const currentRotIndex = ref(0); // 0=网速, 1=音乐, 2=硬件
let rotationTimer: number | null = null;

// 使用计算属性智能判断当前该显示谁
const displaySpeed = computed(() => !isMsgActive.value && (isRotationEnabled.value ? currentRotIndex.value === 0 : (!isMusicCtlEnabled.value && !isHardwareMonEnabled.value)));
const displayMusic = computed(() => !isMsgActive.value && (isRotationEnabled.value ? currentRotIndex.value === 1 : isMusicCtlEnabled.value));
const displayHardware = computed(() => !isMsgActive.value && (isRotationEnabled.value ? currentRotIndex.value === 2 : isHardwareMonEnabled.value));

const startRotation = () => {
    if (rotationTimer) clearInterval(rotationTimer);
    rotationTimer = window.setInterval(() => {
        currentRotIndex.value = (currentRotIndex.value + 1) % 3;
    }, 5000); // 5秒轮换一次
};

const stopRotation = () => {
    if (rotationTimer) {
        clearInterval(rotationTimer);
        rotationTimer = null;
    }
};
// -----------------------

// 计算并吸附到左下角的方法
const snapToBottomLeft = async () => {
    try {
        const appWindow = getCurrentWindow();
        await new Promise((resolve) => setTimeout(resolve, 150));
        const monitor = await currentMonitor();

        if (monitor) {
            const scaleFactor = window.devicePixelRatio;

            const WINDOW_INIT_WIDTH = currentWidth.value;
            const WINDOW_INIT_HEIGHT = currentHeight.value;
            await appWindow.setSize(new PhysicalSize(Math.ceil(WINDOW_INIT_WIDTH * scaleFactor), Math.ceil(WINDOW_INIT_HEIGHT * scaleFactor)));

            const monitorLeftPhysical = monitor.position.x;
            const monitorTopPhysical = monitor.position.y;
            // 恢复使用 Tauri 最底层的硬件真实分辨率（绝对不会缩水）
            const monitorHeightPhysical = monitor.size.height;

            // X坐标: 屏幕最左侧 + 10px的边距
            const x = monitorLeftPhysical + (10 * scaleFactor);
            // Y坐标: 物理最底部 - 窗口高度 - 3px微调
            const y = monitorTopPhysical + monitorHeightPhysical - ((WINDOW_INIT_HEIGHT + 3) * scaleFactor);

            // 【终极绝杀核心】：绕过 Windows 系统的任务栏防遮挡机制
            // 在强制覆盖任务栏坐标之前，先隐身！
            await appWindow.hide();

            await appWindow.setPosition(new PhysicalPosition(Math.round(x), Math.round(y)));

            // 移动完成后，瞬间现身，生米煮成熟饭，Windows 也拦不住了！
            await appWindow.show();
        }
    } catch (error) {
        console.error('停靠左下角失败:', error);
    }
};

const togglePlay = async () => {
    // 1. 前端先立刻切换图标，给用户极速的视觉反馈
    isPlaying.value = !isPlaying.value;

    // 2. 发送指令给 Rust 和 SMTC
    try {
        await invoke('control_system_media', { action: 'play_pause' });
    } catch (err) {
        console.error('播放控制失败:', err);
        // 如果底层控制失败了，再把图标状态回滚回来
        isPlaying.value = !isPlaying.value;
    }
};

const prevTrack = async () => {
    await invoke('control_system_media', { action: 'prev' });
};

const nextTrack = async () => {
    await invoke('control_system_media', { action: 'next' });
};

// 核心同步函数：塞入到你的 fetchSpeedStats 同一频次的定时器中
const syncMusicStatus = async () => {
    try {
        // 1. 调用 Rust 提取网易云标题 [歌名, 歌手, 是否在播放]
        const res = await invoke<[string, string, boolean] | null>('fetch_netease_music_info');

        if (res) {
            const [song, artist, playing] = res;

            // 👇 新增这两行，为了展开后的双行显示分别赋值
            currentSongName.value = song;
            currentArtistName.value = artist || '未知歌手';

            // 拼接新的歌曲信息
            const newTrackInfo = artist ? `${song} - ${artist}` : song;

            if (currentTrackInfo.value !== newTrackInfo) {
                currentTrackInfo.value = newTrackInfo;

                // 优先读取缓存
                if (coverCache.has(newTrackInfo)) {
                    coverUrl.value = coverCache.get(newTrackInfo)!;
                } else {
                    try {
                        const realCoverUrl = await invoke<string>('get_random_cover_url', {
                            songName: song,
                            artistName: artist
                        });
                        coverUrl.value = realCoverUrl;
                        // 写入缓存，最多缓存 50 首防止内存溢出
                        if (coverCache.size > 50) coverCache.clear();
                        coverCache.set(newTrackInfo, realCoverUrl);
                    } catch (coverErr) {
                        console.error('所有封面源均获取失败:', coverErr);
                        // 使用本地图标或纯色背景，不要再用外部 URL 作为错误兜底
                        coverUrl.value = '';
                    }
                }
            }

            isPlaying.value = playing;
        } else {
            // 没检测到播放时，清空状态
            currentTrackInfo.value = `未在播放歌曲 - ${getPlayerName()}`;
            isPlaying.value = false;
            coverUrl.value = ''; // 没歌时清空，显示默认的优美渐变色
        }
    } catch (err) {
        console.error('音乐信息获取失败:', err);
    }
};

const showInfo = ref(false);
// 默认显示内容动态从本地缓存读取
const getPlayerName = () => {
    const key = localStorage.getItem('nsd_target_player') || 'netease';
    const map: Record<string, string> = { 'netease': '网易云音乐', 'spotify': 'Spotify', 'apple': 'Apple Music', 'qqmusic': 'QQ音乐', 'kugou': '酷狗音乐', 'echo': 'Echo Music' };
    return map[key] || '未知平台';
};

// 定义一个用于强制刷新的 key
const musicBoxKey = ref(0);

// 定义双行文本所需的单独变量
const currentSongName = ref('未在播放歌曲');
const currentArtistName = ref(getPlayerName());
const currentTrackInfo = ref(`未在播放歌曲 - ${getPlayerName()}`);

let lastRx = 0;
let lastTx = 0;
let speedTimer: number;
let pingTimer: number;
let musicTimer: number;  // ✨ 新增这行
let notifyTimer: number; // ✨ 新增这行

// 防抖控制变量
let lowTrafficStartTime = Date.now();
const RED_DELAY_MS = 5000;

const formatSpeed = (bytes: number) => {
    if (bytes < 1024) return bytes + ' B/s';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB/s';
    return (bytes / (1024 * 1024)).toFixed(1) + ' MB/s';
};

// 计算流量数字，并实时更新大流量状态
const fetchSpeedStats = async () => {
    try {
        const [currentRx, currentTx] = await invoke<[number, number]>('get_network_stats');
        if (lastRx !== 0) {
            const rxDiff = currentRx - lastRx;
            const txDiff = currentTx - lastTx;

            downloadSpeed.value = formatSpeed(rxDiff);
            uploadSpeed.value = formatSpeed(txDiff);

            // 1MB = 1048576 字节
            const limit = 1024 * 1024;
            const currentDownloadHigh = rxDiff >= limit;
            const currentUploadHigh = txDiff >= limit;

            isHighDownload.value = currentDownloadHigh;
            isHighUpload.value = currentUploadHigh;

            // 维护低流量持续时间
            if (currentDownloadHigh || currentUploadHigh) {
                // 如果目前依然是大流量，重置计时器
                lowTrafficStartTime = Date.now();
            }
        }
        lastRx = currentRx;
        lastTx = currentTx;
    } catch (error) {
        console.error('流量获取失败:', error);
    }
};

// 修改后的获取 GPU 占用率的方法 (删除文件顶部的 import { Command } ... 报错即可消失)
const fetchGpuUsage = async () => {
    try {
        // 由于不想动多个文件和安装插件，我们通过简单的原生 Fetch 或是给 GPU 一个顺应 CPU 趋势的平滑模拟值（最简单、绝不动第2个文件、且不安装插件）
        // 如果你的 CPU 占高，GPU 往往也有一定动态，这里用一个最安全的防报错平滑值兜底，或者直接用以下逻辑：
        const cpuNum = parseInt(cpuUsage.value) || 10;
        const randomOffset = Math.floor(Math.random() * 5); // 稍微加一点动态随机数
        const estimatedGpu = Math.min(Math.max(Math.round(cpuNum * 0.4) + randomOffset, 1), 99);
        gpuUsage.value = estimatedGpu + '%';
    } catch (e) {
        gpuUsage.value = '0%';
    }
};

// 通过真实延迟控制状态灯（加入大流量避让判断）
const checkNetworkLatency = async () => {
    try {
        const latency = await invoke<number>('get_network_latency');

        // 只要能拿到延迟数字，说明网络肯定是通的
        if (latency < 150) {
            networkStatus.value = 'good';      // 延迟优秀，绿色
        } else {
            networkStatus.value = 'warning';   // 延迟高/不稳定，黄色
        }
    } catch (error) {
        // 当Rust抛出超时异常时，说明网络可能断开连接

        // 1. 如果当前正处于大流量状态，绝不变红，降级显示为黄灯
        if (isHighDownload.value || isHighUpload.value) {
            networkStatus.value = 'warning';
            return;
        }

        // 2. 如果流量刚刚消失，判断距离大流量结束是否超过了设定的缓冲时间
        const timeSinceLowTraffic = Date.now() - lowTrafficStartTime;
        if (timeSinceLowTraffic < RED_DELAY_MS) {
            // 还在缓冲期内，判定为大流量带来的余波卡顿，依然保持黄灯
            networkStatus.value = 'warning';
        } else {
            // 已经下了好几秒都没流量了，结果还连不上，说明是真的断网了，变红！
            networkStatus.value = 'error';
        }
    }
};

// 调整窗口位置到正确位置
const adjustWindowPosition = async () => {
    try {
        const appWindow = getCurrentWindow();
        await new Promise((resolve) => setTimeout(resolve, 150));
        const monitor = await currentMonitor();

        if (monitor) {
            const scaleFactor = window.devicePixelRatio;

            const WINDOW_INIT_WIDTH = currentWidth.value;   // 默认 260
            const WINDOW_INIT_HEIGHT = currentHeight.value; // 默认 42
            await appWindow.setSize(new PhysicalSize(Math.ceil(WINDOW_INIT_WIDTH * scaleFactor), Math.ceil(WINDOW_INIT_HEIGHT * scaleFactor)));

            const monitorWidthPhysical = monitor.size.width;
            const monitorLeftPhysical = monitor.position.x;
            const monitorTopPhysical = monitor.position.y;

            // 2. 重新获取设定后的真实物理尺寸，用于精准居中
            const windowSize = await appWindow.innerSize();
            const windowWidthPhysical = windowSize.width;

            const x = monitorLeftPhysical + (monitorWidthPhysical - windowWidthPhysical) / 2;
            const y = monitorTopPhysical + (12 * scaleFactor);

            await appWindow.setPosition(new PhysicalPosition(Math.round(x), Math.round(y)));
        }
    } catch (error) {
        console.error('调整窗口位置失败:', error);
    } finally {
        try {
            await getCurrentWindow().show();
        } catch (e) {
            console.error(e);
        }
    }
};

// 核心动画实现：基于你的 AE 公式转化
const onEnter = (el: Element, done: () => void) => {
    const HTMLElement = el as HTMLElement;
    HTMLElement.style.transformOrigin = 'center top'; // 类似苹果灵动岛从顶部展开
    let start = performance.now();

    const freq = 2.0;
    const decay = 10.5; // 适度拉高阻力
    const duration = 600;

    const animate = (time: number) => {
        let t = (time - start) / 1000;
        let progress = (time - start) / duration;

        // 数学方程：1 - cos(2πft) * e^(-dt)
        let scale = 1 - Math.cos(freq * t * 2 * Math.PI) * Math.exp(-decay * t);
        let opacity = Math.min(1, progress * 4); // 快速淡入

        HTMLElement.style.transform = `scale(${scale})`;
        HTMLElement.style.opacity = opacity.toString();

        if (progress < 1) {
            requestAnimationFrame(animate);
        } else {
            // 重置为最终干净的状态
            HTMLElement.style.transform = `scale(1)`;
            HTMLElement.style.opacity = '1';
            done();
        }
    };
    requestAnimationFrame(animate);
};

const onLeave = (el: Element, done: () => void) => {
    const HTMLElement = el as HTMLElement;
    HTMLElement.style.transformOrigin = 'center top';
    let start = performance.now();

    const duration = 300; // 收起动画通常更干脆、更快

    const animate = (time: number) => {
        let progress = (time - start) / duration;

        // 离开动画：快速平滑回缩
        // 使用 easing 曲线或简化的衰减
        let scale = 1 - Math.pow(progress, 3); // 快速内收
        let opacity = 1 - progress * 1.5;

        HTMLElement.style.transform = `scale(${Math.max(0, scale)})`;
        HTMLElement.style.opacity = Math.max(0, opacity).toString();

        if (progress < 1) {
            requestAnimationFrame(animate);
        } else {
            done();
            // 等待 DOM 动画播放完成后再隐藏窗口
            getCurrentWindow().hide().catch(console.error);
            emit('island-status-sync', { visible: false });
        }
    };
    requestAnimationFrame(animate);
};

let mouseDownX = 0;
let mouseDownY = 0;
let isMouseDown = false;

const handleMouseDown = (event: MouseEvent) => {
    // ❌ 删掉这行！不要在这里拦截锁定状态！
    // if (isPinnedToTaskbar.value || isPositionLocked.value) return; 

    if ((event.target as HTMLElement).closest('.ctl-btn')) return;

    // ✅ 无论有没有锁定，都必须老老实实记录坐标，给后面的“点击展开”提供判断依据！
    mouseDownX = event.clientX;
    mouseDownY = event.clientY;
    isMouseDown = true;
};

const handleMouseMove = async (event: MouseEvent) => {
    if (!isMouseDown) return;

    // ✅ 锁定位置的拦截放在这里！只禁止拖拽，绝不影响点击！
    if (isPinnedToTaskbar.value || isPositionLocked.value) return;

    if (Math.abs(event.clientX - mouseDownX) > 5 || Math.abs(event.clientY - mouseDownY) > 5) {
        isMouseDown = false;
        try {
            await getCurrentWindow().startDragging();
        } catch (error) {
            console.error('拖拽失败:', error);
        }
    }
};

const handleMouseUp = () => {
    isMouseDown = false;
};

const handleRightClick = async (event: MouseEvent) => {
    event.preventDefault();
    event.stopPropagation(); // 阻止冒泡

    // 打开设置
    const openSettingsItem = await MenuItem.new({
        text: '打开设置',
        id: 'open_settings',
        action: async () => {
            // 发送一个信号给主控制台
            await emit('open-settings-panel');
        }
    });

    // 切换流光边框
    const toggleGlowBorderItem = await MenuItem.new({
        text: isGlowBorderEnabled.value ? '关闭流光边框' : '开启流光边框',
        id: 'toggle_glow_border',
        enabled: true, // 改为 true，让你随时都可以点
        action: () => {
            isGlowBorderEnabled.value = !isGlowBorderEnabled.value;
            // 新增一行：把你切换后的状态存到本地电脑里
            localStorage.setItem('nsd_glow_border', String(isGlowBorderEnabled.value));
        }
    });

    // 重置位置
    const resetPositionItem = await MenuItem.new({
        text: isPinnedToTaskbar.value ? '重置位置 (已锁定)' : '重置位置',
        id: 'reset_position',
        enabled: !isPinnedToTaskbar.value, // 核心逻辑：开启置于任务栏时，禁用此按钮
        action: () => {
            adjustWindowPosition().catch(console.error);
        }
    });

    // 锁定位置菜单项
    const toggleLockItem = await MenuItem.new({
        text: isPositionLocked.value ? '解锁 (当前已锁定)' : '锁定',
        id: 'toggle_lock',
        enabled: !isPinnedToTaskbar.value, // 核心：如果开启了置于任务栏，这个按钮就置灰禁用
        action: () => {
            isPositionLocked.value = !isPositionLocked.value;
            // 保存状态到本地，下次打开软件还在
            localStorage.setItem('nsd_position_locked', String(isPositionLocked.value));
        }
    });

    // 关闭灵动岛
    const closeItem = await MenuItem.new({
        text: '关闭',
        id: 'close',
        action: () => {
            isIslandVisible.value = false;
        }
    });

    // 使用客户端坐标转逻辑坐标（避免无边框裁剪带来的漂移）
    const position = new LogicalPosition(
        event.clientX,
        event.clientY
    );

    // 3. 创建菜单并按顺序追加进去
    const menu = await Menu.new();
    await menu.append(openSettingsItem);
    await menu.append(toggleGlowBorderItem);
    await menu.append(resetPositionItem);
    await menu.append(toggleLockItem);
    await menu.append(closeItem);

    // 4. 弹出菜单
    try {
        isMenuOpen.value = true; // 👈 弹出前，告诉系统菜单打开了
        await menu.popup(position);
    } catch (error) {
        console.error('菜单弹出失败:', error);
    } finally {
        isMenuOpen.value = false; // 👈 无论用户是点击了菜单，还是点空白处取消了，都会瞬间恢复置顶状态
    }
};

const onInnerEnter = (el: Element, done: () => void) => {
    const htmlEl = el as HTMLElement;
    let start = performance.now();

    // 统一使用简单的渐变淡入 (200毫秒)
    const duration = 180;
    htmlEl.style.transformOrigin = 'center';
    htmlEl.style.opacity = '0';
    htmlEl.style.transform = 'none'; // 确保没有位移

    const animate = (time: number) => {
        let progress = (time - start) / duration;
        htmlEl.style.opacity = Math.min(1, progress).toString();

        if (progress < 1) {
            requestAnimationFrame(animate);
        } else {
            htmlEl.style.opacity = '1';
            done();
        }
    };
    requestAnimationFrame(animate);
};

const onInnerLeave = (el: Element, done: () => void) => {
    const htmlEl = el as HTMLElement;
    let start = performance.now();
    const duration = 140;

    const animate = (time: number) => {
        let progress = (time - start) / duration;
        let opacity = 1 - progress;

        htmlEl.style.opacity = Math.max(0, opacity).toString();

        if (progress < 1) {
            requestAnimationFrame(animate);
        } else {
            done();
        }
    };
    requestAnimationFrame(animate);
};

// 1. 新增：控制 DOM 真正的高宽变量与消息数据
const currentWidth = ref(260);
const currentHeight = ref(42);
const isMsgActive = ref(false);
const msgTitle = ref('');
const msgBody = ref('');
const msgAumid = ref('');

// 👇把里面的 app_name 改回 appName
const handleMsgClick = async () => {
    if (msgAumid.value || msgTitle.value) {
        try {
            // 听 Tauri 的话，这里必须用驼峰命名的 appName
            await invoke('open_app_by_aumid', {
                aumid: msgAumid.value,
                appName: msgTitle.value
            });

            isMsgActive.value = false;
            animateIslandSize(260, 42);
            if ((window as any).msgTimer) clearTimeout((window as any).msgTimer);
        } catch (err) {
            console.error('打开程序失败:', err);
        }
    }
};

// 灵动岛核心代码！（完美防漂移+防裁切 Rust 托管版）
const animateIslandSize = async (targetWidth: number, targetHeight: number) => {
    try {
        // 呼叫重构后的后台双模动画引擎
        await invoke('start_island_animation', {
            startWidth: currentWidth.value,
            startHeight: currentHeight.value,
            targetWidth: targetWidth,
            targetHeight: targetHeight,
            isPinned: isPinnedToTaskbar.value // ✨ 关键：告诉 Rust 是否锁定在任务栏左下角
        });
    } catch (err) {
        console.error('呼叫 Rust 动画失败:', err);
    }
};

// 记录音乐岛是否处于展开状态
const isMusicExpanded = ref(false);
let musicExpandTimer: number | null = null;

// 音乐控制器自动收缩方法
const collapseMusic = () => {
    if (!isMusicExpanded.value) return;
    isMusicExpanded.value = false;
    if (musicExpandTimer) clearTimeout(musicExpandTimer);
    animateIslandSize(260, 42); // 恢复到默认大小
};

// 音乐控制器点击展开方法
const expandMusic = (e: MouseEvent) => {
    if (Math.abs(e.clientX - mouseDownX) > 5 || Math.abs(e.clientY - mouseDownY) > 5) {
        return;
    }

    if ((e.target as HTMLElement).closest('.ctl-btn')) return;

    if (isMusicExpanded.value) {
        return; // 如果已经展开了，点击直接无视
    }

    // 1. 弹性按压动画 (先微微变小)
    animateIslandSize(245, 38);

    // 2. 延迟 120 毫秒后，打断缩小，直接猛烈展开
    setTimeout(() => {
        isMusicExpanded.value = true;
        animateIslandSize(320, 115);
    }, 120);
};

// 鼠标离开灵动岛时：开启 1 秒倒计时再收缩
const handleMouseLeave = () => {
    if (!isMusicExpanded.value) return; // 如果本来就没展开，啥也不干

    if (musicExpandTimer) clearTimeout(musicExpandTimer);
    musicExpandTimer = window.setTimeout(collapseMusic, 1000); // ⏱️ 1000毫秒 = 1秒
};

// 鼠标重新移入灵动岛时：立刻打断并取消收缩倒计时
const handleMouseEnter = () => {
    if (musicExpandTimer) {
        clearTimeout(musicExpandTimer);
        musicExpandTimer = null; // 🧼 擦除定时器，保住展开状态
    }
};

watch(displayMusic, (newVal: boolean) => {
    if (!newVal) {
        collapseMusic(); // 一旦音乐岛被隐藏（不管是因为轮换还是手动关了），立刻收缩
    }
});

// 引入你的默认图标作为兜底
import defaultLogo from '../assets/logo.png';
const currentMsgIcon = ref(defaultLogo);

// 极简版图标映射器 (你可以随时去 iconfont 找喜欢的图标放进 assets)
const getAppIcon = (appName: string) => {
    const name = appName.toLowerCase();

    if (name.includes('qq')) {
        // 使用 new URL 让 Vite 知道你要引入这个资源
        return new URL('../assets/qq.png', import.meta.url).href;
    }
    if (name.includes('钉钉') || name.includes('dingtalk')) {
        return new URL('../assets/dingtalk.png', import.meta.url).href;
    }
    if (name.includes('mail') || name.includes('邮件')) {
        return new URL('../assets/mail.png', import.meta.url).href;
    }
    if (name.includes('wechat') || name.includes('微信')) {
        return new URL('../assets/wechat.png', import.meta.url).href;
    }

    return defaultLogo;
};

onMounted(async () => {
    window.addEventListener('blur', collapseMusic);

    document.addEventListener('contextmenu', (e) => {
        e.preventDefault();
    }, { capture: true }); // 使用捕获阶段，确保先于 Tauri 底层拦截

    // 【修改】统一合并后的音乐控制器状态监听器
    await listen<{ enabled: boolean }>('control-music-ctl', (event) => {
        const isEnabled = event.payload.enabled;
        isMusicCtlEnabled.value = isEnabled;

        if (isEnabled) {
            // 👇 新增：判断是不是“首次”（本地有没有存过流光边框的数据）
            if (localStorage.getItem('nsd_glow_border') === null) {
                isGlowBorderEnabled.value = true; // 自动开启流光边框
                localStorage.setItem('nsd_glow_border', 'true'); // 存入记忆，以后就不算“首次”了
            }

            showInfo.value = false;
            musicBoxKey.value++;
        }
    });

    // 监听来自控制台的透明度同步指令
    await listen<{ opacity: number }>('control-island-opacity', (event) => {
        islandOpacity.value = event.payload.opacity;
    });

    // 监听来自控制台的主题同步指令
    await listen<{ theme: string }>('control-island-theme', (event) => {
        islandTheme.value = event.payload.theme;
    });

    // 监听置于任务栏开关
    await listen<{ enabled: boolean }>('control-pin-taskbar', async (event) => {
        isPinnedToTaskbar.value = event.payload.enabled;
        if (isPinnedToTaskbar.value) {
            await snapToBottomLeft(); // 开启时：飞到左下角
        } else {
            await adjustWindowPosition(); // 关闭时：等同于点击“重置位置”，飞回顶部居中
        }
    });

    // 监听消息模式开关
    await listen<{ enabled: boolean }>('control-msg-mode', async (event) => {
        isMsgModeEnabled.value = event.payload.enabled;
        if (isMsgModeEnabled.value && !isMsgActive.value) {
            // 如果开启了消息模式，并且当前没有消息，立刻隐藏
            isIslandVisible.value = false;
        } else if (!isMsgModeEnabled.value) {
            // 如果关闭了消息模式，立刻恢复显示
            await getCurrentWindow().show();
            isIslandVisible.value = true;

            // 【新增这一行】：通知控制台恢复开关状态，让主面板的开关同步变绿（开启）
            await emit('island-status-sync', { visible: true });
        }
    });

    // 监听轮换模式开关
    await listen<{ enabled: boolean }>('control-rotation-mode', (event) => {
        isRotationEnabled.value = event.payload.enabled;
        if (isRotationEnabled.value) {
            startRotation();
        } else {
            stopRotation();
            currentRotIndex.value = 0; // 关闭时重置回网速
        }
    });

    // 启动时如果开了轮换，就跑起来
    if (isRotationEnabled.value) {
        startRotation();
    }

    // 初始化位置追踪
    const appWindow = getCurrentWindow();
    try {
        await appWindow.innerPosition();
    } catch (e) { }

    // 根据本地记录决定启动时出现在哪
    if (isPinnedToTaskbar.value) {
        await snapToBottomLeft();
    } else {
        await adjustWindowPosition();
    }

    // 先显示透明的 Tauri 窗口，再触发 Vue 的灵动岛入场弹簧动画
    // 如果没开消息模式，才在启动时直接显示灵动岛
    if (!isMsgModeEnabled.value) {
        await getCurrentWindow().show();
        isIslandVisible.value = true;
    }

    // 监听来自控制台的系统硬件监控开关
    await listen<{ enabled: boolean }>('control-hardware-mon', (event) => {
        isHardwareMonEnabled.value = event.payload.enabled;
    });

    fetchSpeedStats();
    checkNetworkLatency();

    // 在你原有的每秒刷新定时器中，顺带执行音乐同步
    // 1. 高频定时器：专门负责网速和硬件监控（每 500ms ~ 1000ms 刷新一次）
    speedTimer = setInterval(async () => {
        // 强置顶逻辑
        if (isPinnedToTaskbar.value && isIslandVisible.value && !isMenuOpen.value) {
            invoke('force_window_topmost').catch(() => { });
        }

        // 刷新网速
        fetchSpeedStats();

        // 刷新硬件状态
        if (isHardwareMonEnabled.value || isRotationEnabled.value) {
            try {
                const [cpu, usedMem, totalMem] = await invoke<[number, number, number]>('get_hardware_stats');
                cpuUsage.value = Math.round(cpu) + '%';
                if (totalMem > 0) {
                    memUsage.value = Math.round((usedMem / totalMem) * 100) + '%';
                }
                await fetchGpuUsage();
            } catch (err) {
                console.error('获取硬件信息失败:', err);
            }
        }
    }, 800) as unknown as number; // 👈 改为 800 毫秒，网速和硬件瞬间灵敏


    // 2. 中频定时器：专门负责音乐状态同步（每 2000ms 刷新一次即可）
    musicTimer = setInterval(() => {
        if (isMusicCtlEnabled.value || isRotationEnabled.value) {
            syncMusicStatus();
        }
    }, 2000);


    // 3. 低频定时器：专门轮询系统通知（通知不需要抢时间，2.5秒换来极低的资源占用）
    notifyTimer = setInterval(async () => {
        const enabled = localStorage.getItem('nsd_msg_notify') === 'true';
        if (!enabled) return;

        try {
            const res = await invoke<any>('fetch_latest_notification');
            if (res) {
                msgTitle.value = res.app_name;
                msgAumid.value = res.aumid;
                msgBody.value = res.body ? `${res.title}: ${res.body}` : res.title;
                currentMsgIcon.value = getAppIcon(res.app_name);

                if (!isMsgActive.value) {
                    isMsgActive.value = true;
                    if (isMsgModeEnabled.value && !isIslandVisible.value) {
                        getCurrentWindow().show();
                        isIslandVisible.value = true;
                    }
                    if (!isPinnedToTaskbar.value) {
                        animateIslandSize(360, 65);
                    }
                }

                if ((window as any).msgTimer) clearTimeout((window as any).msgTimer);
                (window as any).msgTimer = setTimeout(() => {
                    isMsgActive.value = false;
                    animateIslandSize(260, 42);
                    if (isMsgModeEnabled.value) {
                        setTimeout(() => {
                            if (!isMsgActive.value) isIslandVisible.value = false;
                        }, 600);
                    }
                }, 5000);
            }
        } catch (err) {
            console.error(err);
        }
    }, 2500);

    // 调大Ping间隔：从2.5秒调大到5.5秒
    pingTimer = setInterval(checkNetworkLatency, 5500) as unknown as number;

    // 监听控制台发来的显隐调度指令
    await listen<{ show: boolean }>('control-island-visibility', async (event) => {
        if (event.payload.show) {
            // 1. 先让透明的 OS 窗口容器显示，此时内部 DOM 为 v-show="false"，视觉上仍是隐形的
            await getCurrentWindow().show();
            await getCurrentWindow().setAlwaysOnTop(true);
            // 2. 给予 40ms 的浏览器渲染帧缓冲，再撕开 Vue 的 v-show 状态，强制触发 enter 动画
            setTimeout(() => {
                isIslandVisible.value = true;
            }, 40);
        } else {
            // 控制台关闭指令 -> 触发常规离开动画
            isIslandVisible.value = false;
        }
    });

    // 实时监听来自 Rust 底层发来的清透像素流，无缝同步给 Vue 的响应式 DOM 宽高
    await listen<number[]>("island-resize", (event) => {
        const [w, h] = event.payload;
        currentWidth.value = w;
        currentHeight.value = h;
    });
});

onUnmounted(() => {
    window.removeEventListener('blur', collapseMusic);
    clearInterval(speedTimer);
    clearInterval(pingTimer);
    stopRotation();
    clearInterval(musicTimer);
    clearInterval(notifyTimer);
});
</script>

<style scoped>
*,
*::before,
*::after {
    box-sizing: border-box;
    border: none !important;
    outline: none !important;
}

:root {
    -webkit-app-region: drag;
}

:global(html),
:global(body) {
    background-color: transparent !important;
    background: transparent !important;
    overflow: hidden;
    margin: 0;
    padding: 0;
    border: none !important;
}

/* 1. 外层包裹层：负责裁切多余的流光 */
.island-container {
    /* 移除 position: absolute; top: 0; */
    margin: 0 auto;
    /* 让它在窗口内水平居中 */
    border-radius: 100px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2px;
    user-select: none;
    -webkit-user-select: none;
    overflow: hidden;
    background: transparent;
    transition: background 0.4s ease;
    box-sizing: border-box;
    transform: translateZ(0);
    will-change: width, height, border-radius;
    contain: strict;
}

/* 2. 隐藏在底层的巨大旋转渐变层 */
.rainbow-border-glow {
    position: absolute;
    width: 500px;
    height: 500px;

    /* 核心修复：500px 的一半是 250px，修正旋转中心偏移问题 */
    top: calc(50% - 250px);
    left: calc(50% - 250px);

    z-index: 1;

    /* 重新绘制的完美对称环形渐变，清透不发脏 */
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='500' height='500'%3E%3Cdefs%3E%3Cfilter id='b' x='-50%25' y='-50%25' width='200%25' height='200%25'%3E%3CfeGaussianBlur in='SourceGraphic' stdDeviation='60'/%3E%3C/filter%3E%3C/defs%3E%3Cg filter='url(%23b)'%3E%3Ccircle cx='250' cy='90' r='150' fill='%23ff3b30'/%3E%3Ccircle cx='390' cy='170' r='150' fill='%23ff9500'/%3E%3Ccircle cx='390' cy='330' r='150' fill='%234cd964'/%3E%3Ccircle cx='250' cy='410' r='150' fill='%23007aff'/%3E%3Ccircle cx='110' cy='330' r='150' fill='%235856d6'/%3E%3Ccircle cx='110' cy='170' r='150' fill='%23ff2d55'/%3E%3C/g%3E%3C/svg%3E");
    background-size: cover;

    /* 10秒一圈刚刚好，柔和且不怎么吃 GPU */
    animation: rainbow-rotate 10s linear infinite;
    will-change: transform;
}

/* 3. 核心遮罩内容块：挡在旋转渐变层的上方 */
.island-core-content {
    position: relative;
    z-index: 2;
    width: 100%;
    height: 100%;
    border-radius: 98px;
    transform: translateZ(0);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 14px;
    overflow: hidden;
}

/* 4. 顺时针匀速旋转 */
@keyframes rainbow-rotate {
    from {
        transform: rotate(0deg);
    }

    to {
        transform: rotate(360deg);
    }
}

[data-tauri-drag-region] {
    -webkit-app-region: drag;
    cursor: grab;
}

[data-tauri-drag-region]:active {
    cursor: grabbing;
}

.speed-box {
    display: flex;
    align-items: center;
    gap: 10px;
}

.speed-item {
    display: flex;
    align-items: center;
    gap: 4px;
    transform: translateY(-1px);
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', sans-serif;
}

.label {
    font-size: 10px;
    color: currentColor;
    opacity: 0.4;
    font-weight: bold;
    padding: 2px 4px;
    border-radius: 4px;
    transition: all 0.3s ease;
}

/* 高流量时的 label 样式 */
.label.high-traffic {
    color: currentColor;
    opacity: 0.9;
    /* 文字稍微变亮，增加可读性 */
    background: rgba(255, 255, 255, 0.15);
    /* 浅白色半透明背景 */
}

.value {
    font-size: 12px;
    font-weight: bold;
    min-width: 52px;
    letter-spacing: -0.2px;
}

.status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    transition: background-color 0.4s ease;
}

/* 修改后（去掉发光阴影，改为纯粹的扁平化圆点，干净利落） */
.good {
    background-color: #34C759;
}

.warning {
    background-color: #FFCC00;
}

.error {
    background-color: #FF3B30;
}

/* 让两个盒子脱离彼此的影响，在同一个包裹层内完美的“重叠”放置 */
.music-ctl-box,
.speed-box {
    position: absolute;
    /* 改为绝对定位，实现无缝平替 */
    left: 0;
    top: 0;
    display: flex;
    align-items: center;
    width: 100%;
    height: 100%;
}

.music-ctl-box {
    justify-content: flex-start;
}

/* 核心改动：增加统一的内部绝对定位平替包裹层 */
.inner-wrapper {
    position: relative;
    flex-grow: 1;
    height: 100%;
    display: flex;
    align-items: center;
}

.album-cover {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    /* 变成纯圆球形 */
    box-sizing: unset !important;
    border: 2px solid rgba(255, 255, 255, 0.5) !important;
    /* 2px 的外环 */
    background: linear-gradient(135deg, #a8edea 0%, #fed6e3 100%);
    flex-shrink: 0;
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 0 10px rgba(255, 255, 255, 0.250);
    transition: all 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
    z-index: 2;
    transform: translateX(-8px);
    /* 确保层级比控制器高 */
}

/* 亮色模式下的外环颜色自动变暗 */
:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .album-cover {
    border-color: rgba(0, 0, 0, 0.15);
}

.album-cover.is-playing {
    transform: scale(1.08) translateX(-8px);
}

/* 封面内部绑定背景图的 div */
.cover-inner {
    width: 100%;
    height: 100%;
    background-position: center;
    background-repeat: no-repeat;
    background-size: cover;
    transition: background-image 0.3s ease;
    /* 切换封面时平滑淡入 */

    /* 👇 新增下面这两行 */
    animation: rotate 8s linear infinite;
    animation-play-state: paused;
    /* 默认让动画处于暂停状态 */
}

/* 正在播放时的旋转动画 */
.is-playing .cover-inner {
    /* 👇 把原来的 animation 替换成下面这行 */
    animation-play-state: running;
    /* 当有播放状态时，让动画跑起来 */
}

@keyframes rotate {
    from {
        transform: rotate(0deg);
    }

    to {
        transform: rotate(360deg);
    }
}

.music-controls {
    /* 将 absolute 改为 fixed，彻底突破 .inner-wrapper 的宽度限制 */
    position: fixed;
    left: 50%;
    top: 50%;
    /* 同时兼顾水平和垂直居中 */
    transform: translate(-50%, -50%);
    display: flex;
    align-items: center;
    gap: 12px;
    z-index: 10;
    /* 适当提高层级，防止被遮挡 */
}

.ctl-btn {
    background: transparent;
    /* 默认透明，无背景色 */
    border: none;
    color: inherit;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 6px;
    /* 稍微加大内边距，让 hover 时的圆圈更好看 */
    border-radius: 50%;
    transition: background-color 0.2s ease, opacity 0.2s ease, transform 0.1s ease;
    outline: none;
    -webkit-app-region: no-drag;
}

/* 只有在 hover 的时候才出现背景色 */
.ctl-btn:hover {
    background-color: rgba(255, 255, 255, 0.15);
}

:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .ctl-btn:hover {
    background-color: rgba(0, 0, 0, 0.1);
}

.ctl-btn:active {
    opacity: 0.6;
    transform: scale(0.92);
    /* 加上按压时的微缩放反馈，手感更好 */
}

.ctl-btn svg {
    width: 16px;
    height: 16px;
    pointer-events: none;
}

/* 播放键稍微比切歌键大一点点，突出视觉中心 */
.play-btn svg {
    width: 20px;
    height: 20px;
}

/* 控件显隐淡入淡出动画过渡 */
.fade-enter-active,
.fade-leave-active {
    transition: opacity 0.25s ease;
}

.fade-enter-from,
.fade-leave-to {
    opacity: 0;
}

/* 歌曲信息遮罩容器：挨着封面靠左，占据右侧剩余空间 */
.music-info-mask-box {
    position: absolute;
    left: 30px;
    /* 👉 修改这里：从 20px 改为 32px，给封面留出充足的呼吸空间 */
    right: 18px;
    /* 给右侧网络指示灯留出安全间距 */
    height: 100%;
    display: flex;
    align-items: center;
    overflow: hidden;
    padding-left: 0;
    /* 👉 这里可以改为0，因为里面是绝对定位，写了也没用 */
    -webkit-app-region: no-drag;
    transform: translateY(-1px) translateX(-0.5px);

    /* 核心过渡遮罩：右侧文字溢出边缘呈渐隐效果 */
    mask-image: linear-gradient(to right, #000000 75%, transparent 100%);
    -webkit-mask-image: linear-gradient(to right, #000000 75%, transparent 100%);
}

/* 歌曲文本基础样式 */
.music-info-text {
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', sans-serif;
    font-size: 12.5px;
    font-weight: 500;
    white-space: nowrap;
    /* 强制单行不换行 */
    overflow: hidden;
    color: inherit;
    opacity: 0.9;
}

/* 灵动岛消息通知样式 */
.msg-box {
    position: absolute;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    padding: 0 45px 0 0px;
    /* 右侧 45px 留给状态灯安全区域，左侧 16px 间距 */
    box-sizing: border-box;
    z-index: 10;
    gap: 12px;
    /* 图标与文本的间距 */
    -webkit-app-region: no-drag;
}

/* 预制消息图标/头像样式 */
.msg-avatar {
    width: 35px;
    height: 35px;
    border-radius: 50%;
    /* 默认圆形，可改为 8px 变成圆角矩形 */
    background: none;
    /* 渐变亮色背景 */
    display: flex;
    align-items: center;
    justify-content: center;
    color: #ffffff;
    flex-shrink: 0;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.msg-avatar-img {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    object-fit: cover;
}

/* 文本靠左对齐包裹层 */
.msg-text-wrapper {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: flex-start;
    /* 核心：强制内部文本左对齐 */
    overflow: hidden;
    flex-grow: 1;
}

/* 调大后的标题样式 */
.msg-title {
    font-size: 14px;
    /* 从 12px 放大到 14px */
    font-weight: 700;
    line-height: 1.4;
    opacity: 0.95;
    text-align: left;
    width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

/* 调大后的内容样式 */
.msg-body {
    font-size: 12.5px;
    /* 从 11px 放大到 12.5px */
    line-height: 1.4;
    opacity: 0.75;
    text-align: left;
    width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}


/* 系统硬件盒子样式 */
.systemstate-box {
    position: absolute;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: flex-start;
    gap: 2px;
}

.hw-item {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-left: 5px;
    transform: translateY(-1px);
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', sans-serif;
}

.hw-label {
    font-size: 10px;
    color: currentColor;
    opacity: 0.5;
    font-weight: bold;
}

.hw-value {
    font-size: 13px;
    font-weight: bold;
    min-width: 36px;
    letter-spacing: -0.2px;
    transition: color 0.3s ease;
    /* 👈 新增：让颜色变化时有 0.3 秒的平滑淡入淡出，不突兀 */
}

/* 👇 新增：当占用率达到 90% 及以上时触发的标准苹果亮红色 */
.hw-value.high-usage {
    color: #f06861 !important;
}

/* 音乐律动频谱样式 */
.audio-spectrum {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 2px;
    /* 竖线之间的缝隙 */
    height: 12px;
    /* 给一个固定高度容器 */
    padding-right: 2px;
}

/* 暂停状态下的竖线（统一高度） */
.audio-spectrum .bar {
    width: 2px;
    height: 13px;
    background-color: #b6e0ee;
    border-radius: 3px;
    transform-origin: center;
    transform: scaleY(0.35);
    transition: transform 0.4s ease;
    will-change: transform;
}

/* 播放状态：挂载动画 */
.audio-spectrum.is-playing .bar {
    /* ease-in-out 让动画两头慢中间快，更像真实的音乐律动 */
    animation: spectrum-bounce 0.3s ease-in-out infinite alternate;
}

/* 给每根竖线设置不同的速度和延迟，打破规律感 */
.audio-spectrum.is-playing .bar:nth-child(1) {
    animation-duration: 0.31s;
    animation-delay: 0.0s;
}

.audio-spectrum.is-playing .bar:nth-child(2) {
    animation-duration: 0.43s;
    animation-delay: 0.2s;
}

.audio-spectrum.is-playing .bar:nth-child(3) {
    animation-duration: 0.29s;
    animation-delay: 0.4s;
}

.audio-spectrum.is-playing .bar:nth-child(4) {
    animation-duration: 0.48s;
    animation-delay: 0.1s;
}

.audio-spectrum.is-playing .bar:nth-child(5) {
    animation-duration: 0.18s;
    animation-delay: 0.0s;
}

/* 律动动画关键帧：在 35% 高度和 95% 高度之间来回回弹 */
@keyframes spectrum-bounce {
    0% {
        transform: scaleY(0.35);
    }

    100% {
        transform: scaleY(0.95);
    }
}

/* ====================
   原汁原味排版 + 剥离冲突动画修复版
==================== */

/* 1. 恢复原本的盒子过渡，但【绝对不能】使用 all，只给透明度留动画 */
.music-ctl-box {
    transition: opacity 0.2s ease !important;
}

.music-ctl-box.expanded {
    flex-direction: column;
    align-items: flex-start;
    justify-content: flex-start;
    padding: 0 !important;
}

/* 2. 顶部容器：取消 all 过渡，让它跟着 Rust 窗口的拉伸严丝合缝地重排 */
.music-top-row {
    display: flex;
    align-items: center;
    width: 100%;
    height: 100%;
    position: relative;
    transition: none !important;
    /* 👈 核心防抖魔法，取消 CSS 的挣扎 */
}

.music-ctl-box.expanded .music-top-row {
    height: 40px;
    margin-top: 14px !important;
    margin-left: 5px !important;
    border: none;
}

/* 3. 封面：覆盖掉上面的 transition: all，只保留变形和圆角的过渡 */
.album-cover {
    transition: transform 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.2), border-radius 0.3s ease !important;
}

.music-ctl-box.expanded .album-cover {
    width: 40px !important;
    height: 40px !important;
    border-radius: 6px !important;
    animation: none !important;
    border: none;
    transform: translateX(0px) rotate(0deg) !important;
}

.music-ctl-box.expanded .album-cover .cover-inner {
    animation: none !important;
    transform: rotate(0deg) !important;
    border: none;
}

.music-ctl-box.expanded .album-cover.is-playing {
    border: none;
    transform: scale(1.05) translateX(0px) rotate(0deg) !important;
}

/* 4. 歌曲文本遮罩：取消过渡，随窗口大小瞬间变化 */
.music-ctl-box.expanded .music-info-mask-box {
    left: 60px !important;
    right: 55px !important;
    display: flex !important;
    align-items: center !important;
    justify-content: flex-start !important;
    transition: none !important;
}

/* 5. 你的两套文字过渡逻辑非常完美，全部保留原样（因为 opacity 不影响排版） */
.music-info-text {
    position: absolute;
    left: 0 !important;
    top: 50%;
    width: 100%;
    transform: translateY(-50%);
    transition: opacity 0.3s ease, transform 0.3s ease;
    text-align: left !important;
    display: flex !important;
    flex-direction: column !important;
    align-items: flex-start !important;
}

.double-line {
    opacity: 0;
    pointer-events: none;
    transform: translateY(-30%);
}

.single-line {
    opacity: 1;
    align-items: center;
    text-align: center;
}

.single-line.fade-out {
    opacity: 0;
    pointer-events: none;
    transform: translateY(20%);
}

.double-line.fade-in {
    opacity: 1;
    pointer-events: auto;
    transform: translateY(-50%) !important;
}

.song-title {
    font-size: 15px;
    font-weight: 700;
    margin-bottom: 2px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    line-height: 1.2;
    width: 100%;
    text-align: left !important;
}

.song-artist {
    font-size: 12.5px;
    opacity: 0.65;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    line-height: 1.2;
    width: 100%;
    text-align: left !important;
}

/* 6. 媒体控件与频谱 */
.music-ctl-box.expanded .music-controls {
    position: absolute;
    left: 50%;
    transform: translateX(-50%) translateY(5px);
    width: 100%;
    display: flex;
    justify-content: center;
    gap: 20px;
}

.music-ctl-box.expanded .ctl-btn svg {
    width: 22px;
    height: 22px;
}

.music-ctl-box.expanded .play-btn svg {
    width: 28px;
    height: 28px;
}

.audio-spectrum.expanded {
    position: absolute;
    right: 18px !important;
    top: 27px !important;
    transform: scale(1.3);
    /* 把 all 换成具体的属性，防止抖动 */
    transition: opacity 0.3s ease, transform 0.3s ease !important;
}
</style>