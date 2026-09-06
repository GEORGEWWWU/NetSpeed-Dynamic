<template>
    <transition @enter="onEnter" @leave="onLeave" :css="false">
        <div v-show="isIslandVisible" :class="['island-container', { 'has-music-border': isGlowBorderEnabled }]"
            @mousedown="handleMouseDown" @mousemove="handleMouseMove" @mouseup="handleMouseUp"
            @mouseleave="handleMouseLeave" @mouseenter="handleMouseEnter" :style="islandStyle"
            @contextmenu="handleRightClick">

            <div class="rainbow-border-glow" v-if="isGlowBorderEnabled" :style="{ opacity: glowOpacity }"></div>

            <div v-if="showCoverglassBg" class="coverglass-bg-container" :style="coverglassStyle">
                <div class="coverglass-bg-image" :style="{ backgroundImage: `url(${blurredCoverUrl})` }"></div>
                <div class="coverglass-noise-layer"></div>
                <div class="coverglass-mask-layer"></div>
            </div>

            <div class="island-core-content" :style="coreContentStyle">

                <div class="inner-wrapper">
                    <transition mode="out-in" @enter="onInnerEnter" @leave="onInnerLeave" :css="false">
                        <div v-if="displayActivity && topActivity" class="activity-box" key="activity"
                            :style="topActivity.color ? { '--activity-accent': topActivity.color } : {}">
                            <div class="activity-avatar">
                                <img v-if="topActivity.icon" :src="topActivity.icon" alt="活动图标"
                                    class="activity-avatar-img">
                                <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" class="activity-fallback-icon">
                                    <path d="M22 12h-4l-3 9L9 3l-3 9H2" stroke-width="2" stroke-linecap="round"
                                        stroke-linejoin="round" />
                                </svg>
                            </div>
                            <div class="activity-text-wrapper">
                                <div class="activity-title">
                                    <span class="activity-name">{{ topActivity.title || '任务进行中' }}</span>
                                    <span v-if="topActivity.kind" class="activity-kind">{{ topActivity.kind }}</span>
                                </div>
                                <div class="activity-subtitle" v-if="topActivity.subtitle">{{ topActivity.subtitle }}</div>
                                <div class="activity-progress-row">
                                    <div class="activity-progress-track">
                                        <div class="activity-progress-fill"
                                            :class="{ 'is-indeterminate': topActivity.progress == null }"
                                            :style="topActivity.progress != null ? { width: topActivity.progress + '%' } : {}">
                                        </div>
                                    </div>
                                    <span v-if="topActivity.progress != null" class="activity-progress-text">{{
                                        topActivity.progress }}%</span>
                                </div>
                            </div>
                        </div>

                        <div v-else-if="isMsgActive" class="msg-box" key="msg">
                            <div class="msg-avatar">
                                <img :src="currentMsgIcon" alt="消息图标" class="msg-avatar-img">
                            </div>
                            <div class="msg-text-wrapper">
                                <div class="msg-title">
                                    <span class="sender-name">{{ msgTitle }}</span>
                                    <span class="app-name">{{ msgAppName }}</span>
                                </div>
                                <div class="msg-body">{{ msgBody }}</div>
                            </div>
                        </div>

                        <div v-else-if="displaySysToast" class="system-toast-box" key="systoast">
                            <div v-if="sysToastType === 'app'" class="toast-icon app-icon">
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                    <circle cx="12" cy="12" r="10" stroke-width="2" stroke-linecap="round"
                                        stroke-linejoin="round" opacity="0.3" />
                                    <path d="M8 12.5l3 3 5-6" stroke-width="2.5" stroke-linecap="round"
                                        stroke-linejoin="round" />
                                </svg>
                            </div>

                            <div v-else-if="sysToastType === 'lock'" class="toast-icon sys-icon">
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                    <rect x="4" y="12" width="16" height="8" rx="2" ry="2" stroke-width="2"
                                        stroke-linecap="round" stroke-linejoin="round" />
                                    <path d="M8 12V9a4 4 0 0 1 8 0v3" stroke-width="2" stroke-linecap="round"
                                        stroke-linejoin="round" />
                                </svg>
                            </div>

                            <div v-else-if="sysToastType === 'unlock'" class="toast-icon sys-icon">
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                    <rect x="4" y="12" width="16" height="8" rx="2" ry="2" stroke-width="2"
                                        stroke-linecap="round" stroke-linejoin="round" />
                                    <path d="M8 12V9a4 4 0 0 1 8 0" stroke-width="2" stroke-linecap="round"
                                        stroke-linejoin="round" />
                                </svg>
                            </div>

                            <div v-else-if="sysToastType === 'battery-charge'" class="toast-icon battery-charge-icon">
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                    <rect x="2" y="7" width="16" height="10" rx="2" ry="2" stroke-width="2"
                                        stroke-linecap="round" stroke-linejoin="round" />
                                    <line x1="22" y1="11" x2="22" y2="13" stroke-width="2" stroke-linecap="round"
                                        stroke-linejoin="round" />
                                    <polygon points="11 7 8 12 12 12 11 17 14 12 10 12 11 7" stroke-width="1.5"
                                        stroke-linejoin="round" />
                                </svg>
                            </div>

                            <div v-else-if="sysToastType === 'battery-low'" class="toast-icon battery-low-icon">
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                    <rect x="2" y="7" width="16" height="10" rx="2" ry="2" stroke-width="2"
                                        stroke-linecap="round" stroke-linejoin="round" />
                                    <line x1="22" y1="11" x2="22" y2="13" stroke-width="2" stroke-linecap="round"
                                        stroke-linejoin="round" />
                                    <line x1="6" y1="12" x2="9" y2="12" stroke-width="4" stroke-linecap="round"
                                        stroke-linejoin="round" />
                                </svg>
                            </div>

                            <div v-else class="toast-icon sys-icon">
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                    <circle cx="12" cy="12" r="10" stroke-width="2" stroke-linecap="round"
                                        stroke-linejoin="round" opacity="0.3" />
                                    <g transform="translate(6, 5.5) scale(0.5)">
                                        <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" stroke-width="4"
                                            stroke-linecap="round" stroke-linejoin="round" />
                                        <path d="M13.73 21a2 2 0 0 1-3.46 0" stroke-width="4" stroke-linecap="round"
                                            stroke-linejoin="round" />
                                    </g>
                                </svg>
                            </div>
                            <div class="toast-text">{{ sysToastText }}</div>
                        </div>

                        <div v-else-if="displayClipboard" class="clipboard-box" key="clipboard">
                            <div class="clipboard-icon">
                                <img :src="clipboardIcon" alt="剪贴板" class="clipboard-icon-img">
                            </div>
                            <div class="clipboard-text-wrapper">
                                <div class="clipboard-title">检测到复制了链接</div>
                                <div class="clipboard-link">{{ clipboardLink }}</div>
                            </div>
                            <button class="clipboard-open-btn" @click.stop="handleOpenClipboardLink" title="打开链接">
                                <img :src="openLinkIcon" alt="打开链接" class="clipboard-open-img">
                            </button>
                        </div>

                        <div v-else-if="displayMusic" class="music-ctl-box" :class="{ 'expanded': isMusicExpanded }"
                            :key="'music_' + musicBoxKey" @click="expandMusic" style="cursor: pointer;">
                            <div class="music-top-row">
                                <div class="album-cover" :class="{ 'is-playing': isPlaying }">
                                    <div class="cover-inner"
                                        :style="coverUrl ? { backgroundImage: `url(${coverUrl})`, backgroundSize: 'cover' } : {}">
                                    </div>
                                </div>
                                <div class="music-info-mask-box" ref="maskBoxRef">
                                    <div class="music-info-text single-line" :class="{ 'fade-out': isMusicExpanded }"
                                        style="position: relative; width: 100%; height: 100%;">
                                        <transition name="lyric-fade">
                                            <span class="lyric-render-text" :key="currentTrackInfo">
                                                <!-- 注意：加上了 :data-text="currentTrackInfo" -->
                                                <span class="scroll-inner" ref="textInnerRef"
                                                    :data-text="currentTrackInfo"
                                                    :class="{ 'is-scrolling': scrollDist > 0 }" :style="{
                                                        '--scroll-dist': scrollDist + 'px',
                                                        '--scroll-duration': scrollDuration,
                                                        '--scan-duration': scanDuration,
                                                        animationPlayState: isPlaying ? 'running' : 'paused'
                                                    }">
                                                    {{ currentTrackInfo }}
                                                </span>
                                            </span>
                                        </transition>
                                    </div>
                                    <div class="music-info-text double-line" :class="{ 'fade-in': isMusicExpanded }">
                                        <div class="song-title" ref="expandedTitleBoxRef">
                                            <span class="scroll-inner" ref="expandedTitleRef"
                                                :class="{ 'is-scrolling': expandedTitleScrollDist > 0 }"
                                                :style="{ '--scroll-dist': expandedTitleScrollDist + 'px', '--scroll-duration': expandedTitleScrollDuration }">
                                                {{ currentSongName }}
                                            </span>
                                        </div>
                                        <div class="song-artist" v-show="!isVideoPlayer">{{ currentArtistName }}
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <transition name="fade">
                                <div class="music-expanded-bottom" v-show="isMusicExpanded">

                                    <div class="progress-container">
                                        <span class="time-text">{{ formattedCurrentTime }}</span>
                                        <div class="progress-track">
                                            <div class="progress-fill" :style="{ width: progressPercent + '%' }"></div>
                                        </div>
                                        <span class="time-text">{{ formattedTotalTime }}</span>
                                    </div>

                                    <div class="music-controls">
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

                                </div>
                            </transition>
                        </div>

                        <div v-else-if="displayResource" class="resource-box" key="resource">
                            <div class="res-group">
                                <div class="res-info-row">
                                    <span class="res-label">CPU</span>
                                    <span class="res-value" :class="{ 'high-usage': cpuUsage >= 85 }">{{ cpuUsage
                                        }}%</span>
                                </div>
                                <div class="res-bar-track">
                                    <div class="res-bar-fill" :style="{ width: cpuUsage + '%' }"
                                        :class="{ 'high-usage': cpuUsage >= 85 }"></div>
                                </div>
                            </div>
                            <div class="res-group">
                                <div class="res-info-row">
                                    <span class="res-label">RAM</span>
                                    <span class="res-value" :class="{ 'high-usage': ramUsage >= 85 }">{{ ramUsage
                                        }}%</span>
                                </div>
                                <div class="res-bar-track">
                                    <div class="res-bar-fill" :style="{ width: ramUsage + '%' }"
                                        :class="{ 'high-usage': ramUsage >= 85 }"></div>
                                </div>
                            </div>
                        </div>

                        <div v-else-if="displaySpeed" class="speed-box" key="speed">
                            <Transition name="speed-fade" mode="out-in">
                                <div v-if="nsdBaseWidth >= 230" key="dual" class="speed-dual-box">
                                    <div class="speed-item">
                                        <span :class="['label', { 'high-traffic': isHighUpload }]">⬆</span>
                                        <span class="value">{{ uploadSpeed }}</span>
                                    </div>
                                    <div class="speed-item">
                                        <span :class="['label', { 'high-traffic': isHighDownload }]">⬇</span>
                                        <span class="value">{{ downloadSpeed }}</span>
                                    </div>
                                </div>

                                <div v-else key="single" class="speed-single-box">
                                    <Transition name="speed-fade" mode="out-in">
                                        <div v-if="isShowingUpload" class="speed-item" key="upload">
                                            <span :class="['label', { 'high-traffic': isHighUpload }]">⬆</span>
                                            <span class="value">{{ uploadSpeed }}</span>
                                        </div>
                                        <div v-else class="speed-item" key="download">
                                            <span :class="['label', { 'high-traffic': isHighDownload }]">⬇</span>
                                            <span class="value">{{ downloadSpeed }}</span>
                                        </div>
                                    </Transition>
                                </div>
                            </Transition>
                        </div>

                        <div v-else-if="displayFps" class="speed-box" key="fps">
                            <div class="speed-single-box">
                                <div class="speed-item">
                                    <span class="label">FPS</span>
                                    <span class="value">{{ currentFps }}</span>
                                </div>
                            </div>
                        </div>

                        <div v-else-if="displayCustom" class="custom-display-box" key="custom">
                            <template v-for="(slot, index) in customSlots" :key="'custom' + index">
                                <div v-if="slot" :class="['custom-slot-item', `is-${slot}`]">

                                    <template v-if="slot === 'speed'">
                                        <div class="custom-data-row">
                                            <span :class="['custom-label', { 'high-traffic': isHighUpload }]">⬆</span>
                                            <span class="custom-value">{{ uploadSpeed }}</span>
                                        </div>
                                        <div class="custom-data-row">
                                            <span :class="['custom-label', { 'high-traffic': isHighDownload }]">⬇</span>
                                            <span class="custom-value">{{ downloadSpeed }}</span>
                                        </div>
                                    </template>

                                    <template v-else-if="slot === 'resource'">
                                        <div class="custom-data-row">
                                            <span class="custom-label">CPU</span>
                                            <span class="custom-value" :class="{ 'high-usage': cpuUsage >= 85 }">{{
                                                cpuUsage }}%</span>
                                        </div>
                                        <div class="custom-data-row">
                                            <span class="custom-label">RAM</span>
                                            <span class="custom-value" :class="{ 'high-usage': ramUsage >= 85 }">{{
                                                ramUsage }}%</span>
                                        </div>
                                    </template>

                                    <template v-else-if="slot === 'fps'">
                                        <div class="custom-data-row justify-center">
                                            <span class="custom-label">FPS</span>
                                        </div>
                                        <div class="custom-data-row justify-center">
                                            <span class="custom-value fps-large">{{ currentFps }}</span>
                                        </div>
                                    </template>

                                    <template v-else-if="slot === 'cover'">
                                        <div class="custom-cover-inner"
                                            :style="coverUrl ? { backgroundImage: `url(${coverUrl})` } : {}"></div>
                                    </template>

                                </div>
                                <div v-else class="custom-slot-empty"></div>
                            </template>
                        </div>
                    </transition>
                </div>

                <transition mode="out-in" @enter="onInnerEnter" @leave="onInnerLeave" :css="false">
                    <div v-if="showSpectrumIndicator" class="audio-spectrum"
                        :class="{ 'is-playing': isPlaying, 'expanded': isMusicExpanded }" key="spectrum">
                        <span class="bar" v-for="(val, index) in spectrumData" :key="index"
                            :style="{ transform: `scaleY(${val})` }"></span>
                    </div>

                    <div v-else :class="['status-dot', networkStatus]" key="dot"></div>
                </transition>
            </div>
        </div>
    </transition>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed, watch, nextTick, type CSSProperties } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow, currentMonitor, availableMonitors, PhysicalPosition, LogicalPosition, PhysicalSize, cursorPosition } from '@tauri-apps/api/window'; import { Menu, MenuItem } from '@tauri-apps/api/menu';
import { listen, emit } from '@tauri-apps/api/event';
import { openUrl } from '@tauri-apps/plugin-opener';
import { t, currentLanguage, type AppLanguage } from '../i18n';

const isIslandVisible = ref(false);
const isMenuOpen = ref(false);

// 形变请求序号：用于串行化 animateIslandSize，保证「最新一次请求」永远接管动画，
// 避免旧动画（如正在进行的收缩）在异步读取窗口尺寸后覆盖新状态（如消息展开）。
// 声明必须早于下方 watch(isIslandVisible)（immediate 回调会用到，存在 TDZ）。
let latestAnimationRequest = 0;

// 岛隐藏（窗口缩成 1×1）前记录的物理中心点 { cx, y }：
// 恢复显示时按「保中心」重定位，避免隐藏前后态宽不同（如音乐态 260px → 网速态 150px）导致整体偏移。
let islandHiddenCenter: { cx: number; y: number } | null = null;

// 位置锁定：开启后禁止拖拽灵动岛（右键菜单切换，状态持久化）
const isPositionLocked = ref(localStorage.getItem('nsd_position_locked') === 'true');

// 锁定状态变化时同步托盘菜单勾选（含右键岛菜单/托盘菜单两条切换路径）
watch(isPositionLocked, (val) => invoke('sync_tray_menu', { lock: val }));

// ==================== 岛位置持久化 ====================
// 拖拽结束后保存左上角物理坐标，重启时优先恢复；重置位置时清除。
const SAVED_POS_KEY = 'nsd_island_pos';

const saveIslandPosition = async () => {
    try {
        const [pos, size] = await Promise.all([
            getCurrentWindow().outerPosition(),
            getCurrentWindow().outerSize()
        ]);
        localStorage.setItem(SAVED_POS_KEY, JSON.stringify({ x: pos.x, y: pos.y, w: size.width, h: size.height }));
    } catch (e) {
        console.error('保存岛位置失败:', e);
    }
};

// 恢复已保存的位置；坐标无效（显示器拔掉/分辨率变化导致落在所有现存屏幕外）时清除并返回 false
const restoreSavedPosition = async (): Promise<boolean> => {
    try {
        const raw = localStorage.getItem(SAVED_POS_KEY);
        if (!raw) return false;
        const saved = JSON.parse(raw);
        if (typeof saved?.x !== 'number' || typeof saved?.y !== 'number') return false;

        // 有效性检查：保存的左上角必须落在某一块现存显示器的物理范围内
        const monitors = await availableMonitors();
        const valid = monitors.some(m =>
            saved.x >= m.position.x - 8 && saved.x <= m.position.x + m.size.width - 32 &&
            saved.y >= m.position.y - 8 && saved.y <= m.position.y + m.size.height - 8
        );
        if (!valid) {
            localStorage.removeItem(SAVED_POS_KEY);
            return false;
        }
        await getCurrentWindow().setPosition(new PhysicalPosition(saved.x, saved.y));
        return true;
    } catch {
        return false;
    }
};

// 监听灵动岛显隐变化，同步状态给控制台
watch(isIslandVisible, (newVal) => {
    emit('island-status-sync', { visible: newVal });
});

// 兜底保险：OS 窗口显隐必须严格跟随灵动岛状态。
// 关闭 → 立即开启鼠标透传（点击穿透，不拦截下层窗口）；
//         作废在途形变动画（防止其收尾 SetWindowPos 把窗口重新撑大）；
//         记录当前中心点；等离开动画结束后把窗口物理缩成 1×1，彻底不可点击；
// 开启 → 关闭透传并恢复窗口到正常尺寸，再按隐藏前中心点「保中心」重定位
//         （隐藏期间态宽可能变化，setSize 不动左上角会导致整体偏移）。
watch(isIslandVisible, (visible) => {
    const appWindow = getCurrentWindow();
    appWindow.setIgnoreCursorEvents(!visible).catch(() => { });

    if (visible) {
        // 呼出时先把窗口恢复到应有的物理尺寸（getBaseSize/appScale 此时早已初始化）
        const { w, h } = getBaseSize();
        const scaleFactor = window.devicePixelRatio || 1;
        const physW = Math.ceil(w * appScale.value * scaleFactor);
        const physH = Math.ceil(h * appScale.value * scaleFactor);
        appWindow.setSize(new PhysicalSize(physW, physH)).then(async () => {
            // 保中心恢复：按隐藏前记录的中心点重算左上角，宽度变化也不再偏移
            if (islandHiddenCenter) {
                const { cx, y } = islandHiddenCenter;
                islandHiddenCenter = null;
                try {
                    await appWindow.setPosition(new PhysicalPosition(
                        Math.round(cx - physW / 2),
                        y
                    ));
                } catch (e) { }
            }
        }).catch(() => { });
        return;
    }

    // 关闭时：先作废所有在途形变动画，防止动画收尾的 SetWindowPos 与下面的 1×1 缩放竞态
    latestAnimationRequest++;
    // 记录当前中心点（窗口此刻仍是正常尺寸），供恢复时保中心重定位
    Promise.all([appWindow.innerPosition(), appWindow.innerSize()]).then(([pos, size]) => {
        if (!isIslandVisible.value && size.width > 2 && size.height > 2) {
            islandHiddenCenter = { cx: pos.x + Math.round(size.width / 2), y: pos.y };
        }
    }).catch(() => { });

    // 关闭时等离开动画播完（约 350ms）再缩成 1×1；期间若又被呼出则放弃缩放
    setTimeout(() => {
        if (!isIslandVisible.value) {
            appWindow.setSize(new PhysicalSize(1, 1)).catch(() => { });
        }
    }, 400);
}, { immediate: true });

// 记录全屏自动隐藏开关状态
const isAutoHideEnabled = ref(localStorage.getItem('nsd_autohide_fs') === 'true');
// 全屏隐藏后是否允许“鼠标悬停原位唤出”（默认关闭，需在设置中手动开启）
const fsHoverWakeEnabled = ref(localStorage.getItem('nsd_autohide_fs_hover') === 'true');
// 记录进入全屏前的灵动岛显隐状态，用来决定退回桌面时要不要恢复
let wasVisibleBeforeFullscreen = false;

// ==================== 全屏“悬停唤起”机制 ====================
// 开启“全屏自动隐藏”后，进入全屏时灵动岛会收起。为便于随时查看，
// 收起期间把窗口缩成 1×1 的常驻窗口（配合鼠标透传，视觉不可见且不拦截点击），
// 并按固定频率轮询全局光标位置：光标回到岛的原位置时恢复显示，
// 光标离开该位置超过 FS_HOVER_LEAVE_MS 后再收起，等待下一次唤起。
let fsHoverActive = false; // 是否处于“已收起、等待悬停唤起”状态
let fsHoverSlot: { x: number; y: number; w: number; h: number } | null = null; // 隐藏前记录的岛体物理区域，作为唤起探测区
let lastRealFsHoverSlot: { x: number; y: number; w: number; h: number } | null = null; // 最近一次取得的真实探测区（停止逻辑不会清空它），供 1×1 待命态下重新开启悬停时定位
let fsHoverPollTimer: number | null = null; // 光标轮询定时器句柄
let fsHoverHideTimer: number | null = null; // 光标离开后的收起倒计时句柄
let fsHoverAwaitExit = false; // 收起瞬间光标可能仍停在原位，需先离开一次才能再次唤起，避免进入全屏立即弹出
let fsHoverTicking = false; // 轮询去重：tick 内含 await，防止并发重叠
const FS_HOVER_PAD = 18; // 唤起触发区相对岛体四边的外扩像素，减少边缘抖动引起的反复收起/唤出
const FS_HOVER_POLL_MS = 120; // 光标轮询间隔（毫秒）
const FS_HOVER_LEAVE_MS = 1000; // 光标离开触发区后再次收起的延迟（毫秒）

