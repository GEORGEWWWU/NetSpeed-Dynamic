<template>
    <div class="panel-container">
        <header class="panel-header">
            <div class="brand">
                <img src="../assets/logo.png" class="logo-icon">
                <div>
                    <h1>NetSpeed Dynamic Pro</h1>
                    <p class="subtitle">NSD 桌面动态组件 v{{ appVersion }}</p>
                </div>
            </div>

            <div class="header-controls">
                <button class="dynamicset-btn" :class="{ 'is-active': isDynamicSet }" @click="toggleDynamicSet">
                    灵动岛设置
                </button>
                <span class="control-separator"></span>

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

        <div class="main-content" :class="{ 'dynamicset-layout': isDynamicSet }">
            <template v-if="!isDynamicSet">
                <div class="card status-card">
                    <div class="card-header-row">
                        <h3>当前实时状态</h3>
                        <button class="stats-toggle-btn" @click="toggleRightPanel">
                            {{ rightPanel === 'settings' ? '数据统计' : '退出' }}
                        </button>
                    </div>
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

                <div class="card settings-card" v-if="rightPanel === 'settings'">
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

                    <div class="setting-item">
                        <div class="item-meta">
                            <span class="item-title">开机自动启动</span>
                            <span class="item-desc">跟随系统启动 NSD</span>
                        </div>
                        <label class="switch">
                            <input type="checkbox" v-model="autoStart" @change="toggleAutoStart">
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

                <template v-else>
                    <div class="card stats-card">
                        <div class="card-header-row">
                            <h3>数据统计</h3>
                            <select v-model="statChartType" class="theme-select" @change="updateStatsChart">
                                <option value="bar">柱状图</option>
                                <option value="line">折线图</option>
                            </select>
                        </div>

                        <div class="stats-overview">
                            <div class="stat-box">
                                <span class="stat-label">总上传</span>
                                <span class="stat-val">{{ formatBytesValue(totalUpload) }}</span>
                                <span class="stat-unit">{{ formatBytesUnit(totalUpload) }}</span>
                            </div>
                            <div class="stat-box">
                                <span class="stat-label">总下载</span>
                                <span class="stat-val">{{ formatBytesValue(totalDownload) }}</span>
                                <span class="stat-unit">{{ formatBytesUnit(totalDownload) }}</span>
                            </div>
                            <div class="stat-box">
                                <span class="stat-label">本月流量</span>
                                <span class="stat-val">{{ formatBytesValue(monthTraffic) }}</span>
                                <span class="stat-unit">{{ formatBytesUnit(monthTraffic) }}</span>
                            </div>
                        </div>

                        <div ref="statsChartRef" class="stats-chart-container"></div>
                    </div>
                </template>
            </template>

            <template v-else>
                <!-- 移除内部标题头，直接使用 grid 布局铺满 -->
                <div class="dynamicset-grid">
                    <div class="set-item">
                        <div class="set-item-meta">
                            <span class="set-item-title">灵动岛颜色</span>
                            <span class="set-item-desc">切换灵动岛的默认背景色调</span>
                        </div>
                        <div class="capsule-switch">
                            <div class="capsule-btn" :class="{ 'is-active': islandTheme === 'black' }"
                                @click="islandTheme = 'black'">暗色</div>
                            <div class="capsule-btn" :class="{ 'is-active': islandTheme === 'white' }"
                                @click="islandTheme = 'white'">亮色</div>
                        </div>
                    </div>

                    <div class="set-item">
                        <div class="set-item-meta">
                            <span class="set-item-title">
                                音乐控制器
                                <span class="tooltip-wrapper" data-tooltip="若要使用此功能请将网易云音乐最小化或保持窗口，请不要关闭窗口！">
                                    <p class="set-item-tips-tag">🙋‍♂️</p>
                                </span>
                            </span>
                            <span class="set-item-desc">支持网易云音乐控制及歌曲信息显示</span>
                        </div>
                        <label class="switch">
                            <input type="checkbox" v-model="enableMusicCtrl">
                            <span class="slider"></span>
                        </label>
                    </div>

                    <div class="set-item">
                        <div class="set-item-meta">
                            <span class="set-item-title">消息通知接收 <p class="set-item-pro-tag">PRO</p></span>
                            <span class="set-item-desc">启用系统控制中心消息弹窗提醒</span>
                        </div>
                        <label class="switch">
                            <input type="checkbox" v-model="enableMsgNotify" @change="toggleMsgNotify">
                            <span class="slider"></span>
                        </label>
                    </div>

                    <div class="set-item">
                        <div class="set-item-meta">
                            <span class="set-item-title">系统硬件监控 <p class="set-item-pro-tag">PRO</p>
                            </span>
                            <span class="set-item-desc">显示 CPU / GPU / 内存实时占用率</span>
                        </div>
                        <label class="switch">
                            <input type="checkbox" v-model="enableHardwareMon" @change="toggleHardwareMon">
                            <span class="slider"></span>
                        </label>
                    </div>

                    <div class="set-item">
                        <div class="set-item-meta">
                            <span class="set-item-title">
                                置于任务栏
                                <span class="tooltip-wrapper" data-tooltip="若要在全屏游戏中使用灵动岛建议关闭此项">
                                    <p class="set-item-tips-tag">🙋</p>
                                </span>
                            </span>
                            <span class="set-item-desc">将灵动岛锁定至任务栏左下角</span>
                        </div>
                        <label class="switch">
                            <input type="checkbox" v-model="pinToTaskbar" @change="togglePinTaskbar">
                            <span class="slider"></span>
                        </label>
                    </div>

                    <div class="set-item disabled-set-item">
                        <div class="set-item-meta">
                            <span class="set-item-title">消息模式 <p class="set-item-pro-tag">即将推出</p></span>
                            <span class="set-item-desc">默认隐藏灵动岛，收到消息后弹出</span>
                        </div>
                        <label class="switch">
                            <input type="checkbox" v-model="disableBtn" disabled>
                            <span class="slider"></span>
                        </label>
                    </div>
                </div>
            </template>
        </div>

        <footer class="panel-footer">
            <span>&copy; 2026 <button class="openmywebsite" @click="openMywebsite">Ryen.</button> All rights
                reserved.</span>
            <span class="action-link"
                :style="{ opacity: isChecking ? 0.5 : 1, pointerEvents: isChecking ? 'none' : 'auto', position: 'relative' }"
                @click="checkUpdate">
                <span v-if="hasNewVersion" class="update-dot"></span>
                {{ isChecking ? '检查中...' : (hasNewVersion ? '检测到新版本' : '检查更新') }}
            </span>
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
import { ref, onMounted, onUnmounted, watch, nextTick, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { emit, listen } from '@tauri-apps/api/event';
import { getVersion } from '@tauri-apps/api/app';
import * as echarts from 'echarts';
import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart';
import { openUrl } from '@tauri-apps/plugin-opener';
import { getCurrentWindow } from '@tauri-apps/api/window';

const isWidgetVisible = ref(false);
const autoStart = ref(false);
const opacity = ref(Number(localStorage.getItem('nsd_island_opacity') || '100'));

const savedTheme = localStorage.getItem('nsd_theme_mode') || 'light';
const themeMode = ref(['light', 'dark', 'system'].includes(savedTheme) ? savedTheme : 'light');

const uploadSpeed = ref('0 B/s');
const downloadSpeed = ref('0 B/s');

const appVersion = ref('1.0.0');

const isDynamicSet = ref(false);

const isChecking = ref(false);
const hasNewVersion = ref(false);

// 灵动岛设置相关的 UI 状态绑定
const islandTheme = ref(localStorage.getItem('nsd_island_theme') || 'black');
const enableMusicCtrl = ref(localStorage.getItem('nsd_music_ctrl') === 'true');
const enableMsgNotify = ref(localStorage.getItem('nsd_msg_notify') === 'true');
const enableHardwareMon = ref(localStorage.getItem('nsd_hardware_mon') === 'true');
const disableBtn = ref(false);
let wasMusicEnabledBeforeHardware = false;

// 置于任务栏状态，默认从本地存储读取
const pinToTaskbar = ref(localStorage.getItem('nsd_pin_taskbar') === 'true');
// 切换开关时保存本地并发送信号给灵动岛
const togglePinTaskbar = async () => {
    localStorage.setItem('nsd_pin_taskbar', String(pinToTaskbar.value));
    await emit('control-pin-taskbar', { enabled: pinToTaskbar.value });
};

// 新增切换保存方法
const toggleMsgNotify = () => {
    localStorage.setItem('nsd_msg_notify', String(enableMsgNotify.value));
};

// 切换灵动岛设置
const toggleDynamicSet = () => {
    isDynamicSet.value = !isDynamicSet.value;
};

// 切换灵动岛设置时，更新图表
watch(isDynamicSet, async (newVal) => {
    if (!newVal) {
        // 销毁所有旧实例，防止内存泄漏或节点挂载错位
        chartInstance?.dispose();
        statsChartInstance?.dispose();

        // 等待 Vue 将 DOM 节点重新渲染出来
        await nextTick();

        // 重新初始化网速波形图
        initChart();

        // 如果用户切走前打开的是数据统计面板，则同步重新初始化统计图表
        if (rightPanel.value === 'stats') {
            initStatsChart();
        }
    }
});

const rightPanel = ref<'settings' | 'stats'>('settings');
const statChartType = ref<'bar' | 'line'>('bar');
const statsChartRef = ref<HTMLElement | null>(null);
let statsChartInstance: any = null;

const trafficData = ref<Record<string, { up: number; down: number }>>({});
let saveThrottleCounter = 0;

// 格式化字节数为人类可读格式
const formatBytesValue = (bytes: number) => {
    if (bytes === 0) return '0';
    const k = 1024;
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)).toString();
};

