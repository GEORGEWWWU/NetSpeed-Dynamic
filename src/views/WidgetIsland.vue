<template>
    <div class="island-container" data-tauri-drag-region>
        <div class="speed-box" data-tauri-drag-region>
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
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow, currentMonitor, PhysicalPosition } from '@tauri-apps/api/window';

const uploadSpeed = ref('0 KB/s');
const downloadSpeed = ref('0 KB/s');

// 记录当前是否属于大流量状态
const isHighDownload = ref(false);
const isHighUpload = ref(false);

// 网络状态指示灯：good(绿), warning(黄), error(红)
const networkStatus = ref<'good' | 'warning' | 'error'>('good');

let lastRx = 0;
let lastTx = 0;
let speedTimer: number;
let pingTimer: number;

// === 新增：防抖控制变量 ===
let lowTrafficStartTime = Date.now(); // 记录进入“无流量/低流量”的起始时间
const RED_DELAY_MS = 5000;          // 流量消失后，等待 5 秒再允许变红

const formatSpeed = (bytes: number) => {
    if (bytes < 1024) return bytes + ' B/s';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB/s';
    return (bytes / (1024 * 1024)).toFixed(1) + ' MB/s';
};

// 任务一：计算流量数字，并实时更新大流量状态
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

            // === 核心逻辑：维护低流量持续时间 ===
            if (currentDownloadHigh || currentUploadHigh) {
                // 如果目前依然是大流量，重置计时器，强制让“低流量开始时间”保持在当下
                lowTrafficStartTime = Date.now();
            }
        }
        lastRx = currentRx;
        lastTx = currentTx;
    } catch (error) {
        console.error('流量获取失败:', error);
    }
};

// 任务二：通过真实延迟控制状态灯（加入大流量避让判断）
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
        // === 核心逻辑：当 Rust 抛出超时异常时 ===

        // 1. 如果当前正处于大流量状态，绝不变红，降级显示为黄灯
        if (isHighDownload.value || isHighUpload.value) {
            networkStatus.value = 'warning';
            return;
        }

        // 2. 如果流量刚刚消失，判断距离大流量结束是否超过了设定的缓冲时间（例如 5 秒）
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

const adjustWindowPosition = async () => {
    try {
        const appWindow = getCurrentWindow();
        await new Promise((resolve) => setTimeout(resolve, 150));
        const monitor = await currentMonitor();

        if (monitor) {
            const scaleFactor = await appWindow.scaleFactor();
            const monitorWidthPhysical = monitor.size.width;
            const monitorLeftPhysical = monitor.position.x;
            const monitorTopPhysical = monitor.position.y;

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

onMounted(async () => {
    await adjustWindowPosition();

    fetchSpeedStats();
    checkNetworkLatency();

    // 流量 1 秒刷一次保持数字灵敏度
    speedTimer = setInterval(fetchSpeedStats, 1000) as unknown as number;

    // 调大 Ping 间隔：从 2.5 秒调大到 5.5 秒，避免自身高频检测造成的网络竞争
    pingTimer = setInterval(checkNetworkLatency, 5500) as unknown as number;
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
    background: rgba(0, 0, 0, 0.95);
    backdrop-filter: blur(20px);
    border-radius: 18px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 14px;
    color: white;
    user-select: none;
    box-shadow: none !important;
    border: none !important;
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
    color: rgba(255, 255, 255, 0.4);
    font-weight: bold;
    /* 预留过渡动画，让视觉变化更平滑 */
    padding: 2px 4px;
    border-radius: 4px;
    transition: all 0.3s ease;
}

/* 新增：高流量时的 label 样式 */
.label.high-traffic {
    color: rgba(255, 255, 255, 0.9);
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
</style>