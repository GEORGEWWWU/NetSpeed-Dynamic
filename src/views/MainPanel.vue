<template>
    <div class="panel-container">
        <header class="panel-header">
            <div class="brand">
                <img src="../assets/logo.png" class="logo-icon">
                <div>
                    <h1>NetSpeed Dynamic</h1>
                    <p class="subtitle">NSD 桌面动态组件 v1.0.0</p>
                </div>
            </div>

            <div class="header-controls">
                <span class="status-badge" :class="{ 'is-active': isWidgetVisible }">
                    {{ isWidgetVisible ? '已开启' : '已关闭' }}
                </span>
                <label class="switch header-switch">
                    <input type="checkbox" :checked="isWidgetVisible" @change="toggleWidget">
                    <span class="slider"></span>
                </label>
            </div>
        </header>

        <hr class="divider" />

        <div class="main-content">
            <div class="card status-card">
                <h3>当前实时状态</h3>
                <div class="speed-monitor">
                    <div class="speed-item">
                        <span class="arrow up">
                            <svg viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <path
                                    d="M16 4C16.8 4 17.5 4.3 18.1 4.9L28.1 14.9C29.3 16.1 29.3 18 28.1 19.1C26.9 20.3 25 20.3 23.9 19.1L18 13.2V26C18 27.7 16.7 29 15 29C13.3 29 12 27.7 12 26V13.2L6.1 19.1C4.9 20.3 3 20.3 1.9 19.1C0.7 18 0.7 16.1 1.9 14.9L11.9 4.9C12.5 4.3 13.2 4 14 4H16Z"
                                    fill="currentColor" />
                            </svg>
                        </span>
                        <div class="speed-info">
                            <span class="label">上传速度</span>
                            <span class="value">{{ uploadSpeed }}</span>
                        </div>
                    </div>
                    <div class="speed-item">
                        <span class="arrow down">
                            <svg viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <path
                                    d="M16 28C15.2 28 14.5 27.7 13.9 27.1L3.9 17.1C2.7 15.9 2.7 14 3.9 12.9C5.1 11.7 7 11.7 8.1 12.9L14 18.8V6C14 4.3 15.3 3 17 3C18.7 3 20 4.3 20 6V18.8L25.9 12.9C27.1 11.7 29 11.7 30.1 12.9C31.3 14 31.3 15.9 30.1 17.1L20.1 27.1C19.5 27.7 18.8 28 18 28H16Z"
                                    fill="currentColor" />
                            </svg>
                        </span>
                        <div class="speed-info">
                            <span class="label">下载速度</span>
                            <span class="value">{{ downloadSpeed }}</span>
                        </div>
                    </div>
                </div>
                <div ref="chartRef" class="mini-chart"></div>
            </div>

            <div class="card settings-card">
                <h3>常规设置</h3>

                <div class="setting-item flex-row-item">
                    <div class="item-meta">
                        <span class="item-title">显示模式</span>
                        <span class="item-desc">切换亮色、暗色或跟随系统</span>
                    </div>
                    <select v-model="themeMode" class="theme-select" @change="handleThemeChange">
                        <option value="light">浅色模式</option>
                        <option value="dark">深色模式</option>
                        <option value="system">跟随系统</option>
                    </select>
                </div>

                <div class="setting-item is-disabled">
                    <div class="item-meta">
                        <span class="item-title">开机自动启动 <span class="tag-dev">未实现</span></span>
                        <span class="item-desc">跟随系统启动 NSD</span>
                    </div>
                    <label class="switch">
                        <input type="checkbox" v-model="autoStart" disabled>
                        <span class="slider"></span>
                    </label>
                </div>

                <div class="setting-item slider-item">
                    <div class="item-meta">
                        <span class="item-title">悬浮窗不透明度</span>
                        <span class="item-desc">调节灵动岛的外观透明度 ({{ opacity }}%)</span>
                    </div>
                    <input type="range" min="0" max="100" v-model="opacity" class="range-input" />
                </div>
            </div>
        </div>

        <footer class="panel-footer">
            <span>&copy; 2026 Ryen. All rights reserved.</span>
            <span class="action-link" @click="checkUpdate">检查更新</span>
        </footer>

        <Transition name="fade">
            <div v-if="dialog.visible" class="modal-overlay" @click.self="closeDialog">
                <div class="modal-card">
                    <div class="modal-header">
                        <h4>{{ dialog.title }}</h4>
                    </div>
                    <div class="modal-body">
                        <p>{{ dialog.message }}</p>
                    </div>
                    <div class="modal-footer">
                        <button v-if="dialog.isConfirm" class="btn btn-secondary" @click="closeDialog">取消</button>
                        <button class="btn btn-primary" @click="handleDialogConfirm">确定</button>
                    </div>
                </div>
            </div>
        </Transition>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { emit, listen } from '@tauri-apps/api/event';
