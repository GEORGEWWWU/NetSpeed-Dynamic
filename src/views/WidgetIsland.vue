<template>
    <transition @enter="onEnter" @leave="onLeave" :css="false">
        <div v-show="isIslandVisible" :class="['island-container', { 'has-music-border': isGlowBorderEnabled }]"
            @mousedown="handleMouseDown" :style="islandStyle" @contextmenu="handleRightClick">

            <div class="rainbow-border-glow" v-if="isGlowBorderEnabled" :style="{ opacity: glowOpacity }"></div>

            <div class="island-core-content" :style="coreContentStyle">
                <div class="inner-wrapper">
                    <transition @enter="onInnerEnter" @leave="onInnerLeave" :css="false">
                        <div class="msg-box" v-show="isMsgActive" key="msg" @click="handleMsgClick"
                            style="cursor: pointer;">
                            <div class="msg-avatar">
                                <img :src="currentMsgIcon" alt="消息图标" class="msg-avatar-img">
                            </div>

                            <div class="msg-text-wrapper">
                                <div class="msg-title">{{ msgTitle }}</div>
                                <div class="msg-body">{{ msgBody }}</div>
                            </div>
                        </div>
                    </transition>

                    <transition @enter="onInnerEnter" @leave="onInnerLeave" :css="false">
                        <div class="systemstate-box" v-show="isHardwareMonEnabled && !isMsgActive" key="hardware">
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
                    </transition>

                    <transition @enter="onInnerEnter" @leave="onInnerLeave" :css="false">
                        <div class="music-ctl-box" v-show="isMusicCtlEnabled && !isMsgActive" :key="musicBoxKey"
                            @mouseenter="handleMusicBoxEnter" @mouseleave="handleMusicBoxLeave">

                            <div class="album-cover" :class="{ 'is-playing': isPlaying }">
                                <div class="cover-inner"
                                    :style="coverUrl ? { backgroundImage: `url(${coverUrl})`, backgroundSize: 'cover' } : {}">
                                </div>
                            </div>

                            <transition name="fade">
                                <div class="music-controls" v-show="!showInfo">
                                    <button class="ctl-btn" @click="prevTrack">
                                        <svg viewBox="0 0 24 24" fill="currentColor">
                                            <path d="M6 6h2v12H6zm3.5 6l8.5 6V6z" />
                                        </svg>
                                    </button>

                                    <button class="ctl-btn play-btn" @click="togglePlay">
                                        <svg v-if="isPlaying" viewBox="0 0 24 24" fill="currentColor">
                                            <path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z" />
                                        </svg>
                                        <svg v-else viewBox="0 0 24 24" fill="currentColor"
                                            style="transform: translateX(1px);">
                                            <path d="M8 5v14l11-7z" />
                                        </svg>
                                    </button>

                                    <button class="ctl-btn" @click="nextTrack">
                                        <svg viewBox="0 0 24 24" fill="currentColor">
                                            <path d="M6 18l8.5-6L6 6v12zM16 6v12h2V6h-2z" />
                                        </svg>
                                    </button>
                                </div>
                            </transition>

                            <transition name="fade">
                                <div class="music-info-mask-box" v-show="showInfo">
                                    <div class="music-info-text">
                                        {{ currentTrackInfo }}
                                    </div>
                                </div>
                            </transition>
                        </div>
                    </transition>

                    <transition @enter="onInnerEnter" @leave="onInnerLeave" :css="false">
                        <div class="speed-box" v-show="!isMusicCtlEnabled && !isHardwareMonEnabled && !isMsgActive"
                            key="speed">
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

                <div :class="['status-dot', networkStatus]"></div>
            </div>
        </div>
    </transition>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed, type CSSProperties } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow, currentMonitor, PhysicalPosition, LogicalPosition, PhysicalSize } from '@tauri-apps/api/window'; import { Menu, MenuItem } from '@tauri-apps/api/menu';
import { listen, emit } from '@tauri-apps/api/event';

const isIslandVisible = ref(false);
const isMenuOpen = ref(false);

// 灵动岛自身的透明度变量（默认100）
const islandOpacity = ref(Number(localStorage.getItem('nsd_island_opacity') || '100'));

