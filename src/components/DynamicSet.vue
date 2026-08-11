<template>
    <div class="dynamic-set-dashboard">

        <div class="grid-section">

            <div class="neo-card">
                <div class="card-header">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                        stroke-linejoin="round" class="title-icon">
                        <path
                            d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
                    </svg>
                    <span>{{ t('dynamicPhysicsFeedback') }}</span>
                </div>
                <div class="spring-selector">
                    <button class="spring-btn" :class="{ active: springStyle === 'stiff' }"
                        @click="springStyle = 'stiff'">
                        <svg viewBox="0 0 24 24" class="spring-icon">
                            <path d="M4 12 L8 8 L12 16 L16 8 L20 12" stroke="currentColor" stroke-width="2" fill="none"
                                stroke-linejoin="round" />
                        </svg>
                        <span>{{ t('springStiff') }}</span>
                    </button>
                    <button class="spring-btn" :class="{ active: springStyle === 'bouncy' }"
                        @click="springStyle = 'bouncy'">
                        <svg viewBox="0 0 24 24" class="spring-icon">
                            <path d="M3 12 C 7 2, 10 22, 14 12 C 16 7, 18 16, 21 12" stroke="currentColor"
                                stroke-width="2" fill="none" stroke-linecap="round" />
                        </svg>
                        <span>{{ t('springBouncy') }}</span>
                    </button>
                </div>
            </div>

            <div class="neo-card">
                <div class="card-header">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                        stroke-linejoin="round" class="title-icon">
                        <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
                        <circle cx="8.5" cy="8.5" r="1.5" />
                        <polyline points="21 15 16 10 5 21" />
                    </svg>
                    <span>{{ t('appearanceEdge') }}</span>
                </div>
                <div class="form-group-list">
                    <div class="form-item mt-auto">
                        <span class="label">{{ t('islandColor') }}</span>
                        <div class="shape-toggle">
                            <button :class="{ active: islandTheme === 'black' }" @click="islandTheme = 'black'"
                                :title="t('darkColor')" style="background: #1a1a1a;"></button>
                            <button :class="{ active: islandTheme === 'white' }" @click="islandTheme = 'white'"
                                :title="t('lightColor')" style="background: #f5f5f5; border: 1px solid #ccc;"></button>
                            <button :class="{ active: islandTheme === 'coverglass' }"
                                @click="islandTheme = 'coverglass'" :title="t('coverglassMode')"
                                style="background: linear-gradient(135deg, #2c3e50 0%, #000000 100%); border: none;"></button>
                        </div>
                    </div>
                    <div class="form-item">
                        <span class="label">{{ t('edgeShape') }}</span>
                        <div class="shape-toggle">
                            <button :class="{ active: borderRadius === 100 }" @click="borderRadius = 100"
                                :title="t('classicCapsule')"></button>
                            <button :class="{ active: borderRadius === 12 }" @click="borderRadius = 12"
                                :title="t('roundedRectangle')" style="border-radius: 6px;"></button>
                        </div>
                    </div>
                </div>
            </div>

            <div class="neo-card">
                <div class="card-header">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                        stroke-linejoin="round" class="title-icon">
                        <rect x="4" y="4" width="12" height="12" rx="2" ry="2" />
                        <rect x="8" y="8" width="12" height="12" rx="2" ry="2" />
                    </svg>
                    <span>{{ t('windowHierarchy') }}</span>
                </div>
                <div class="form-group-list">
                    <div class="form-item">
                        <span class="label">任务栏组件</span>
                        <label class="mock-switch">
                            <input type="checkbox" v-model="enableTaskbarPlugin" @change="toggleTaskbar">
                            <span class="slider"></span>
                        </label>
                    </div>

                    <div class="form-item mt-auto">
                        <span class="label">{{ t('lyricDelay') }}</span>
                        <div class="stepper-control">
                            <button @click="lyricDelay -= 0.25">-</button>
                            <input type="text" :value="lyricDelay > 0 ? '+' + lyricDelay + 's' : lyricDelay + 's'"
                                readonly style="width: 50px;">
                            <button @click="lyricDelay += 0.25">+</button>
                        </div>
                    </div>
                </div>
            </div>

            <div class="neo-card position-card">
                <div class="card-header">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                        stroke-linejoin="round" class="title-icon">
                        <path d="M12 2v20M2 12h20" />
                        <circle cx="12" cy="12" r="4" />
                    </svg>
                    <span>{{ t('islandPosition') }}</span>
                </div>
                <div class="position-controls">
                    <span class="position-hint">{{ t('islandPositionDesc') }}</span>
                    <div class="position-actions">
                        <button class="position-btn" @click="startPositionAdjustment">{{ t('adjustPosition') }}</button>
                        <button class="position-btn position-btn-secondary" @click="centerIsland">{{ t('topCenter') }}</button>
                        <button class="position-btn position-btn-primary" @click="applyPosition">{{ t('applyPosition') }}</button>
                    </div>
                </div>
            </div>

        </div>

        <div class="list-section">
            <div class="card-header" style="margin-bottom: 12px;">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                    stroke-linejoin="round" class="title-icon">
                    <path d="M15 3h6v6M9 21H3v-6M21 3l-7 7M3 21l7-7" />
                </svg>
                <span style="font-size: 16px; letter-spacing: 0.5px;">{{ t('sizeBoundary') }}</span>
            </div>

            <div class="slider-list-container">
                <div class="slider-row">
                    <div class="row-info">
                        <div class="title-wrapper">
                            <span class="row-title">{{ t('baseWidth') }}</span>
                            <transition name="badge-fade">
                                <span v-if="baseWidth !== 150" class="status-badge">已生效</span>
                            </transition>
                        </div>
                        <span class="row-desc">{{ t('baseWidthDesc') }}</span>
                    </div>
                    <div class="row-action">
                        <input type="range" min="140" max="300" v-model.number="baseWidth"
                            class="track-slider highlight-slider" />
                        <div class="value-box">{{ baseWidth }}<span class="unit">PX</span></div>
                        <button class="reset-btn" @click="baseWidth = 150" :title="t('restoreDefault')">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                                stroke-linecap="round" stroke-linejoin="round">
                                <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
                                <path d="M3 3v5h5" />
                            </svg>
                        </button>
                    </div>
                </div>

                <div class="slider-row">
                    <div class="row-info">
                        <div class="title-wrapper">
                            <span class="row-title">{{ t('baseHeight') }}</span>
                            <transition name="badge-fade"> <span v-if="baseHeight !== 34"
                                    class="status-badge">已生效</span>
                            </transition>
                        </div>
                        <span class="row-desc">{{ t('baseHeightDesc') }}</span>
                    </div>
                    <div class="row-action">
                        <input type="range" min="30" max="60" v-model.number="baseHeight"
                            class="track-slider highlight-slider" />
                        <div class="value-box">{{ baseHeight }}<span class="unit">PX</span></div>
                        <button class="reset-btn" @click="baseHeight = 34" :title="t('restoreDefault')">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                                stroke-linecap="round" stroke-linejoin="round">
                                <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
                                <path d="M3 3v5h5" />
                            </svg>
                        </button>
                    </div>
                </div>

                <div class="slider-row">
                    <div class="row-info">
                        <div class="title-wrapper">
                            <span class="row-title">{{ t('musicBaseWidth') }}</span>
                            <transition name="badge-fade"> <span v-if="musicBaseWidth !== 260"
                                    class="status-badge">已生效</span>
                            </transition>
                        </div>
                        <span class="row-desc">{{ t('musicBaseWidthDesc') }}</span>
                    </div>
                    <div class="row-action">
                        <input type="range" min="200" max="400" v-model.number="musicBaseWidth"
                            class="track-slider highlight-slider" />
                        <div class="value-box">{{ musicBaseWidth }}<span class="unit">PX</span></div>
                        <button class="reset-btn" @click="musicBaseWidth = 260" :title="t('restoreDefault')">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                                stroke-linecap="round" stroke-linejoin="round">
                                <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
                                <path d="M3 3v5h5" />
                            </svg>
                        </button>
                    </div>
                </div>

                <div class="slider-row">
                    <div class="row-info">
                        <div class="title-wrapper">
                            <span class="row-title">{{ t('mediaCardWidth') }}</span>
                            <transition name="badge-fade"> <span v-if="musicExpandedWidth !== 320"
                                    class="status-badge">已生效</span>
                            </transition>
                        </div>
                        <span class="row-desc">{{ t('mediaCardWidthDesc') }}</span>
                    </div>
                    <div class="row-action">
                        <input type="range" min="260" max="480" v-model.number="musicExpandedWidth"
                            class="track-slider highlight-slider" />
                        <div class="value-box">{{ musicExpandedWidth }}<span class="unit">PX</span></div>
                        <button class="reset-btn" @click="musicExpandedWidth = 320" :title="t('restoreDefault')">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                                stroke-linecap="round" stroke-linejoin="round">
                                <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
                                <path d="M3 3v5h5" />
                            </svg>
                        </button>
                    </div>
                </div>

                <div class="slider-row">
                    <div class="row-info">
                        <div class="title-wrapper">
                            <span class="row-title">{{ t('msgCardWidth') }}</span>
                            <transition name="badge-fade"> <span v-if="msgExpandedWidth !== 360"
                                    class="status-badge">已生效</span>
                            </transition>
                        </div>
                        <span class="row-desc">{{ t('msgCardWidthDesc') }}</span>
                    </div>
                    <div class="row-action">
                        <input type="range" min="300" max="600" v-model.number="msgExpandedWidth"
                            class="track-slider highlight-slider" />
                        <div class="value-box">{{ msgExpandedWidth }}<span class="unit">PX</span></div>
                        <button class="reset-btn" @click="msgExpandedWidth = 360" :title="t('restoreDefault')">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                                stroke-linecap="round" stroke-linejoin="round">
                                <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
                                <path d="M3 3v5h5" />
                            </svg>
                        </button>
                    </div>
                </div>

                <div class="slider-row">
                    <div class="row-info">
                        <div class="title-wrapper">
                            <span class="row-title">{{ t('globalScale') }}</span>
                            <transition name="badge-fade">
                                <span v-if="appScale !== 1.0" class="status-badge">已生效</span>
                            </transition>
                        </div>
                        <span class="row-desc">{{ t('globalScaleDesc') }}</span>
                    </div>
                    <div class="row-action">
                        <input type="range" min="1" max="1.75" step="0.25" v-model.number="appScale"
                            class="track-slider highlight-slider" />
                        <div class="value-box">{{ Math.round(appScale * 100) }}<span class="unit">%</span></div>
                        <button class="reset-btn" @click="appScale = 1.0" :title="t('restoreDefault')">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                                stroke-linecap="round" stroke-linejoin="round">
                                <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
                                <path d="M3 3v5h5" />
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        </div>

    </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { emit } from '@tauri-apps/api/event';