// 记录岛体当前在屏幕上占用的物理区域（隐藏前记录；岛可见期间随位置/尺寸变化实时刷新）。
// 读取窗口内边距坐标/尺寸失败时，依次回退到外边距坐标/尺寸；
// 若仍不可用（例如窗口处于 1×1 隐藏态），先使用最近一次真实区域，
// 最终按“个性化尺寸 + 所在显示器”反推岛默认贴靠位置（显示器顶部居中）作为唤起区。
const captureFsHoverSlot = async () => {
    const appWindow = getCurrentWindow();
    let x: number | null = null, y: number | null = null, w: number | null = null, h: number | null = null;
    try {
        const [pos, size] = await Promise.all([appWindow.innerPosition(), appWindow.innerSize()]);
        x = pos.x; y = pos.y; w = size.width; h = size.height;
    } catch (e1) {
        console.error('[fsHover] innerPosition/innerSize 失败，尝试 outer*', e1);
        try {
            const [pos, size] = await Promise.all([appWindow.outerPosition(), appWindow.outerSize()]);
            x = pos.x; y = pos.y; w = size.width; h = size.height;
        } catch (e2) {
            console.error('[fsHover] innerPosition/outerPosition 均失败，将改用最近真实区域或合成区域', e2);
        }
    }
    // 宽高 > 2px 才视为有效区域并记录；尺寸异常（如 1×1 隐藏态）时，
    // 优先复用最近一次真实区域，否则进入下方合成区域
    if (x != null && y != null && w != null && h != null && w > 2 && h > 2) {
        const rect = { x, y, w, h };
        fsHoverSlot = rect;
        lastRealFsHoverSlot = rect;
        return;
    }
    if (lastRealFsHoverSlot) {
        fsHoverSlot = lastRealFsHoverSlot;
        return;
    }
    try {
        // 合成区域：与 adjustWindowPosition 使用相同定位公式 —— 岛默认贴靠所在显示器顶部居中
        let monitor = await currentMonitor();
        if (!monitor) {
            const monitors = await availableMonitors();
            if (monitors.length > 0) monitor = monitors[0];
        }
        if (!monitor) { fsHoverSlot = null; return; }
        const scaleFactor = monitor.scaleFactor;
        const { w: lw, h: lh } = getBaseSize();
        const pw = Math.round(lw * appScale.value * scaleFactor);
        const ph = Math.round(lh * appScale.value * scaleFactor);
        const px = monitor.position.x + Math.round((monitor.size.width - pw) / 2);
        const py = monitor.position.y + Math.round(12 * scaleFactor);
        fsHoverSlot = { x: px, y: py, w: pw, h: ph };
    } catch (e3) {
        console.error('[fsHover] 合成唤醒区失败', e3);
        fsHoverSlot = null;
    }
};

// 判断光标是否处于唤醒/保持区域
const isCursorInsideFsHoverZone = async (rect: { x: number; y: number; w: number; h: number }) => {
    try {
        const cur = await cursorPosition();
        const pad = Math.round(FS_HOVER_PAD * (window.devicePixelRatio || 1));
        return cur.x >= rect.x - pad && cur.x <= rect.x + rect.w + pad
            && cur.y >= rect.y - pad && cur.y <= rect.y + rect.h + pad;
    } catch {
        return false;
    }
};

// 悬停唤起：仅切换 v-show 无法让岛显示——收起时窗口已被缩成 1×1，
// 且可能被全屏应用压在下方。因此先按“个性化设置”的岛尺寸恢复窗口大小，
// 再显示并置顶窗口，最后触发入场动画让内容出现。
const raiseFsHoverIsland = async () => {
    const appWindow = getCurrentWindow();
    // 1. 按个性化设置恢复窗口尺寸：物理像素 = 逻辑宽高(getBaseSize) × 应用缩放(appScale) × 系统缩放(DPR)
    const { w, h } = getBaseSize();
    const scaleFactor = window.devicePixelRatio || 1;
    try {
        // 仅恢复尺寸即可：缩成 1×1 时窗口左上角位置保持不变，原显示位置自动保留
        await appWindow.setSize(new PhysicalSize(
            Math.ceil(w * appScale.value * scaleFactor),
            Math.ceil(h * appScale.value * scaleFactor)
        ));
    } catch (e) { console.error('[fsHover] 恢复尺寸失败', e); }
    // 2. 显示窗口并重新置顶（待命的 1×1 窗口可能被全屏应用盖在底层）
    try {
        await invoke('show_window_no_activate', { label: 'widget' });
        await appWindow.setAlwaysOnTop(true);
        // 置为“不可激活”：点击岛不抢占焦点，前台始终留给全屏应用，避免任务栏弹出
        await invoke('set_window_no_activate', { label: 'widget', enabled: true });
    } catch (e) { console.error('[fsHover] 置顶失败', e); }
    // 3. 等待 40ms 让透明窗口完成挂载，再切换 v-show 触发入场动画（与手动亮岛流程一致，防止闪烁）
    await new Promise((resolve) => setTimeout(resolve, 40));
    if (fsHoverActive && !isIslandVisible.value) {
        isIslandVisible.value = true;
    }
};

// 悬停唤起主循环：每次 tick 读取一次全局光标位置，据此决定唤起或收起
const fsHoverTick = async () => {
    if (!fsHoverActive || fsHoverTicking) return;
    fsHoverTicking = true;
    try {
        // 岛可见时用实时窗口区域探测（跟随展开/拖拽变化）；岛隐藏时使用隐藏前记录的探测区
        if (isIslandVisible.value) {
            await captureFsHoverSlot();
        }
        const rect = fsHoverSlot;
        if (!rect) return;

        const inside = await isCursorInsideFsHoverZone(rect);

        if (inside) {
            // 光标在收起后从未离开过原位，本次先不唤起，等待它先离开一次
            if (fsHoverAwaitExit) return;
            // 光标位于探测区内：取消未触发的收起倒计时；岛已收起则按个性化尺寸恢复并显示
            if (fsHoverHideTimer) {
                clearTimeout(fsHoverHideTimer);
                fsHoverHideTimer = null;
            }
            if (!isIslandVisible.value) {
                await raiseFsHoverIsland();
            }
        } else {
            fsHoverAwaitExit = false; // 光标已离开过原位，下次进入即可唤起
            // 光标已离开探测区且岛仍可见 → 启动收起倒计时（FS_HOVER_LEAVE_MS 后执行）
            if (isIslandVisible.value && fsHoverHideTimer == null) {
                fsHoverHideTimer = window.setTimeout(() => {
                    fsHoverHideTimer = null;
                    if (fsHoverActive && isIslandVisible.value) {
                        isIslandVisible.value = false; // watch 会开启鼠标透传并缩回 1×1，回到待命状态
                    }
                }, FS_HOVER_LEAVE_MS);
            }
        }
    } finally {
        fsHoverTicking = false;
    }
};

// 进入“收起 + 悬停唤起”待命状态：隐藏岛体并开始轮询光标
const startFsHoverMode = async () => {
    if (fsHoverActive) return;
    fsHoverActive = true;
    await captureFsHoverSlot(); // 必须在缩成 1×1 之前记录当前探测区域

    // 收起岛体（保留正常离场动画；不隐藏窗口，仅保持“可见”，避免 WebView 后台定时器被系统节流）
    fsHoverAwaitExit = true;
    isIslandVisible.value = false;

    if (fsHoverHideTimer) { clearTimeout(fsHoverHideTimer); fsHoverHideTimer = null; }
    // 防御性清理：若残留旧轮询定时器（正常情况下 stopFsHoverMode 会将其置空），先清除再重建，避免产生重叠定时器
    if (fsHoverPollTimer != null) {
        clearInterval(fsHoverPollTimer);
        fsHoverPollTimer = null;
    }
    fsHoverPollTimer = window.setInterval(fsHoverTick, FS_HOVER_POLL_MS);
};

// 退出“收起 + 悬停唤起”待命状态：停止轮询并清理相关定时器
const stopFsHoverMode = () => {
    fsHoverActive = false;
    if (fsHoverPollTimer != null) {
        clearInterval(fsHoverPollTimer);
        fsHoverPollTimer = null;
    }
    if (fsHoverHideTimer != null) {
        clearTimeout(fsHoverHideTimer);
        fsHoverHideTimer = null;
    }
    fsHoverSlot = null;
    // 解除“不可激活”样式，恢复桌面上的正常点击/聚焦行为
    invoke('set_window_no_activate', { label: 'widget', enabled: false }).catch(() => { });
};

// 记录当前是否显示上行网速（用于轮换）
const isShowingUpload = ref(false);
let speedCycleTimer: number | null = null;

// 控制 DOM 真正的高宽变量与消息数据
const currentWidth = ref(150);
const currentHeight = ref(34);
const isMsgActive = ref(false);
const msgTitle = ref('');
const msgAppName = ref('');
const msgBody = ref('');
const msgAumid = ref('');

// ==================== 活动池（外部服务经 47300 HTTP 推送，Rust 30Hz 节流快照） ====================
interface ActivityData {
    id: string;
    title: string;
    subtitle: string;
    kind: string;
    icon: string;
    color: string;
    progress: number | null;
    priority: number;
    remaining_ms: number | null;
    extra: unknown;
}

// 快照已按 (priority, updated) 排好序，第一个即当前应展示的活动
const activityPool = ref<ActivityData[]>([]);
const topActivity = computed<ActivityData | null>(() => activityPool.value.length > 0 ? activityPool.value[0] : null);
const displayActivity = computed(() => !!topActivity.value);

let stopActivityPoolListener: (() => void) | null = null;
const startActivityPoolListening = async () => {
    if (stopActivityPoolListener) return;
    stopActivityPoolListener = await listen<{ ts: number, activities: ActivityData[] }>('activity-pool', (event) => {
        // 只替换一次引用：同 id 活动仅字段变化时不会触发 displayActivity 翻转
        activityPool.value = Array.isArray(event.payload?.activities) ? event.payload.activities : [];
    });
};
const stopActivityPoolListening = () => {
    if (stopActivityPoolListener) {
        stopActivityPoolListener();
        stopActivityPoolListener = null;
    }
};

// 活动出现 → 立即接管岛体（顶掉消息 / 音乐展开态）；
// 活动结束 → 显式收起回当前基础内容尺寸（displayXxx 不感知活动，无法靠内容切换 watch 触发）
watch(displayActivity, (showing) => {
    if (!showing) {
        const { w, h } = getBaseSize();
        animateIslandSize(w, h);
        return;
    }
    isMsgActive.value = false;
    if ((window as any).msgTimer) {
        clearTimeout((window as any).msgTimer);
        (window as any).msgTimer = null;
    }
    isMusicExpanded.value = false;
    isMusicExpanding.value = false;
    if (musicExpandAnimTimer) {
        clearTimeout(musicExpandAnimTimer);
        musicExpandAnimTimer = null;
    }
    animateIslandSize(Math.max(nsdMsgExpandedWidth.value, 320), 70);
});

// 跟踪底层是否有真实的媒体活动
const isMediaActive = ref(true); // 默认 true，交给首次轮询决定去留
let isFirstMediaCheck = true;    // 标记首次检查，防止开机启动时乱弹窗
let isNewlyEnabled = false;

// 系统操作通知专用变量
const displaySysToast = ref(false);
const sysToastText = ref('');
const sysToastType = ref<'app' | 'sys' | 'battery-charge' | 'battery-low' | 'lock' | 'unlock'>('app');
const toastQueue = ref<{ text: string, type: 'app' | 'sys' | 'battery-charge' | 'battery-low' | 'lock' | 'unlock' }[]>([]);
let isProcessingToast = false;

// 队列处理函数
const processToastQueue = async () => {
    // 正在处理或队列为空时直接返回
    if (isProcessingToast || toastQueue.value.length === 0) return;

    // 优先级判断：如果当前正在显示消息通知(最高优先级)，则挂起等待
    if (isMsgActive.value || displayActivity.value) return;

    isProcessingToast = true;
    const nextToast = toastQueue.value.shift();

    if (nextToast) {
        sysToastText.value = nextToast.text;
        sysToastType.value = nextToast.type;
        displaySysToast.value = true;

        // 停留显示
        await new Promise(resolve => setTimeout(resolve, 2000));

        displaySysToast.value = false;
        // 等待离开动画播完 (约 200ms) 再处理下一个
        await new Promise(resolve => setTimeout(resolve, 200));
    }

    isProcessingToast = false;
    processToastQueue(); // 递归检查是否还有下一个通知
};

// 监听系统通知显示状态，解决网速模式下尺寸过小导致文字溢出/遮挡指示灯的问题
watch(displaySysToast, (newVal) => {
    if (newVal) {
        // 当有系统操作通知出现时，强制展开到默认标准尺寸
        animateIslandSize(260, 42);
    } else {
        // 通知消失时，恢复到当前状态该有的尺寸
        // （前提是没有被应用消息或音乐面板占用）
        if (!isMsgActive.value && !displayActivity.value && !isMusicExpanded.value && !isMusicExpanding.value) {
            const { w, h } = getBaseSize();
            animateIslandSize(w, h);
        }
    }
});

// 暴露给外部调用的触发函数
const showToast = (text: string, type: 'app' | 'sys' | 'battery-charge' | 'battery-low' | 'lock' | 'unlock' = 'app') => {
    toastQueue.value.push({ text, type });
    processToastQueue();
};

// 监听消息通知状态，消息通知消失时唤醒可能被挂起的操作通知队列
watch(isMsgActive, (newVal) => {
    if (!newVal) {
        processToastQueue();
    }
});

// ==================== 剪贴板链接通知 ====================
const displayClipboard = ref(false);
const clipboardLink = ref('');
let clipboardHideTimer: ReturnType<typeof setTimeout> | null = null;
// 剪贴板读取开关（默认开启）
const enableClipboard = ref(localStorage.getItem('nsd_clipboard') !== 'false');

// 打开剪贴板中的链接（调用系统默认浏览器）
const handleOpenClipboardLink = async () => {
    if (!clipboardLink.value) return;
    try {
        await openUrl(clipboardLink.value);
    } catch (err) {
        console.error('打开链接失败:', err);
    }
    // 打开后关闭通知
    displayClipboard.value = false;
    if (clipboardHideTimer) clearTimeout(clipboardHideTimer);
    if (!isMsgActive.value && !displaySysToast.value && !isMusicExpanded.value && !isMusicExpanding.value) {
        const { w, h } = getBaseSize();
        animateIslandSize(w, h);
    }
};

// 每次复制发生时读取剪贴板，若内容是 http/https 链接则弹出卡片
const pollClipboard = async () => {
    try {
        const text = await invoke<string>('get_clipboard_text');
        if (!text) return;

        // 提取第一个 http/https 链接
        const m = text.match(/https?:\/\/[^\s]+/);
        if (!m) return;

        const link = m[0].replace(/[.,;,，。]$/, '');

        clipboardLink.value = link;
        displayClipboard.value = true;
        // 若消息通知此刻正在占用，等它消失后再让出尺寸（见下方 watch）
        if (!isMsgActive.value && !displayActivity.value) {
            animateIslandSize(Math.max(nsdMsgExpandedWidth.value, 320), 70);
        }

        if (clipboardHideTimer) clearTimeout(clipboardHideTimer);
        clipboardHideTimer = setTimeout(() => {
            displayClipboard.value = false;
            if (!isMsgActive.value && !displayActivity.value && !displaySysToast.value && !isMusicExpanded.value && !isMusicExpanding.value) {
                const { w, h } = getBaseSize();
                animateIslandSize(w, h);
            }
        }, 5000);
    } catch (err) {
        // 剪贴板读取失败时静默忽略
        console.error(err);
    }
};

// 剪贴板通知挂起时，等消息消失后补上尺寸
watch(isMsgActive, (newVal) => {
    if (!newVal && displayClipboard.value) {
        animateIslandSize(Math.max(nsdMsgExpandedWidth.value, 320), 70);
    }
});

// 启动剪贴板监听：由后端在复制操作时通过事件驱动，无需轮询
let stopClipboardListener: (() => void) | null = null;
const startClipboardPolling = async () => {
    if (stopClipboardListener) return;
    stopClipboardListener = await listen('clipboard-changed', () => {
        // 复制发生时再去读剪贴板并检测链接
        pollClipboard();
    });
};

// 停止剪贴板监听（组件卸载时调用）
const stopClipboardPolling = () => {
    if (stopClipboardListener) {
        stopClipboardListener();
        stopClipboardListener = null;
    }
    if (clipboardHideTimer) {
        clearTimeout(clipboardHideTimer);
        clipboardHideTimer = null;
    }
    // 关闭开关时同步收起正在显示的剪贴板卡片
    if (displayClipboard.value) {
        displayClipboard.value = false;
        if (!isMsgActive.value && !displayActivity.value && !displaySysToast.value && !isMusicExpanded.value && !isMusicExpanding.value) {
            const { w, h } = getBaseSize();
            animateIslandSize(w, h);
        }
    }
};

// 记录音乐岛是否处于展开状态
const isMusicExpanded = ref(false);
const isMusicExpanding = ref(false); // 记录是否正在播放弹性按压展开动画
let musicExpandAnimTimer: number | null = null; // 用于接管展开时的定时器，防止冲突

// 灵动岛自身的透明度变量（默认100）
const islandOpacity = ref(Number(localStorage.getItem('nsd_island_opacity') || '100'));

// 灵动岛自身主题色
const islandTheme = ref(localStorage.getItem('nsd_island_theme') || 'black');

// 个性化中心绑定状态
const nsdBaseWidth = ref(Number(localStorage.getItem('nsd_base_width')) || 150);
const nsdBaseHeight = ref(Number(localStorage.getItem('nsd_base_height')) || 34);
const nsdMusicBaseWidth = ref(Number(localStorage.getItem('nsd_music_base_width')) || 260);
const nsdMusicExpandedWidth = ref(Number(localStorage.getItem('nsd_music_expanded_width')) || 320);
const nsdMsgExpandedWidth = ref(Number(localStorage.getItem('nsd_msg_expanded_width')) || 360);
const nsdBorderRadius = ref(Number(localStorage.getItem('nsd_border_radius')) || 100);
const nsdSpringStyle = ref(localStorage.getItem('nsd_spring_style') || 'bouncy');
const nsdLyricDelay = ref(Number(localStorage.getItem('nsd_lyric_delay')) || 0);

// WS 歌词专属额外延迟（毫秒），调谐时只改这里
const WS_LYRIC_DELAY_MS = 500;

// 1. 判定当前是否处于大窗口状态
const isExpandedSize = computed(() => isMusicExpanded.value || isMsgActive.value || displayActivity.value);

// 2. 外层容器：状态一变，立马切成目标圆角
const islandStyle = computed<CSSProperties>(() => {
    const linear = islandOpacity.value / 100;
    const alpha = Math.pow(linear, 1 / 2.2);
    let bg = `rgba(0, 0, 0, ${alpha})`;
    let color = '#ffffff';

    if (islandTheme.value === 'white') {
        bg = `rgba(255, 255, 255, ${alpha})`;
        color = '#000000';
    } else if (showCoverglassBg.value) {
        // coverglass 主题（沉浸背景启用）时，外层使用深色半透明底色，实际画面由沉浸背景层提供
        bg = `rgba(20, 20, 20, ${alpha})`;
    }

    return {
        backgroundColor: bg,
        color: color,
        width: '100%',
        height: '100%',
        borderRadius: isExpandedSize.value ? '24px' : `${nsdBorderRadius.value}px`,
        position: 'relative',
    };
});

// 3. 内层核心：永远比外层小 2px
const coreContentStyle = computed(() => {
    const linear = islandOpacity.value / 100;
    const alpha = Math.pow(linear, 1 / 2.2);
    const innerRadiusValue = Math.max(nsdBorderRadius.value - 2, 8);
    const innerRadius = isExpandedSize.value ? '22px' : `${innerRadiusValue}px`;

    if (islandTheme.value === 'white') {
        return { backgroundColor: `rgba(255, 255, 255, ${alpha})`, borderRadius: innerRadius };
    } else if (showCoverglassBg.value) {
        // coverglass 主题时内层透明，让沉浸背景层直接透出
        return { backgroundColor: `transparent`, borderRadius: innerRadius };
    }
    return { backgroundColor: `rgba(0, 0, 0, ${alpha})`, borderRadius: innerRadius };
});

