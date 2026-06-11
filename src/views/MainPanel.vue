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
                        <span class="arrow up">↑</span>
                        <div class="speed-info">
                            <span class="label">上传速度</span>
                            <span class="value">124.5 KB/s</span>
                        </div>
                    </div>
                    <div class="speed-item">
                        <span class="arrow down">↓</span>
                        <div class="speed-info">
                            <span class="label">下载速度</span>
                            <span class="value">2.4 MB/s</span>
                        </div>
                    </div>
                </div>
                <div class="mini-chart">
                    <div class="bar" style="height: 40%"></div>
                    <div class="bar" style="height: 60%"></div>
                    <div class="bar" style="height: 45%"></div>
                    <div class="bar" style="height: 75%"></div>
                    <div class="bar" style="height: 90%"></div>
                    <div class="bar" style="height: 55%"></div>
                </div>
            </div>

            <div class="card settings-card">
                <h3>常规设置</h3>

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
            <span class="action-link">检查更新</span>
        </footer>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { emit, listen } from '@tauri-apps/api/event';

const isWidgetVisible = ref(false);
// 开机自启虽然在这定义了，但在 UI 上已禁用
const autoStart = ref(false);
const opacity = ref(Number(localStorage.getItem('nsd_island_opacity') || '100'));

// 监听滑块变化
watch(opacity, async (newVal) => {
    // 1. 将新值存入本地缓存，这样刷新页面就不会丢了
    localStorage.setItem('nsd_island_opacity', newVal.toString());
    // 2. 同步发射事件通知灵动岛
    await emit('control-island-opacity', { opacity: newVal });
});

onMounted(async () => {
    // 监听来自灵动岛的自发性隐藏通知
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
        } catch { /* 窗口还没注册好，忽略 */ }
        await new Promise(r => setTimeout(r, 200));
    }
    isWidgetVisible.value = false;
});

// 切换逻辑
const toggleWidget = async () => {
    const nextState = !isWidgetVisible.value;
    await emit('control-island-visibility', { show: nextState });
    isWidgetVisible.value = nextState;
};
</script>

<style scoped>
/* 全局样式基础重置 */
:global(body) {
    background-color: #f8fafc;
    /* 换成极浅的灰蓝色背景，突出白色卡片 */
    color: #1e293b;
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', 'Segoe UI', Roboto, sans-serif;
    margin: 0;
    padding: 0;
    user-select: none;
    -webkit-font-smoothing: antialiased;
}

.panel-container {
    padding: 28px 32px;
    max-width: 800px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    min-height: calc(100vh - 56px);
}

/* 头部样式 */
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

.svg-lightning {
    width: 26px;
    height: 26px;
    /* 配合渐变加一点微小的阴影让图标更立体 */
    filter: drop-shadow(0 2px 2px rgba(0, 0, 0, 0.1));
}

.brand h1 {
    font-size: 20px;
    margin: 0;
    font-weight: 700;
    letter-spacing: 0.2px;
    color: #0f172a;
}

.subtitle {
    font-size: 13px;
    color: #64748b;
    margin: 4px 0 0 0;
}

/* 右上角控制区 */
.header-controls {
    display: flex;
    align-items: center;
    gap: 16px;
    background: #ffffff;
    padding: 8px 12px 8px 16px;
    border-radius: 24px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
    border: 1px solid #e2e8f0;
}

.status-badge {
    font-size: 13px;
    font-weight: 600;
    color: #94a3b8;
    transition: all 0.3s;
}

.status-badge.is-active {
    color: #2b2b2b;
}

.divider {
    border: none;
    border-top: 1px solid #e2e8f0;
    margin-bottom: 24px;
}

/* 主体内部分栏 */
.main-content {
    display: grid;
    grid-template-columns: 1fr 1.3fr;
    gap: 24px;
    flex-grow: 1;
}

/* 现代化卡片 */
.card {
    background: #ffffff;
    border: 1px solid #e2e8f0;
    border-radius: 20px;
    padding: 24px;
    display: flex;
    flex-direction: column;
    box-shadow: 0 4px 20px -2px rgba(0, 0, 0, 0.03);
    transition: transform 0.2s, box-shadow 0.2s;
}

.card:hover {
    box-shadow: 0 8px 24px -4px rgba(0, 0, 0, 0.06);
}

.card h3 {
    font-size: 15px;
    color: #334155;
    margin: 0 0 20px 0;
    font-weight: 600;
}

/* 左侧状态监控面板 */
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

.arrow.up {
    background: #eff6ff;
    color: #3b82f6;
}

.arrow.down {
    background: #ecfdf5;
    color: #10b981;
}

.speed-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
}

.speed-info .label {
    font-size: 12px;
    color: #64748b;
    font-weight: 500;
}

.speed-info .value {
    font-size: 18px;
    font-weight: 700;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    color: #0f172a;
    letter-spacing: -0.5px;
}

/* 波动图表 */
.mini-chart {
    display: flex;
    align-items: flex-end;
    gap: 8px;
    height: 50px;
    margin-top: auto;
    padding-top: 16px;
    border-top: 1px solid #f1f5f9;
}

.mini-chart .bar {
    flex: 1;
    background: linear-gradient(to top, #3b82f6, #60a5fa);
    border-radius: 4px 4px 0 0;
    opacity: 0.85;
    transition: height 0.3s ease;
}

/* 右侧设置列表 */
.setting-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 0;
    border-bottom: 1px solid #f1f5f9;
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

.item-meta {
    display: flex;
    flex-direction: column;
    gap: 4px;
}

.item-title {
    font-size: 14px;
    font-weight: 600;
    color: #1e293b;
    display: flex;
    align-items: center;
    gap: 8px;
}

.tag-dev {
    font-size: 10px;
    background: #f1f5f9;
    color: #64748b;
    padding: 2px 6px;
    border-radius: 4px;
    font-weight: normal;
}

.item-desc {
    font-size: 13px;
    color: #64748b;
}

/* 置灰状态样式 */
.is-disabled {
    opacity: 0.5;
    pointer-events: none;
}

/* Switch 开关样式 */
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
    background-color: #cbd5e1;
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
    background-color: #2b2b2b;
}

input:checked+.slider:before {
    transform: translateX(20px);
}

input:disabled+.slider {
    background-color: #e2e8f0;
    cursor: not-allowed;
}

/* Range 滑块样式 */
.range-input {
    width: 100%;
    -webkit-appearance: none;
    appearance: none;
    background: #e2e8f0;
    height: 6px;
    border-radius: 3px;
    outline: none;
}

.range-input::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: #ffffff;
    border: 2px solid #2b2b2b;
    cursor: pointer;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);
    transition: transform 0.1s;
}

.range-input::-webkit-slider-thumb:hover {
    transform: scale(1.1);
}

/* 页脚 */
.panel-footer {
    margin-top: 32px;
    display: flex;
    justify-content: space-between;
    font-size: 12px;
    color: #94a3b8;
    font-weight: 500;
}

.action-link {
    color: #2b2b2b;
    cursor: pointer;
    transition: color 0.2s;
}

.action-link:hover {
    color: #2b2b2b;
    text-decoration: underline;
}
</style>