import { t } from '../i18n';
import { invoke } from '@tauri-apps/api/core';

// 尺寸状态
const baseWidth = ref(Number(localStorage.getItem('nsd_base_width')) || 150);
const baseHeight = ref(Number(localStorage.getItem('nsd_base_height')) || 34);
const musicBaseWidth = ref(Number(localStorage.getItem('nsd_music_base_width')) || 260);
const musicExpandedWidth = ref(Number(localStorage.getItem('nsd_music_expanded_width')) || 320);
const msgExpandedWidth = ref(Number(localStorage.getItem('nsd_msg_expanded_width')) || 360);
const appScale = ref(Number(localStorage.getItem('nsd_app_scale')) || 1.0);

// 形态与外观
const borderRadius = ref(Number(localStorage.getItem('nsd_border_radius')) || 100);
const islandTheme = ref(localStorage.getItem('nsd_island_theme') || 'black');

// 物理动效
const springStyle = ref<'stiff' | 'bouncy'>((localStorage.getItem('nsd_spring_style') as 'stiff' | 'bouncy') || 'bouncy');

// 替换掉坐标偏移，改为窗口交互特性
const lyricDelay = ref(Number(localStorage.getItem('nsd_lyric_delay')) || 0);

// 任务栏组件
const emits = defineEmits(['show-plugin-dialog']);
const enableTaskbarPlugin = ref(localStorage.getItem('nsd_taskbar_plugin') === 'true');
const toggleTaskbar = async () => {
    try {
        await invoke('toggle_taskbar_plugin', { enable: enableTaskbarPlugin.value });
        localStorage.setItem('nsd_taskbar_plugin', String(enableTaskbarPlugin.value));
    } catch (err: any) {
        // 启动失败，回退开关状态
        enableTaskbarPlugin.value = false;
        localStorage.setItem('nsd_taskbar_plugin', 'false');

        // 2. 删掉原来的 alert(err); 替换为呼叫父组件的弹窗事件
        emits('show-plugin-dialog');
    }
};