const formatBytesUnit = (bytes: number) => {
    if (bytes === 0) return 'B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return sizes[i];
};

const totalUpload = computed(() => Object.values(trafficData.value).reduce((acc, curr) => acc + curr.up, 0));
const totalDownload = computed(() => Object.values(trafficData.value).reduce((acc, curr) => acc + curr.down, 0));
const monthTraffic = computed(() => {
    const currentMonth = getLocalYYYYMMDD(new Date()).slice(0, 7);
    return Object.entries(trafficData.value)
        .filter(([date]) => date.startsWith(currentMonth))
        .reduce((acc, [, data]) => acc + data.up + data.down, 0);
});

// 获取本地日期格式为 YYYY-MM-DD
const getLocalYYYYMMDD = (date: Date) => {
    const y = date.getFullYear();
    const m = String(date.getMonth() + 1).padStart(2, '0');
    const d = String(date.getDate()).padStart(2, '0');
    return `${y}-${m}-${d}`;
};

// 加载网络流量统计
const loadTrafficData = () => {
    try {
        const stored = localStorage.getItem('nsd_traffic_stats');
        if (stored) trafficData.value = JSON.parse(stored);
    } catch (e) {
        console.error("加载统计数据失败", e);
    }
};
loadTrafficData();

// 切换右侧面板
const toggleRightPanel = async () => {
    rightPanel.value = rightPanel.value === 'settings' ? 'stats' : 'settings';
    localStorage.setItem('nsd_traffic_stats', JSON.stringify(trafficData.value));
    saveThrottleCounter = 0;

    if (rightPanel.value === 'stats') {
        await nextTick();
        initStatsChart();
    } else {
        statsChartInstance?.dispose();
        statsChartInstance = null;
    }

    // 侧边栏布局变化会挤压左侧卡片，强制让实时走势图重新计算高宽
    await nextTick();
    chartInstance?.resize();
};