import { getVersion } from '@tauri-apps/api/app';
import * as echarts from 'echarts';

const isWidgetVisible = ref(false);
const autoStart = ref(false);
const opacity = ref(Number(localStorage.getItem('nsd_island_opacity') || '100'));

// 主题状态：'light' | 'dark' | 'system'
// 严格校验缓存值，如果不合法，强制回退到 'light'
const savedTheme = localStorage.getItem('nsd_theme_mode') || 'light'; // 如果为 null 直接给 'light' 字符串
const themeMode = ref(['light', 'dark', 'system'].includes(savedTheme) ? savedTheme : 'light');

const uploadSpeed = ref('0 B/s');
const downloadSpeed = ref('0 B/s');

const dialog = ref({
    visible: false,
    title: 'NetSpeed Dynamic',
    message: '',
    isConfirm: false,
    callback: null as (() => void) | null
});

const showDialog = (title: string, message: string, isConfirm = false, onConfirm: (() => void) | null = null) => {
    dialog.value = { visible: true, title, message, isConfirm, callback: onConfirm };
};

const closeDialog = () => {
    dialog.value.visible = false;
};

const handleDialogConfirm = () => {
    if (dialog.value.callback) dialog.value.callback();
    closeDialog();
};

const parseVersion = (v: string) => {
    return v.replace(/^v/i, '').split('.').map(Number);
};

let lastRx = 0;
let lastTx = 0;
let speedTimer: number;
let systemThemeMedia: MediaQueryList;

const chartRef = ref<HTMLElement | null>(null);
let chartInstance: any = null;
const chartDataQueue: number[] = Array(15).fill(0);

const formatSpeed = (bytes: number) => {
    if (bytes < 1024) return bytes + ' B/s';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB/s';
    return (bytes / (1024 * 1024)).toFixed(1) + ' MB/s';
};

// 动态获取图表主色调线
const getChartColors = () => {
    const isDark = document.documentElement.classList.contains('dark-theme');
    return {
        line: isDark ? '#60a5fa' : '#3b82f6',
        areaStart: isDark ? 'rgba(96, 165, 250, 0.4)' : 'rgba(59, 130, 246, 0.4)',
        areaEnd: isDark ? 'rgba(96, 165, 250, 0.0)' : 'rgba(59, 130, 246, 0.0)'
    };
};

const initChart = () => {
    if (!chartRef.value || !echarts) return;
    chartInstance = echarts.init(chartRef.value);
    updateChartOption();
};

const updateChartOption = () => {
    if (!chartInstance) return;
    const colors = getChartColors();
    chartInstance.setOption({
        grid: { top: 5, bottom: 5, left: 0, right: 0 },
        xAxis: { type: 'category', boundaryGap: false, show: false },
        yAxis: { type: 'value', show: false, min: 0 },
        series: [
            {
                data: chartDataQueue,
                type: 'line',
                smooth: true,
                symbol: 'none',
                lineStyle: { color: colors.line, width: 2 },
                areaStyle: {
                    color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
                        { offset: 0, color: colors.areaStart },
                        { offset: 1, color: colors.areaEnd }
                    ]),
                },
            },
        ],
    });
};

const fetchSpeedStats = async () => {
    try {
        const [currentRx, currentTx] = await invoke<[number, number]>('get_network_stats');
        if (lastRx !== 0) {
            const rxDiff = currentRx - lastRx;
            const txDiff = currentTx - lastTx;
            downloadSpeed.value = formatSpeed(rxDiff);
            uploadSpeed.value = formatSpeed(txDiff);
            const speedMB = rxDiff / (1024 * 1024);

            chartDataQueue.push(Number(speedMB.toFixed(2)));
            if (chartDataQueue.length > 15) chartDataQueue.shift();

            chartInstance?.setOption({ series: [{ data: chartDataQueue }] });
        }
        lastRx = currentRx;
        lastTx = currentTx;
    } catch (error) {
        console.error('控制台流量获取失败:', error);
    }
};