const startPositionAdjustment = async () => {
    await emit('adjust-island-position', { action: 'start' });
};

const applyPosition = async () => {
    await emit('adjust-island-position', { action: 'apply' });
};

const centerIsland = async () => {
    await emit('adjust-island-position', { action: 'center' });
};

// 统一监听更新逻辑入口
watch([baseWidth, baseHeight, musicBaseWidth, musicExpandedWidth, msgExpandedWidth, borderRadius, islandTheme, springStyle, appScale, lyricDelay], async () => {
    // 1. 写入本地缓存
    localStorage.setItem('nsd_base_width', String(baseWidth.value));
    localStorage.setItem('nsd_base_height', String(baseHeight.value));
    localStorage.setItem('nsd_music_base_width', String(musicBaseWidth.value));
    localStorage.setItem('nsd_music_expanded_width', String(musicExpandedWidth.value));
    localStorage.setItem('nsd_msg_expanded_width', String(msgExpandedWidth.value));
    localStorage.setItem('nsd_border_radius', String(borderRadius.value));
    localStorage.setItem('nsd_island_theme', String(islandTheme.value));
    localStorage.setItem('nsd_spring_style', springStyle.value);
    localStorage.setItem('nsd_app_scale', String(appScale.value));
    localStorage.setItem('nsd_lyric_delay', String(lyricDelay.value));

    // 发送颜色专属广播
    await emit('control-island-theme', { theme: islandTheme.value });

    // 2. 发送 IPC 事件广播给 Tauri 后端
    await emit('sync-dynamic-settings', {
        baseWidth: baseWidth.value,
        baseHeight: baseHeight.value,
        musicBaseWidth: musicBaseWidth.value,
        musicExpandedWidth: musicExpandedWidth.value,
        msgExpandedWidth: msgExpandedWidth.value,
        borderRadius: borderRadius.value,
        springStyle: springStyle.value,
        appScale: appScale.value,
        lyricDelay: lyricDelay.value,
    });
}, { deep: true });
</script>