const initStatsChart = () => {
    if (!statsChartRef.value || !echarts) return;
    statsChartInstance = echarts.init(statsChartRef.value);
    updateStatsChart();
};

// 更新数据统计图表
const updateStatsChart = () => {
    if (!statsChartInstance) return;
    const isDark = document.documentElement.classList.contains('dark-theme');
    const textColor = isDark ? '#94a3b8' : '#64748b';
    const splitLineColor = isDark ? '#383c41' : '#f1f5f9';

    const days: string[] = [];
    const upData: number[] = [];
    const downData: number[] = [];

    for (let i = 6; i >= 0; i--) {
        const d = new Date();
        d.setDate(d.getDate() - i);
        const dateStr = getLocalYYYYMMDD(d);
        days.push(dateStr.slice(5));

        const dayData = trafficData.value[dateStr] || { up: 0, down: 0 };
        upData.push(Number((dayData.up / (1024 * 1024)).toFixed(2)));
        downData.push(Number((dayData.down / (1024 * 1024)).toFixed(2)));
    }

    statsChartInstance.setOption({
        tooltip: { trigger: 'axis', axisPointer: { type: 'shadow' } },
        legend: { data: ['上传 (MB)', '下载 (MB)'], textStyle: { color: textColor }, top: 0 },
        grid: { left: '2%', right: '2%', bottom: '0%', containLabel: true },
        xAxis: {
            type: 'category',
            data: days,
            axisLabel: { color: textColor },
            axisLine: { lineStyle: { color: splitLineColor } }
        },
        yAxis: {
            type: 'value',
            splitLine: { lineStyle: { color: splitLineColor, type: 'dashed' } },
            axisLabel: { color: textColor }
        },
        series: [
            {
                name: '上传 (MB)',
                type: statChartType.value,
                smooth: true,
                data: upData,
                itemStyle: { color: getChartColors().line },
                barMaxWidth: 15
            },
            {
                name: '下载 (MB)',
                type: statChartType.value,
                smooth: true,
                data: downData,
                itemStyle: { color: isDark ? '#34d399' : '#10b981' },
                barMaxWidth: 15
            }
        ]
    });
};

