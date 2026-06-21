<template>
    <transition @enter="onEnter" @leave="onLeave" :css="false">
        <div v-show="isIslandVisible" class="island-container" @mousedown="handleMouseDown" :style="islandStyle"
            @contextmenu="handleRightClick">
            <div class="music-ctl-box" v-if="isMusicCtlEnabled">
                <div class="album-cover" :class="{ 'is-playing': isPlaying }">
                    <div class="cover-inner"></div>
                </div>

                <div class="music-controls">
                    <button class="ctl-btn" @click="prevTrack">
                        <svg viewBox="0 0 24 24" fill="currentColor">
                            <path d="M6 6h2v12H6zm3.5 6l8.5 6V6z" />
                        </svg>
                    </button>

                    <button class="ctl-btn play-btn" @click="togglePlay">
                        <svg v-if="isPlaying" viewBox="0 0 24 24" fill="currentColor">
                            <path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z" />
                        </svg>

                        <svg v-else viewBox="0 0 24 24" fill="currentColor" style="transform: translateX(1px);">
                            <path d="M8 5v14l11-7z" />
                        </svg>
                    </button>

                    <button class="ctl-btn" @click="nextTrack">
                        <svg viewBox="0 0 24 24" fill="currentColor">
                            <path d="M6 18l8.5-6L6 6v12zM16 6v12h2V6h-2z" />
                        </svg>
                    </button>
                </div>
            </div>
            <div class="speed-box" v-else>
                <div class="speed-item">
                    <span :class="['label', { 'high-traffic': isHighUpload }]">↑</span>
                    <span class="value">{{ uploadSpeed }}</span>
                </div>
                <div class="divider"></div>
                <div class="speed-item">
                    <span :class="['label', { 'high-traffic': isHighDownload }]">↓</span>
                    <span class="value">{{ downloadSpeed }}</span>
                </div>
            </div>
            <div :class="['status-dot', networkStatus]"></div>
        </div>
    </transition>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow, currentMonitor, PhysicalPosition, LogicalPosition, LogicalSize } from '@tauri-apps/api/window';
import { Menu, MenuItem } from '@tauri-apps/api/menu';
import { listen, emit } from '@tauri-apps/api/event';

const isIslandVisible = ref(false);

// 灵动岛自身的透明度变量（默认100）
const islandOpacity = ref(Number(localStorage.getItem('nsd_island_opacity') || '100'));

const islandTheme = ref(localStorage.getItem('nsd_island_theme') || 'black');

const islandStyle = computed(() => {
    const alpha = islandOpacity.value / 100;
    if (islandTheme.value === 'white') {
        return {
            backgroundColor: `rgba(255, 255, 255, ${alpha})`,
            color: '#000000'
        };
    }
    return {
        backgroundColor: `rgba(0, 0, 0, ${alpha})`,
        color: '#ffffff'
    };
});

const uploadSpeed = ref('0 KB/s');
const downloadSpeed = ref('0 KB/s');

// 记录当前是否属于大流量状态
const isHighDownload = ref(false);
const isHighUpload = ref(false);

// 网络状态指示灯：good(绿), warning(黄), error(红)
const networkStatus = ref<'good' | 'warning' | 'error'>('good');

// 音乐控制功能开关
const isMusicCtlEnabled = ref(localStorage.getItem('nsd_music_ctrl') === 'true');
const isPlaying = ref(false);
const togglePlay = () => {
    // 将状态取反 (true 变 false，false 变 true)
    isPlaying.value = !isPlaying.value;

    // （可选）之后你可以把调用 Tauri Rust 后端控制系统媒体播放的代码写在这里
};

const prevTrack = () => {
    console.log('上一首');
    // 调用上一首逻辑
};

const nextTrack = () => {
    console.log('下一首');
    // 调用下一首逻辑
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
            const scaleFactor = await appWindow.scaleFactor();

            // 1. 在这里按像素死死固定住灵动岛的尺寸
            // 替换为你想要的基准像素尺寸
            const ISLAND_WIDTH = 260;
            const ISLAND_HEIGHT = 42;
            await appWindow.setSize(new LogicalSize(ISLAND_WIDTH, ISLAND_HEIGHT));

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

const handleMouseDown = async (event: MouseEvent) => {
    // 如果点击的是按钮或按钮内部的 SVG 图标，直接返回，不触发拖拽
    if ((event.target as HTMLElement).closest('.ctl-btn')) return;

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

    // 1. 创建“重置位置”菜单项
    const resetPositionItem = await MenuItem.new({
        text: '重置位置',
        id: 'reset_position',
        action: () => {
            // 点击后直接调用你原本写好的位置调整逻辑
            adjustWindowPosition().catch(console.error);
        }
    });

    // 2. 创建“关闭”菜单项
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
    await menu.append(resetPositionItem);
    await menu.append(closeItem); // 两个菜单项会上下排列

    // 4. 弹出菜单
    try {
        await menu.popup(position);
    } catch (error) {
        console.error('菜单弹出失败:', error);
    }
};

onMounted(async () => {
    document.addEventListener('contextmenu', (e) => {
        e.preventDefault();
    }, { capture: true }); // 使用捕获阶段，确保先于 Tauri 底层拦截

    // 监听来自控制台的透明度同步指令
    await listen<{ opacity: number }>('control-island-opacity', (event) => {
        islandOpacity.value = event.payload.opacity;
    });

    await listen<{ theme: string }>('control-island-theme', (event) => {
        islandTheme.value = event.payload.theme;
    });

    await listen<{ enabled: boolean }>('control-music-ctl', (event) => {
        isMusicCtlEnabled.value = event.payload.enabled;
    });

    await adjustWindowPosition();

    // 先显示透明的 Tauri 窗口，再触发 Vue 的灵动岛入场弹簧动画
    await getCurrentWindow().show();
    isIslandVisible.value = true;

    fetchSpeedStats();
    checkNetworkLatency();

    // 流量1秒刷一次保持数字灵敏度
    speedTimer = setInterval(fetchSpeedStats, 1000) as unknown as number;

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
});

onUnmounted(() => {
    clearInterval(speedTimer);
    clearInterval(pingTimer);
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

.island-container {
    position: absolute;
    top: 0;
    left: 0;
    width: 100% !important;
    height: 100% !important;
    background: rgba(0, 0, 0, 1);
    backdrop-filter: blur(20px) !important;
    border-radius: 100px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 14px;
    color: white;
    user-select: none;
    box-shadow: none !important;
    border: none !important;
    -webkit-user-select: none;
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
    font-size: 11px;
    font-weight: 500;
    min-width: 52px;
    letter-spacing: -0.2px;
}

.divider {
    width: 1px;
    height: 12px;
    background: rgba(255, 255, 255, 0.12);
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

/* --- 音乐控制器样式 --- */
.music-ctl-box {
    position: relative;
    /* 改为相对定位，作为内部绝对居中的参考系 */
    display: flex;
    align-items: center;
    width: 100%;
    height: 100%;
    /* 确保撑满高度 */
}

.album-cover {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    /* 变成纯圆球形 */
    box-sizing: unset !important;
    border: 2px solid rgba(255, 255, 255, 0.2) !important;
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

.cover-inner {
    width: 100%;
    height: 100%;
    border-radius: 50%;
    background-color: rgba(255, 255, 255, 0.2);
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
</style>