<style scoped>
/* 全局容器 (绝对禁止滚动，铺满高度) */
.dynamic-set-dashboard {
    display: flex;
    flex-direction: column;
    gap: 20px;
    height: 100%;
    max-height: calc(100vh - 120px);
    overflow: hidden;
    box-sizing: border-box;
    user-select: none;
}

/* 宫格区域 */
.grid-section {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 16px;
    flex-shrink: 0;
}

/* 现代科技感卡片 */
.neo-card {
    background: var(--card-bg);
    border: 1px solid var(--card-border);
    border-radius: 16px;
    padding: 16px;
    display: flex;
    flex-direction: column;
    transition: transform 0.2s, border-color 0.2s;
}

.position-card {
    grid-column: span 3;
}

.position-controls {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
}

.position-hint {
    font-size: 13px;
    color: var(--item-desc-color);
}

.position-actions {
    display: flex;
    gap: 8px;
    flex-shrink: 0;
}

.position-btn {
    min-width: 84px;
    height: 32px;
    padding: 0 12px;
    border: 1px solid var(--control-border);
    border-radius: 9px;
    background: var(--btn-pri-bg);
    color: var(--btn-sec-color);
    cursor: pointer;
    transition: all 0.2s;
}

.position-btn:hover {
    border-color: var(--btn-pri-border);
}