const toggleAutoStart = async () => {
    try {
        if (autoStart.value) {
            await enable();
        } else {
            await disable();
        }
    } catch (error) {
        autoStart.value = !autoStart.value;
        showDialog('设置失败', '无法修改开机自启动状态，请检查系统权限。');
    }
};

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
    // 使用正则匹配出类似于 X.Y.Z 的纯数字版本号部分
    const match = v.match(/\d+\.\d+\.\d+/);
    if (match) {
        return match[0].split('.').map(Number);
    }
    // 如果实在没匹配到，返回 [0, 0, 0] 防止代码崩溃
    return [0, 0, 0];
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

// 更新图表选项
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

// 获取并更新网络流量统计
const fetchSpeedStats = async () => {
    try {
        const [currentRx, currentTx] = await invoke<[number, number]>('get_network_stats');
        if (lastRx !== 0) {
            const rxDiff = currentRx - lastRx;
            const txDiff = currentTx - lastTx;
            downloadSpeed.value = formatSpeed(rxDiff);
            uploadSpeed.value = formatSpeed(txDiff);

            const speedMB = rxDiff / (1024 * 1024);

            // 核心修复：直接压入完整的 speedMB 浮点数，不做保留两位的截断。
            // 从而使 ECharts 面对极小流量（如 B/s, KB/s 级别）也能捕捉到微小的轴缩放波动。
            chartDataQueue.push(speedMB);
            if (chartDataQueue.length > 15) chartDataQueue.shift();

            chartInstance?.setOption({ series: [{ data: chartDataQueue }] });

            if (rxDiff > 0 || txDiff > 0) {
                const todayStr = getLocalYYYYMMDD(new Date());
                if (!trafficData.value[todayStr]) {
                    trafficData.value[todayStr] = { up: 0, down: 0 };
                }
                trafficData.value[todayStr].down += rxDiff;
                trafficData.value[todayStr].up += txDiff;

                saveThrottleCounter++;
                if (saveThrottleCounter >= 5) {
                    localStorage.setItem('nsd_traffic_stats', JSON.stringify(trafficData.value));
                    saveThrottleCounter = 0;
                }
            }
        }
        lastRx = currentRx;
        lastTx = currentTx;
    } catch (error) {
        console.error('控制台流量获取失败:', error);
    }
};

const openMywebsite = () => {
    openUrl('https://blog.georgewu.top');
}

// 新增：静默检查更新（后台偷偷查，不弹窗，报错了也不干扰用户）
const silentCheckUpdate = async () => {
    try {
        const localVersionStr = await getVersion();
        const response = await fetch('https://api.github.com/repos/GEORGEWWWU/NetSpeed-Dynamic/releases/latest', {
            method: 'GET',
            headers: { 'Accept': 'application/vnd.github.v3+json', 'User-Agent': 'Tauri-App-NetSpeed-Dynamic' }
        });
        if (!response.ok) return;

        const data = await response.json();
        const remoteVersionStr = data.tag_name;
        const local = parseVersion(localVersionStr);
        const remote = parseVersion(remoteVersionStr);

        for (let i = 0; i < 3; i++) {
            const rNum = remote[i] || 0;
            const lNum = local[i] || 0;
            if (rNum > lNum) {
                hasNewVersion.value = true; // 发现新版本，把红点亮起来
                break;
            } else if (rNum < lNum) {
                break;
            }
        }
    } catch (error) {
        // 静默模式失败就当无事发生
    }
};