const checkUpdate = async () => {
    try {
        const localVersionStr = await getVersion();
        const response = await fetch('https://api.github.com/repos/GEORGEWWWU/NetSpeed-Dynamic/releases/latest', {
            method: 'GET',
            headers: {
                'Accept': 'application/vnd.github.v3+json',
                'User-Agent': 'Tauri-App-NetSpeed-Dynamic'
            }
        });

        if (response.status === 404) {
            showDialog('检查更新', '未找到可用发行版');
            return;
        }

        if (!response.ok) {
            showDialog('检查更新', '检查更新失败，请稍后再试');
            return;
        }

        const data = await response.json();
        const remoteVersionStr = data.tag_name;
        const local = parseVersion(localVersionStr);
        const remote = parseVersion(remoteVersionStr);

        let hasNewVersion = false;
        for (let i = 0; i < 3; i++) {
            const rNum = remote[i] || 0;
            const lNum = local[i] || 0;
            if (rNum > lNum) {
                hasNewVersion = true;
                break;
            } else if (rNum < lNum) {
                break;
            }
        }

        if (hasNewVersion) {
            showDialog(
                '发现新版本',
                `发现新版本 ${remoteVersionStr}！当前版本为 v${localVersionStr}。是否前往 GitHub 下载更新？`,
                true,
                () => { window.open(data.html_url, '_blank'); }
            );
        } else {
            showDialog('提示', '当前已是最新版本！');
        }
    } catch (error) {
        console.error('检查更新时出错:', error);
        showDialog('网络错误', '请求失败，请检查您的网络连接');
    }
};

// 主题切换核心逻辑
const applyTheme = () => {
    const root = document.documentElement;
    if (themeMode.value === 'dark') {
        root.classList.add('dark-theme');
    } else if (themeMode.value === 'light') {
        root.classList.remove('dark-theme');
    } else if (themeMode.value === 'system') {
        const media = window.matchMedia('(prefers-color-scheme: dark)');
        if (media.matches) {
            root.classList.add('dark-theme');
        } else {
            root.classList.remove('dark-theme');
        }
    }
    updateChartOption(); // 重新给 Echarts 染色
};

const handleThemeChange = () => {
    localStorage.setItem('nsd_theme_mode', themeMode.value);
    applyTheme();
};

const handleSystemThemeUpdate = () => {
    if (themeMode.value === 'system') {
        applyTheme();
    }
};

watch(opacity, async (newVal) => {
    localStorage.setItem('nsd_island_opacity', newVal.toString());
    await emit('control-island-opacity', { opacity: newVal });
});

onMounted(async () => {
    window.addEventListener('contextmenu', (e) => {
        e.preventDefault();
    }, { capture: true });

    // 初始化应用主题
    applyTheme();
    systemThemeMedia = window.matchMedia('(prefers-color-scheme: dark)');
    systemThemeMedia.addEventListener('change', handleSystemThemeUpdate);

    initChart();
    fetchSpeedStats();
    speedTimer = setInterval(fetchSpeedStats, 1000) as unknown as number;
    window.addEventListener('resize', () => chartInstance?.resize());

    await listen<{ visible: boolean }>('island-status-sync', (event) => {
        isWidgetVisible.value = event.payload.visible;
    });

    for (let i = 0; i < 6; i++) {
        try {
            const visible = await invoke<boolean>('is_widget_visible');
            if (visible) {
                isWidgetVisible.value = true;
                return;
            }
        } catch { /* 忽略 */ }
        await new Promise(r => setTimeout(r, 200));
    }
    isWidgetVisible.value = false;
});

onUnmounted(() => {
    clearInterval(speedTimer);
    chartInstance?.dispose();
    systemThemeMedia?.removeEventListener('change', handleSystemThemeUpdate);
});

const toggleWidget = async () => {
    const nextState = !isWidgetVisible.value;
    await emit('control-island-visibility', { show: nextState });
    isWidgetVisible.value = nextState;
};
</script>

<style scoped>
/* ==========================================================================
   1. 提取出的颜色变量层（默认全盘照抄原样式颜色，绝不动原来的色值）
   ========================================================================== */