.position-btn:not(.position-btn-secondary) {
    color: #fff;
}

.position-btn:not(.position-btn-secondary):hover {
    color: #fff;
}

.position-btn-primary {
    border-color: var(--btn-pri-border);
    background: var(--btn-pri-bg);
    color: #fff;
}

.position-btn-secondary {
    background: transparent;
}

.card-header {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 16px;
    font-weight: 600;
    color: var(--item-title-color);
    margin-bottom: 13px;
}

.title-icon {
    width: 16px;
    height: 16px;
    color: var(--item-desc-color);
}

/* 卡片1：物理选择器 */
.spring-selector {
    display: flex;
    gap: 10px;
    height: 100%;
}

.spring-btn {
    flex: 1;
    background: transparent;
    border: 1px solid var(--control-border);
    border-radius: 12px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 8px;
    color: var(--btn-sec-color);
    cursor: pointer;
    transition: all 0.2s;
}

.spring-icon {
    width: 24px;
    height: 24px;
}

.spring-btn:hover {
    background: var(--btn-pri-bg);
    color: var(--btn-sec-hover-color);
}

.spring-btn.active {
    background: var(--btn-pri-bg);
    border-color: var(--btn-pri-border);
    color: var(--btn-pri-color);
    box-shadow: 0 2px 8px var(--card-shadow-hover);
}

/* 卡片2：形态与开关 */
.form-group-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
    height: 100%;
}

.form-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.mt-auto {
    margin-top: auto;
}

.label {
    font-size: 13px;
    color: var(--item-title-color);
}

.shape-toggle {
    display: flex;
    gap: 6px;
}

.shape-toggle button {
    width: 32px;
    height: 20px;
    background: var(--btn-pri-bg);
    border: 2px solid transparent;
    border-radius: 10px;
    cursor: pointer;
    transition: all 0.2s;
}

.shape-toggle button.active {
    border-color: var(--item-title-active-color);
    background: var(--btn-pri-bg);
}

/* Switch开关 */
.mock-switch {
    position: relative;
    display: inline-block;
    width: 44px;
    height: 24px;
}

.mock-switch input {
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
    transition: .3s;
    border-radius: 24px;
    border: 1px solid var(--control-border);
}

.slider:before {
    position: absolute;
    content: "";
    height: 18px;
    width: 18px;
    left: 2px;
    bottom: 2px;
    background-color: #ffffff;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
    transition: .3s;
    border-radius: 50%;
}

input:checked+.slider {
    background-color: var(--slider-checked-bg);
    border-color: var(--slider-checked-bg);
}

input:checked+.slider:before {
    transform: translateX(20px);
}

/* 步进器 */
.stepper-group {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    height: 100%;
    gap: 10px;
}

.stepper-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: var(--select-bg);
    padding: 6px 8px;
    border-radius: 8px;
    border: 1px solid var(--control-border);
}

.axis {
    font-size: 12px;
    font-weight: bold;
    color: var(--item-desc-color);
    width: 30px;
    text-align: center;
}

.stepper-control {
    display: flex;
    align-items: center;
    background: var(--bg-body);
    border-radius: 6px;
    border: 1px solid var(--control-border);
    overflow: hidden;
}

.stepper-control button {
    width: 28px;
    height: 24px;
    background: transparent;
    border: none;
    color: var(--item-title-color);
    cursor: pointer;
    font-weight: bold;
}

.stepper-control button:hover {
    background: var(--btn-pri-bg);
    color: var(--btn-pri-color);
}

.stepper-control input {
    width: 44px;
    text-align: center;
    background: transparent;
    border: none;
    color: var(--item-title-color);
    font-size: 13px;
    font-family: monospace;
    pointer-events: none;
}