const checkUpdate = async () => {
    if (isChecking.value) return; // 防止连点
    isChecking.value = true;

    try {
        const localVersionStr = await getVersion();

        // 加一个 10 秒超时控制器
        const controller = new AbortController();
        const timeoutId = setTimeout(() => controller.abort(), 10000);

        const response = await fetch('https://api.github.com/repos/GEORGEWWWU/NetSpeed-Dynamic/releases/latest', {
            method: 'GET',
            headers: {
                'Accept': 'application/vnd.github.v3+json',
                'User-Agent': 'Tauri-App-NetSpeed-Dynamic'
            },
            signal: controller.signal
        });

        clearTimeout(timeoutId);

        if (response.status === 404) {
            showDialog('检查更新', '未找到可用版本');
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

        let findNew = false;
        for (let i = 0; i < 3; i++) {
            const rNum = remote[i] || 0;
            const lNum = local[i] || 0;
            if (rNum > lNum) {
                findNew = true;
                break;
            } else if (rNum < lNum) {
                break;
            }
        }

        if (findNew) {
            hasNewVersion.value = true;
            showDialog(
                '发现新版本',
                `发现新版本 ${remoteVersionStr}！当前版本为 v${localVersionStr}。是否前往 GitHub 下载更新？`,
                true,
                () => {
                    openUrl(data.html_url);
                    hasNewVersion.value = false; // 用户点击去更新后，消掉红点并恢复文字
                }
            );
        } else {
            hasNewVersion.value = false;
            showDialog('提示', '当前已是最新版本！');
        }
    } catch (error: any) {
        console.error('检查更新时出错:', error);
        // 👇 精准识别是不是超时导致的
        if (error.name === 'AbortError') {
            showDialog('网络超时', '连接 GitHub 超时，请检查网络或稍后再试');
        } else {
            showDialog('网络错误', '请求失败，请检查您的网络连接');
        }
    } finally {
        isChecking.value = false; // 👈 无论成功失败，最后都恢复状态
    }
};

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
    updateChartOption();
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

const toggleHardwareMon = async () => {
    localStorage.setItem('nsd_hardware_mon', String(enableHardwareMon.value));
    await emit('control-hardware-mon', { enabled: enableHardwareMon.value });

    if (enableHardwareMon.value) {
        // 如果开启硬件监控，记录音乐状态，并把音乐关掉
        wasMusicEnabledBeforeHardware = enableMusicCtrl.value;
        if (enableMusicCtrl.value) {
            enableMusicCtrl.value = false;
        }
    } else {
        // 如果关闭硬件监控，且原来音乐是开着的，就恢复音乐
        if (wasMusicEnabledBeforeHardware) {
            enableMusicCtrl.value = true;
            wasMusicEnabledBeforeHardware = false; // 用完重置
        }
    }
};

watch(opacity, async (newVal) => {
    localStorage.setItem('nsd_island_opacity', newVal.toString());
    await emit('control-island-opacity', { opacity: newVal });
});

watch(islandTheme, async (newVal) => {
    localStorage.setItem('nsd_island_theme', newVal);
    await emit('control-island-theme', { theme: newVal });
    console.log('灵动岛颜色切换为:', newVal);
});

// 添加监听器，将状态同步给灵动岛
watch(enableMusicCtrl, async (newVal) => {
    localStorage.setItem('nsd_music_ctrl', newVal.toString());
    await emit('control-music-ctl', { enabled: newVal });
    console.log('音乐控制器状态切换为:', newVal);

    // 👇新增互斥防呆逻辑：如果用户手动开启音乐，强行把硬件关掉
    if (newVal && enableHardwareMon.value) {
        enableHardwareMon.value = false;
        localStorage.setItem('nsd_hardware_mon', 'false');
        await emit('control-hardware-mon', { enabled: false });
    }
});

onMounted(async () => {
    silentCheckUpdate();

    window.addEventListener('contextmenu', (e) => {
        e.preventDefault();
    }, { capture: true });

    applyTheme();
    systemThemeMedia = window.matchMedia('(prefers-color-scheme: dark)');
    systemThemeMedia.addEventListener('change', handleSystemThemeUpdate);

    initChart();
    fetchSpeedStats();
    speedTimer = setInterval(fetchSpeedStats, 2000) as unknown as number;
    window.addEventListener('resize', () => {
        chartInstance?.resize();
        statsChartInstance?.resize();
    });

    try {
        autoStart.value = await isEnabled();
    } catch (e) {
        console.error("获取自启动状态失败:", e);
    }

    try {
        appVersion.value = await getVersion();
    } catch (e) {
        console.error("获取应用版本号失败:", e);
    }

    // 监听来自灵动岛右键菜单的“打开设置”信号
    await listen('open-settings-panel', async () => {
        // 1. 如果当前不在灵动岛设置页，就切过去
        if (!isDynamicSet.value) {
            isDynamicSet.value = true;
        }

        // 2. 唤醒并聚焦主窗口
        const appWindow = getCurrentWindow();
        await appWindow.show();        // 确保窗口显示
        await appWindow.unminimize();  // 如果最小化了，就恢复
        await appWindow.setFocus();    // 强制抢占焦点弹到最前面
    });

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
    statsChartInstance?.dispose();
    systemThemeMedia?.removeEventListener('change', handleSystemThemeUpdate);
    localStorage.setItem('nsd_traffic_stats', JSON.stringify(trafficData.value));
});

const toggleWidget = async () => {
    const nextState = !isWidgetVisible.value;
    await emit('control-island-visibility', { show: nextState });
    isWidgetVisible.value = nextState;
};
</script>

<style scoped>
/*提取出的颜色变量层*/
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
    --slider-bg: #d7dce2;
    --slider-checked-bg: #b9b9b9;
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
    --btn-sec-bg: #ebebeb;
    --btn-sec-color: #64748b;
    --btn-sec-border: #e2e8f0;
    --btn-sec-hover-bg: #e2e8f0;
    --btn-sec-hover-color: #334155;
    --btn-pri-bg: #2b2b2b;
    --btn-pri-color: #ffffff;
    --btn-pri-border: #2b2b2b;
    --btn-pri-hover-bg: #1a1a1a;
    --btn-pri-shadow-hover: rgba(0, 0, 0, 0.15);
    --select-bg: #ffffff;
    --select-border: #e2e8f0;
    --select-text: #1e293b;
    --data-tag-bg: #ececec;
    --data-tag-color: #2b2b2b;
}