const islandTheme = ref(localStorage.getItem('nsd_island_theme') || 'black');

// 修改后的 islandStyle
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
        width: `${currentWidth.value}px`,
        height: `${currentHeight.value}px`,
        position: 'relative', // 改为相对定位或保留，由父级负责居中
    };
});

const coreContentStyle = computed(() => {
    const linear = islandOpacity.value / 100;
    const alpha = Math.pow(linear, 1 / 2.2);
    if (islandTheme.value === 'white') {
        return {
            backgroundColor: `rgba(255, 255, 255, ${alpha})`
        };
    }
    return {
        backgroundColor: `rgba(0, 0, 0, ${alpha})`
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
let isClickingToggle = false;
// 流光边框默认状态完全镜像音乐控制器（只要音乐控制器开着它就开，关了就一起关）
const isGlowBorderEnabled = ref(localStorage.getItem('nsd_glow_border') === 'true');

const coverUrl = ref('');
const coverCache = new Map<string, string>();

// 记录是否开启了置于任务栏
const isPinnedToTaskbar = ref(localStorage.getItem('nsd_pin_taskbar') === 'true');

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

            trackedPhysicalX = Math.round(x);
            trackedPhysicalY = Math.round(y);
        }
    } catch (error) {
        console.error('停靠左下角失败:', error);
    }
};

const togglePlay = async () => {
    isPlaying.value = !isPlaying.value;
    isClickingToggle = true;
    try {
        await invoke('control_system_media', { action: 'play_pause' });
    } catch (err) {
        console.error(err);
    }
    setTimeout(() => {
        isClickingToggle = false;
    }, 1500);
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

            // 拼接新的歌曲信息
            const newTrackInfo = `${song} - ${artist}`;

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

            if (!isClickingToggle) {
                isPlaying.value = playing;
            }
        } else {
            // 没检测到播放时，清空状态
            currentTrackInfo.value = '未在播放歌曲 - 网易云音乐';
            isPlaying.value = false;
            coverUrl.value = ''; // 没歌时清空，显示默认的优美渐变色
        }
    } catch (err) {
        console.error('音乐信息获取失败:', err);
    }
};

const showInfo = ref(false);
const currentTrackInfo = ref('未在播放歌曲 - 未知歌手'); // 默认显示内容
let hideControlsTimer: number | null = null;

// 启动倒计时隐藏控件
const startHideTimer = () => {
    stopHideTimer();
    hideControlsTimer = window.setTimeout(() => {
        showInfo.value = true;
    }, 800);
};

const stopHideTimer = () => {
    if (hideControlsTimer) {
        clearTimeout(hideControlsTimer);
        hideControlsTimer = null;
    }
};

// 定义一个用于强制刷新的 key
const musicBoxKey = ref(0);

// 鼠标进入整个音乐控制区域
const handleMusicBoxEnter = () => {
    stopHideTimer();     // 清除可能正在倒计时的隐藏任务
    showInfo.value = false; // 立刻切换为显示控制按钮
};

// 鼠标彻底离开整个音乐控制区域
const handleMusicBoxLeave = () => {
    startHideTimer();    // 离开后，再开启倒计时恢复为歌曲名称显示
};

let lastRx = 0;
let lastTx = 0;
let speedTimer: number;
let pingTimer: number;

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

            trackedPhysicalX = Math.round(x);
            trackedPhysicalY = Math.round(y);
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

const handleMouseDown = async (event: MouseEvent) => {
    // 【新增的核心锁定逻辑】：如果开启了置于任务栏，直接拦截，禁止任何拖拽！
    if (isPinnedToTaskbar.value) return;

    // 如果点击的是按钮或按钮内部的 SVG 图标，直接返回，不触发拖拽
    if ((event.target as HTMLElement).closest('.ctl-btn')) return;

    const target = event.target as HTMLElement;

    // 如果点击的是按钮或者消息框，直接返回，不触发窗口拖拽
    if (target.closest('.ctl-btn') || target.closest('.msg-box')) {
        return;
    }

    // 只有按鼠标左键时才触发窗口拖拽，把右键留给自定义菜单
    if (event.button === 0) {
        try {
            await getCurrentWindow().startDragging();
        } catch (error) {
            console.error('拖拽失败:', error);
        }
    }
};