/* 列表区域 */
.list-section {
    flex: 1;
    display: flex;
    flex-direction: column;
    background: var(--control-bg, rgba(255, 255, 255, 0.03));
    border: 1px solid var(--card-border, rgba(255, 255, 255, 0.08));
    border-radius: 16px;
    padding: 16px;
    overflow-y: auto;
}

/* 隐藏原生粗糙的滚动条，替换为细线条*/
.list-section::-webkit-scrollbar {
    width: 4px;
}

.list-section::-webkit-scrollbar-track {
    background: transparent;
}

.list-section::-webkit-scrollbar-thumb {
    background-color: var(--slider-bg);
    border-radius: 4px;
}

.list-section::-webkit-scrollbar-thumb:hover {
    background-color: var(--slider-checked-bg);
}

/* 网格容器 */
.slider-list-container {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 10px;
    align-content: flex-start;
}

/* 独立滑块卡片 */
.slider-row {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 6px;
    padding: 8px 12px;
    background: var(--bg-body, rgba(255, 255, 255, 0.02));
    border-radius: 12px;
}

/* 标题区 */
.row-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
}

/* 标题区元素 */
.row-title {
    font-size: 14px;
    font-weight: 500;
    color: var(--item-title-color);
}

.row-desc {
    font-size: 11px;
    color: #888;
}

.row-action {
    display: flex;
    align-items: center;
    width: 100%;
    gap: 10px;
}

/* 滑动条元素 */
.track-slider {
    -webkit-appearance: none;
    appearance: none;
    flex: 1;
    width: auto;
    height: 6px;
    background: var(--slider-bg, rgba(255, 255, 255, 0.1));
    border-radius: 3px;
    outline: none;
}

.track-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: #fff;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.3);
    cursor: pointer;
    transition: transform 0.1s;
}

.track-slider::-webkit-slider-thumb:hover {
    transform: scale(1.15);
}

.highlight-slider::-webkit-slider-thumb {
    border: 3px solid #666;
}

/* 数值展示框 */
.value-box {
    width: 54px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-body, rgba(0, 0, 0, 0.3));
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    font-size: 12px;
    font-weight: 600;
    color: var(--item-title-color);
    font-family: ui-monospace, monospace;
}

.value-box .unit {
    font-size: 10px;
    padding: 1px 3px;
    border-radius: 4px;
    background: var(--select-bg, rgba(255, 255, 255, 0.03));
    color: #666;
    margin-left: 2px;
    transform: translateX(3px);
}

/* 重置按钮 */
.reset-btn {
    width: 26px;
    height: 26px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-body, rgba(0, 0, 0, 0.3));
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    color: var(--item-title-color);
    cursor: pointer;
    transition: all 0.2s;
    padding: 0;
}

/* 悬浮时，跟随系统主按钮的高亮色 */
.reset-btn:hover {
    background: var(--btn-pri-bg);
    color: var(--btn-pri-color);
}

/* 内嵌 SVG 重启图标 */
.reset-btn svg {
    width: 12px;
    height: 12px;
}

/* 将标题和标签横向排列 */
.title-wrapper {
    display: flex;
    align-items: center;
    gap: 6px;
}

/* “已生效”标签的样式 */
.status-badge {
    font-size: 11px;
    padding: 3px 6px;
    border-radius: 4px;
    background: rgba(74, 222, 128, 0.15);
    color: #4ade80;
    font-weight: 500;
    line-height: 1;
    transform: translateY(0.5px);
}

/* 标签显隐动画 */
.badge-fade-enter-active,
.badge-fade-leave-active {
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
}

/* 进场前和退场后的状态：完全透明 + 稍微向左偏移并缩小 */
.badge-fade-enter-from,
.badge-fade-leave-to {
    opacity: 0;
    transform: scale(0.8);
}

/* 进场后和退场前的状态：默认状态 */
.badge-fade-enter-to,
.badge-fade-leave-from {
    opacity: 1;
    transform: scale(1);
}
</style>