/*暗色模式变量覆盖*/
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
    --chart-border: #474c53;
    --item-title-color: #f8fafc;
    --tag-dev-bg: #334155;
    --tag-dev-color: #94a3b8;
    --item-desc-color: #94a3b8;
    --slider-bg: #3e4247;
    --slider-checked-bg: #5d646d;
    --slider-disabled-bg: #334155;
    --range-bg: #42474e;
    --range-thumb-bg: #1e293b;
    --range-thumb-border: #60a5fa;
    --range-thumb-shadow: rgba(0, 0, 0, 0.5);
    --footer-text: #94a3b8aa;
    --overlay-bg: rgba(0, 0, 0, 0.6);
    --modal-bg: #292b2e;
    --modal-border: #383c41;
    --modal-h4: #f8fafc;
    --modal-p: #94a3b8;
    --btn-sec-bg: #1a1a1a;
    --btn-sec-color: #cbd5e1;
    --btn-sec-border: #475569;
    --btn-sec-hover-bg: #475569;
    --btn-sec-hover-color: #f8fafc;
    --btn-pri-bg: #1a1a1a;
    --btn-pri-color: #ffffff;
    --btn-pri-border: #2b2b2b;
    --btn-pri-hover-bg: #161616;
    --btn-pri-shadow-hover: rgba(0, 0, 0, 0.15);
    --select-bg: #292b2e;
    --select-border: #383c41;
    --select-text: #f8fafc;
    --data-tag-bg: #202020;
    --data-tag-color: #f8fafc;
}