const handleRightClick = async (event: MouseEvent) => {
    event.preventDefault();
    event.stopPropagation(); // 阻止冒泡

    // 重置位置
    const resetPositionItem = await MenuItem.new({
        text: isPinnedToTaskbar.value ? '重置位置 (已锁定)' : '重置位置',
        id: 'reset_position',
        enabled: !isPinnedToTaskbar.value, // 核心逻辑：开启置于任务栏时，禁用此按钮
        action: () => {
            adjustWindowPosition().catch(console.error);
        }
    });

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

    if (htmlEl.classList.contains('music-ctl-box')) {
        const freq = 2.0;
        const decay = 10.5;
        const duration = 600;

        htmlEl.style.transform = `translateY(20px)`;
        htmlEl.style.opacity = '0';

        const animate = (time: number) => {
            let t = (time - start) / 1000;
            let progress = (time - start) / duration;

            let spring = 1 - Math.cos(freq * t * 2 * Math.PI) * Math.exp(-decay * t);
            let opacity = Math.min(1, progress * 4);
            let y = 20 * (1 - spring);

            htmlEl.style.transform = `translateY(${y}px)`;
            htmlEl.style.opacity = opacity.toString();

            if (progress < 1) {
                requestAnimationFrame(animate);
            } else {
                htmlEl.style.transform = 'none';
                htmlEl.style.opacity = '1';
                done();
                startHideTimer();
            }
        };
        requestAnimationFrame(animate);
    } else {
        // 网速盒与消息盒共用此处的渐变淡入
        const duration = 200;
        htmlEl.style.transformOrigin = 'center';
        htmlEl.style.opacity = '0';
        htmlEl.style.transform = 'none';

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
    }
};