:global(:root) {
    --bg-body: #f8fafc;
    --text-body: #1e293b;
    --h1-color: #0f172a;
    --subtitle-color: #64748b;
    --control-bg: #ffffff;
    --control-border: #e2e8f0;
    --status-badge-inactive: #94a3b8;
    --status-badge-active: #2b2b2b;
    --divider-border: #e2e8f0;
    --card-bg: #ffffff;
    --card-border: #e2e8f0;
    --card-shadow: rgba(0, 0, 0, 0.03);
    --card-shadow-hover: rgba(0, 0, 0, 0.06);
    --card-h3-color: #334155;
    --arrow-up-bg: #eff6ff;
    --arrow-up-color: #3b82f6;
    --arrow-down-bg: #ecfdf5;
    --arrow-down-color: #10b981;
    --speed-label: #64748b;
    --speed-value: #0f172a;
    --chart-border: #f1f5f9;
    --item-title-color: #1e293b;
    --tag-dev-bg: #f1f5f9;
    --tag-dev-color: #64748b;
    --item-desc-color: #64748b;
    --slider-bg: #cbd5e1;
    --slider-checked-bg: #2b2b2b;
    --slider-disabled-bg: #e2e8f0;
    --range-bg: #e2e8f0;
    --range-thumb-bg: #ffffff;
    --range-thumb-border: #2b2b2b;
    --range-thumb-shadow: rgba(0, 0, 0, 0.3);
    --footer-text: #2b2b2b89;
    --overlay-bg: rgba(15, 23, 42, 0.3);
    --modal-bg: #ffffff;
    --modal-border: #e2e8f0;
    --modal-h4: #0f172a;
    --modal-p: #64748b;
    --btn-sec-bg: #f1f5f9;
    --btn-sec-color: #64748b;
    --btn-sec-border: #e2e8f0;
    --btn-sec-hover-bg: #e2e8f0;
    --btn-sec-hover-color: #334155;
    --btn-pri-bg: #2b2b2b;
    --btn-pri-color: #ffffff;
    --btn-pri-border: #2b2b2b;
    --btn-pri-hover-bg: #1a1a1a;
    --btn-pri-shadow-hover: rgba(0, 0, 0, 0.15);

    /* 新增下拉选择器在亮色下的专属变量 */
    --select-bg: #ffffff;
    --select-border: #e2e8f0;
    --select-text: #1e293b;
}

/* ==========================================================================
   2. 暗色模式变量覆盖（只有在这个类名下，才会用黑灰色覆盖上述变量）
   ========================================================================== */
:global(.dark-theme) {
    --bg-body: #1e2021;
    --text-body: #cbd5e1;
    --h1-color: #f8fafc;
    --subtitle-color: #94a3b8;
    --control-bg: #292b2e;
    --control-border: #383c41;
    --status-badge-inactive: #64748b;
    --status-badge-active: #f8fafc;
    --divider-border: #334155;
    --card-bg: #292b2e;
    --card-border: #383c41;
    --card-shadow: rgba(0, 0, 0, 0.2);
    --card-shadow-hover: rgba(0, 0, 0, 0.3);
    --card-h3-color: #e2e8f0;
    --arrow-up-bg: rgba(59, 130, 246, 0.15);
    --arrow-up-color: #60a5fa;
    --arrow-down-bg: rgba(16, 185, 129, 0.15);
    --arrow-down-color: #34d399;
    --speed-label: #94a3b8;
    --speed-value: #f8fafc;
    --chart-border: #334155;
    --item-title-color: #f8fafc;
    --tag-dev-bg: #334155;
    --tag-dev-color: #94a3b8;
    --item-desc-color: #94a3b8;
    --slider-bg: #475569;
    --slider-checked-bg: #47576b;
    --slider-disabled-bg: #334155;
    --range-bg: #334155;
    --range-thumb-bg: #1e293b;
    --range-thumb-border: #60a5fa;
    --range-thumb-shadow: rgba(0, 0, 0, 0.5);
    --footer-text: #94a3b8aa;
    --overlay-bg: rgba(0, 0, 0, 0.6);
    --modal-bg: #292b2e;
    --modal-border: #383c41;
    --modal-h4: #f8fafc;
    --modal-p: #94a3b8;
    --btn-sec-bg: #334155;
    --btn-sec-color: #cbd5e1;
    --btn-sec-border: #475569;
    --btn-sec-hover-bg: #475569;
    --btn-sec-hover-color: #f8fafc;
    --btn-pri-bg: #1a1a1a;
    --btn-pri-color: #ffffff;
    --btn-pri-border: #2b2b2b;
    --btn-pri-hover-bg: #161616;
    --btn-pri-shadow-hover: rgba(0, 0, 0, 0.15);

    /* 下拉选择器在暗色下的变量 */
    --select-bg: #292b2e;
    --select-border: #383c41;
    --select-text: #f8fafc;
}