/*原有布局及节点样式 */
:global(html) {
    color: var(--text-body);
    transition: background-color 0.3s ease, color 0.3s ease;
}

:global(body) {
    background-color: transparent !important;
    color: inherit;
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', 'Segoe UI', Roboto, sans-serif;
    user-select: none;
    -webkit-font-smoothing: antialiased;
}

.panel-container {
    background-color: var(--bg-body);
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
    padding: 8px 16px;
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
    gap: 18px;
    flex-grow: 1;
    transition: all 0.3s ease;
}

/* 游戏模式自适应列宽 */
.main-content.game-mode-layout {
    grid-template-columns: 1fr;
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
    font-size: 22px;
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

.item-desc {
    font-size: 13px;
    color: var(--item-desc-color);
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
    flex-direction: row;
    justify-content: space-between;
    font-size: 12px;
    color: var(--footer-text);
    font-weight: 500;
}

.panel-footer span {
    display: flex;
}

.openmywebsite {
    background: none;
    border: none;
    cursor: pointer;
    outline: none;
    font-size: 12px;
    color: var(--footer-text);
    font-weight: bold;
}

.openmywebsite:hover {
    text-decoration: underline;
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

.update-dot {
    position: absolute;
    top: 2px;
    right: -8px;
    width: 5px;
    height: 5px;
    background-color: #ff3b30;
    border-radius: 50%;
    box-shadow: 0 0 4px rgba(255, 59, 48, 0.4);
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




/* 灵动岛设置按钮样式 */
.dynamicset-btn {
    background: transparent;
    border: 1px solid var(--control-border);
    color: var(--text-body);
    padding: 6px 14px;
    font-size: 12px;
    font-weight: 700;
    border-radius: 16px;
    cursor: pointer;
    transition: all 0.2s ease;
}

.dynamicset-btn:hover {
    background: var(--btn-sec-bg);
    border-color: var(--slider-checked-bg);
}

.dynamicset-btn.is-active {
    background: var(--btn-pri-bg);
    color: var(--btn-pri-color);
    border-color: var(--btn-pri-border);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.15);
}

.control-separator {
    width: 1px;
    height: 16px;
    background: var(--control-border);
}





/* =========================================
   灵动岛设置面板 - 扁平化全宽布局
   ========================================= */

/* 核心修复：当处于灵动岛设置模式时，强制单列全宽，解决只有半宽的问题 */
.main-content.dynamicset-layout {
    grid-template-columns: 1fr !important;
}

/* 双列网格结构，自动填充 */
.dynamicset-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-template-rows: 1fr 1fr 1fr;
    gap: 0;
    background: var(--card-bg);
    border: 1px solid var(--card-border);
    border-radius: 20px;
    padding: 0;
    box-shadow: 0 4px 20px -2px var(--card-shadow);
    height: 100%;
    min-height: 0;
    /* overflow: hidden; */
}

/* 设置项：去掉独立背景和边框，融入容器 */
.set-item {
    background: transparent;
    border: none;
    padding: 20px 24px;
    margin: 0;
    border-radius: 0;
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.disabled-set-item {
    opacity: 0.6;
    /* cursor: not-allowed; */
}

.set-item-meta {
    display: flex;
    flex-direction: column;
    gap: 4px;
}

.set-item-title {
    font-size: 14px;
    font-weight: 600;
    display: flex;
    align-items: center;
    max-height: 24px;
    color: var(--item-title-color);
}

.set-item-pro-tag {
    font-size: 11px;
    font-weight: 600;
    color: var(--btn-pri-color);
    background: var(--btn-pri-bg);
    padding: 2px 6px;
    border-radius: 4px;
    margin-left: 6px;
    max-height: 24px;
}

.set-item-new-tag {
    font-size: 10px;
    font-weight: 600;
    color: #dd1f1f;
    background: #b9101020;
    padding: 2px 6px;
    border-radius: 4px;
    margin-left: 6px;
    max-height: 24px;
    transform: translateY(1px);
}

.set-item-tips-tag {
    font-size: 10px;
    font-weight: bold;
    color: var(--btn-pri-color);
    background: transparent;
    padding: 2px 6px;
    border-radius: 4px;
    margin-left: 6px;
    max-height: 24px;
    transform: translateY(1px);
    opacity: 0.8;
}

/* Tooltip 容器 */
.tooltip-wrapper {
    position: relative;
    display: inline-flex;
    align-items: center;
    cursor: help;
    /* 鼠标悬停时显示帮助光标 */
}

/* Tooltip 气泡本体 */
.tooltip-wrapper::after {
    content: attr(data-tooltip);
    position: absolute;
    bottom: calc(100% + 2px);
    /* 位于元素上方 8px */
    left: 50%;
    transform: translateX(-50%) translateY(4px);
    /* 初始位置稍微偏下，用于动画 */

    /* 样式：复用现有主题变量 */
    background: var(--modal-bg);
    color: var(--text-body);
    border: 1px solid var(--card-border);
    box-shadow: 0 4px 12px var(--card-shadow-hover);

    padding: 8px 12px;
    border-radius: 8px;
    font-size: 12px;
    font-weight: 500;
    line-height: 1.4;
    white-space: nowrap;
    /* 保持单行，若文本过长可改为 normal 并设置 max-width */

    /* 交互与动画 */
    opacity: 0;
    visibility: hidden;
    pointer-events: none;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    z-index: 999;
}

/* Hover 时显示 */
.tooltip-wrapper:hover::after {
    opacity: 1;
    visibility: visible;
    transform: translateX(-50%) translateY(0);
}

.tooltip-wrapper:hover::before {
    opacity: 1;
    transform: translateX(-50%) scale(1);
}

.set-item-desc {
    font-size: 12px;
    color: var(--item-desc-color);
}

/* 胶囊形滑块开关 (保持不变，仅确保层级) */
.capsule-switch {
    display: flex;
    background: var(--slider-bg);
    border-radius: 6px;
    padding: 2px;
    position: relative;
    box-shadow: inset 0 1px 3px rgba(0, 0, 0, 0.05);
    flex-shrink: 0;
}

.capsule-btn {
    padding: 4px 14px;
    font-size: 12px;
    font-weight: 600;
    border-radius: 4px;
    cursor: pointer;
    color: var(--text-body);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    z-index: 1;
    opacity: 0.6;
}

.capsule-btn.is-active {
    background: var(--card-bg);
    color: var(--item-title-color);
    box-shadow: 0 1px 4px var(--card-shadow-hover);
    opacity: 1;
}






/* 数据统计模块样式 */
.card-header-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
}

.card-header-row h3 {
    margin-bottom: 0;
}

.stats-toggle-btn {
    background: transparent;
    color: var(--item-title-color);
    border: 1px solid var(--chart-border);
    padding: 4px 10px;
    font-size: 12px;
    font-weight: 600;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
}

.stats-toggle-btn:hover {
    background: var(--btn-sec-bg);
}

.stats-card {
    display: flex;
    flex-direction: column;
}

.stats-overview {
    display: flex;
    gap: 8px;
    margin-bottom: 20px;
}

.stat-box {
    flex: 1;
    background: var(--control-bg);
    border: 1px solid var(--card-border);
    border-radius: 12px;
    padding: 12px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: start;
    gap: 4px;
    height: 70px;
    /* 固定高度，不会因内容变化而撑高 */
    box-sizing: border-box;
    position: relative;
}

.stat-label {
    font-size: 12px;
    color: var(--item-desc-color);
    font-weight: 500;
    flex-shrink: 0;
    transform: translateY(-5px);
}

.stat-val {
    font-size: 16px;
    font-weight: 700;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    color: var(--speed-value);
    white-space: nowrap;
    /* 数值不会换行 */
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 100%;
    flex-shrink: 0;
    transform: translateY(-5px);
}

.stat-unit {
    font-size: 10px;
    font-weight: 600;
    color: var(--item-desc-color);
    opacity: 0.7;
    flex-shrink: 0;
    position: absolute;
    bottom: 3px;
    background: var(--data-tag-bg);
    color: var(--data-tag-color);
    padding: 1px 5px;
    border-radius: 4px;
}

.stats-chart-container {
    width: 100%;
    flex-grow: 1;
    min-height: 180px;
    border-top: 1px solid var(--chart-border);
    padding-top: 16px;
}
</style>