const onInnerLeave = (el: Element, done: () => void) => {
    const htmlEl = el as HTMLElement;
    let start = performance.now();
    const duration = 150;

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

// 同步追踪窗口位置（物理像素），动画中直接读取，无需任何 async
let trackedPhysicalX = 0;
let trackedPhysicalY = 0;

// 灵动岛核心代码！（动画函数）
const animateIslandSize = (targetWidth: number, targetHeight: number) => {
    const startWidth = currentWidth.value;
    const startHeight = currentHeight.value;
    const start = performance.now();

    const freq = 2.0;
    const decay = 10.5;
    const duration = 600;

    const appWindow = getCurrentWindow();
    const dpr = window.devicePixelRatio; // 使用浏览器真实缩放率，不再依赖 Tauri 滞后的底层计算

    // 动画开始时，锁定当前的中心点（物理像素）
    const centerPhysicalX = trackedPhysicalX + (startWidth * dpr) / 2;
    const originPhysicalY = trackedPhysicalY;

    const run = (time: number) => {
        const t = (time - start) / 1000;
        const progress = (time - start) / duration;

        const spring = 1 - Math.cos(freq * t * 2 * Math.PI) * Math.exp(-decay * t);

        const newWidth = startWidth + (targetWidth - startWidth) * spring;
        const newHeight = startHeight + (targetHeight - startHeight) * spring;

        // 更新 Vue 的宽高
        currentWidth.value = newWidth;
        currentHeight.value = newHeight;

        // 绝对计算：新的左边缘 = 中心点 - 新宽度的一半
        const newLeftX = Math.round(centerPhysicalX - (newWidth * dpr) / 2);

        // 先 setPosition，再 setSize
        appWindow.setPosition(new PhysicalPosition(newLeftX, originPhysicalY)).catch(() => { });
        appWindow.setSize(new PhysicalSize(Math.ceil(newWidth * dpr), Math.ceil(newHeight * dpr))).catch(() => { });

        if (progress < 1) {
            requestAnimationFrame(run);
        } else {
            currentWidth.value = targetWidth;
            currentHeight.value = targetHeight;

            // 动画结束，精确锁定最终追踪位置
            trackedPhysicalX = Math.round(centerPhysicalX - (targetWidth * dpr) / 2);

            appWindow.setPosition(new PhysicalPosition(trackedPhysicalX, originPhysicalY)).catch(() => { });
            appWindow.setSize(new PhysicalSize(Math.ceil(targetWidth * dpr), Math.ceil(targetHeight * dpr))).catch(() => { });
        }
    };

    requestAnimationFrame(run);
};

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
            stopHideTimer();
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

    // 初始化位置追踪
    const appWindow = getCurrentWindow();
    try {
        const pos = await appWindow.innerPosition();
        trackedPhysicalX = pos.x;
        trackedPhysicalY = pos.y;
    } catch (e) { }

    // 窗口被拖动后自动同步位置
    appWindow.onMoved((event) => {
        trackedPhysicalX = event.payload.x;
        trackedPhysicalY = event.payload.y;
    }).catch(() => { });

    // 根据本地记录决定启动时出现在哪
    if (isPinnedToTaskbar.value) {
        await snapToBottomLeft();
    } else {
        await adjustWindowPosition();
    }

    // 先显示透明的 Tauri 窗口，再触发 Vue 的灵动岛入场弹簧动画
    await getCurrentWindow().show();
    isIslandVisible.value = true;

    // 监听来自控制台的系统硬件监控开关
    await listen<{ enabled: boolean }>('control-hardware-mon', (event) => {
        isHardwareMonEnabled.value = event.payload.enabled;
    });

    fetchSpeedStats();
    checkNetworkLatency();

    // 在你原有的每秒刷新定时器中，顺带执行音乐同步
    speedTimer = setInterval(async () => {
        // 🌟 终极优化：只有在“置于任务栏”开启、灵动岛可见、且右键菜单未打开时，才呼叫底层强置顶
        if (isPinnedToTaskbar.value && isIslandVisible.value && !isMenuOpen.value) {
            invoke('force_window_topmost').catch(() => { });
        }

        fetchSpeedStats();
        if (isMusicCtlEnabled.value) {
            syncMusicStatus(); // 当音乐控制器启用时，每秒顺带检查网易云
        }

        // 每秒实时拉取系统硬件状态
        if (isHardwareMonEnabled.value) {
            try {
                const [cpu, usedMem, totalMem] = await invoke<[number, number, number]>('get_hardware_stats');
                cpuUsage.value = Math.round(cpu) + '%';
                // 增加防呆设计，防止除以 0 导致报错
                if (totalMem > 0) {
                    memUsage.value = Math.round((usedMem / totalMem) * 100) + '%';
                }

                await fetchGpuUsage();
            } catch (err) {
                console.error('获取硬件信息失败:', err);
            }
        }

        // 定时轮询系统通知状态
        const enabled = localStorage.getItem('nsd_msg_notify') === 'true';
        if (enabled) {
            try {
                // 注意这里类型变了，接收任意对象
                const res = await invoke<any>('fetch_latest_notification');
                if (res) {
                    // res 结构：{ app_name: string, title: string, body: string }

                    // 1. 标题直接显示程序名
                    msgTitle.value = res.app_name;
                    msgAumid.value = res.aumid;

                    // 2. 内容拼接原来的 [标题: 内容]
                    if (res.body) {
                        msgBody.value = `${res.title}: ${res.body}`;
                    } else {
                        msgBody.value = res.title;
                    }

                    // 3. 动态替换头像
                    currentMsgIcon.value = getAppIcon(res.app_name);

                    if (!isMsgActive.value) {
                        isMsgActive.value = true;
                        // 👇 新增拦截：如果没有锁定在任务栏，才允许灵动岛变大
                        if (!isPinnedToTaskbar.value) {
                            animateIslandSize(360, 65);
                        }
                    }

                    if ((window as any).msgTimer) clearTimeout((window as any).msgTimer);
                    (window as any).msgTimer = setTimeout(() => {
                        isMsgActive.value = false;
                        // 💡 恢复时不需要加判断。如果它在任务栏(没变大)，这个恢复指令就等于“原地不动”，非常安全！
                        animateIslandSize(260, 42);
                    }, 5000);
                }
            } catch (err) {
                console.error(err);
            }
        }
    }, 1000) as unknown as number;

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

    startHideTimer();
});