/* ==========================================================================
   3. 原有布局及节点样式（全盘保留，不改任何一行 layout/padding/size）
   ========================================================================== */
/* 优化后的组件全局根样式 */
:global(html) {
    /* 变量由 html 层级掌控 */
    color: var(--text-body);
    transition: background-color 0.3s ease, color 0.3s ease;
}

:global(body) {
    background-color: transparent !important;
    /* 强制透明，让 html 说了算 */
    color: inherit;
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', 'Segoe UI', Roboto, sans-serif;
    user-select: none;
    -webkit-font-smoothing: antialiased;
}

.panel-container {
    background-color: var(--bg-body);
    /* 加上这一行，让它跟随主题变量 */
    padding: 28px 32px;
    max-width: 800px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    min-height: calc(100vh - 56px);
}

.panel-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
}

.brand {
    display: flex;
    align-items: center;
    gap: 16px;
}

.logo-icon {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.25);
}

.brand h1 {
    font-size: 20px;
    margin: 0;
    font-weight: 700;
    letter-spacing: 0.2px;
    color: var(--h1-color);
}

.subtitle {
    font-size: 13px;
    color: var(--subtitle-color);
    margin: 4px 0 0 0;
}

.header-controls {
    display: flex;
    align-items: center;
    gap: 16px;
    background: var(--control-bg);
    padding: 8px 12px 8px 16px;
    border-radius: 24px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
    border: 1px solid var(--control-border);
}

.status-badge {
    font-size: 13px;
    font-weight: 600;
    color: var(--status-badge-inactive);
    transition: all 0.3s;
}

.status-badge.is-active {
    color: var(--status-badge-active);
}

.divider {
    border: none;
    border-top: 1px solid var(--divider-border);
    margin-bottom: 24px;
}

.main-content {
    display: grid;
    grid-template-columns: 1fr 1.3fr;
    gap: 24px;
    flex-grow: 1;
}

.card {
    background: var(--card-bg);
    border: 1px solid var(--card-border);
    border-radius: 20px;
    padding: 24px;
    display: flex;
    flex-direction: column;
    box-shadow: 0 4px 20px -2px var(--card-shadow);
    transition: transform 0.2s, box-shadow 0.2s;
}

.card:hover {
    box-shadow: 0 8px 24px -4px var(--card-shadow-hover);
}

.card h3 {
    font-size: 15px;
    color: var(--card-h3-color);
    margin: 0 0 20px 0;
    font-weight: 600;
}

.speed-monitor {
    display: flex;
    flex-direction: column;
    gap: 20px;
    margin-bottom: 24px;
}

.speed-item {
    display: flex;
    align-items: center;
    gap: 16px;
}

.arrow {
    width: 36px;
    height: 36px;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 800;
    font-size: 16px;
}

.arrow svg {
    width: 20px;
    height: 20px;
}

.arrow.up {
    background: var(--arrow-up-bg);
    color: var(--arrow-up-color);
}

.arrow.down {
    background: var(--arrow-down-bg);
    color: var(--arrow-down-color);
}

.speed-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
}

.speed-info .label {
    font-size: 12px;
    color: var(--speed-label);
    font-weight: 500;
}

.speed-info .value {
    font-size: 18px;
    font-weight: 700;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    color: var(--speed-value);
    letter-spacing: -0.5px;
}

.mini-chart {
    width: 100%;
    height: 80px;
    margin-top: auto;
    padding-top: 16px;
    border-top: 1px solid var(--chart-border);
}

.setting-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 0;
    border-bottom: 1px solid var(--chart-border);
}

.setting-item:last-child {
    border-bottom: none;
    padding-bottom: 0;
}

.slider-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 16px;
}

/* 兼容新增的主题单选行布局 */
.flex-row-item {
    flex-direction: row;
    align-items: center;
}

.item-meta {
    display: flex;
    flex-direction: column;
    gap: 4px;
}