// 4. 沉浸模式背景层：智能规避黑边与遮挡，并绑定不透明度
const coverglassStyle = computed<CSSProperties>(() => {
    // 将控制台传来的透明度转换为视觉 alpha 值（gamma 校正）
    const linear = islandOpacity.value / 100;
    const alpha = Math.pow(linear, 1 / 2.2);

    if (isGlowBorderEnabled.value) {
        // 流光边框开启时：内缩 2px 给边框让路，并匹配内层圆角
        const innerRadiusValue = Math.max(nsdBorderRadius.value - 2, 8);
        return {
            top: '2px', left: '2px', right: '2px', bottom: '2px',
            borderRadius: isExpandedSize.value ? '22px' : `${innerRadiusValue}px`,
            opacity: alpha // 将透明度应用到沉浸背景层
        };
    }
    // 流光边框关闭时：铺满整个灵动岛，并匹配外层大圆角
    return {
        top: '0', left: '0', right: '0', bottom: '0',
        borderRadius: isExpandedSize.value ? '24px' : `${nsdBorderRadius.value}px`,
        opacity: alpha // 将透明度应用到沉浸背景层
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

// 音乐控制功能开关
const isMusicCtlEnabled = ref(localStorage.getItem('nsd_music_ctrl') === 'true');
const isPlaying = ref(false);
// 歌词显示
const parsedLyrics = ref<{ time: number; text: string }[]>([]);
const currentBaseInfo = ref(''); // 无歌词时兜底显示 "歌名 - 歌手"
// 歌词时间推算专用变量
const localPositionMs = ref(0);
let lastTickTime = performance.now();
const currentDurationMs = ref(0);
// 将毫秒转换为 mm:ss 格式
const formatTime = (ms: number) => {
    if (!ms || ms < 0) return '00:00';
    const totalSeconds = Math.floor(ms / 1000);
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;
    return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
};
// 进度条百分比 (限制在 0-100 之间)
const progressPercent = computed(() => {
    if (currentDurationMs.value <= 0) return 0;
    const percent = (localPositionMs.value / currentDurationMs.value) * 100;
    return Math.min(Math.max(percent, 0), 100);
});
const formattedCurrentTime = computed(() => formatTime(localPositionMs.value));
const formattedTotalTime = computed(() => formatTime(currentDurationMs.value));
// 歌词防吞字与队列控制
const lyricQueue = ref<string[]>([]);
let lastLyricChangeTime = 0;
let currentMatchedIndex = -1;

// 简单的 LRC 解析器
const parseLrc = (lrcStr: string) => {
    const lines = lrcStr.split('\n');
    const result: { time: number; text: string }[] = [];
    const timeReg = /\[(\d{2}):(\d{2})\.(\d{2,3})\]/;

    for (const line of lines) {
        const match = timeReg.exec(line);
        if (match) {
            const min = parseInt(match[1]);
            const sec = parseInt(match[2]);
            const msStr = match[3].length === 2 ? match[3] + '0' : match[3];
            const ms = parseInt(msStr);
            const time = min * 60000 + sec * 1000 + ms;
            const text = line.replace(timeReg, '').trim();

            // 过滤掉只有全角空格、零宽字符的“幽灵歌词”
            const realText = text.replace(/[\s\u200B-\u200D\uFEFF\u3000]/g, '');

            if (realText.length > 0 && !text.includes('纯音乐') && text !== 'lrc' && text !== '//') {
                result.push({ time, text });
            }
        }
    }
    return result.sort((a, b) => a.time - b.time);
};

// 流光边框默认状态完全镜像音乐控制器（只要音乐控制器开着它就开，关了就一起关）
const isGlowBorderEnabled = ref(localStorage.getItem('nsd_glow_border') === 'true');
// 监听流光边框状态变化，同步托盘菜单的勾选状态
watch(isGlowBorderEnabled, (val) => invoke('sync_tray_menu', { glow: val }));

// 律动频谱
const spectrumData = ref([0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35]);
let spectrumTimer: number;

// 播放中但没有声音（静音/无声视频）时的拟真频谱：让指示器保持律动，而不是瘫成一条线
const SILENCE_THRESHOLD = 0.12; // 7 柱全部低于该值视为"没有声音"
let fakeSpectrum = [0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35];
let fakeSpectrumTargets = [...fakeSpectrum];
let fakeSpectrumTargetTs = 0;
const generateFakeSpectrum = (): number[] => {
    const now = Date.now();
    // 每 250ms 换一批随机目标，模拟频段起伏
    if (now - fakeSpectrumTargetTs > 250) {
        fakeSpectrumTargetTs = now;
        for (let i = 0; i < fakeSpectrumTargets.length; i++) {
            fakeSpectrumTargets[i] = 0.3 + Math.random() * 0.7;
        }
    }
    for (let i = 0; i < fakeSpectrum.length; i++) {
        // 两端略低，山丘形更自然
        const side = 1 - Math.abs(3 - i) * 0.12;
        fakeSpectrum[i] += (fakeSpectrumTargets[i] * side - fakeSpectrum[i]) * 0.45;
    }
    return [...fakeSpectrum];
};
// 频谱数据"无声音"判定：全部低于阈值时用拟真频谱兜底
const ensureSpectrumLive = (data: number[]): number[] =>
    data.every(v => v < SILENCE_THRESHOLD) ? generateFakeSpectrum() : data;

// Just Solo LyricServer v1.2.0：12 频段 -> 7 频段（与本地频谱柱数一致，低到高分组取均值）
const convertWs12To7 = (bands12: number[]): number[] => {
    const groups: number[][] = [
        bands12.slice(0, 1),    // 低频 1 段
        bands12.slice(1, 3),    // 2 段
        bands12.slice(3, 5),    // 2 段
        bands12.slice(5, 7),    // 2 段
        bands12.slice(7, 9),    // 2 段
        bands12.slice(9, 11),   // 2 段
        bands12.slice(11, 12),  // 高频 1 段
    ];
    return groups.map(g => g.reduce((a, b) => a + b, 0) / g.length);
};

// 封面url
const coverUrl = ref('');
const coverCache = new Map<string, string>();

// 当前 SMTC 来源应用是否为浏览器（edge/chrome）——仅表示"正在用浏览器播放"，不等于浏览器Pro模式
const currentIsBrowser = computed(() =>
    currentAppIdStr.value.includes('edge') || currentAppIdStr.value.includes('chrome')
);
// 用户是否在设置中选择了"浏览器Pro"媒体模式（与 SMTC 来源无关）
// 浏览器Pro=用户主动选择 browserPro 平台 + 实际 SMTC 来源是浏览器，两者都满足才算数
const isBrowserProMode = () => localStorage.getItem('nsd_target_player') === 'browserPro';
// 浏览器Pro标签页判定的额外结果：'music'|'video'；null 表示不做额外判定（按歌词兜底）
const browserContentOverride = ref<'music' | 'video' | null>(null);
const isSmtcCoverActive = ref(false);
// 浏览器是否成功获取到封面/歌词（成功即视为播放音乐，而非视频）
const isBrowserMusic = ref(false);
// 浏览器标题命中视频站后缀（如优酷 "-电视剧-高清完整正版视频在线观看-优酷"）时强制判定为视频模式
const isBrowserVideoTitle = ref(false);
// 当前 SMTC 来源应用的包名（用于 PotPlayer 音乐模式等封面策略判断）
const currentAppIdStr = ref('');

// ===== 浏览器 音乐/视频 判定（统一入口）=====
// 判定优先级（高→低）：视频站标题后缀 > 浏览器Pro标签页"正在播放: 歌名 - 歌手"匹配（命中即音乐并覆盖元数据）> 浏览器Pro标签页关键词 > 拉到歌词兜底
// 所有判定状态集中收敛，任何调用点都只问这两个函数，不再散落各自判断。

// 同步中枢：基于当前 reactive 状态立即得出音乐/视频结论
// 供 isVideoPlayer 同步使用；也作为 judgeBrowserMode 的最终返回
const resolveBrowserMode = (): 'music' | 'video' => {
    if (isBrowserVideoTitle.value) return 'video';          // ① 标题命中视频站后缀 → 强制视频
    if (browserContentOverride.value) return browserContentOverride.value; // ② 浏览器Pro标签页判定
    return isBrowserMusic.value ? 'music' : 'video';        // ③ 兜底：拉到歌词即音乐，否则视频
};

// 从浏览器窗口标题中识别音乐类标签页，返回 { song, artist }（artist 可能为空）：
// ① "正在播放: 歌名 - 歌手"（网易云/QQ音乐等网页版）
// ② "歌名MP3/FLAC免费下载-下载站"（音乐下载站，如"青花瓷MP3免费下载-音乐下载网"）
// ③ "歌名 - 歌手 - 平台"（如"青花瓷 - 周杰伦 - 网易云音乐"）
// ④ "歌名 - 平台"（如"青花瓷 - 网易云音乐"，artist 留空交给搜索兜底）
// 真实窗口标题在歌名/歌手之后还带浏览器附加的尾巴（如" - 个人 - Microsoft Edge"、" 和另外 N 个页面"、
// " 和另外 N 个标签页"），所以先统一清理浏览器后缀与多标签尾巴再匹配；① 取前缀后前两段为歌名/歌手；
// ② 只取格式词前的歌名；③④ 靠标题末尾的平台词收尾来识别，分别取前两段/前一段为歌名/歌手。
// 音乐关键词（统一数据源，统一小写）：既供 ③④ 的平台收尾匹配，也供 judgeBrowserMode ② 的标签页关键词判定复用
const TAB_MUSIC_KEYWORDS = ['music', '音乐', 'spotify', '网易云', '云音乐', 'netease', 'qq音乐', 'qqmusic', '酷狗', 'kugou', '酷我', 'kuwo', '虾米', '咪咕', '汽水音乐', '5sing', 'apple music', 'itunes', 'youtube music', 'soundcloud', 'bandcamp', 'tidal', 'deezer', 'pandora', 'amazon music'];
// ③④ 平台收尾判定正则（由 TAB_MUSIC_KEYWORDS 派生，大小写不敏感；词内空白用 \s* 容忍任意空格，如"Apple Music"/"AppleMusic"）
const TAB_MUSIC_PLATFORM_RE = new RegExp(TAB_MUSIC_KEYWORDS.map(k => k.replace(/\s+/g, '\\s*')).join('|'), 'i');
const parsePlayingTabTitle = (tabs: string[]): { song: string; artist: string } | null => {
    for (const raw of tabs) {
        console.log(raw);
        // 去掉窗口标题尾部的浏览器后缀（如" - Microsoft Edge" / " - Google Chrome"）
        // 以及多标签尾巴（" 和另外 N 个页面/标签页" / "and N other tabs"），统一清理后再匹配各模式
        const s = raw.trim()
            .replace(/\s*[-－–]\s*(Microsoft Edge Canary|Microsoft Edge|Google Chrome|Edge|Chrome)\s*$/i, '')
            .replace(/\s*和另外\s*\d+\s*(?:个页面|个标签页)\s*$/g, '')
            .replace(/\s+and\s+\d+\s+other\s+tabs?\s*$/gi, '')
            .replace(/\s+/g, ' ') // 连续空白折叠为单个空格，避免"Apple  Music"这类多余空格导致平台词匹配不到
            .trim();
        // ① 正在播放: 歌名 - 歌手
        const m = s.match(/^(正在播放|Now Playing|Playing)\s*[:：]\s*(.+)$/);
        if (m) {
            const parts = m[2].trim().split(/\s*[-－–]\s*/).map(p => p.trim()).filter(Boolean);
            // 歌名/歌手是前两段；后面的" - 个人"等窗口尾巴直接忽略，
            // 歌手段可能被 Edge/Chrome 追加" 和另外 N 个页面 / 和另外 N 个标签页 / and N other tabs"尾巴，需要清掉
            if (parts.length >= 2) {
                const artist = parts[1]
                    .replace(/\s*和另外\s*\d+\s*(?:个页面|个标签页)\s*$/g, '')
                    .replace(/\s+and\s+\d+\s+other\s+tabs?\s*$/gi, '')
                    .trim();
                return { song: parts[0], artist };
            }
        }
        // ② 歌名MP3/FLAC免费下载-下载站（音乐下载站标题）
        // 必须带音频格式/品质词，避免"某某视频免费下载"这类视频站标题被误判为音乐
        const m2 = s.match(/^(.+?)(?:MP3|FLAC|WAV|APE|AAC|OGG|M4A|WMA|DSD|320\s?[Kk]|无损|高品质|高音质)\s*(?:免费)?下载/i);
        if (m2) {
            return { song: m2[1].trim(), artist: '' };
        }
        // ③ 歌名 - 歌手 - 平台（如"青花瓷 - 周杰伦 - 网易云音乐"）
        // 最后一段必须以平台词收尾才命中，避免把"xxx - 腾讯视频"等视频标题误判为音乐
        const m3 = s.match(/^(.+?)\s*[-－–]\s*(.+?)\s*[-－–]\s*(.+?)\s*$/);
        if (m3 && TAB_MUSIC_PLATFORM_RE.test(m3[3])) {
            return { song: m3[1].trim(), artist: m3[2].trim() };
        }
        // ④ 歌名 - 平台（如"青花瓷 - 网易云音乐"），artist 留空交给搜索兜底
        const m4 = s.match(/^(.+?)\s*[-－–]\s*(.+?)\s*$/);
        if (m4 && TAB_MUSIC_PLATFORM_RE.test(m4[2])) {
            return { song: m4[1].trim(), artist: '' };
        }
    }
    return null;
};

// 已按"歌名|歌手"搜索过元数据的缓存键：同一首歌只搜一次，避免每 2s 轮询反复请求后端搜索
let lastTabMetaSearchKey = '';
// 最近一次 judgeBrowserMode 的标签页正则命中结果（每次判定开头重置，未命中保持 null）：
// 供歌词回调判断"正则是否已命中并搜索过一次元数据"，避免再用 SMTC 原始值重复搜索导致歌手乱跳
let lastTabPlayingResult: { song: string; artist: string } | null = null;

// 刷新函数：先刷新浏览器Pro的标签页额外判定，再返回统一结论
// 浏览器Pro模式（用户选了browserPro平台 且 SMTC来源是浏览器）才读活动标签页做关键词判定；
// 否则（通用媒体/未选浏览器Pro）→ override 置 null，退回歌词兜底，保证行为与旧逻辑一致
const judgeBrowserMode = async (song = '', durationMs = 0): Promise<'music' | 'video'> => {
    lastTabPlayingResult = null; // 每次判定重置，未命中保持 null，命中的在下方赋值
    if (isBrowserProMode() && currentIsBrowser.value) {
        // 浏览器Pro 专属分支：额外读取活动标签页做判定
        try {
            const tabs = await invoke<string[]>('get_active_browser_tabs');
            // ① 高优先级：标签页标题（以及 SMTC 标题，作为一条"伪标签页"）命中正则 →
            //    提取歌名/歌手搜索；拿到结果就用 fetch_song_meta 覆盖歌名/歌手/封面，并直接判定为音乐（优先于关键词判定）
            //    正则匹配只在浏览器Pro模式下生效
            const playing = parsePlayingTabTitle(song ? [song, ...tabs] : tabs);
            if (playing) {
                lastTabPlayingResult = playing;
                isBrowserMusic.value = true;
                browserContentOverride.value = 'music';
                const searchKey = `${playing.song}|${playing.artist}`;
                if (searchKey !== lastTabMetaSearchKey) {
                    lastTabMetaSearchKey = searchKey;
                    applyBrowserMusicMeta(playing.song, playing.artist, durationMs);
                }
                return resolveBrowserMode();
            }
            // ② 关键词判定（原有逻辑）：标题与关键词都去掉空白后做子串匹配，容忍多余空格（如"Apple  Music"/"QQ 音乐"）
            const VideoKeywords = ['bilibili', '哔哩哔哩', 'qqlive', '腾讯视频', 'youku', '优酷', 'youtube', 'iqiyi', '爱奇艺', '芒果tv', 'tv', '芒果TV', '影视', 'Tv', 'TV', 'cctv', 'CCTV', '央视'];
            const lowerTabs = tabs.map(t => t.toLowerCase().replace(/\s+/g, ''));
            const isVideo = VideoKeywords.some(keyword => lowerTabs.some(t => t.includes(keyword)));
            const isMusic = TAB_MUSIC_KEYWORDS.some(keyword => lowerTabs.some(t => t.includes(keyword.replace(/\s+/g, ''))));
            isBrowserMusic.value = (!isVideo || isMusic) && isMusic !== isVideo; // 非视频站且有音乐关键词，或 音乐关键词且非视频站
            browserContentOverride.value = isBrowserMusic.value ? 'music' : 'video';
        } catch {
            browserContentOverride.value = null; // 标签页读取失败 → 不做额外判定，走歌词兜底
        }
    } else {
        browserContentOverride.value = null; // 非浏览器Pro：无标签页信号，交给歌词兜底
    }
    return resolveBrowserMode();
};

// 同步标记：该曲目已拉到歌词，判定为音乐模式
// - 通用媒体（非浏览器Pro）：直接生效为音乐
// - 浏览器Pro：最终结论仍由 resolveBrowserMode 的 ② 标签页层把关（标签页未命中会判为视频，不被此标记覆盖）
const markBrowserMusic = (): void => {
    isBrowserMusic.value = true;
};

// 浏览器拉到歌词后，用后端元数据把标题/歌手修正为真实音乐信息（覆盖 SMTC 原始值如"正在播放: xxx"/"edge"），
// 封面按"SMTC优先、网络兜底"重新获取（applyCoverForApp 音乐分支 preferSmtc=true）
const applyBrowserMusicMeta = (song: string, artist: string, durationMs: number) => {
    invoke<[string, string]>('fetch_song_meta', { songName: song, artistName: artist, durationMs })
        .then(([metaTitle, metaArtist]) => {
            if (!metaTitle) return;
            // 歌手优先级：传入的歌手（浏览器Pro下由标签页正则解析得出）> 后端搜索到的歌手；
            // 占位歌手（edge/chrome/potplayer/bilibili）与平台名（如"网易云音乐"，TAB_MUSIC_PLATFORM_RE 命中）都视为无歌手，
            // 回退用后端 fetch_song_meta 返回的歌手，避免平台名被当歌手显示、并连带污染封面搜索
            const finalArtist = artist && !TAB_MUSIC_PLATFORM_RE.test(artist) && !/^(edge|chrome|potplayer|bilibili)$/i.test(artist.trim()) ? artist : metaArtist;
            const displayArtist = finalArtist || t('unknownArtist');
            // 强制置为音乐模式：元数据修正只在音乐判定成立时发生，但 isBrowserMusic 可能在切歌时被复位，
            // 若不强制，封面会走 SMTC 分支（浏览器 SMTC 通常无封面）导致一直显示默认图标
            isBrowserMusic.value = true;
            // 后端解析出的歌名/歌手与当前显示一致（如同首歌重复触发、SMTC 标题只是格式变化）→ 不重复修改显示与封面
            if (metaTitle === currentSongName.value && displayArtist === currentArtistName.value) {
                return;
            }
            currentSongName.value = metaTitle;
            currentArtistName.value = displayArtist;
            fillCollapsedWithTrackInfo();
            // 用正确的歌名/歌手重新获取封面（此前 watch(isBrowserMusic) 用的是 SMTC 原始值）
            const trackInfo = finalArtist ? `${metaTitle} - ${finalArtist}` : metaTitle;
            applyCoverForApp(trackInfo, metaTitle, finalArtist, currentAppIdStr.value, true, true);
        }).catch(() => { });
};

// 统一判定：当前播放器是否按"视频类"处理（决定是否做歌词匹配/标题常驻/封面策略）
// 仅浏览器进入 resolveBrowserMode；其他来源（B站/PotPlayer）保持原有逻辑
const isVideoPlayer = computed(() => {
    if (currentIsBrowser.value) return resolveBrowserMode() === 'video';
    const id = currentAppIdStr.value;
    if (id.includes('bilibili')) return true; // B站：始终视频
    if (id.includes('potplayer')) return currentArtistName.value === 'potplayer'; // PotPlayer：artist 占位=视频
    return false; // 其他媒体默认音乐
});

// 沉浸模式专属的静态模糊封面
const blurredCoverUrl = ref('');
const blurredCoverCache = new Map<string, string>();

// 从 MainPanel 抄过来的 CPU 静态模糊烘焙机
const bakeBlurImage = (url: string): Promise<string> => {
    return new Promise((resolve) => {
        const img = new Image();
        if (url.startsWith('http')) img.crossOrigin = 'anonymous';
        img.onload = () => {
            const canvas = document.createElement('canvas');
            canvas.width = 120; // 降低物理分辨率以提升性能
            canvas.height = 120;
            const ctx = canvas.getContext('2d');
            if (!ctx) return resolve(url);
            ctx.filter = 'blur(10px)';
            ctx.drawImage(img, -10, -10, 140, 140);
            try { resolve(canvas.toDataURL('image/jpeg', 0.6)); }
            catch (e) { resolve(url); }
        };
        img.onerror = () => resolve(url);
        img.src = url;
    });
};

// 记录最近一次封面请求序号，防止切歌竞态下旧请求的结果覆盖新内容封面
// 网络与 SMTC 两条路径共用，保证任意来源的过期结果都会被丢弃
let coverReqSeq = 0;

// 记录最近一次"切歌"时刻与"真实封面被应用"的时刻：
// 若当前显示的真实封面是在本首歌开始之后才应用的，说明它是本首歌的封面，
// 晚到的兜底逻辑（如 SMTC 重试循环约 3s 后、延迟重试）不应再把它覆盖成默认图标
// （否则会出现"先音乐封面后 edge 图标"的闪跳）
let songChangeTime = 0;
let lastRealCoverApplyTime = 0;

// 把拿到的封面写入状态 + 缓存 + 烘焙模糊封面（网络与 SMTC 两条路径的公共落点）
// onlyIfChanged=true 且新封面与当前显示相同 → 不应用（恢复播放时避免无意义刷新）
const applyCoverToState = async (trackInfo: string, url: string, onlyIfChanged: boolean) => {
    if (onlyIfChanged && url === coverUrl.value) return;
    coverUrl.value = url;
    // 记录真实（非占位）封面被应用的时刻，供 fallbackBrowserLogo 判断"当前封面是否本首歌拉到的"
    if (!isCoverPlaceholder(url)) lastRealCoverApplyTime = Date.now();
    // 缓存超过 50 条时整体清空，防止内存无限增长
    if (coverCache.size > 50) {
        coverCache.clear();
        blurredCoverCache.clear();
    }
    coverCache.set(trackInfo, url);
    // 同步烘焙沉浸模式用的静态模糊封面
    const bakedImage = await bakeBlurImage(url);
    blurredCoverUrl.value = bakedImage;
    blurredCoverCache.set(trackInfo, bakedImage);
};

// 封面缓存命中恢复（网络与 SMTC 共用）：
//   命中返回 true；mySeq !== coverReqSeq（请求已过期，期间切歌）时不应用封面，仅返回命中
//   markSmtcActive 命中即置 isSmtcCoverActive（SMTC 专用：缓存命中说明该曲目拿到过 SMTC 封面）
const applyCachedCover = (trackInfo: string, onlyIfChanged: boolean, mySeq: number, markSmtcActive = false): boolean => {
    if (!coverCache.has(trackInfo)) return false;
    if (markSmtcActive) isSmtcCoverActive.value = true;
    if (mySeq !== coverReqSeq) return true; // 请求已过期，放弃应用，防止旧封面覆盖新封面
    const cached = coverCache.get(trackInfo)!;
    if (!onlyIfChanged || cached !== coverUrl.value) {
        coverUrl.value = cached;
        if (!isCoverPlaceholder(cached)) lastRealCoverApplyTime = Date.now();
        blurredCoverUrl.value = blurredCoverCache.get(trackInfo) || '';
    }
    return true;
};

// 网络封面获取/应用逻辑：查缓存 → 未命中就调接口 → 写缓存 → 烘焙模糊封面
// preferSmtc: 后端内部会先尝试 SMTC 本地封面，拿不到才走网络（PotPlayer 音乐模式等场景传 false 强制纯网络）
// onlyIfChanged: 与当前显示的封面对比，不一样才应用（恢复播放时用）
// clearOnError: 获取失败时是否清空封面（切歌时清空，恢复播放时保留）
const fetchAndApplyCover = async (trackInfo: string, song: string, artist: string, onlyIfChanged = false, clearOnError = true, preferSmtc = true) => {
    // 每次调用自增序号，请求返回后若序号已变（期间切歌）则丢弃结果，防止旧封面覆盖新封面
    const mySeq = ++coverReqSeq;
    if (applyCachedCover(trackInfo, onlyIfChanged, mySeq)) return;
    try {
        const url = await invoke<string>('get_random_cover_url', { songName: song, artistName: artist, preferSmtc });
        // 期间已切到新内容，丢弃过期结果
        if (mySeq !== coverReqSeq) return;
        // 同曲目精修（onlyIfChanged=true）时，后端返回占位图但当前已是真实封面 → 保留现有封面，
        // 避免真实封面被"无封面占位 SVG / 默认图标"覆盖（如封面源超时返回占位图）
        if (onlyIfChanged && isCoverPlaceholder(url) && !isCoverPlaceholder(coverUrl.value)) return;
        await applyCoverToState(trackInfo, url, onlyIfChanged);
    } catch (e) {
        // 仅当仍是当前请求且允许清空时，才清空封面（切歌时清空，恢复播放时保留现有封面）
        // 同曲目精修（onlyIfChanged=true）且当前已是真实封面时不清空，避免网络抖动把真实封面清成默认图标
        if (mySeq === coverReqSeq && clearOnError && !(onlyIfChanged && !isCoverPlaceholder(coverUrl.value))) {
            coverUrl.value = '';
            blurredCoverUrl.value = '';
        }
    }
};

// SMTC 本地封面获取/应用逻辑（只读本地封面，不联网兜底）
// 成功拿到或缓存命中时置 isSmtcCoverActive=true，供 PotPlayer 音乐模式判断"SMTC 是否已覆盖"，
// 也供浏览器 logo 兜底判断"是否已拿到 SMTC 封面、无需退回默认图标"
const fetchAndApplySmtcCover = async (trackInfo: string, onlyIfChanged = false) => {
    const mySeq = ++coverReqSeq;
    // 缓存命中（含置 isSmtcCoverActive=true）则直接使用
    if (applyCachedCover(trackInfo, onlyIfChanged, mySeq, true)) return;
    // 切歌场景：SMTC 封面可能晚于标题就绪，最多重试 3 次（间隔 1.5s）确保拿到新封面
    const maxAttempts = onlyIfChanged ? 1 : 3;
    for (let attempt = 0; attempt < maxAttempts; attempt++) {
        try {
            const smtcCover = await invoke<string | null>('get_smtc_cover');
            // 期间已切到新内容，丢弃过期结果
            if (mySeq !== coverReqSeq) return;
            if (smtcCover) {
                isSmtcCoverActive.value = true;
                await applyCoverToState(trackInfo, smtcCover, onlyIfChanged);
                return;
            }
        } catch (e) {
            // 单次失败继续重试，最终仍拿不到就静默保留 logo
        }
        if (attempt < maxAttempts - 1) {
            await new Promise(r => setTimeout(r, 1500));
        }
    }
};

// 网络封面"无封面"判定：后端拿不到封面时返回占位 SVG（data:image/svg+xml）而非空串，
// 因此空串或占位 SVG 都视为"没拿到真实封面"，用于触发默认图标回退
const isCoverPlaceholder = (url: string) => !url || url.startsWith('data:image/svg+xml');

// 浏览器音乐模式：网络封面失败回退到默认图标后，定时重试拉真实封面（拿到后自动替换图标）
let coverRetryTimer: number | undefined;
let coverRetryTrack = '';
let coverRetryCount = 0;
const COVER_RETRY_MS = 5000; // 重试间隔
const COVER_RETRY_MAX = 4;   // 每首歌最多重试次数
const scheduleCoverRetry = (trackInfo: string) => {
    if (coverRetryTimer) clearTimeout(coverRetryTimer);
    // 换了首歌重试计数复位
    if (coverRetryTrack !== trackInfo) {
        coverRetryTrack = trackInfo;
        coverRetryCount = 0;
    }
    coverRetryTimer = window.setTimeout(async () => {
        coverRetryTimer = undefined;
        // 已切歌 → 放弃重试
        // （是否音乐由 applyCoverForApp 按当前状态重新分派：音乐走网络重拉，视频重试 SMTC 晚到封面）
        const curTrackInfo = currentArtistName.value ? `${currentSongName.value} - ${currentArtistName.value}` : currentSongName.value;
        if (curTrackInfo !== trackInfo) return;
        if (coverRetryCount >= COVER_RETRY_MAX) return;
        coverRetryCount++;
        // 清缓存强制重新请求，重新走封面决策；拿到真实封面后不会再进回退分支，重试自然停止
        coverCache.delete(trackInfo);
        blurredCoverCache.delete(trackInfo);
        await applyCoverForApp(trackInfo, currentSongName.value, currentArtistName.value, currentAppIdStr.value, true, true);
    }, COVER_RETRY_MS);
};

// 统一设置“logo 封面”：凡需要把软件/平台 logo 当作封面时都调用本函数。
// 除了替换前景封面外，同时复位 SMTC 封面标记、清空沉浸模糊背景及对应缓存，
// 避免切到 logo 封面后残留上一首真实封面的模糊背景（coverglass 背景与前景不一致）。
const applyLogoCover = (logoUrl: string, trackInfo: string) => {
    coverUrl.value = logoUrl;
    isSmtcCoverActive.value = false;
    blurredCoverUrl.value = '';
    blurredCoverCache.delete(trackInfo);
    coverCache.delete(trackInfo);
};

// 浏览器音乐/视频模式：未取得真实封面（SMTC 或网络）时统一退回浏览器 logo 封面，
// 并定时重试拉取真实封面（成功后自动替换 logo）
const fallbackBrowserLogo = (appIdStr: string, trackInfo: string) => {
    // 若当前显示的真实封面在本首歌开始后才应用（非占位、时刻晚于切歌时刻），说明封面已就绪，
    // 不再退回 logo——否则慢路径（isNewTrack 的 SMTC 重试循环约 3s 才结束）会在真实封面之后
    // 又把封面替换成 edge/chrome 图标，出现"先音乐封面后 edge 图标"的闪跳
    if (!isCoverPlaceholder(coverUrl.value) && lastRealCoverApplyTime >= songChangeTime) {
        return;
    }
    applyLogoCover(APP_COVER_LOGO_MAP[appIdStr.includes("edge") ? "edge" : "chrome"], trackInfo);
    scheduleCoverRetry(trackInfo);
};

// 集中封面决策函数：根据应用类型统一决定封面策略，供切歌与恢复播放复用
// 参数：
//   trackInfo   缓存 key（"歌名 - 歌手"）
//   song/artist 歌名/歌手（artist 可能被后端占位为 "potplayer"）
//   appIdStr    SMTC 来源应用包名（决定应用类型）
//   onlyIfChanged 与当前显示封面对比，不一样才应用（恢复播放时用 true）
//   clearOnError  获取失败时是否清空封面（切歌时 true，恢复播放时 false）
// 各应用类型都会自行完成封面应用（含 logo 兜底），无需调用方二次兜底
const applyCoverForApp = async (trackInfo: string, song: string, artist: string, appIdStr: string, onlyIfChanged = false, clearOnError = true) => {
    // PotPlayer 视频/音乐模式判定：
    //   后端在 PotPlayer 无歌手元数据时会把 artist 占位为 "potplayer"（视频通常无歌手）→ 视频模式
    //   来源是 PotPlayer 但 artist 不是占位值（有真实歌手元数据）→ 音乐模式
    const isPotplayerVideo = artist === "potplayer";
    const isPotplayerMusic = appIdStr.includes("potplayer") && !isPotplayerVideo;
    const isBrowser = appIdStr.includes("edge") || appIdStr.includes("chrome");
    const isBilibili = appIdStr.includes("bilibili");
    const isJustSolo = appIdStr.includes("justsolo");

    // PotPlayer 视频模式：统一使用 logo 封面（无歌手元数据说明播放的是视频）
    if (isPotplayerVideo) {
        applyLogoCover(potplayerLogo, trackInfo);
        return;
    }

    // 浏览器音乐模式：优先 SMTC 本地封面，拿不到再走网络兜底
    // 浏览器视频模式：先尝试用 SMTC 本地封面覆盖，若拿不到再走默认浏览器图标
    if (isBrowser) {
        isSmtcCoverActive.value = false;
        if (isBrowserMusic.value) { // 浏览器已判定为播放音乐（拉到歌词），则直接走网络封面
            await fetchAndApplyCover(trackInfo, song, artist, onlyIfChanged, clearOnError, true);
            // 网络封面拿不到（空串或后端占位 SVG 都视为无封面）→ 回退到默认图标，并定时重试拉真实封面
            if (isCoverPlaceholder(coverUrl.value)) {
                fallbackBrowserLogo(appIdStr, trackInfo);
            }
        } else {
            await fetchAndApplySmtcCover(trackInfo, onlyIfChanged);
            // SMTC 没拿到 → 回退到默认图标，并定时重试 SMTC（封面可能晚于标题就绪）
            if (!isSmtcCoverActive.value) {
                fallbackBrowserLogo(appIdStr, trackInfo);
            }
        }
        return;
    }

    // JustSolo：直接使用 SMTC 封面，若拿不到再走网络兜底
    if (isJustSolo) {
        // 先重置 SMTC 活跃标志，防止上首歌置 true 后残留，导致本首歌 SMTC 失败时跳过网络兜底
        isSmtcCoverActive.value = false;
        await fetchAndApplySmtcCover(trackInfo, onlyIfChanged);
        if (!isSmtcCoverActive.value) { // 如果 SMTC 没拿到封面，走网络兜底
            await fetchAndApplyCover(trackInfo, song, artist, onlyIfChanged, clearOnError, false);
        }
        return;
    }

    // bilibili：统一使用固定 logo 封面
    if (isBilibili) {
        applyLogoCover(bilibiliLogo, trackInfo);
        return;
    }

    // PotPlayer 音乐模式：优先 SMTC 本地封面，拿不到再走网络兜底
    // 流程：先置 isSmtcCoverActive=false → 尝试 SMTC（成功则置 true）→
    //       若仍为 false（SMTC 没拿到）→ 走网络兜底（preferSmtc=false，避免后端重复试 SMTC）
    if (isPotplayerMusic) {
        isSmtcCoverActive.value = false;
        await fetchAndApplySmtcCover(trackInfo, onlyIfChanged);
        // SMTC 没拿到（isSmtcCoverActive 仍为 false）时，走网络兜底
        if (!isSmtcCoverActive.value) {
            await fetchAndApplyCover(trackInfo, song, artist, onlyIfChanged, clearOnError, false);
        }
        return;
    }

    // 其他应用：走网络（后端内部默认先尝试 SMTC 再网络）
    await fetchAndApplyCover(trackInfo, song, artist, onlyIfChanged, clearOnError, true);
};

// 实时FPS功能相关
const enableFps = ref(localStorage.getItem('nsd_fps_monitor') === 'true');
const currentFps = ref(0);

// 智能判断并按需启停后端的 FPS 采集插件
const checkAndToggleFpsPlugin = () => {
    const needFps = enableFps.value || (enableCustomDisplay.value && customSlots.value.includes('fps'));
    invoke('toggle_fps_plugin', { enable: needFps }).catch((err) => {
        console.error('FPS 插件启动失败:', err);
        // FPS 插件不存在时，前端就地退出 FPS 显示，避免灵动岛停留在 FPS 模式
        enableFps.value = false;
        localStorage.setItem('nsd_fps_monitor', 'false');
        if (customSlots.value.includes('fps')) {
            const newSlots = [...customSlots.value];
            const index = newSlots.indexOf('fps');
            if (index !== -1) newSlots[index] = null;
            customSlots.value = newSlots;
        }
        // 再发送全局求救信号给控制台主窗口（由主窗口弹下载提示并同步状态）
        emit('fps-plugin-missing');
    });
};

// 记录最后一次接收到真实 WS 歌词的时间
let lastWsLyricTime = 0;

// 记录最后一次收到 WS 12 频段频谱数据的时间（用于判定 WS 频谱是否新鲜）
let lastWsSpectrumTime = 0;

// WebSocket 状态管理
const isWsConnected = ref(false);
const isWsConnecting = ref(false);
// 一次性连接标志：只要尝试过（无论成功失败）就不再尝试第二次
let wsConnectAttempted = false;
let unlistenWsStatus: (() => void) | null = null;

// 后端发现 JustSolo 事件的监听器（唯一驱动 WS 连接/重连的入口，不轮询）
let unlistenJustSolo: (() => void) | null = null;

// WebSocket 实时歌词监听
let unlistenWs: (() => void) | null = null;

// 用 SMTC 已获取到的 "标题 - 歌手" 填充折叠态文本
const fillCollapsedWithTrackInfo = () => {
    if (!currentSongName.value || currentSongName.value === t('noSongPlaying')) return;
    // 标记当前显示的是 "标题 - 歌手" 占位文本，供歌词第一句恰好等于标题时强制接管显示
    isTitlePlaceholder = true;
    // PotPlayer：直接用标题当常驻歌词显示，不再拼 "标题 - potplayer"
    if (isPotplayerSource.value) {
        setSafeTrackInfo(currentSongName.value);
        return;
    }
    // 视频类播放源（B站/浏览器视频）：只显示标题，不拼歌手
    if (isVideoPlayer.value) {
        setSafeTrackInfo(currentSongName.value);
        return;
    }
    const artist = currentArtistName.value === t('unknownArtist') ? '' : currentArtistName.value;
    setSafeTrackInfo(artist ? `${currentSongName.value} - ${artist}` : currentSongName.value);
};

const initWebSocket = async () => {
    // 一次性连接：尝试过就不再连第二次
    if (wsConnectAttempted) return;
    wsConnectAttempted = true;

    try {
        // 防重入：已连上或正在连就不再调
        if (isWsConnected.value || isWsConnecting.value) return;

        // 必须先挂载监听器，再去呼叫 Rust 连接！
        // 因为本地 WS 连接是毫秒级的，如果先 invoke 再 listen，Vue 会完美错过连接成功的初始信号！
        if (!unlistenWsStatus) {
            unlistenWsStatus = await listen('websocket-status', (event: any) => {
                isWsConnected.value = event.payload;
                isWsConnecting.value = false;
                if (isWsConnected.value) {
                    parsedLyrics.value = []; // 连上 WS 后清空可能残存的网络歌词，避免冲突
                    // 连上 WS 后用 SMTC 已拿到的 "标题 - 歌手" 填充折叠态文本（歌词接管前先兜底显示）
                    fillCollapsedWithTrackInfo();
                } else {
                    // 断开/连接失败时重置一次性标志，允许下次检测到 JustSolo 时重试连接
                    wsConnectAttempted = false;
                }
            });
        }

        if (!unlistenWs) {
            unlistenWs = await listen('websocket-lyrics', (event: any) => {
                let payload = event.payload;

                // 如果传过来的是字符串包装的 JSON，尝试解开它
                if (typeof payload === 'string') {
                    try { payload = JSON.parse(payload); } catch (e) { }
                }

                // Just Solo LyricServer 专属协议
                if (payload && payload.type) {
                    // 1. 收到完整歌词列表 (init)
                    if (payload.type === 'init' && Array.isArray(payload.lyrics)) {
                        lastWsLyricTime = Date.now();
                        isMediaActive.value = true; // 强制激活音乐卡片展示
                        isPlaying.value = true;     // 强制启动 50ms 歌词比对定时器

                        parsedLyrics.value = payload.lyrics.map((l: any) => ({
                            time: l.time,
                            text: l.text
                        }));
                        lyricQueue.value = [];
                        currentMatchedIndex = -1;
                        lastLyricChangeTime = 0; // 重置时间锁，允许立即显示第一句歌词

                        // 浏览器收到完整歌词 → 判定为播放音乐（而非视频）
                        markBrowserMusic();

                        return;
                    }

                    // 收到实时进度 (progress)
                    if (payload.type === 'progress') {
                        lastWsLyricTime = Date.now();
                        isMediaActive.value = true; // 心跳保活

                        if (typeof payload.position === 'number') {
                            // 修正：绝不能无脑覆盖，而是跟 HTTP 逻辑一样，误差大于 500ms 时才校准
                            // 这样才能让 50ms 定时器里的 `localPositionMs.value += delta` 完美发挥顺滑推算的作用！
                            if (Math.abs(payload.position - localPositionMs.value) > 500) {
                                localPositionMs.value = payload.position;
                            }
                        }
                        return;
                    }

                    // 收到播放状态 (playback)
                    if (payload.type === 'playback') {
                        lastWsLyricTime = Date.now();
                        if (payload.status === 'playing') {
                            isPlaying.value = true;
                            isMediaActive.value = true;
                        } else if (payload.status === 'paused') {
                            isPlaying.value = false;
                        }
                        return;
                    }

                    // 收到实时频谱 (协议 v1.2.0)：12 频段 -> 7 频段（与本地频谱柱数一致）
                    if (payload.type === 'spectrum') {
                        lastWsLyricTime = Date.now();
                        if (Array.isArray(payload.bands) && payload.bands.length === 12) {
                            lastWsSpectrumTime = Date.now();
                            spectrumData.value = ensureSpectrumLive(convertWs12To7(payload.bands));
                        }
                        return;
                    }
                }

                // 下方保留单句纯文本推送的兼容逻辑
                let lyricText = "";
                if (typeof payload === 'string') {
                    lyricText = payload;
                } else if (payload) {
                    lyricText = payload?.data?.currentLyric
                        || payload?.data?.lyric
                        || payload?.data?.text
                        || payload?.data?.content
                        || payload?.lyric
                        || payload?.content
                        || payload?.text
                        || "";
                }

                if (lyricText && lyricText.trim() !== "") {
                    lastWsLyricTime = Date.now();
                    isMediaActive.value = true;
                    isPlaying.value = true;
                    parsedLyrics.value = [];
                    lyricQueue.value = [];
                    setSafeTrackInfo(lyricText.trim());
                }
            });
        }

        // 监听器就绪后，再发车连接
        isWsConnecting.value = true;
        await invoke('start_websocket_lyrics', { url: "ws://127.0.0.1:47290/" });

    } catch (err) {
        console.error("WebSocket 启动失败:", err);
        isWsConnecting.value = false;
    }
};

const stopWebSocket = async () => {
    try {
        await invoke('stop_websocket_lyrics');
        if (unlistenWs) {
            unlistenWs();
            unlistenWs = null;
        }
        // 同步销毁状态监听器
        if (unlistenWsStatus) {
            unlistenWsStatus();
            unlistenWsStatus = null;
        }
        isWsConnected.value = false;
        isWsConnecting.value = false;
        // 关闭媒体控制时重置一次性标志，允许下次开启时重新连接
        wsConnectAttempted = false;
    } catch (err) {
        console.error("WebSocket 停止失败:", err);
    }
};

// 记录消息模式开关状态
const isMsgModeEnabled = ref(localStorage.getItem('nsd_msg_mode') === 'true');

// 记录系统资源监控状态
const enableSysResource = ref(localStorage.getItem('nsd_sys_resource') === 'true');
const cpuUsage = ref(0);
const ramUsage = ref(0);

// 灵动岛自定义显示
const enableCustomDisplay = ref(localStorage.getItem('nsd_custom_display') === 'true');
const customSlots = ref<(string | null)[]>(JSON.parse(localStorage.getItem('nsd_custom_slots') || '[null, null, null]'));

// 新增 FPS 判定
const displayFps = computed(() => !enableCustomDisplay.value && !isMsgActive.value && !displaySysToast.value && enableFps.value);

// 使用计算属性智能判断当前该显示谁
const displayCustom = computed(() => !isMsgActive.value && !displaySysToast.value && enableCustomDisplay.value);
const displayResource = computed(() => !enableCustomDisplay.value && !isMsgActive.value && !displaySysToast.value && enableSysResource.value && !enableFps.value && (!isMusicCtlEnabled.value || !isMediaActive.value));
const displaySpeed = computed(() => !enableCustomDisplay.value && !isMsgActive.value && !displaySysToast.value && !enableSysResource.value && !enableFps.value && (!isMusicCtlEnabled.value || !isMediaActive.value));
const displayMusic = computed(() => !isMsgActive.value && !displaySysToast.value && isMusicCtlEnabled.value && isMediaActive.value && !enableCustomDisplay.value);

// 智能判断静默模式下是否该显示：有消息、有系统提示、剪贴板链接通知，或开启了音乐控制且正在播放
const shouldShowInQuietMode = computed(() =>
    isMsgActive.value || displayActivity.value || displaySysToast.value || displayClipboard.value || (isMusicCtlEnabled.value && isMediaActive.value)
);
watch(shouldShowInQuietMode, async (newVal) => {
    if (isMsgModeEnabled.value) {
        if (newVal && !isIslandVisible.value) {
            // 条件满足且当前隐藏时，呼出灵动岛
            await invoke('show_window_no_activate', { label: 'widget' });
            isIslandVisible.value = true;
        } else if (!newVal && isIslandVisible.value) {
            // 条件不满足时，延迟 600ms 后再次确认状态，防止短时间内状态反复横跳
            setTimeout(() => {
                if (isMsgModeEnabled.value && !shouldShowInQuietMode.value) {
                    isIslandVisible.value = false;
                }
            }, 600);
        }
    }
});

// 沉浸背景的独立存活逻辑
// 只要媒体活跃且未被消息弹窗/活动池占用，背景就一直存在，即使正在显示系统通知
// 前景封面是软件/平台 logo（非真实专辑封面）时全局禁用：logo 用于模糊背景无意义，
// 也可避免残留的旧模糊封面在切到 logo 封面后继续渲染
const showCoverglassBg = computed(() => {
    return islandTheme.value === 'coverglass' &&
        isMusicCtlEnabled.value &&
        isMediaActive.value &&
        !isMsgActive.value &&
        !displayActivity.value &&
        blurredCoverUrl.value &&
        !APP_COVER_LOGOS.includes(coverUrl.value);
});

// 辅助函数：获取当前状态应该拥有的默认大小
const getBaseSize = () => {
    if (displayFps.value) return { w: nsdBaseWidth.value, h: nsdBaseHeight.value };
    if (displayCustom.value || displayResource.value) return { w: nsdMusicBaseWidth.value, h: Math.max(nsdBaseHeight.value + 8, 42) };
    if (displaySpeed.value) return { w: nsdBaseWidth.value, h: nsdBaseHeight.value };
    return { w: nsdMusicBaseWidth.value, h: Math.max(nsdBaseHeight.value + 8, 42) };
};

// 监听内容切换，触发丝滑动画过渡
watch([displaySpeed, displayMusic, displayResource, displayFps], () => {
    // 仅在未被临时弹窗（消息、活动、音乐展开）占用时，才执行基础大小切换
    if (!isMsgActive.value && !displayActivity.value && !displaySysToast.value && !isMusicExpanded.value && !isMusicExpanding.value) {
        const { w, h } = getBaseSize();
        animateIslandSize(w, h);
    }
});

// 专门用于控制右侧常驻指示灯的独立计算属性（完全不受消息通知打断）
const showSpectrumIndicator = computed(() => {
    return isMusicCtlEnabled.value && isMediaActive.value;
});

const togglePlay = async () => {
    // 1. 前端先切换图标，给用户即时的视觉反馈
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

// 从暂停恢复到播放时，重新获取封面并与当前显示对比（不一样才更新）
let isCoverRefreshing = false;
const refreshCoverOnResume = async () => {
    // 防重入，避免连续触发时并发请求
    if (isCoverRefreshing) return;

    const song = currentSongName.value;
    const artist = currentArtistName.value;
    // 与 syncMusicStatus 里的缓存 key 格式保持一致
    const trackInfo = artist ? `${song} - ${artist}` : song;
    // 没有有效歌曲信息（如"暂无歌曲播放"占位）时跳过
    if (!trackInfo || !song || song === t('noSongPlaying')) return;
    // 浏览器/视频类应用用的是固定 logo 封面，无需刷新
    if (APP_COVER_LOGOS.includes(coverUrl.value)) return;

    isCoverRefreshing = true;
    try {
        // 统一走集中决策函数：按应用类型分派封面策略
        // onlyIfChanged=true 与当前显示对比，不一样才更新；clearOnError=false 失败时保持现有封面不动
        await applyCoverForApp(trackInfo, song, artist, currentAppIdStr.value, true, false);
    } finally {
        isCoverRefreshing = false;
    }
};

// 监听暂停 -> 播放的切换（SMTC 与 WS 两条路径都会走到这里），触发封面刷新对比
// 注意：WS 连接/重连不再由播放状态触发，统一由后端发现 JustSolo 后的事件驱动
watch(isPlaying, (now, prev) => {
    if (now && !prev) {
        refreshCoverOnResume();
    }
});

// 浏览器视频站标题后缀列表：命中任一后缀即判定为浏览器视频模式，并统一删除该后缀
// 注意：判定要用清理前的原始标题（清理后后缀已被删掉，无法再判）
const BROWSER_VIDEO_SUFFIX_RE = [
    /_[ _]*哔哩哔哩[ _]*bilibili\s*$/i,
    /-电视剧-高清完整正版视频在线观看-优酷\s*$/i,
    /-电影-高清完整正版视频在线观看-优酷\s*$/i,
    /-综艺-高清完整正版视频在线观看-优酷\s*$/i,
    /-最新热门短剧大全-免费短剧在线观看\s*$/i,
    /-动漫-高清完整正版视频在线观看-优酷\s*$/i,
    /-少儿-高清完整正版视频在线观看-优酷\s*$/i,
    /-纪录片-高清完整正版视频在线观看-优酷\s*$/i,
    /-体育-高清完整正版视频在线观看-优酷\s*$/i,
    /-文化-高清完整正版视频在线观看-优酷\s*$/i,
    /-游戏-高清完整正版视频在线观看-优酷\s*$/i,
    /-音乐-高清完整正版视频在线观看-优酷\s*$/i,
];

// 统一后缀删除函数：去掉标题里的所有视频站后缀及残留分隔符
const cleanSongTitle = (title: string) => {
    let s = title;
    for (const re of BROWSER_VIDEO_SUFFIX_RE) {
        s = s.replace(re, '');
    }
    return s.replace(/[_\- ]+$/, '').trim();
};

// 核心同步函数：负责获取状态并智能降级
const syncMusicStatus = async () => {
    try {
        const res = await invoke<[string, string, boolean, number, number, string] | null>('fetch_netease_music_info');

        // 判定过去 3 秒内是否有活跃的本地 WebSocket 推送
        const isWsActive = (Date.now() - lastWsLyricTime < 3000);

        if (res) {
            const [rawSong, artist, playing, positionMs, durationMs, app_id_str] = res;

            // 标题命中任一视频站后缀（B站/优酷等）→ 强制判定为浏览器视频模式（用清理前的原始标题判断）
            // 统一清理标题：删除视频站后缀，展示与搜索都用干净标题
            const song = cleanSongTitle(rawSong);
            isBrowserVideoTitle.value = song !== rawSong;

            // 先检测来源应用是否切换，并尽早记录来源包名，
            // 让 currentIsBrowser / isVideoPlayer 等 computed 立即反映本次来源
            const appSwitched = currentAppIdStr.value !== '' && currentAppIdStr.value !== app_id_str;
            currentAppIdStr.value = app_id_str;

            // 刷新浏览器 音乐/视频 判定（内部已按浏览器Pro/非Pro分派；浏览器Pro下 SMTC 标题也走标签页正则）
            await judgeBrowserMode(song, durationMs).catch(() => { /* 判定失败沿用歌词兜底 */ });

            // 浏览器Pro 且标签页正则已命中音乐：SMTC 原始值（如"正在播放: xxx"/"edge"）不再反馈到前端显示，
            // 前端保持上一次的歌名/歌手/封面，等后端 fetch_song_meta 解析出真实歌名/歌手后有变化再统一更新
            // （关键词兜底判为音乐时 lastTabPlayingResult 为 null，此标记不成立，仍按原逻辑即时显示原始值）
            const browserTabMusicPending = isBrowserProMode() && currentIsBrowser.value && browserContentOverride.value === 'music' && !!lastTabPlayingResult;

            // 切换 SMTC 来源应用：立即清空旧应用残留的歌词，避免串歌词（新应用歌词就绪前先显示标题）
            if (appSwitched) {
                parsedLyrics.value = [];
                lyricQueue.value = [];
                currentMatchedIndex = -1;
                console.log('clear lyrics');
            }

            // 仅在 WS 不活跃时，使用 SMTC 的播放状态
            if (!isWsActive) {
                isPlaying.value = playing;
            }
            if (!isMediaActive.value) isMediaActive.value = true;
            isFirstMediaCheck = false;
            isNewlyEnabled = false;

            // SMTC 已连上应用但还没有有效标题：单行展示改为显示已连接的应用名（而不是"未在播放"）
            if (!song) {
                if (!isWsActive) {
                    const connectedName = getConnectedAppName(app_id_str);
                    if (currentBaseInfo.value !== connectedName) {
                        currentBaseInfo.value = connectedName;
                        setSafeTrackInfo(connectedName);
                    }
                }
                return;
            }

            // 拦截无效的时长，并智能利用歌词反推
            if (durationMs > 0) {
                currentDurationMs.value = durationMs; // 系统给的时长，直接使用
            } else if (parsedLyrics.value.length > 0) {
                // 系统未提供时长但有歌词：用最后一句歌词时间 + 8 秒尾奏估算
                const lastLyric = parsedLyrics.value[parsedLyrics.value.length - 1];
                currentDurationMs.value = lastLyric.time + 8000;
            }

            const newTrackInfo = artist ? `${song} - ${artist}` : song;
            // 是否切到了新内容（切歌或浏览器切换 SMTC 播放的内容）
            const isNewTrack = currentBaseInfo.value !== newTrackInfo;

            // 浏览器已判定为播放音乐时，标题/歌手由 fetch_song_meta 提供（更准），
            // 不再用 SMTC 的原始值（如"正在播放: 歌名 - 歌手" / "edge"）覆盖；
            // 但切到新内容时必须立即刷新为 SMTC 原始值，避免旧标题残留
            // 浏览器Pro 正则已命中音乐（browserTabMusicPending）时彻底不显示原始值：保持上一次显示，
            // 等后端解析出真实歌名/歌手后由 applyBrowserMusicMeta 统一修改
            if (!browserTabMusicPending && !(currentIsBrowser.value && isBrowserMusic.value && !isNewTrack)) {
                currentSongName.value = song;
                currentArtistName.value = artist || t('unknownArtist');
            }

            if (isNewTrack) {
                currentBaseInfo.value = newTrackInfo;
                // 记录切歌时刻：用于 fallbackBrowserLogo 判断当前显示封面是否本首歌拉到的（避免晚到兜底覆盖真实封面）
                songChangeTime = Date.now();

                // 切歌时重置浏览器音乐判定，等封面/歌词获取结果再确认是音乐还是视频
                isBrowserMusic.value = false;

                // 切歌时，第一时间重置本地时间轴！
                if (!isWsActive) {
                    localPositionMs.value = positionMs; // 必须补上这行，否则新歌会继承老歌的时间！
                    lastLyricChangeTime = performance.now() + 2000;
                }

                // 彻底清除上首歌的残留歌词状态和强制渲染队列
                // 注意：WS 活跃时不能清空，否则会覆盖 WS 刚发来的新歌 init 歌词，
                // 且 WS 不会为同一首歌再发一次 init，导致歌词一直不显示、标题常驻
                if (!isWsActive) {
                    parsedLyrics.value = [];
                    lyricQueue.value = [];
                    currentMatchedIndex = -1;
                    renderQueue.length = 0;
                }

                // 切歌时立即把折叠态文本更新为 "标题 - 歌手"
                fillCollapsedWithTrackInfo();

                // PotPlayer：不做歌词匹配，清空可能残留的歌词队列，标题常驻显示
                if (isPotplayerSource.value) {
                    parsedLyrics.value = [];
                    lyricQueue.value = [];
                    currentMatchedIndex = -1;
                }

                // 切换播放内容时强制重新获取封面：清掉该曲目的封面缓存，避免沿用旧封面
                // 切换 SMTC 应用时同样刷新封面，确保新应用显示正确的封面
                coverCache.delete(newTrackInfo);
                blurredCoverCache.delete(newTrackInfo);

                // 统一走集中决策函数：按应用类型分派封面策略（浏览器/PotPlayer/bilibili/JustSolo/其他）
                // 浏览器Pro 正则已命中音乐时跳过：封面保持上一次显示（前端先显示上一次的封面），
                // 等后端元数据修正后由 applyBrowserMusicMeta 用真实歌名/歌手重新获取，避免先被原始值请求带偏
                if (!browserTabMusicPending) {
                    applyCoverForApp(newTrackInfo, song, artist, app_id_str, false, true);
                }

                // 仅在 WS 不活跃时，发起 HTTP 网络歌词兜底（PotPlayer 不拉歌词，标题常驻）
                // 切换 SMTC 应用后 WS 心跳可能仍属于旧应用，此时也立即用 HTTP 兜底，保证新歌歌词及时到位
                // 标题命中视频站后缀不拉歌词，保持视频模式
                if ((!isWsActive || appSwitched) && !isPotplayerSource.value && !isBrowserVideoTitle.value) {
                    invoke<string>('fetch_netease_lyrics', { songName: song, artistName: artist, durationMs })
                        .then(async (lrc) => {
                            if (appSwitched || Date.now() - lastWsLyricTime > 3000) {
                                if (lrc) {
                                    parsedLyrics.value = parseLrc(lrc);
                                    if (isBrowserProMode() && currentIsBrowser.value) {
                                        // 浏览器Pro：先做标签页判定，通过（标签页命中音乐）才判定为音乐模式
                                        const mode = await judgeBrowserMode(song, durationMs).catch((): 'music' | 'video' => 'music'); // 判定失败沿用歌词兜底
                                        // 标签页未命中音乐（判定为视频）→ 不动 SMTC 标题/歌手/封面，保持原样
                                        if (mode === 'music') {
                                            // 标签页正则已命中（lastTabPlayingResult 非空）时，judgeBrowserMode 内部已用解析值
                                            // 搜索过一次元数据并修正标题/歌手/封面，这里不再用 SMTC 原始值重复搜索（一次搜索即可，
                                            // 不同查询词会返回不同结果，导致歌手乱跳）；仅当正则未命中（关键词兜底判为音乐）时才补搜一次
                                            if (!lastTabPlayingResult) {
                                                applyBrowserMusicMeta(song, artist, durationMs);
                                            }
                                        }
                                    } else if (currentIsBrowser.value) {
                                        // 通用媒体 + 浏览器来源：拉到歌词直接判定为音乐，并把标题/歌手修正为真实音乐信息，封面 SMTC 优先、网络兜底
                                        markBrowserMusic();
                                        applyBrowserMusicMeta(song, artist, durationMs);
                                    } else {
                                        // 非浏览器来源：拉到歌词直接判定为音乐，不改 SMTC 标题/歌手/封面
                                        markBrowserMusic();
                                    }
                                    // 刚拉到歌词时，若时长仍为 0，用歌词反推补救
                                    if (currentDurationMs.value <= 0 && parsedLyrics.value.length > 0) {
                                        const lastLyric = parsedLyrics.value[parsedLyrics.value.length - 1];
                                        currentDurationMs.value = lastLyric.time + 8000;
                                    }
                                }
                            }
                        }).catch(() => { });
                }
            } else {
                // 同一首歌，仅在 WS 不活跃时使用 SMTC 进度校准
                if (!isWsActive && positionMs > 1000 && Math.abs(positionMs - localPositionMs.value) > 800) {
                    localPositionMs.value = positionMs - 250;
                }
                // 修复：SMTC 短暂无返回会往折叠态写入"未在播放歌曲"，同歌恢复后需重新填充标题，
                // 否则 currentTrackInfo 一直卡在"未在播放"，而展开态（currentSongName）仍正常
                if (currentTrackInfo.value.startsWith(t('noSongPlaying'))) {
                    fillCollapsedWithTrackInfo();
                }
                // 封面被清空过（如 SMTC 短暂断开）但沉浸背景还在时，补回圆形封面，避免"背景对、圆形空白"
                if (!coverUrl.value && blurredCoverUrl.value) {
                    refreshCoverOnResume();
                }
            }
        } else {
            // SMTC 未检测到播放器
            if (!isWsActive) {
                setSafeTrackInfo(`${t('noSongPlaying')} - ${getPlayerName()}`);
                isPlaying.value = false;
                // 圆形封面与沉浸背景同步清空，避免 SMTC 短暂断开后留下不一致的旧背景
                coverUrl.value = '';
                blurredCoverUrl.value = '';

                if (isMediaActive.value) {
                    isMediaActive.value = false;

                    if (isNewlyEnabled) {
                        showToast('已开启媒体控制，暂无音频播放', 'sys');
                        isNewlyEnabled = false;
                    } else if (!isFirstMediaCheck && isMusicCtlEnabled.value) {
                        showToast('无媒体活动，已切换为网速显示', 'sys');
                    }
                }
                isFirstMediaCheck = false;
            }
        }
    } catch (err) {
        console.error('音乐信息获取失败:', err);
    }
};

const showInfo = ref(false);
// 默认显示内容动态从本地缓存读取
const getPlayerName = () => {
    const key = localStorage.getItem('nsd_target_player') || 'netease';
    const map: Record<string, string> = {
        'netease': t('neteaseMusic'),
        'spotify': 'Spotify',
        'apple': 'Apple Music',
        'qqmusic': t('qqMusicFull'),
        'kugou': t('kugouMusicFull'),
        'echo': 'Echo Music',
        'lx-music': t('lxMusicFull'),
        'other': t('genericMediaFull'),
        'browserPro': t('browserPro')
    };
    return map[key] || t('unknownPlatform');
};

// SMTC 连上应用但没有有效标题时，把应用包名转成可读的应用名
const getConnectedAppName = (appId: string) => {
    const id = appId.toLowerCase();
    if (id.includes('edge')) return 'Microsoft Edge';
    if (id.includes('chrome')) return 'Google Chrome';
    if (id.includes('bilibili')) return '哔哩哔哩';
    if (id.includes('cloudmusic') || id.includes('netease')) return '网易云音乐';
    if (id.includes('spotify')) return 'Spotify';
    if (id.includes('qqmusic')) return 'QQ音乐';
    if (id.includes('justsolo')) return 'JustSolo';
    // 兜底：去掉 .exe 后缀后展示包名
    return id.replace(/\.exe$/i, '');
};

// 定义一个用于强制刷新的 key
const musicBoxKey = ref(0);

// 定义双行文本所需的单独变量
const currentSongName = ref(t('noSongPlaying'));
const currentArtistName = ref(getPlayerName());
const currentTrackInfo = ref(`${t('noSongPlaying')} - ${getPlayerName()}`);

// PotPlayer 无歌手元数据时，后端会把歌手占位为 "potplayer"。
// 此时不做歌词匹配，直接用标题当常驻歌词显示
const isPotplayerSource = computed(() => currentArtistName.value === 'potplayer');

// 视频类判定变化时，重新填充折叠态文本（音乐显示"标题 - 歌手"，视频只显示标题）
watch(isVideoPlayer, () => {
    if (displayMusic.value && currentSongName.value !== t('noSongPlaying')) {
        fillCollapsedWithTrackInfo();
    }
});

// 浏览器判定为播放音乐（拉到歌词）时，封面改走网络获取（歌词元数据更准，封面也以网络为准）
// 注意：isVideoPlayer 现为派生 computed，无需再手动同步视频标记
watch(isBrowserMusic, (now) => {
    if (now && currentIsBrowser.value) {
        const song = currentSongName.value;
        const artist = currentArtistName.value;
        const trackInfo = artist ? `${song} - ${artist}` : song;
        if (trackInfo && song && song !== t('noSongPlaying')) {
            applyCoverForApp(trackInfo, song, artist, currentAppIdStr.value, true, true);
        }
    }
});

watch(currentLanguage, () => {
    if (!displayMusic.value || currentSongName.value === t('noSongPlaying')) {
        currentSongName.value = t('noSongPlaying');
        currentArtistName.value = getPlayerName();
        currentTrackInfo.value = `${t('noSongPlaying')} - ${getPlayerName()}`;
    }
});

// 强制视觉渲染队列（绝对防闪烁/防空壳）
const renderQueue: string[] = [];
let isRendering = false;
// 强制渲染下一句（用于歌词第一句恰好等于标题占位文本时，强制歌词接管显示）
let forceRenderNext = false;
// 当前显示的是否为 "标题 - 歌手" 占位文本（而非真实歌词）
let isTitlePlaceholder = false;

const setSafeTrackInfo = (text: string, force = false) => {
    // 1. 终极过滤：剔除所有空白、零宽字符
    if (!text || text.replace(/[\s\u200B-\u200D\uFEFF\u3000]/g, '').length === 0) return;

    // 2. 防重判定：如果和当前屏幕上的一样，或者和队列排在最后的一样，拒收
    //    force=true 时跳过防重（用于歌词第一句恰好等于标题占位文本时，强制歌词接管显示）
    if (!force && text === currentTrackInfo.value && renderQueue.length === 0) return;
    if (!force && renderQueue.length > 0 && renderQueue[renderQueue.length - 1] === text) return;

    // 3. 扔进强制渲染队列，绝不使用 clearTimeout 取消任何一句话！
    renderQueue.push(text);
    if (force) forceRenderNext = true;
    drainRenderQueue();
};

const drainRenderQueue = () => {
    if (isRendering || renderQueue.length === 0) return;

    const nextText = renderQueue.shift();
    if (!nextText || (nextText === currentTrackInfo.value && !forceRenderNext)) {
        forceRenderNext = false;
        drainRenderQueue();
        return;
    }
    forceRenderNext = false;

    // 上锁！开始渲染新文字
    isRendering = true;
    currentTrackInfo.value = nextText;

    // 渲染的是真实歌词时，标题占位标记失效
    if (isPlaying.value && parsedLyrics.value.length > 0) {
        isTitlePlaceholder = false;
    }

    // 计算并赋予歌词扫描时长
    if (isPlaying.value && parsedLyrics.value.length > 0) {
        // 原本：const lineDurationSec = getCurrentLineDuration() / 1000;

        // 修改为：乘以 0.85，意味着在整句时间的 85% 时就扫描完毕，提速了 15%
        const lineDurationSec = (getCurrentLineDuration() / 1000) * 0.85;

        scanDuration.value = `${lineDurationSec}s`;
    } else {
        scanDuration.value = '0s';
    }

    // 每次切换歌词后重新计算滚动距离（等 DOM 更新完再量宽度，防止拿到旧宽度/0）
    nextTick(() => {
        setTimeout(() => {
            if (displayMusic.value) {
                calculateScroll();
            } else {
                scrollDist.value = 0;
            }
        }, 100);
    });

    // 4. 动画锁：强制等待 350ms，确保 Vue 的 out-in 动画结束后才渲染下一句
    setTimeout(() => {
        isRendering = false;
        drainRenderQueue();
    }, 350);
};

// 音乐滚动相关变量
const maskBoxRef = ref<HTMLElement | null>(null);
const textInnerRef = ref<HTMLElement | null>(null);
const scrollDist = ref(0);
const scrollDuration = ref('0s');
const scanDuration = ref('0s');

// 展开态标题（B站/浏览器视频等长标题）滚动相关变量
const expandedTitleBoxRef = ref<HTMLElement | null>(null);
const expandedTitleRef = ref<HTMLElement | null>(null);
const expandedTitleScrollDist = ref(0);
const expandedTitleScrollDuration = ref('0s');

// 计算展开态标题是否需要滚动：标题超出容器宽度时，以固定速度来回滚动
const calculateExpandedTitleScroll = () => {
    if (!expandedTitleRef.value || !expandedTitleBoxRef.value) return;
    const textWidth = expandedTitleRef.value.getBoundingClientRect().width;
    const containerWidth = expandedTitleBoxRef.value.clientWidth;
    if (textWidth > containerWidth) {
        // 把文字末尾拖到最右，外加 5px 呼吸空间
        expandedTitleScrollDist.value = Math.ceil(textWidth - containerWidth + 5);
        // 固定 20px/s 的滚动速度（视频类标题放慢），来回 ping-pong（滚动占 70%，开头/末尾各停 15%）
        const timeToMove = expandedTitleScrollDist.value / 20;
        expandedTitleScrollDuration.value = `${(timeToMove / 0.7).toFixed(2)}s`;
    } else {
        expandedTitleScrollDist.value = 0;
    }
};

// 标题变化、展开/折叠切换、或音乐/视频判定变化时，重新计算展开态标题的滚动
watch([currentSongName, isMusicExpanded, isVideoPlayer], async () => {
    await nextTick();
    // 点击展开立即计算一次，让标题马上开始滚动（此时容器还是折叠态宽度，滚动距离偏大）
    if (isMusicExpanded.value) {
        calculateExpandedTitleScroll();
    } else {
        expandedTitleScrollDist.value = 0;
    }
    // 等 0.4s 宽度过渡 + 形变动画结束再量一次，用稳定后的宽度修正滚动距离
    setTimeout(() => {
        if (isMusicExpanded.value) {
            calculateExpandedTitleScroll();
        } else {
            expandedTitleScrollDist.value = 0;
        }
    }, 500);
});

// 获取当前歌词句的演唱时长（毫秒），用于动态滚动调速
const getCurrentLineDuration = (): number => {
    const lyrics = parsedLyrics.value;
    const idx = currentMatchedIndex;
    if (lyrics.length === 0 || idx < 0 || idx >= lyrics.length) return 4000;

    // 有下一句：用两句时间差作为本句时长
    if (idx + 1 < lyrics.length) {
        return Math.max(lyrics[idx + 1].time - lyrics[idx].time, 400);
    }

    // 最后一句：用歌曲总时长减去本句起始时间，兜底 4 秒
    const remain = currentDurationMs.value - lyrics[idx].time;
    return remain > 800 ? remain : 4000;
};

// 折叠态容器宽度缓存：展开/收缩尺寸动画期间，容器实时宽度会被拉宽或收窄，
// 不能用它来计算滚动距离，否则会得到错误结果甚至把滚动距离算成 0
let collapsedMaskWidth = 0;

// 核心计算函数：判断文本是否超出容器，并动态调整滚动速度和时长
const calculateScroll = () => {
    if (!textInnerRef.value || !maskBoxRef.value) return;

    const textWidth = textInnerRef.value.getBoundingClientRect().width;
    const realContainerWidth = maskBoxRef.value.clientWidth;

    // 只有折叠态且尺寸动画结束（尺寸稳定）时才更新缓存；
    // 展开中 / 展开态 / 收缩动画中都沿用折叠态宽度，保证滚动距离稳定不跳变
    let containerWidth: number;
    if (!isMusicExpanded.value && !isSizeAnimating) {
        collapsedMaskWidth = realContainerWidth;
        containerWidth = realContainerWidth;
    } else {
        containerWidth = collapsedMaskWidth || realContainerWidth;
    }

    // 让文字末尾滚到容器最右侧，完整显示整句歌词（而非只滚到 75% 安全区）
    const safeWidth = containerWidth;

    // 只要文字超出容器宽度就必须开始滚动
    if (textWidth > safeWidth) {
        // 计算滚动距离：把文字的末尾拖到最右，外加 5px 的微小呼吸空间
        scrollDist.value = Math.ceil(textWidth - safeWidth + 5);

        // 动态滚动速度：滚动距离已由「歌词长度 - 灵动岛安全区」决定，
        // 速度再参照「当前歌词句的演唱时长」，让滚动跟随节奏而非固定 30px/s
        const lineDurationSec = getCurrentLineDuration() / 1000;

        // 本句实际展示时长：歌词队列消费闸门是 800ms，短句不会低于这个展示窗口
        const displayWindowSec = Math.max(lineDurationSec, 0.8);

        // 整段动画 = 开头停 15% + 滚动 70% + 末尾停 15%，纯滚动占歌词时长的 70%
        const timeToMove = displayWindowSec * 0.7;

        // 速度保护：过快看不清、过慢拖沓，钳制在 18~90 px/s
        const rawSpeed = scrollDist.value / timeToMove;
        const speed = Math.min(Math.max(rawSpeed, 18), 90);
        const safeTimeToMove = scrollDist.value / speed;

        // 由纯滚动时间反推总动画时长（纯滚动占 70%）
        let totalDuration = safeTimeToMove / 0.7;

        // 视频类播放源（B站/浏览器视频）：标题常驻不随歌词变化，用固定慢速滚动，
        // 且不受歌词展示时长限制，让长标题从容滚完再回来
        if (isVideoPlayer.value) {
            const slowTimeToMove = scrollDist.value / 20;
            totalDuration = slowTimeToMove / 0.7;
        } else {
            // 滚到底的保证：动画总时长不得超过本句展示时长。
            // 否则长句被 90px/s 钳制 / 4.5s 保底拉长后，还没滚到末尾就被下一句顶掉，
            // 表现为「有时候歌词滚不到底」。文字在展示期内即可滚完并停住结尾。
            totalDuration = Math.max(Math.min(totalDuration, displayWindowSec), 0.8);
        }

        scrollDuration.value = `${totalDuration.toFixed(2)}s`;
    } else {
        scrollDist.value = 0;
    }
};

// 核心修复 2：只监听 displayMusic 的显隐切换，展开/折叠不再重算滚动，
// 让滚动动画在展开期间继续在后台推进，折叠后自然回到「理论未展开时」的位置。
// 歌词切换时的滚动重算已移入 drainRenderQueue。
watch(displayMusic, async () => {
    await nextTick();
    setTimeout(() => {
        if (displayMusic.value) {
            calculateScroll();
        } else {
            // 切到其他界面（比如网速）时，归零重置
            scrollDist.value = 0;
        }
    }, 100);
    // 尺寸形变动画（约 500ms）结束后再算一次：
    // 消息通知消失回到媒体折叠态时，若折叠态宽度缓存尚未建立（collapsedMaskWidth 为 0），
    // 首次计算会因容器仍处于消息宽度而误判为无需滚动；动画结束后用稳定宽度重算即可恢复滚动
    setTimeout(() => {
        if (displayMusic.value) {
            calculateScroll();
        }
    }, 600);
});

let lastRx = 0;
let lastTx = 0;
let speedTimer: number;
let pingTimer: number;
let musicTimer: number;
let notifyTimer: number;

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
        let rxDiff = currentRx - lastRx;
        let txDiff = currentTx - lastTx;

        if (lastRx !== 0) {

            // 网速为负说明网络计数器被重置（网卡重连/断网），判定为断网
            if (rxDiff < 0 || txDiff < 0) {
                networkStatus.value = 'error';
            }

            downloadSpeed.value = formatSpeed(rxDiff);
            uploadSpeed.value = formatSpeed(txDiff);

            // 负数置零，避免显示负网速
            if (rxDiff < 0) downloadSpeed.value = '0 B/s';
            if (txDiff < 0) uploadSpeed.value = '0 B/s';

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
        } else {
            try {
                const latency = await invoke<number>('get_network_latency');
                if (rxDiff > 0 && txDiff > 0) {
                    networkStatus.value = 'good';      // 有流量变化，说明网络正常
                } else if (rxDiff <= 0)
                    networkStatus.value = 'error';     // 无流量变化，说明网络异常
                console.log(latency, rxDiff, txDiff);
            } catch (error) {
                networkStatus.value = 'error';
            }
        }
        lastRx = currentRx;
        lastTx = currentTx;
    } catch (error) {
        console.error('流量获取失败:', error);
    }
};

// 连续 ping 失败计数：避免单次网络抖动就误判断网
let consecutiveFailures = 0;
// 连续失败 2 次（约 11s）才确认断网，单次失败只降为黄灯
const FAIL_THRESHOLD = 2;

// 通过真实延迟控制状态灯（加入大流量避让 + 连续失败确认）
const checkNetworkLatency = async () => {
    try {
        const latency = await invoke<number>('get_network_latency');

        // 拿到当前流量统计，计算流量变化
        const [currentRx, currentTx] = await invoke<[number, number]>('get_network_stats');
        let rxDiff = currentRx - lastRx;
        let txDiff = currentTx - lastTx;

        // 只要能拿到延迟数字，说明网络肯定是通的，立即清零失败计数
        consecutiveFailures = 0;
        if (latency < 150) {
            if (rxDiff > 0 && txDiff > 0) { // 有流量变化，说明网络正常
                networkStatus.value = 'good';      // 延迟优秀，绿色
            }
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
            return;
        }

        // 3. 连续多次失败才确认断网（单次抖动只降为黄灯，避免误判断网）
        consecutiveFailures++;
        if (consecutiveFailures >= FAIL_THRESHOLD) {
            networkStatus.value = 'error';
        } else {
            networkStatus.value = 'warning';
        }
    }
};

// 监听网络状态变化，触发系统通知
watch(networkStatus, (newStatus, oldStatus) => {
    // 忽略初始化时的变化，确保是真的状态翻转
    if (oldStatus && oldStatus !== newStatus) {
        if (newStatus === 'error') {
            showToast(t('networkDisconnected'), 'sys');
        } else if (newStatus === 'good' && oldStatus === 'error') {
            showToast(t('networkRestored'), 'sys');
        }
    }
});

// 极速强制居中核心函数
const adjustWindowPosition = async () => {
    try {
        const appWindow = getCurrentWindow();
        await new Promise((resolve) => setTimeout(resolve, 50)); // 给点缓冲，等待显示器底层加载

        let monitor = await currentMonitor();
        if (!monitor) {
            const monitors = await availableMonitors();
            if (monitors.length > 0) monitor = monitors[0];
            else return;
        }

        const scaleFactor = monitor.scaleFactor;
        const { w, h } = getBaseSize();
        const finalW = w * appScale.value;
        const finalH = h * appScale.value;

        // 1. 设置正确尺寸
        await appWindow.setSize(new PhysicalSize(
            Math.round(finalW * scaleFactor),
            Math.round(finalH * scaleFactor)
        ));

        const windowSize = await appWindow.innerSize();
        
        // 2. 计算居中坐标 (顶部留 12px 间距)
        const x = monitor.position.x + (monitor.size.width - windowSize.width) / 2;
        const y = monitor.position.y + (12 * scaleFactor);

        // 3. 应用位置
        await appWindow.setPosition(new PhysicalPosition(Math.round(x), Math.round(y)));
    } catch (error) {
        console.error('居中调整失败:', error);
    }
};

const onEnter = (el: Element, done: () => void) => {
    // 确保入场时窗口可以被正常点击
    getCurrentWindow().setIgnoreCursorEvents(false).catch(() => { });

    const HTMLElement = el as HTMLElement;
    HTMLElement.style.transformOrigin = 'center top';
    let start = performance.now();

    // 根据用户选择的弹性风格，调整出场缩放的物理曲线参数
    const isStiff = nsdSpringStyle.value === 'stiff';
    const freq = isStiff ? 3.2 : 2.0;
    const decay = isStiff ? 18.0 : 10.5;
    const duration = isStiff ? 350 : 600;

    const animate = (time: number) => {
        let t = (time - start) / 1000;
        let progress = (time - start) / duration;

        let scale = 1 - Math.cos(freq * t * 2 * Math.PI) * Math.exp(-decay * t);
        let opacity = Math.min(1, progress * 4);

        HTMLElement.style.transform = `scale(${scale})`;
        HTMLElement.style.opacity = opacity.toString();

        if (progress < 1) {
            requestAnimationFrame(animate);
        } else {
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
    const duration = 300;

    // 设置一个标志位，防止重复执行
    let isFinished = false;

    const finishAnimation = () => {
        if (isFinished) return;
        isFinished = true;
        done();

        // 不再在这里物理隐藏窗口：统一交给顶部兜底 watch，在离开动画结束后
        // 把窗口缩成 1×1（配合鼠标透传），避免 hide/show 造成的闪烁与重显延迟；
        // 若动画超时期间灵动岛又被呼出，watch 的 400ms 定时器会自行放弃缩放。
    };

    const animate = (time: number) => {
        if (isFinished) return;
        let progress = (time - start) / duration;

        let scale = 1 - Math.pow(progress, 3);
        let opacity = 1 - progress * 1.5;

        HTMLElement.style.transform = `scale(${Math.max(0, scale)})`;
        HTMLElement.style.opacity = Math.max(0, opacity).toString();

        if (progress < 1) {
            requestAnimationFrame(animate);
        } else {
            finishAnimation();
        }
    };
    requestAnimationFrame(animate);

    // 防休眠保险：即使系统冻结了 requestAnimationFrame，
    // 时间一到（350ms）也强制结束动画并隐藏物理窗口
    setTimeout(() => {
        if (!isFinished) {
            // 兜底：若动画卡死，强制将透明度归零，防止残留像素拦截鼠标
            HTMLElement.style.opacity = '0';
            finishAnimation();
        }
    }, duration + 50);
};

let mouseDownX = 0;
let mouseDownY = 0;
let isMouseDown = false;

const handleMouseDown = (event: MouseEvent) => {
    if ((event.target as HTMLElement).closest('.ctl-btn')) return;

    // 无论有没有锁定，都必须老老实实记录坐标，给后面的“点击展开”提供判断依据！
    mouseDownX = event.clientX;
    mouseDownY = event.clientY;
    isMouseDown = true;
};

const handleMouseMove = async (event: MouseEvent) => {
    if (!isMouseDown) return;
    if (isSizeAnimating) return;
    if (isMusicExpanded.value || isMusicExpanding.value || isMsgActive.value || displaySysToast.value) {
        isMouseDown = false;
        return;
    }

    // 位置锁定：禁止拖拽（不影响点击展开——isMouseDown 保持原状，mouseDown 坐标仍有效）
    if (isPositionLocked.value) return;

    // 直接拖拽，再也没有 isPositionLocked 拦截了
    if (Math.abs(event.clientX - mouseDownX) > 5 || Math.abs(event.clientY - mouseDownY) > 5) {
        isMouseDown = false;
        try {
            await getCurrentWindow().startDragging();
            // 拖拽结束（系统拖拽循环返回）后保存新位置，重启时恢复
            await new Promise((resolve) => setTimeout(resolve, 80));
            await saveIslandPosition();
        } catch (error) {
            console.error('拖拽失败:', error);
        }
    }
};

const handleMouseUp = async () => {
    isMouseDown = false;
};

const handleRightClick = async (event: MouseEvent) => {
    event.preventDefault();
    event.stopPropagation(); // 阻止冒泡

    // 如果音乐灵动岛正在展开或已完全展开，强制禁止呼出右键菜单
    if (isMusicExpanded.value || isMusicExpanding.value || isMsgActive.value || displaySysToast.value) {
        return;
    }

    // 打开控制台
    const openSettingsItem = await MenuItem.new({
        text: t('openConsole'),
        id: 'open_settings',
        action: async () => {
            await emit('open-settings-panel');
            showToast(t('consoleOpened'));
        }
    });

    // 切换流光边框
    const toggleGlowBorderItem = await MenuItem.new({
        text: isGlowBorderEnabled.value ? t('disableGlowBorder') : t('enableGlowBorder'),
        id: 'toggle_glow_border',
        enabled: true,
        action: () => {
            isGlowBorderEnabled.value = !isGlowBorderEnabled.value;
            localStorage.setItem('nsd_glow_border', String(isGlowBorderEnabled.value));
            showToast(isGlowBorderEnabled.value ? t('glowBorderEnabled') : t('glowBorderDisabled'));
        }
    });

    // 锁定/解锁位置：锁定后禁止拖拽岛
    const togglePositionLockItem = await MenuItem.new({
        text: isPositionLocked.value ? t('unlockPosition') : t('lockPosition'),
        id: 'toggle_position_lock',
        action: () => {
            isPositionLocked.value = !isPositionLocked.value;
            localStorage.setItem('nsd_position_locked', String(isPositionLocked.value));
            showToast(isPositionLocked.value ? t('positionLocked') : t('positionUnlocked'));
        }
    });

    // 重置位置
    const resetPositionItem = await MenuItem.new({
        text: t('resetPosition'),
        id: 'reset_position',
        action: async () => {
            localStorage.removeItem(SAVED_POS_KEY); // 清除拖拽保存的位置，重置后不再恢复
            await adjustWindowPosition();
            showToast(t('positionReset'));
        }
    });

    // 关闭灵动岛
    const closeItem = await MenuItem.new({
        text: t('close'),
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
    await menu.append(togglePositionLockItem);
    await menu.append(resetPositionItem);
    await menu.append(closeItem);

    // 4. 弹出菜单
    try {
        isMenuOpen.value = true; // 弹出前标记菜单已打开
        await menu.popup(position);
    } catch (error) {
        console.error('菜单弹出失败:', error);
    } finally {
        isMenuOpen.value = false; // 无论点击菜单项还是取消，都恢复置顶状态
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

// 记录全局灵动岛是否正在执行形变动画
let isSizeAnimating = false;
let sizeAnimTimer: number | null = null;
// 形变请求序号 latestAnimationRequest 已上移至文件顶部 watch(isIslandVisible) 之前声明（TDZ 要求）

// 在顶部声明缩放变量
const appScale = ref(Number(localStorage.getItem('nsd_app_scale')) || 1.0);

// 监听缩放变化，直接修改 html 根节点的 zoom（Webkit 渲染最完美的缩放方式）
watch(appScale, (newScale) => {
    (document.documentElement.style as any).zoom = newScale;
}, { immediate: true });

// 灵动岛尺寸动画核心（防漂移、防裁切、防打断抖动）
const animateIslandSize = async (targetWidth: number, targetHeight: number) => {
    // 岛隐藏（窗口已/将缩成 1×1）时不做形变动画：
    // Rust 侧 start_island_animation 会用 GetWindowRect 取锚点中心，
    // 对着 1×1 窗口取到的锚点是错的，之后所有动画都围着错误中心定位，偏移被固化。
    // 先作废所有在途请求（防止缩 1×1 后动画收尾又把窗口撑大），再放弃本次。
    // 注意：仍需同步 DOM 逻辑宽高（正常路径由动画结束的 island-resize 事件同步），
    // 否则隐藏期间态切换（如音乐停止）后，恢复显示时 DOM 停在旧宽度，内容被窗口裁切。
    if (!isIslandVisible.value) {
        latestAnimationRequest++;
        currentWidth.value = targetWidth * appScale.value;
        currentHeight.value = targetHeight * appScale.value;
        return;
    }
    const myRequest = ++latestAnimationRequest;
    try {
        // 核心：计算最终的缩放尺寸
        const finalWidth = targetWidth * appScale.value;
        const finalHeight = targetHeight * appScale.value;

        // 1. 触发形变前上锁
        isSizeAnimating = true;
        if (sizeAnimTimer) clearTimeout(sizeAnimTimer);

        sizeAnimTimer = window.setTimeout(() => {
            isSizeAnimating = false;
        }, 500);

        const appWindow = getCurrentWindow();
        const realSize = await appWindow.innerSize();

        // 若在异步读取窗口尺寸期间，又有更新的尺寸请求到来，则放弃本次，
        // 让最新请求接管动画，避免旧动画覆盖新状态（如收缩动画覆盖消息展开）
        if (myRequest !== latestAnimationRequest) return;

        const scaleFactor = window.devicePixelRatio;

        const realStartW = realSize.width / scaleFactor;
        const realStartH = realSize.height / scaleFactor;

        await invoke('start_island_animation', {
            startWidth: realStartW,
            startHeight: realStartH,
            targetWidth: finalWidth,    // 传给 Rust 放大后的目标宽度
            targetHeight: finalHeight,  // 传给 Rust 放大后的目标高度
            springStyle: nsdSpringStyle.value
        });
    } catch (err) {
        console.error('呼叫 Rust 动画失败:', err);
        isSizeAnimating = false;
    }
};

// 动画锁与等待队列标志
let isAnimationLocked = false;
let isPendingCollapse = false;

// 音乐控制器自动收缩方法
const collapseMusic = () => {
    if (!isMusicExpanded.value && !isMusicExpanding.value) return;

    // 【核心逻辑】：如果正在猛烈展开中，绝对不打断！把收缩请求挂起，等它展开完自动执行。
    if (isAnimationLocked) {
        isPendingCollapse = true;
        return;
    }

    isMusicExpanded.value = false;
    isMusicExpanding.value = false;
    isPendingCollapse = false; // 清除队列

    if (musicExpandAnimTimer) {
        clearTimeout(musicExpandAnimTimer);
        musicExpandAnimTimer = null;
    }

    // 消息通知正在显示时，只复位媒体展开状态，不收缩岛体尺寸，
    // 否则会把正在展示的消息压回折叠尺寸（消息应保持消息展开宽度）
    if (isMsgActive.value || displayActivity.value) return;

    const { w, h } = getBaseSize();
    animateIslandSize(w, h);
};

// 音乐控制器点击展开方法
const expandMusic = (e: MouseEvent) => {
    if (Math.abs(e.clientX - mouseDownX) > 5 || Math.abs(e.clientY - mouseDownY) > 5) return;
    if ((e.target as HTMLElement).closest('.ctl-btn')) return;

    if (isMusicExpanded.value || isMusicExpanding.value) return;

    isMusicExpanding.value = true;
    isPendingCollapse = false;  // 重置待办任务
    isAnimationLocked = true;   // 锁定动画，防止展开期间被其他操作打断

    animateIslandSize(nsdBaseWidth.value + 95, nsdBaseHeight.value + 4);

    // 2. 延迟 120 毫秒后，打断缩小，直接展开
    musicExpandAnimTimer = window.setTimeout(() => {
        isMusicExpanded.value = true;
        isMusicExpanding.value = false;
        animateIslandSize(nsdMusicExpandedWidth.value, 135);

        // 3. 根据 Rust 端的弹簧衰减频率，约 400ms 后动画结束，此时解锁
        setTimeout(() => {
            isAnimationLocked = false;

            // 检查：若展开期间用户鼠标已移走，则补发收缩命令
            if (isPendingCollapse) {
                isPendingCollapse = false;
                collapseMusic();
            }
        }, 400);
    }, 120);
};

// 鼠标离开灵动岛时：收缩音乐岛
const handleMouseLeave = () => {
    if (!isMusicExpanded.value && !isMusicExpanding.value) return;

    // 直接呼叫收缩；若动画锁着，collapseMusic 会记录待办稍后执行
    collapseMusic();
};

// 鼠标重新移入灵动岛时：取消待执行的收缩
const handleMouseEnter = () => {
    // 若之前移出留下了收缩待办，但动画未播完鼠标又回来，则取消该待办
    isPendingCollapse = false;
};

watch(displayMusic, (newVal: boolean) => {
    if (!newVal) {
        collapseMusic(); // 音乐岛被隐藏（轮换或手动关闭）时立即收缩
    }
});

// 引入默认图标作为兜底
import defaultLogo from '../assets/logo.png';
const currentMsgIcon = ref(defaultLogo);

// 浏览器/视频类应用的内置 logo 封面（必须 import 引用，Vite 打包时才会重写资源路径）
import bilibiliLogo from '../assets/bilibili-logo.png';
import edgeLogo from '../assets/edge-logo.png';
import chromeLogo from '../assets/chrome-logo.png';
import potplayerLogo from '../assets/potplayer-logo.jpg';

// 剪贴板链接通知卡片图标
import clipboardIcon from '../assets/Clipboard.png';
import openLinkIcon from '../assets/open_the_link.png';

const APP_COVER_LOGOS = [bilibiliLogo, edgeLogo, chromeLogo, potplayerLogo];
const APP_COVER_LOGO_MAP: Record<string, string> = {
    bilibili: bilibiliLogo,
    edge: edgeLogo,
    chrome: chromeLogo,
    potplayer: potplayerLogo,
};

// 图标映射器
const getAppIcon = (appName: string) => {
    const name = appName.toLowerCase();

    if (name.includes('qq')) {
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
    const appWindow = getCurrentWindow();

    // 启动剪贴板链接检测轮询（默认开启）
    if (enableClipboard.value) {
        startClipboardPolling();
    }

    // 启动活动池事件监听（30Hz 快照推送）
    startActivityPoolListening();

    window.addEventListener('blur', collapseMusic);

    document.addEventListener('contextmenu', (e) => {
        e.preventDefault();
    }, { capture: true }); // 使用捕获阶段，确保先于 Tauri 底层拦截

    // 接收自定义显示配置指令
    await listen<{ enabled: boolean, slots: (string | null)[] }>('control-custom-display', (event) => {
        enableCustomDisplay.value = event.payload.enabled;
        customSlots.value = event.payload.slots;

        // 检查自定义槽位中是否有 fps，如果有则自动唤醒采集
        checkAndToggleFpsPlugin();

        if (!isMsgActive.value && !displayActivity.value && !displaySysToast.value && !isMusicExpanded.value && !isMusicExpanding.value) {
            const { w, h } = getBaseSize();
            animateIslandSize(w, h);
        }
    });

    // 音乐控制器状态监听器
    await listen<{ enabled: boolean }>('control-music-ctl', (event) => {
        const isEnabled = event.payload.enabled;
        isMusicCtlEnabled.value = isEnabled;
        if (isEnabled) {
            enableSysResource.value = false; // 开启音乐，关资源监控
            if (localStorage.getItem('nsd_glow_border') === null) {
                isGlowBorderEnabled.value = true;
                localStorage.setItem('nsd_glow_border', 'true');
            }
            isMediaActive.value = true;
            isNewlyEnabled = true;
            showInfo.value = false;
            musicBoxKey.value++;
            // 启动 SMTC 时主动尝试连接一次 WS（initWebSocket 内部有一次性保护，不会重复连接）
            initWebSocket();
        } else {
            stopWebSocket();
            isMediaActive.value = true;
            isNewlyEnabled = false;
        }
    });

    // 监听个性化中心发来的同步指令
    await listen<any>('sync-dynamic-settings', async (event) => {
        const data = event.payload;
        nsdBaseWidth.value = Number(data.baseWidth);
        nsdBaseHeight.value = Number(data.baseHeight);
        nsdMusicBaseWidth.value = Number(data.musicBaseWidth) || 260;
        nsdMusicExpandedWidth.value = Number(data.musicExpandedWidth);
        nsdMsgExpandedWidth.value = Number(data.msgExpandedWidth);
        nsdBorderRadius.value = Number(data.borderRadius);
        nsdSpringStyle.value = data.springStyle;
        nsdLyricDelay.value = Number(data.lyricDelay) || 0;

        // 检测重绘逻辑
        const oldScale = appScale.value;
        appScale.value = Number(data.appScale) || 1.0;

        // 如果缩放比例被用户拖动改变了，强制刷新当前展现的尺寸
        if (oldScale !== appScale.value) {
            if (isMusicExpanded.value) {
                animateIslandSize(nsdMusicExpandedWidth.value, 135);
            } else if (isMsgActive.value) {
                animateIslandSize(nsdMsgExpandedWidth.value, 65);
            } else if (displayActivity.value) {
                animateIslandSize(Math.max(nsdMsgExpandedWidth.value, 320), 70);
            } else {
                const { w, h } = getBaseSize();
                animateIslandSize(w, h);
            }
        }

        // 收到设置修改后，如果此时没有展开音乐或显示通知，则立即触发形变更新外观！
        if (!isMsgActive.value && !displayActivity.value && !displaySysToast.value && !isMusicExpanded.value && !isMusicExpanding.value) {
            const { w, h } = getBaseSize();
            animateIslandSize(w, h);
        }
    });

    // 监听控制台发来的资源监控开关
    await listen<{ enabled: boolean }>('control-sys-resource', (event) => {
        const isEnabled = event.payload.enabled;
        enableSysResource.value = isEnabled;
        if (isEnabled) {
            isMusicCtlEnabled.value = false; // 开启资源监控，关音乐控制器
            stopWebSocket();
        }
    });

    // 监听控制台发来的剪贴板读取开关
    await listen<{ enabled: boolean }>('control-clipboard', (event) => {
        const isEnabled = event.payload.enabled;
        enableClipboard.value = isEnabled;
        if (isEnabled) {
            startClipboardPolling();
        } else {
            stopClipboardPolling();
        }
    });

    // 监听 Rust 底层发来的资源数据
    await listen<{ cpu: number, ram: number }>('resource-event', (event) => {
        cpuUsage.value = event.payload.cpu;
        ramUsage.value = event.payload.ram;
    });

    // 监听系统底层事件（音量、电源）
    await listen<string>('system-event', (event) => {
        let text = event.payload;
        const volumeMatch = text.match(/当前系统音量 (\d+)%/);
        if (volumeMatch) {
            text = t('systemVolume', { volume: volumeMatch[1] });
        } else if (text === '正在使用电池供电') {
            text = t('batteryPowered');
        }
        showToast(text, 'sys');
    });

    // 监听电量显示
    await listen<{ state: 'charging' | 'discharging', percent: number }>('battery-event', (event) => {
        const { state, percent } = event.payload;

        if (state === 'charging') {
            showToast(t('powerPlugged', { percent }), 'battery-charge');
        } else if (state === 'discharging' && percent <= 20) {
            // 这里还可以加入防抖：只在刚掉到 20%、10%、5% 等关键节点触发一次，避免疯狂弹窗
            showToast(t('batteryLow', { percent }), 'battery-low');
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

    // 监听静默模式开关
    await listen<{ enabled: boolean }>('control-msg-mode', async (event) => {
        isMsgModeEnabled.value = event.payload.enabled;
        if (isMsgModeEnabled.value) {
            // 进入静默：退出全屏悬停待命，避免“瞄一眼”与静默意图冲突
            stopFsHoverMode();
            // 静默模式开启时：无活跃事件则隐藏，有则保持显示
            if (!shouldShowInQuietMode.value && isIslandVisible.value) {
                isIslandVisible.value = false;
            } else if (shouldShowInQuietMode.value && !isIslandVisible.value) {
                await invoke('show_window_no_activate', { label: 'widget' });
                isIslandVisible.value = true;
            }
        } else {
            // 静默模式关闭时，恢复常驻显示
            await invoke('show_window_no_activate', { label: 'widget' });
            isIslandVisible.value = true;
        }
    });

    await listen<{ language: AppLanguage }>('control-language', (event) => {
        currentLanguage.value = event.payload.language;
    });

    // 监听控制台发来的“全屏自动隐藏”配置变更
    await listen<{ enabled: boolean }>('control-autohide-fs', async (event) => {
        isAutoHideEnabled.value = event.payload.enabled;
        // 全屏期间若用户关闭“全屏自动隐藏”，应立即把岛恢复显示；
        // 不区分是否处于悬停唤起待命（悬停唤出被关闭时的“仅隐藏”路径同样适用）
        if (!event.payload.enabled && wasVisibleBeforeFullscreen) {
            stopFsHoverMode();
            wasVisibleBeforeFullscreen = false;
            await invoke('show_window_no_activate', { label: 'widget' });
            setTimeout(() => {
                isIslandVisible.value = true;
            }, 40);
        }
    });

    // 监听控制台发来的“全屏隐藏悬停唤出”配置变更
    await listen<{ enabled: boolean }>('control-autohide-fs-hover', async (event) => {
        fsHoverWakeEnabled.value = event.payload.enabled;
        if (event.payload.enabled) {
            // 全屏隐藏期间重新开启悬停：从“仅隐藏”状态恢复到可悬停唤起的待命状态
            if (isAutoHideEnabled.value && wasVisibleBeforeFullscreen && !fsHoverActive) {
                await startFsHoverMode();
            }
        } else if (fsHoverActive && wasVisibleBeforeFullscreen) {
            // 关闭悬停唤出：若岛正在全屏中显示则立即收起，若已隐藏则保持隐藏；
            // 两种情形都停止轮询，退出全屏后再按进入前的状态恢复
            if (isIslandVisible.value) {
                isIslandVisible.value = false;
            }
            stopFsHoverMode();
        }
    });

    // 监听 Rust 发来的系统级全屏状态变化
    await listen<boolean>('fullscreen-changed', async (event) => {
        const isFullscreen = event.payload;

        // 未开启“全屏自动隐藏”时直接忽略
        if (!isAutoHideEnabled.value) return;

        if (isFullscreen) {
            // 检测到进入全屏：若灵动岛当前可见则收起
            if (isIslandVisible.value) {
                wasVisibleBeforeFullscreen = true;
                if (fsHoverWakeEnabled.value) {
                    // 已开启悬停唤出：进入“收起 + 可悬停唤起”的待命状态
                    await startFsHoverMode();
                } else {
                    // 未开启悬停唤出：仅收起并保持隐藏直至退出全屏
                    // （先记录探测区域位置，便于全屏期间重新开启悬停时仍能准确定位）
                    await captureFsHoverSlot();
                    isIslandVisible.value = false;
                }
            }
        } else {
            // 退出全屏：先退出悬停待命（停止光标轮询与相关定时器），再按进入全屏前的状态恢复
            const needRestore = wasVisibleBeforeFullscreen;
            stopFsHoverMode();
            if (needRestore) {
                await invoke('show_window_no_activate', { label: 'widget' });

                // 等待 40ms 让透明窗口完成挂载，再切换 v-show 触发入场动画，防止闪烁
                setTimeout(() => {
                    isIslandVisible.value = true;
                }, 40);

                wasVisibleBeforeFullscreen = false; // 恢复动作已完成，清除标记
            }
        }
    });

    try {
        await appWindow.innerPosition();
    } catch (e) { }

    // 在启动调整位置前，先校准初始宽高变量
    const { w, h } = getBaseSize();
    currentWidth.value = w * appScale.value;
    currentHeight.value = h * appScale.value;

    // 检查本地记录的灵动岛开关状态
    const isWidgetEnabled = localStorage.getItem('nsd_widget_visible') !== 'false';

    // 启动定位：优先恢复用户拖拽后保存的位置（含静默模式/岛关闭的启动路径，
    // 否则静默模式唤起时岛会出现在窗口默认位置而非用户拖放处）；
    // 无保存或已失效（显示器拔掉等）时回退顶部居中。
    const restored = await restoreSavedPosition();
    if (!restored) {
        await adjustWindowPosition();
    }

    // 只有在用户开启了灵动岛且没开静默模式时，启动才自动拉开灵动岛
    if (isWidgetEnabled && !isMsgModeEnabled.value) {
        // 位置就绪后，再让透明的 OS 窗口容器显示
        await invoke('show_window_no_activate', { label: 'widget' });
        
        // 切换 v-show 显示岛体内容（窗口此前已显示且定位完成，不会在左上角闪烁）
        isIslandVisible.value = true;

        // 延时加固：防止某些显示器的 DPI 汇报过慢，0.5秒后再居中夯实一次
        // （仅未恢复保存位置时执行，否则会把用户拖拽的岛拉回居中）
        if (!restored) {
            setTimeout(async () => {
                await adjustWindowPosition();
            }, 500);
        }
    }

    fetchSpeedStats();
    checkNetworkLatency();

    // 启动网速和硬件显示轮换定时器 (每 5 秒切换一次)
    speedCycleTimer = window.setInterval(() => {
        // 仅在宽度小于 230px 时才进行上下行网速轮换
        if (displaySpeed.value && nsdBaseWidth.value < 230) {
            isShowingUpload.value = !isShowingUpload.value;
        }
    }, 5000);

    // 向任务栏插件同步数据的方法
    const syncToTaskbar = async () => {
        if (localStorage.getItem('nsd_taskbar_plugin') === 'true') {
            try {
                // 智能判断当前该发什么模式
                let currentMode = 'speed';
                if (displayActivity.value) {
                    currentMode = 'message'; // 活动池复用消息通道，正文用活动内容
                } else if (isMsgActive.value) {
                    currentMode = 'message';
                } else if (displayMusic.value) {
                    currentMode = 'music';
                } else if (displayResource.value) {
                    currentMode = 'resource';
                }

                await invoke('sync_to_taskbar', {
                    up: uploadSpeed.value,
                    down: downloadSpeed.value,
                    lyric: currentTrackInfo.value,
                    mode: currentMode,
                    isPlaying: isPlaying.value,
                    cover: coverUrl.value || "",
                    msgTitle: displayActivity.value
                        ? (topActivity.value?.title || "任务进行中")
                        : (msgTitle.value || msgAppName.value || "新通知"),
                    msgBody: displayActivity.value
                        ? (topActivity.value?.subtitle || "")
                        : (msgBody.value || ""),
                    msgIcon: currentMsgIcon.value || "",
                    cpu: Math.round(cpuUsage.value),
                    ram: Math.round(ramUsage.value)
                });
            } catch (e) {
                console.error("同步任务栏失败:", e);
            }
        }
    };

    // 后端在 SMTC 中发现 JustSolo 时才通知前端，前端据此发起 WS 连接/重连（不再轮询触发）
    unlistenJustSolo = await listen('justsolo-discovered', () => {
        initWebSocket();
    });

    // 接收来自控制台的独立 FPS 开关指令
    await listen<{ enabled: boolean }>('control-fps-monitor', (event) => {
        enableFps.value = event.payload.enabled;
        if (enableFps.value) {
            isMusicCtlEnabled.value = false;
            enableSysResource.value = false;
        }
        // 统一交由调度函数决定是否关闭后端插件
        checkAndToggleFpsPlugin();
    });

    // 监听后端发来的高频 UDP FPS 信号
    await listen<{ fps: number }>('fps-event', (event) => {
        currentFps.value = event.payload.fps;
    });

    // 启动时初始化同步一次托盘流光/锁定状态
    invoke('sync_tray_menu', { glow: isGlowBorderEnabled.value, lock: isPositionLocked.value });

    // 监听托盘发来的 流光边框 开关信号
    await listen('tray-toggle-glow', () => {
        isGlowBorderEnabled.value = !isGlowBorderEnabled.value;
        localStorage.setItem('nsd_glow_border', String(isGlowBorderEnabled.value));
        showToast(isGlowBorderEnabled.value ? t('glowBorderEnabled') : t('glowBorderDisabled'));
    });

    // 监听托盘发来的 锁定位置 开关信号
    await listen('tray-toggle-lock', () => {
        isPositionLocked.value = !isPositionLocked.value;
        localStorage.setItem('nsd_position_locked', String(isPositionLocked.value));
        showToast(isPositionLocked.value ? t('positionLocked') : t('positionUnlocked'));
    });

    // 监听托盘发来的 重置位置 信号
    await listen('tray-reset-pos', async () => {
        localStorage.removeItem(SAVED_POS_KEY); // 清除拖拽保存的位置
        await adjustWindowPosition();
        showToast(t('positionReset'), 'sys');
    });

    // 在你原有的每秒刷新定时器中，顺带执行音乐同步
    // 1. 高频定时器：专门负责网速和硬件监控（每 500ms ~ 1000ms 刷新一次）
    speedTimer = setInterval(async () => {
        // 刷新网速
        fetchSpeedStats();

        // 实时同步给任务栏插件
        syncToTaskbar();
    }, 800) as unknown as number;


    // 2. 中频定时器：专门负责音乐状态同步（每 2000ms 刷新一次即可）
    musicTimer = setInterval(() => {
        if (isMusicCtlEnabled.value) {
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
                msgAumid.value = res.aumid;

                // 标题只存发送者（如果没有单独标题就显示 '新通知'）
                msgTitle.value = (res.title && res.title !== res.app_name) ? res.title : t('newNotification');
                // 单独把程序名存起来
                msgAppName.value = res.app_name;
                // 消息正文兜底：无正文时用标题（标题与应用名相同时用通用文案）
                msgBody.value = res.body || (res.title === res.app_name ? t('receivedNotification') : res.title);

                currentMsgIcon.value = getAppIcon(res.app_name);

                // 活动池占用时让路给活动（下轮轮询再尝试接管）
                if (!isMsgActive.value && !displayActivity.value) {
                    isMsgActive.value = true;
                    // 消息接管时复位媒体展开状态：避免消息消失后音乐盒以展开态显示在折叠尺寸上
                    isMusicExpanded.value = false;
                    isMusicExpanding.value = false;
                    // 若媒体正在展开动画中，取消其定时器，防止稍后把 isMusicExpanded 重新置回 true
                    if (musicExpandAnimTimer) {
                        clearTimeout(musicExpandAnimTimer);
                        musicExpandAnimTimer = null;
                    }
                    animateIslandSize(nsdMsgExpandedWidth.value, 65);

                    // 仅接管成功才安排 5s 后的收起定时器，避免活动展示期间被误收起
                    if ((window as any).msgTimer) clearTimeout((window as any).msgTimer);
                    (window as any).msgTimer = setTimeout(() => {
                        isMsgActive.value = false;
                        const { w, h } = getBaseSize();
                        animateIslandSize(w, h);
                    }, 5000);
                }
            }
        } catch (err) {
            console.error(err);
        }
    }, 2500);

    // 调大Ping间隔：从2.5秒调大到5.5秒
    pingTimer = setInterval(checkNetworkLatency, 5500) as unknown as number;

    // 周期性强保顶：灵动岛常驻显示时，防止被后激活的其他置顶窗口（全屏应用/播放器悬浮窗等）盖住
    setInterval(async () => {
        if (isIslandVisible.value) {
            await getCurrentWindow().setAlwaysOnTop(true).catch(() => { });
        }
    }, 3000);

    // 监听控制台发来的显隐调度指令
    await listen<{ show: boolean }>('control-island-visibility', async (event) => {
        if (event.payload.show) {
            // 手动强制点亮：退出全屏悬停待命，让岛保持常亮，不被 FS_HOVER_LEAVE_MS 收起倒计时收回
            stopFsHoverMode();
            // 1. 先让透明的 OS 窗口容器显示，此时内部 DOM 为 v-show="false"，视觉上仍是隐形的
            await invoke('show_window_no_activate', { label: 'widget' });
            await getCurrentWindow().setAlwaysOnTop(true);
            // 2. 给予 40ms 的浏览器渲染帧缓冲，再切换 v-show 触发入场动画
            setTimeout(() => {
                isIslandVisible.value = true;
            }, 40);
        } else {
            // 控制台关闭指令 -> 退出悬停待命并触发常规离开动画
            stopFsHoverMode();
            isIslandVisible.value = false;
        }
    });

    // 实时监听来自 Rust 底层发来的清透像素流，无缝同步给 Vue 的响应式 DOM 宽高
    await listen<number[]>("island-resize", (event) => {
        const [w, h] = event.payload;
        currentWidth.value = w;
        currentHeight.value = h;
    });

    // 高频频谱拉取 (大约 20 帧/秒) 兼顾 歌词高频匹配
    spectrumTimer = setInterval(async () => {
        // 计算这 50ms 里真实流逝的时间（防掉帧补偿）
        const now = performance.now();
        const delta = now - lastTickTime;
        lastTickTime = now;

        // 判定 WS 12 频段频谱是否新鲜（服务端 100ms 一帧，500ms 内未收到即视为无 WS 频谱）
        const wsSpectrumFresh = (Date.now() - lastWsSpectrumTime < 500);

        if (isPlaying.value) {
            // 1. 播放状态下，本地时钟疯狂往前推算
            localPositionMs.value += delta;

            // 2. 毫秒级歌词匹配与队列逻辑 (解决快节奏吞字、闪烁消失问题)
            // 视频类应用（potplayer/浏览器视频）：不做歌词匹配，标题常驻显示
            if (!isVideoPlayer.value && parsedLyrics.value.length > 0) {
                let matchedIndex = -1;

                // 找出当前时间进度应该播放哪一句
                // 仅当 3 秒内有 WS 推送（即当前歌词源是 WS）时才附加专属延迟，不影响 HTTP 歌词
                const wsDelayMs = (Date.now() - lastWsLyricTime < 3000) ? WS_LYRIC_DELAY_MS : 0;
                for (let i = 0; i < parsedLyrics.value.length; i++) {
                    // 抢跑 550ms：完美抵消 150ms 叠化动画 + 100ms 滤镜模糊 + 听觉视觉生理时差
                    // 再额外减去 WS 专属延迟，让 WS 歌词整体晚 WS_LYRIC_DELAY_MS 毫秒显示
                    if (parsedLyrics.value[i].time <= localPositionMs.value + 550 - (nsdLyricDelay.value * 1000) - wsDelayMs) {
                        matchedIndex = i;
                    } else {
                        break;
                    }
                }

                // 如果匹配到了新进度的歌词
                if (matchedIndex > currentMatchedIndex) {
                    // 1. 如果是首次匹配（刚启动/刚解析完歌词）
                    if (currentMatchedIndex === -1) {
                        lyricQueue.value = [];
                        lyricQueue.value.push(parsedLyrics.value[matchedIndex].text);
                    }
                    // 2. 或者是用户大幅快进导致跨度超过 2 句
                    else if (matchedIndex - currentMatchedIndex > 2) {
                        lyricQueue.value = [];
                        lyricQueue.value.push(parsedLyrics.value[matchedIndex].text);
                    }
                    // 3. 正常连续播放推进，把期间极快节奏的短歌词全部推入队列排队
                    else {
                        for (let i = currentMatchedIndex + 1; i <= matchedIndex; i++) {
                            lyricQueue.value.push(parsedLyrics.value[i].text);
                        }
                    }
                    currentMatchedIndex = matchedIndex;
                } else if (matchedIndex < currentMatchedIndex && matchedIndex !== -1) {
                    // 用户往回倒退了进度条
                    lyricQueue.value = [];
                    lyricQueue.value.push(parsedLyrics.value[matchedIndex].text);
                    currentMatchedIndex = matchedIndex;
                }

                // 3. 消费队列：确保每句歌词展示充足的时间，避免 Vue 叠化动画打架
                if (lyricQueue.value.length > 0) {
                    const now = performance.now();
                    // out-in 动画加起来需要 300ms，设定 800ms 能让文字至少稳定停留 0.5 秒
                    if (now - lastLyricChangeTime >= 800) {
                        const nextLyric = lyricQueue.value.shift();
                        if (nextLyric) {
                            // 歌词第一句恰好等于标题占位文本时，强制歌词接管显示
                            // （否则会被 setSafeTrackInfo 的防重判定拒收，导致一直显示标题、歌词不出现）
                            const force = isTitlePlaceholder && nextLyric === currentTrackInfo.value;
                            setSafeTrackInfo(nextLyric, force);
                            lastLyricChangeTime = now;
                        }
                    }
                }
            }

            // 3. 频谱逻辑：有 WS 频谱数据时用 v1.2.0 的 12->7 频谱（由 websocket-lyrics 事件实时写入）
            //    没有 WS 频谱数据就回退到本地 7 频段采集（两路柱数统一为 7，视觉一致）
            if (showSpectrumIndicator.value && !wsSpectrumFresh) {
                try {
                    const data = await invoke<number[]>('get_audio_spectrum');
                    spectrumData.value = ensureSpectrumLive(data);
                } catch (err) {
                    // 忽略错误，防止刷屏
                }
            }
        } else {
            // 没在播放时，让柱子平滑回落到最低点（统一 7 柱）
            spectrumData.value = [0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35];
        }
    }, 50) as unknown as number;

    // 组件挂载时检查一次是否需要开启 FPS 采集
    checkAndToggleFpsPlugin();

    // 初始化时触发一次计算
    setTimeout(() => {
        calculateScroll();
    }, 700);
});

onUnmounted(() => {
    stopWebSocket();
    stopClipboardPolling();
    stopActivityPoolListening();
    if (unlistenJustSolo) {
        unlistenJustSolo();
        unlistenJustSolo = null;
    }
    window.removeEventListener('blur', collapseMusic);
    clearInterval(speedTimer);
    clearInterval(pingTimer);
    clearInterval(musicTimer);
    clearInterval(notifyTimer);
    clearInterval(spectrumTimer);
    if (speedCycleTimer) clearInterval(speedCycleTimer);
    if (coverRetryTimer) clearTimeout(coverRetryTimer);
    stopFsHoverMode();
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
    width: 100%;
    height: 100%;
    -webkit-font-smoothing: subpixel-antialiased;
    text-rendering: optimizeLegibility;
}

:global(#app) {
    width: 100%;
    height: 100%;
}

/* 外层包裹层：负责裁切多余的流光 */
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

/* 隐藏在底层的巨大旋转渐变层 */
.rainbow-border-glow {
    position: absolute;
    width: 500px;
    height: 500px;

    /* 修正旋转中心偏移问题 */
    top: calc(50% - 250px);
    left: calc(50% - 250px);
    z-index: 0;

    /* 重新绘制的完美对称环形渐变，清透不发脏 */
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='500' height='500'%3E%3Cdefs%3E%3Cfilter id='b' x='-50%25' y='-50%25' width='200%25' height='200%25'%3E%3CfeGaussianBlur in='SourceGraphic' stdDeviation='60'/%3E%3C/filter%3E%3C/defs%3E%3Cg filter='url(%23b)'%3E%3Ccircle cx='250' cy='90' r='150' fill='%23ff3b30'/%3E%3Ccircle cx='390' cy='170' r='150' fill='%23ff9500'/%3E%3Ccircle cx='390' cy='330' r='150' fill='%234cd964'/%3E%3Ccircle cx='250' cy='410' r='150' fill='%23007aff'/%3E%3Ccircle cx='110' cy='330' r='150' fill='%235856d6'/%3E%3Ccircle cx='110' cy='170' r='150' fill='%23ff2d55'/%3E%3C/g%3E%3C/svg%3E");
    background-size: cover;

    /* 10秒一圈刚刚好，柔和且不怎么吃 GPU */
    animation: rainbow-rotate 10s linear infinite;
    will-change: transform;
}

/* 核心遮罩内容块：挡在旋转渐变层的上方 */
.island-core-content {
    position: relative;
    z-index: 2;
    width: 100%;
    height: 100%;
    border-radius: 98px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 14px;
    overflow: hidden;
}

/* 顺时针匀速旋转 */
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

/* 修改网速盒子布局，强制靠左，并加入左侧内边距 */
.speed-box {
    position: absolute;
    left: 0;
    top: 0;
    display: flex;
    align-items: center;
    justify-content: flex-start;
    width: 100%;
    height: 100%;
}

.speed-item {
    display: flex;
    align-items: center;
    gap: 6px;
    /* 稍微拉开箭头和数字的距离 */
    transform: translateY(-1px);
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', sans-serif;
}

.label {
    font-size: 10px;
    /* 稍微调大箭头 */
    color: currentColor;
    opacity: 0.5;
    font-weight: 800;
    padding: 2px 5px;
    border-radius: 4px;
    transition: all 0.3s ease;
    background: rgba(150, 150, 150, 0.15);
    /* 默认给一个淡淡的底色，增加质感 */
}

/* 高流量时的 label 样式 */
.label.high-traffic {
    color: currentColor;
    opacity: 1;
    background: rgba(255, 255, 255, 0.25);
}

:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .label.high-traffic {
    background: rgba(0, 0, 0, 0.15);
}

.value {
    font-size: 12px;
    transform: translateY(-0.5px);
    font-weight: 600;
    letter-spacing: 0.2px;
    font-variant-numeric: tabular-nums;
    min-width: 65px;
    text-align: left;
}

/* 网速轮换的淡入淡出动画 */
.speed-fade-enter-active,
.speed-fade-leave-active {
    transition: opacity 0.3s ease, transform 0.3s ease;
}

.speed-fade-enter-from {
    opacity: 0;
    transform: translateY(4px);
    /* 微微从下方滑入 */
}

.speed-fade-leave-to {
    opacity: 0;
    transform: translateY(-4px);
    /* 微微向上滑出 */
}

.status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    transition: background-color 0.4s ease;
}

/* 去掉发光阴影，改为纯粹的扁平化圆点，干净利落 */
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

/* 增加统一的内部绝对定位平替包裹层 */
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
    box-sizing: unset !important;
    border: 2px solid rgba(255, 255, 255, 0.5) !important;
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
    animation: rotate 8s linear infinite;
    animation-play-state: paused;
    /* 默认让动画处于暂停状态 */
}

/* 正在播放时的旋转动画 */
.is-playing .cover-inner {
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
    position: fixed;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    display: flex;
    align-items: center;
    gap: 12px;
    z-index: 10;
}

.ctl-btn {
    background: transparent;
    border: none;
    color: inherit;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 6px;
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
    right: 10px;
    height: 100%;
    display: flex;
    align-items: center;
    overflow: hidden;
    padding-left: 0;
    -webkit-app-region: no-drag;
    transform: translateY(-1px) translateX(-0.5px);
    mask-image: linear-gradient(to right, #000000 96%, transparent 100%);
    -webkit-mask-image: linear-gradient(to right, #000000 96%, transparent 100%);
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
    box-sizing: border-box;
    z-index: 10;
    gap: 12px;
    -webkit-app-region: no-drag;
}

/* 预制消息图标/头像样式 */
.msg-avatar {
    width: 35px;
    height: 35px;
    border-radius: 50%;
    background: none;
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
    overflow: hidden;
    flex-grow: 1;
}

/* 消息弹窗容器 */
.msg-title {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 14px;
    font-weight: 700;
    line-height: 1.4;
    width: 100%;
    overflow: hidden;
}

/* 发送者昵称（允许超长省略号） */
.sender-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

/* 尾部的程序名 */
.app-name {
    font-size: 10.5px;
    font-weight: 600;
    flex-shrink: 0;
    padding: 2px 6px;
    border-radius: 6px;
    background-color: rgba(150, 150, 150, 0.25);
    color: inherit;
    opacity: 0.9;
    letter-spacing: 0.2px;
    transform: translateY(-0.5px);
}

/* 调大后的内容样式 */
.msg-body {
    font-size: 12.5px;
    line-height: 1.4;
    opacity: 0.75;
    text-align: left;
    width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

/* 灵动岛活动池卡片样式（外部服务经 47300 HTTP 推送） */
.activity-box {
    position: absolute;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    padding: 0 16px;
    box-sizing: border-box;
    z-index: 10;
    gap: 12px;
    -webkit-app-region: no-drag;
}

.activity-avatar {
    width: 34px;
    height: 34px;
    border-radius: 50%;
    background: var(--activity-accent, rgba(255, 255, 255, 0.16));
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--activity-accent, #ffffff);
    flex-shrink: 0;
    overflow: hidden;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.activity-avatar-img {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    object-fit: cover;
}

.activity-fallback-icon {
    width: 18px;
    height: 18px;
}

.activity-text-wrapper {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: flex-start;
    overflow: hidden;
    flex-grow: 1;
    min-width: 0;
    gap: 2px;
}

.activity-title {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 13.5px;
    font-weight: 700;
    line-height: 1.3;
    width: 100%;
    overflow: hidden;
    color: #ffffff;
}

.activity-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

/* 尾部的活动类型徽标 */
.activity-kind {
    font-size: 10px;
    font-weight: 600;
    flex-shrink: 0;
    padding: 1px 5px;
    border-radius: 5px;
    background-color: rgba(255, 255, 255, 0.22);
    color: inherit;
    opacity: 0.9;
    letter-spacing: 0.2px;
    transform: translateY(-0.5px);
}

.activity-subtitle {
    font-size: 11.5px;
    line-height: 1.3;
    opacity: 0.68;
    width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: #ffffff;
}

.activity-progress-row {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    margin-top: 1px;
}

.activity-progress-track {
    flex: 1;
    height: 3px;
    border-radius: 2px;
    background: rgba(255, 255, 255, 0.18);
    overflow: hidden;
}

.activity-progress-fill {
    height: 100%;
    border-radius: 2px;
    background: var(--activity-accent, rgba(255, 255, 255, 0.9));
    transition: width 0.12s linear;
}

/* 不确定进度：流动动画 */
.activity-progress-fill.is-indeterminate {
    width: 34% !important;
    animation: activity-indeterminate 1.1s ease-in-out infinite;
}

.activity-progress-text {
    font-size: 10.5px;
    font-weight: 600;
    opacity: 0.8;
    color: #ffffff;
    min-width: 30px;
    text-align: right;
    flex-shrink: 0;
}

@keyframes activity-indeterminate {
    from { transform: translateX(-120%); }
    to { transform: translateX(320%); }
}

/* 灵动岛剪贴板链接通知卡片样式 */
.clipboard-box {
    position: absolute;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    padding: 0 12px;
    box-sizing: border-box;
    z-index: 10;
    gap: 12px;
    -webkit-app-region: no-drag;
}

.clipboard-icon {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    overflow: hidden;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.clipboard-icon-img {
    width: 24px;
    height: 24px;
    object-fit: contain;
}

.clipboard-text-wrapper {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: flex-start;
    overflow: hidden;
    flex-grow: 1;
    min-width: 0;
}

.clipboard-title {
    font-size: 13.5px;
    font-weight: 700;
    line-height: 1.4;
    white-space: nowrap;
}

.clipboard-link {
    font-size: 12.5px;
    line-height: 1.4;
    opacity: 0.78;
    text-align: left;
    width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    direction: ltr;
    unicode-bidi: plaintext;
}

/* 右侧的打开链接按钮 */
.clipboard-open-btn {
    flex-shrink: 0;
    width: 30px;
    height: 30px;
    border-radius: 50%;
    border: none;
    outline: none;
    background-color: rgba(150, 150, 150, 0.25);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    padding: 0;
    transition: background-color 0.2s ease, transform 0.1s ease;
}

.clipboard-open-btn:hover {
    background-color: rgba(255, 255, 255, 0.35);
}

.clipboard-open-btn:active {
    transform: scale(0.92);
}

.clipboard-open-img {
    width: 20px;
    height: 20px;
    object-fit: contain;
}

.value.high-usage {
    color: #f06861 !important;
}


/* 音乐律动频谱样式 */
.audio-spectrum {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1.5px;
    height: 12px;
    padding-right: 2px;
}

/* 暂停状态下的竖线（统一高度） */
.audio-spectrum .bar {
    width: 2px;
    height: 18px;
    min-height: 3px;
    background-color: #b6e0ee;
    border-radius: 8px;
    transform-origin: center;
    /* 改用极速的 ease-out 过渡，让前端完美衔接后端的帧率 */
    transition: transform 0.08s ease-out;
    will-change: transform;
}

.music-ctl-box {
    transition: opacity 0.2s ease !important;
}

.music-ctl-box.expanded {
    flex-direction: column;
    align-items: flex-start;
    justify-content: flex-start;
    padding: 0 !important;
}

/* 顶部容器：取消 all 过渡，让它跟着 Rust 窗口的拉伸严丝合缝地重排 */
.music-top-row {
    display: flex;
    align-items: center;
    width: 100%;
    height: 100%;
    position: relative;
    transition: none !important;
    /* 核心防抖魔法，取消 CSS 的挣扎 */
}

.music-ctl-box.expanded .music-top-row {
    height: 40px;
    margin-top: 14px !important;
    margin-left: 5px !important;
    border: none;
}

/* 封面：覆盖掉上面的 transition: all，只保留变形和圆角的过渡 */
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

/* 歌曲文本遮罩：取消过渡，随窗口大小瞬间变化 */
.music-ctl-box.expanded .music-info-mask-box {
    left: 60px !important;
    right: 55px !important;
    display: flex !important;
    align-items: center !important;
    justify-content: flex-start !important;
    transition: none !important;
}

/* 你的两套文字过渡逻辑非常完美，全部保留原样（因为 opacity 不影响排版） */
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

/* 媒体控件与频谱 */
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

/* 强制靠左对齐，干掉原本的 align-items: center。否则长文本会向两边溢出，导致开头被裁 */
.music-info-text.single-line {
    overflow: visible !important;
    align-items: flex-start !important;
    text-align: left !important;
}

/* 滚动的内部容器 */
.scroll-inner {
    display: inline-block;
    white-space: nowrap;
    width: max-content;
    flex-shrink: 0;
    vertical-align: top;
    backface-visibility: hidden;
    transform: translateZ(0);
    -webkit-font-smoothing: antialiased;
    transform-style: preserve-3d;
}

/* 挂载动画 */
.scroll-inner.is-scrolling {
    animation: scroll-ping-pong var(--scroll-duration) linear infinite alternate;
}

/* 滚动动画帧：开头停留 15% 后滚动，末尾停留 15% 便于读完歌词 */
@keyframes scroll-ping-pong {

    0%,
    15% {
        transform: translateX(0);
    }

    85%,
    100% {
        /* JS 里已经拼好了 px 单位，这里直接 -1 乘过去即可 */
        transform: translateX(calc(-1 * var(--scroll-dist)));
    }
}

/* 系统操作通知样式 */
.system-toast-box {
    position: absolute;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    padding-left: 0;
    z-index: 10;
    -webkit-app-region: no-drag;
}

.toast-icon {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    transform: translateX(-8px);
}

/* 灵动岛通知 */
.toast-icon.app-icon {
    color: currentColor;
}

/* 系统通知使用跟随字体的原生对比色 (黑白) */
.toast-icon.sys-icon {
    color: currentColor;
    opacity: 0.85;
}

.toast-icon svg {
    width: 22px;
    height: 22px;
    display: block;
}

.toast-icon.battery-charge-icon {
    color: #34C759;
}

.toast-icon.battery-low-icon {
    color: #FF3B30;
}

.toast-text {
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', sans-serif;
    font-size: 12.5px;
    font-weight: 600;
    white-space: nowrap;
    opacity: 0.95;
    transform: translateX(-2px) translateY(-1px);
}

/* 歌词渲染单句定位 */
.lyric-render-text {
    position: absolute;
    left: 0;
    right: 0;
    top: 50%;
    transform: translateY(-50%);
    /* 严格垂直居中 */
    white-space: nowrap;
    overflow: hidden;
    text-align: left !important;
    display: inline-block;
    will-change: opacity, filter;
}

.lyric-fade-enter-active,
.lyric-fade-leave-active {
    /* 180ms 顺滑交替，既有原先的质感，又不会因为时间太长导致空壳 */
    transition: opacity 0.2s ease, filter 0.22s ease;
}

/* 新歌词进来：从透明、模糊，逐渐变得清晰可见 */
.lyric-fade-enter-from {
    opacity: 0;
    filter: blur(8px);
}

.lyric-fade-enter-to {
    opacity: 1;
    filter: blur(0px);
}

/* 旧歌词离开：在原地直接开始变模糊、变透明，直到被新歌词完全平滑盖过去 */
.lyric-fade-leave-from {
    opacity: 1;
    filter: blur(0px);
}

.lyric-fade-leave-to {
    opacity: 0;
    filter: blur(8px);
}

/* 灵动岛沉浸模式专属样式 */
.coverglass-bg-container {
    position: absolute;
    z-index: 1;
    /* 关键：压在 0层 流光之上，但在 2层 核心内容之下 */
    pointer-events: none;
    overflow: hidden;
}

.coverglass-bg-image {
    position: absolute;
    top: -10%;
    left: -10%;
    width: 120%;
    height: 120%;
    background-size: cover;
    background-position: center;
    opacity: 0.9;
    transition: background-image 0.8s ease;
    transform: translateZ(0);
    /* 开启硬件加速 */
}

.coverglass-noise-layer {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    opacity: 0.15;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='256' height='256'%3E%3Cfilter id='noise'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='2.5' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noise)'/%3E%3C/svg%3E");
}

.coverglass-mask-layer {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    /* 铺一层浅黑色遮罩，确保白色的文字和图标绝对清晰可读 */
    background: rgba(0, 0, 0, 0.45);
}

/* 确保岛内的核心内容层压在背景图上方 */
.inner-wrapper,
.audio-spectrum,
.status-dot {
    position: relative;
    z-index: 2;
}

/* 系统资源监控 */
.resource-box {
    position: absolute;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-right: 8px;
    box-sizing: border-box;
    gap: 12px;
    /* 稍微拉开 CPU 和 RAM 的距离 */
    -webkit-app-region: no-drag;
    overflow: hidden;
}

/* 单个资源组 (CPU/RAM) */
.res-group {
    flex: 1 1 0%;
    min-width: 0;
    display: flex;
    flex-direction: column;
    /* 改为纵向两行布局 */
    justify-content: center;
    gap: 6px;
    /* 上下排间距 */
}

/* 第一行的文字容器 (标签 + 数值) */
.res-info-row {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    /* 底部对齐，让文字重心更稳 */
    width: 100%;
}

/* 标签 (CPU/RAM) */
.res-label {
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', sans-serif;
    font-size: 10px;
    font-weight: 800;
    opacity: 0.75;
    color: currentColor;
    background: rgba(150, 150, 150, 0.15);
    padding: 2px 5px;
    border-radius: 4px;
    line-height: 1;
}

/* 进度条轨道 */
.res-bar-track {
    width: 100%;
    /* 占满下面一整行 */
    height: 4px;
    /* 压低进度条高度，把空间留给上层文字 */
    background: rgba(150, 150, 150, 0.2);
    border-radius: 2px;
    overflow: hidden;
    position: relative;
}

/* 进度条填充 */
.res-bar-fill {
    height: 100%;
    width: 0%;
    background: currentColor;
    border-radius: 2px;
    opacity: 0.9;
    transition: width 0.4s cubic-bezier(0.25, 1, 0.5, 1), background-color 0.3s ease;
}

/* 百分比数值 */
.res-value {
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', sans-serif;
    font-size: 12px;
    /* 稍微调大凸显数值 */
    font-weight: 700;
    color: currentColor;
    opacity: 0.95;
    text-align: right;
    font-variant-numeric: tabular-nums;
    line-height: 1;
    transform: translateY(-1px);
}

/* 高负载告警态 (>=85%) */
.high-usage {
    color: #b6170f !important;
}

/* 亮色主题适配 (可选，如果全局 currentColor 处理得当可省略) */
:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .res-label {
    background: rgba(0, 0, 0, 0.08);
}

:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .res-bar-track {
    background: rgba(0, 0, 0, 0.1);
}

.speed-dual-box {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
}

/* 上下行同时显示时，下行网速整体向左微调 */
.speed-dual-box .speed-item:last-child {
    transform: translate(-10px, -1px);
}

.speed-single-box {
    display: flex;
    align-items: center;
    width: 100%;
}

/* --- 媒体控制器展开底部布局 --- */
.music-expanded-bottom {
    position: absolute;
    top: 70px;
    left: 0;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 0 8px;
    box-sizing: border-box;
}

/* 覆盖掉原有的按钮容器绝对定位，改为由父元素流式排版 */
.music-ctl-box.expanded .music-controls {
    position: relative;
    transform: none !important;
    top: 0 !important;
    left: 0;
    width: 100%;
    display: flex;
    justify-content: center;
    gap: 20px;
}

/* 进度条容器 */
.progress-container {
    display: flex;
    align-items: center;
    width: 100%;
    gap: 10px;
}

/* 两侧的时间文本 */
.time-text {
    font-size: 10.5px;
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', sans-serif;
    opacity: 0.7;
    font-variant-numeric: tabular-nums;
    width: 32px;
    /* 固定宽度防止抖动 */
    text-align: center;
    flex-shrink: 0;
}

/* 进度条底轨 */
.progress-track {
    flex-grow: 1;
    height: 5px;
    background: rgba(255, 255, 255, 0.2);
    border-radius: 2px;
    overflow: hidden;
    position: relative;
    cursor: default;
}

/* 亮色主题适配进度条底轨 */
:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .progress-track {
    background: rgba(0, 0, 0, 0.15);
}

/* 进度条填充：这里坚决不加 transition，因为你的 localPositionMs 每 50ms 更新，原生刷新最丝滑 */
.progress-fill {
    height: 100%;
    background: currentColor;
    border-radius: 2px;
    opacity: 0.95;
    will-change: width;
}

/* 自定义显示组合样式 */
.custom-display-box {
    position: absolute;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-right: 8px;
    gap: 4px;
    box-sizing: border-box;
    -webkit-app-region: no-drag;
}

.custom-slot-item {
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: 4px;
    height: 95%;
    min-width: 0;
    background: rgba(128, 128, 128, 0.1);
    box-shadow: 0 2px 4px inset #3e3e3e42;
    border-radius: 4px;
    padding: 2px 4px;
}

/* (可选) 针对亮色主题的独立优化，让亮色下的模块边框更干净 */
:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .custom-slot-item {
    background: rgba(0, 0, 0, 0.05);
}

/* 给 FPS 分配更窄的宽度占比 */
.custom-slot-item.is-fps {
    flex: 0.65;
}

/* 把省出来的宽度补充给网速和资源 */
.custom-slot-item.is-speed,
.custom-slot-item.is-resource {
    flex: 1.12;
    /* 放大占比 */
}

.custom-slot-empty {
    flex: 1;
}

/* 统一双行内联样式 */
.custom-data-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
}

.custom-data-row.justify-center {
    justify-content: center;
}

/* 统一的小标签风格 (类似 ⬆, CPU, FPS) */
.custom-label {
    font-size: 10px;
    font-weight: 800;
    opacity: 0.75;
    color: currentColor;
    background: rgba(150, 150, 150, 0.15);
    padding: 1px 2px;
    border-radius: 2px;
    line-height: 1;
    flex-shrink: 0;
    /* 防止标签被挤变形 */
}

/* 统一的数值风格 */
.custom-value {
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', sans-serif;
    font-size: 11px;
    font-weight: 700;
    font-variant-numeric: tabular-nums;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    /* 超过长度自动打点，绝对不撑破 */
    text-align: right;
    transform: translateY(-0.5px);
}

/* FPS 底部数字专属微调 */
.fps-large {
    font-size: 14px;
    letter-spacing: 0.5px;
}

/* 给歌曲封面分配极窄的 flex 宽度，仅保留空间即可 */
.custom-slot-item.is-cover {
    flex: 0.45;
    padding: 0;
    margin-right: 2px;
    margin-left: 2px;
    background: transparent;
    box-shadow: none;
    align-items: center;
    /* 保证居中 */
}

/* 静态封面本体：方形、小圆角 */
.custom-cover-inner {
    width: 32px;
    height: 32px;
    border-radius: 4px;
    background-color: rgba(150, 150, 150, 0.15);
    box-shadow: 0 2px 4px inset #3e3e3e42;
    background-size: cover;
    background-position: center;
    background-repeat: no-repeat;
    flex-shrink: 0;
}

/* --- 终极无损 0 负担卡拉OK效果 --- */

/* 1. 父元素：隐藏原本文字，仅用来撑开宽度和滚动，绝不破坏 color 继承 */
.lyric-render-text .scroll-inner {
    position: relative;
    -webkit-text-fill-color: transparent;
    font-weight: 600;
    /* 放心加粗，已经恢复原生渲染，绝对不会发虚！ */
}

/* 2. 底层伪元素：完美的半透明未激活态（自适应黑/白主题） */
.lyric-render-text .scroll-inner::before {
    content: attr(data-text);
    position: absolute;
    left: 0;
    top: 0;
    -webkit-text-fill-color: currentColor;
    /* 提取灵动岛原本的纯白或纯黑 */
    opacity: 0.35;
    /* 直接调低透明度作为底色，无论啥主题都能完美变暗 */
    white-space: nowrap;
}

/* 3. 顶层伪元素：高亮激活态，像拉窗帘一样盖在上面扫过 */
.lyric-render-text .scroll-inner::after {
    content: attr(data-text);
    position: absolute;
    left: 0;
    top: 0;
    -webkit-text-fill-color: currentColor;
    /* 提取真正的纯白/纯黑，绝对高亮！ */
    white-space: nowrap;

    /* 核心 0 负担动画：利用 GPU 硬件加速的裁切展开 */
    clip-path: inset(0 100% 0 0);
    animation: scan-lyric var(--scan-duration) linear forwards;
    animation-play-state: inherit;
    /* 跟随父元素一起暂停/播放 */
}

/* 4. 覆盖掉上一版错误的叠加动画，让父元素老老实实只负责横向滚动 */
.lyric-render-text .scroll-inner.is-scrolling {
    animation: scroll-ping-pong var(--scroll-duration) linear infinite alternate;
}

/* 5. 扫描裁切关键帧 */
@keyframes scan-lyric {
    0% {
        clip-path: inset(0 100% 0 0);
    }

    100% {
        clip-path: inset(0 0 0 0);
    }
}
</style>