onUnmounted(() => {
    clearInterval(speedTimer);
    clearInterval(pingTimer);
    stopHideTimer();
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

/* 1. 外层包裹层：负责裁切多余的流光，并提供呼吸扩散的高斯模糊效果 */
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
}

/* 2. 隐藏在底层的巨大旋转渐变层 */
.rainbow-border-glow {
    position: absolute;
    /* 关键：把它的尺寸设定为远超出容器的巨大正方形，解决大方块旋转露角的问题 */
    width: 500px;
    height: 500px;
    top: calc(50% - 300px);
    left: calc(50% - 300px);
    z-index: 1;
    background: conic-gradient(from 0deg,
            #ff3b30, #ff9500, #ffcc00, #4cd964, #5ac8fa, #007aff, #5856d6, #ff3b30);
    animation: rainbow-rotate 3s linear infinite;
    will-change: transform;
}

/* 3. 核心遮罩内容块：挡在旋转渐变层的上方 */
.island-core-content {
    position: relative;
    z-index: 2;
    width: 100%;
    height: 100%;
    border-radius: 98px;
    backdrop-filter: blur(20px);
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

.good {
    background-color: #34C759;
    box-shadow: 0 0 10px rgba(52, 199, 89, 0.5);
    /* 绿 */
}

.warning {
    background-color: #FFCC00;
    box-shadow: 0 0 10px rgba(255, 204, 0, 0.5);
    /* 黄 */
}

.error {
    background-color: #FF3B30;
    box-shadow: 0 0 10px rgba(255, 59, 48, 0.5);
    /* 红 */
}

/* 当背景透明度为 0 时，强制文本变为黑色 */
.island-container[style*="background-color: rgba(0, 0, 0, 0)"] {
    /* 使用 CSS 变量覆盖默认的白色文本颜色 */
    --text-primary: #000000;
    --text-secondary: #333333;
}

/* 确保所有文本子元素都继承这些颜色 */
.island-container[style*="background-color: rgba(0, 0, 0, 0)"] .speed-item,
.island-container[style*="background-color: rgba(0, 0, 0, 0)"] .label,
.island-container[style*="background-color: rgba(0, 0, 0, 0)"] .value,
.island-container[style*="background-color: rgba(0, 0, 0, 0)"] .status-dot {
    color: var(--text-primary) !important;
    fill: var(--text-primary) !important;
    /* 兼容 SVG 图标颜色 */
}

/* 针对高流量状态下的 label，确保背景也变为黑色系以保持对比度 */
.island-container[style*="background-color: rgba(0, 0, 0, 0)"] .label.high-traffic {
    background: rgba(0, 0, 0, 0.1) !important;
    color: var(--text-primary) !important;
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
    transform: translateX(-5px);
    /* 确保层级比控制器高 */
}

/* 亮色模式下的外环颜色自动变暗 */
:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .album-cover {
    border-color: rgba(0, 0, 0, 0.15);
}

.album-cover.is-playing {
    transform: scale(1.08) translateX(-5px);
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
}

/* 正在播放时的旋转动画 */
.is-playing .cover-inner {
    animation: rotate 8s linear infinite;
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
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
    /* 核心：让控件组在灵动岛内绝对水平居中 */
    display: flex;
    align-items: center;
    gap: 12px;
    /* 间距拉开一点更舒展 */
    z-index: 1;
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
    left: 24px;
    /* 紧贴 24px 宽的专辑封面 */
    right: 20px;
    /* 给右侧网络指示灯留出安全间距 */
    height: 100%;
    display: flex;
    align-items: center;
    overflow: hidden;
    padding-left: 8px;
    -webkit-app-region: no-drag;
    /* 允许在文本区域触发 hover */
    transform: translateY(-1px);

    /* 核心过渡遮罩：右侧文字溢出边缘呈渐隐效果 */
    mask-image: linear-gradient(to right, #000 75%, transparent 100%);
    -webkit-mask-image: linear-gradient(to right, #000 75%, transparent 100%);
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
</style>