.item-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--item-title-color);
    display: flex;
    align-items: center;
    gap: 8px;
}

.tag-dev {
    font-size: 10px;
    background: var(--tag-dev-bg);
    color: var(--tag-dev-color);
    padding: 2px 6px;
    border-radius: 4px;
    font-weight: normal;
}

.item-desc {
    font-size: 13px;
    color: var(--item-desc-color);
}

.is-disabled {
    opacity: 0.5;
    pointer-events: none;
}

.switch {
    position: relative;
    display: inline-block;
    width: 48px;
    height: 28px;
}

.switch input {
    opacity: 0;
    width: 0;
    height: 0;
}

.slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: var(--slider-bg);
    transition: 0.4s cubic-bezier(0.4, 0.0, 0.2, 1);
    border-radius: 28px;
}

.slider:before {
    position: absolute;
    content: "";
    height: 22px;
    width: 22px;
    left: 3px;
    bottom: 3px;
    background-color: white;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
    transition: 0.4s cubic-bezier(0.4, 0.0, 0.2, 1);
    border-radius: 50%;
}

input:checked+.slider {
    background-color: var(--slider-checked-bg);
}

input:checked+.slider:before {
    transform: translateX(20px);
}

input:disabled+.slider {
    background-color: var(--slider-disabled-bg);
    cursor: not-allowed;
}

.range-input {
    width: 100%;
    -webkit-appearance: none;
    appearance: none;
    background: var(--range-bg);
    height: 6px;
    border-radius: 3px;
    outline: none;
}

.range-input::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: var(--range-thumb-bg);
    border: 2px solid var(--range-thumb-border);
    cursor: pointer;
    box-shadow: 0 2px 6px var(--range-thumb-shadow);
    transition: transform 0.1s;
}

.range-input::-webkit-slider-thumb:hover {
    transform: scale(1.1);
}

.panel-footer {
    margin-top: 32px;
    display: flex;
    justify-content: space-between;
    font-size: 12px;
    color: var(--footer-text);
    font-weight: 500;
}

.action-link {
    color: var(--footer-text);
    cursor: pointer;
    transition: color 0.2s;
}

.action-link:hover {
    color: var(--footer-text);
    text-decoration: underline;
}

.modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: var(--overlay-bg);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 9999;
}

.modal-card {
    background: var(--modal-bg);
    border: 1px solid var(--modal-border);
    border-radius: 20px;
    width: 360px;
    padding: 24px;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
}

.modal-header h4 {
    margin: 0 0 12px 0;
    font-size: 16px;
    font-weight: 700;
    color: var(--modal-h4);
}

.modal-body p {
    margin: 0 0 24px 0;
    font-size: 14px;
    color: var(--modal-p);
    line-height: 1.5;
}

.modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
}

.btn {
    padding: 8px 18px;
    font-size: 13px;
    font-weight: 600;
    border-radius: 10px;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    outline: none;
}

.btn-secondary {
    background: var(--btn-sec-bg);
    color: var(--btn-sec-color);
    border: 1px solid var(--btn-sec-border);
}

.btn-secondary:hover {
    background: var(--btn-sec-hover-bg);
    color: var(--btn-sec-hover-color);
}

.btn-primary {
    background: var(--btn-pri-bg);
    color: var(--btn-pri-color);
    border: 1px solid var(--btn-pri-border);
}

.btn-primary:hover {
    background: var(--btn-pri-hover-bg);
    box-shadow: 0 4px 12px var(--btn-pri-shadow-hover);
}

.fade-enter-active,
.fade-leave-active {
    transition: opacity 0.25s ease, transform 0.25s ease;
}

.fade-enter-from,
.fade-leave-to {
    opacity: 0;
}

.fade-enter-from .modal-card {
    transform: scale(0.95);
}

.fade-leave-to .modal-card {
    transform: scale(0.95);
}

/* ==========================================================================
   4. 新增的下拉选择框 UI 样式（保持和整体视觉风格契合）
   ========================================================================== */
.theme-select {
    padding: 6px 12px;
    font-size: 13px;
    font-weight: 600;
    border-radius: 8px;
    background-color: var(--select-bg);
    border: 1px solid var(--select-border);
    color: var(--select-text);
    outline: none;
    cursor: pointer;
    transition: all 0.2s ease;
}

.theme-select:hover {
    border-color: var(--slider-checked-bg);
}
</style>