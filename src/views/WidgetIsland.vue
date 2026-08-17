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
                        <div v-if="isMsgActive" class="msg-box" key="msg">
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
                                                <span class="scroll-inner" ref="textInnerRef"
                                                    :class="{ 'is-scrolling': scrollDist > 0 }"
                                                    :style="{ '--scroll-dist': scrollDist + 'px', '--scroll-duration': scrollDuration }">
                                                    {{ currentTrackInfo }}
                                                </span>
                                            </span>
                                        </transition>
                                    </div>
                                    <div class="music-info-text double-line" :class="{ 'fade-in': isMusicExpanded }">
                                        <div class="song-title">{{ currentSongName }}</div>
                                        <div class="song-artist">{{ currentArtistName }}</div>
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
import { getCurrentWindow, currentMonitor, availableMonitors, PhysicalPosition, LogicalPosition, PhysicalSize, type Window } from '@tauri-apps/api/window'; import { Menu, MenuItem } from '@tauri-apps/api/menu';
import { listen, emit } from '@tauri-apps/api/event';
import { t, currentLanguage, type AppLanguage } from '../i18n';

const isIslandVisible = ref(false);
const isMenuOpen = ref(false);

// 统一监听灵动岛显隐，无论因为什么原因出现/消失，都立刻向控制台汇报
watch(isIslandVisible, (newVal) => {
    emit('island-status-sync', { visible: newVal });
});

// 兜底保险：OS 窗口显隐必须严格跟随灵动岛状态。
// 关闭 → 立即开启鼠标透传（点击穿透，不拦截下层窗口）；
//         等离开动画结束后把窗口物理缩成 1×1，彻底点不到；
// 开启 → 关闭透传并恢复窗口到正常尺寸（正常交互）。
watch(isIslandVisible, (visible) => {
    const appWindow = getCurrentWindow();
    appWindow.setIgnoreCursorEvents(!visible).catch(() => { });

    if (visible) {
        // 呼出时先把窗口恢复到应有的物理尺寸（getBaseSize/appScale 此时早已初始化）
        const { w, h } = getBaseSize();
        const scaleFactor = window.devicePixelRatio || 1;
        appWindow.setSize(new PhysicalSize(
            Math.ceil(w * appScale.value * scaleFactor),
            Math.ceil(h * appScale.value * scaleFactor)
        )).catch(() => { });
        return;
    }

    // 关闭时等离开动画播完（约 350ms）再缩成 1×1；期间若又被呼出则放弃缩放
    setTimeout(() => {
        if (!isIslandVisible.value) {
            appWindow.setSize(new PhysicalSize(1, 1)).catch(() => { });
        }
    }, 400);
}, { immediate: true });

// 记录全屏自动隐藏开关状态
const isAutoHideEnabled = ref(localStorage.getItem('nsd_autohide_fs') === 'true');
// 记录进入全屏前的灵动岛显隐状态，用来决定退回桌面时要不要恢复
let wasVisibleBeforeFullscreen = false;

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
    // 如果正在处理，或者队列为空，则直接返回
    if (isProcessingToast || toastQueue.value.length === 0) return;

    // 优先级判断：如果当前正在显示消息通知(最高优先级)，则挂起等待
    if (isMsgActive.value) return;

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
        // （前提是没有被应用消息或音乐面板霸占）
        if (!isMsgActive.value && !isMusicExpanded.value && !isMusicExpanding.value) {
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

// 监听消息通知状态，一旦消息通知消失，立刻唤醒可能被挂起的操作通知队列
watch(isMsgActive, (newVal) => {
    if (!newVal) {
        processToastQueue();
    }
});

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

// 1. 瞬间判定当前是否处于大窗口状态
const isExpandedSize = computed(() => isMusicExpanded.value || isMsgActive.value);

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
        // 关键修改：使用 showCoverglassBg.value 替换原判断
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
        // 关键修改：使用 showCoverglassBg.value 替换原判断
        return { backgroundColor: `transparent`, borderRadius: innerRadius };
    }
    return { backgroundColor: `rgba(0, 0, 0, ${alpha})`, borderRadius: innerRadius };
});

// 4. 沉浸模式背景层：智能规避黑边与遮挡，并绑定不透明度
const coverglassStyle = computed<CSSProperties>(() => {
    // 关键修复：把控制台传来的透明度转换成视觉 alpha 值
    const linear = islandOpacity.value / 100;
    const alpha = Math.pow(linear, 1 / 2.2);

    if (isGlowBorderEnabled.value) {
        // 当流光边框开启时：往内缩进 2px 给边框让路，并匹配内层圆角
        const innerRadiusValue = Math.max(nsdBorderRadius.value - 2, 8);
        return {
            top: '2px', left: '2px', right: '2px', bottom: '2px',
            borderRadius: isExpandedSize.value ? '22px' : `${innerRadiusValue}px`,
            opacity: alpha // 新增：将透明度应用到沉浸背景层
        };
    }
    // 当流光边框关闭时：无死角铺满整个灵动岛，并匹配外层大圆角
    return {
        top: '0', left: '0', right: '0', bottom: '0',
        borderRadius: isExpandedSize.value ? '24px' : `${nsdBorderRadius.value}px`,
        opacity: alpha // 新增：将透明度应用到沉浸背景层
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
const currentBaseInfo = ref(''); // 用于在没有歌词时兜底显示 "歌名 - 歌手"
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
// 监听流光边框状态，改变立刻通知托盘
watch(isGlowBorderEnabled, (val) => invoke('sync_tray_menu', { glow: val }));

// 律动频谱
const spectrumData = ref([0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35]);
let spectrumTimer: number;

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

// 当前播放器是否为浏览器（edge/chrome），以及是否正在展示 SMTC 本地封面
const currentIsBrowser = ref(false);
const isSmtcCoverActive = ref(false);
// 当前 SMTC 来源应用的包名（用于 PotPlayer 音乐模式等封面策略判断）
const currentAppIdStr = ref('');

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
            canvas.width = 120; // 降低物理分辨率榨干性能
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

// 共享的封面获取/应用逻辑：查缓存 → 未命中就调接口 → 写缓存 → 烘焙模糊封面
// onlyIfChanged: 与当前显示的封面对比，不一样才应用（恢复播放时用）
// clearOnError: 获取失败时是否清空封面（切歌时清空，恢复播放时保留）
const fetchAndApplyCover = async (trackInfo: string, song: string, artist: string, onlyIfChanged = false, clearOnError = true, preferSmtc = true) => {
    if (coverCache.has(trackInfo)) {
        const cached = coverCache.get(trackInfo)!;
        if (!onlyIfChanged || cached !== coverUrl.value) {
            coverUrl.value = cached;
            blurredCoverUrl.value = blurredCoverCache.get(trackInfo) || '';
        }
        return;
    }
    try {
        console.log("尝试获取封面");
        const url = await invoke<string>('get_random_cover_url', { songName: song, artistName: artist, preferSmtc });
        if (!onlyIfChanged || url !== coverUrl.value) {
            coverUrl.value = url;
            if (coverCache.size > 50) {
                coverCache.clear();
                blurredCoverCache.clear();
            }
            coverCache.set(trackInfo, url);
            const bakedImage = await bakeBlurImage(url);
            blurredCoverUrl.value = bakedImage;
            blurredCoverCache.set(trackInfo, bakedImage);
            console.log("获取封面成功");
        }
    } catch (e) {
        if (clearOnError) {
            coverUrl.value = '';
            blurredCoverUrl.value = '';
        }
    }
};

// 记录最近一次 SMTC 封面请求序号，防止切歌竞态下旧请求的结果覆盖新内容封面
let smtcCoverReqSeq = 0;

// 浏览器专用封面：只认 SMTC 本地封面，拿不到就保留默认应用图标（绝不走网络兜底，避免串错图）
const fetchAndApplySmtcCover = async (trackInfo: string, onlyIfChanged = false) => {
    const mySeq = ++smtcCoverReqSeq;
    if (coverCache.has(trackInfo)) {
        const cached = coverCache.get(trackInfo)!;
        if (mySeq === smtcCoverReqSeq && (!onlyIfChanged || cached !== coverUrl.value)) {
            coverUrl.value = cached;
            blurredCoverUrl.value = blurredCoverCache.get(trackInfo) || '';
        }
        return;
    }
    // 切歌场景：SMTC 封面可能晚于标题就绪，最多重试 3 次（间隔 1.5s）确保拿到新封面
    const maxAttempts = onlyIfChanged ? 1 : 3;
    for (let attempt = 0; attempt < maxAttempts; attempt++) {
        try {
            const smtcCover = await invoke<string | null>('get_smtc_cover');
            // 期间已切到新内容，丢弃过期结果
            if (mySeq !== smtcCoverReqSeq) return;
            if (smtcCover) {
                isSmtcCoverActive.value = true;
                if (!onlyIfChanged || smtcCover !== coverUrl.value) {
                    coverUrl.value = smtcCover;
                    if (coverCache.size > 50) {
                        coverCache.clear();
                        blurredCoverCache.clear();
                    }
                    coverCache.set(trackInfo, smtcCover);
                    const bakedImage = await bakeBlurImage(smtcCover);
                    blurredCoverUrl.value = bakedImage;
                    blurredCoverCache.set(trackInfo, bakedImage);
                }
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

// 实时FPS功能相关
const enableFps = ref(localStorage.getItem('nsd_fps_monitor') === 'true');
const currentFps = ref(0);

// 智能判断并按需启停后端的 FPS 采集插件
const checkAndToggleFpsPlugin = () => {
    const needFps = enableFps.value || (enableCustomDisplay.value && customSlots.value.includes('fps'));
    invoke('toggle_fps_plugin', { enable: needFps }).catch((err) => {
        console.error('FPS 插件启动失败:', err);
        // 👈 检测到 FPS 插件不存在时，前端就地退出 FPS 显示，避免灵动岛停留在 FPS 模式
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

// 用 SMTC 已获取到的 "标题 - 歌手" 立刻填充折叠态文本
const fillCollapsedWithTrackInfo = () => {
    if (!currentSongName.value || currentSongName.value === t('noSongPlaying')) return;
    // PotPlayer：直接用标题当常驻歌词显示，不再拼 "标题 - potplayer"
    if (isPotplayerSource.value) {
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
                    parsedLyrics.value = []; // 连上 WS 后，立刻清空可能残存的网络歌词，防止打架
                    // 连上 WS 就立刻用 SMTC 已拿到的 "标题 - 歌手" 填充折叠态文本（歌词接管前先兜底显示）
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
                        lastLyricChangeTime = 0; // 重置时间锁，允许立刻显示第一句歌词

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
                            spectrumData.value = convertWs12To7(payload.bands);
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

// 记录是否锁定了位置，并存到本地
const isPositionLocked = ref(localStorage.getItem('nsd_position_locked') === 'true');
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

// 智能判断静默模式下是否该显示：有消息、有系统提示，或开启了音乐控制且正在播放
const shouldShowInQuietMode = computed(() =>
    isMsgActive.value || displaySysToast.value || (isMusicCtlEnabled.value && isMediaActive.value)
);
watch(shouldShowInQuietMode, async (newVal) => {
    if (isMsgModeEnabled.value) {
        if (newVal && !isIslandVisible.value) {
            // 条件满足且当前隐藏时，立刻呼出灵动岛
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
// 只要媒体活跃且没被“消息弹窗(Msg)”霸占，背景就一直存在，即使此时正在显示系统通知(Toast)
const showCoverglassBg = computed(() => {
    return islandTheme.value === 'coverglass' &&
        isMusicCtlEnabled.value &&
        isMediaActive.value &&
        !isMsgActive.value &&
        blurredCoverUrl.value;
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
    // 只有在没有被临时弹窗（消息、音乐展开）霸占时，才执行基础大小切换
    if (!isMsgActive.value && !displaySysToast.value && !isMusicExpanded.value && !isMusicExpanding.value) {
        const { w, h } = getBaseSize();
        animateIslandSize(w, h);
    }
});

// 专门用于控制右侧常驻指示灯的独立计算属性（完全不受消息通知打断）
const showSpectrumIndicator = computed(() => {
    return isMusicCtlEnabled.value && isMediaActive.value;
});

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
        // 浏览器只认 SMTC 本地封面，避免网络兜底串到错误图片
        if (currentIsBrowser.value) {
            await fetchAndApplySmtcCover(trackInfo, true);
        } else {
            // 与当前显示的封面对比，不一样才更新；失败时保持现有封面不动
            // PotPlayer 音乐模式同样忽略 SMTC 封面，恢复播放时仍走网络获取
            await fetchAndApplyCover(trackInfo, song, artist, true, false, !currentAppIdStr.value.includes("potplayer"));
        }
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

// 核心同步函数：负责获取状态并智能降级
const syncMusicStatus = async () => {
    try {
        const res = await invoke<[string, string, boolean, number, number, string] | null>('fetch_netease_music_info');

        // 判定过去 3 秒内是否有活跃的本地 WebSocket 推送
        const isWsActive = (Date.now() - lastWsLyricTime < 3000);

        if (res) {
            const [song, artist, playing, positionMs, durationMs, app_id_str] = res;

            // 记录当前是否为浏览器类应用（edge/chrome），供封面刷新逻辑区分处理
            currentIsBrowser.value =
                app_id_str.includes("edge") || app_id_str.includes("chrome");
            // 检测 SMTC 来源应用是否发生了切换（应用包名变更），用于及时刷新歌词
            const appSwitched = currentAppIdStr.value !== '' && currentAppIdStr.value !== app_id_str;
            // 记录当前 SMTC 来源应用的包名，供恢复播放时的封面刷新逻辑判断
            currentAppIdStr.value = app_id_str;

            // 切换 SMTC 来源应用：立即清空旧应用残留的歌词，避免串歌词（新应用歌词就绪前先显示标题）
            if (appSwitched) {
                parsedLyrics.value = [];
                lyricQueue.value = [];
                currentMatchedIndex = -1;
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
                currentDurationMs.value = durationMs; // 系统给的准，直接用
            } else if (parsedLyrics.value.length > 0) {
                // 系统没给时长，但我们有歌词！直接拿最后一句歌词时间 + 8秒尾奏
                const lastLyric = parsedLyrics.value[parsedLyrics.value.length - 1];
                currentDurationMs.value = lastLyric.time + 8000;
            }

            currentSongName.value = song;
            currentArtistName.value = artist || t('unknownArtist');
            const newTrackInfo = artist ? `${song} - ${artist}` : song;

            if (currentBaseInfo.value !== newTrackInfo) {
                currentBaseInfo.value = newTrackInfo;

                // 切歌时，第一时间重置本地时间轴！
                if (!isWsActive) {
                    localPositionMs.value = positionMs; // 必须补上这行，否则新歌会继承老歌的时间！
                    lastLyricChangeTime = performance.now() + 2000;
                }
                
                // 切歌立刻把折叠态文本更新为 "标题 - 歌手"，
                fillCollapsedWithTrackInfo();

                // PotPlayer：不做歌词匹配，清空可能残留的歌词队列，标题常驻显示
                if (isPotplayerSource.value) {
                    parsedLyrics.value = [];
                    lyricQueue.value = [];
                    currentMatchedIndex = -1;
                }

                // 切换播放内容时强制重新获取封面：清掉该曲目的封面缓存，避免沿用旧封面
                coverCache.delete(newTrackInfo);
                blurredCoverCache.delete(newTrackInfo);

                if (app_id_str.includes("edge") || app_id_str.includes("chrome")) {
                    // 浏览器：先回退到默认应用图标，再尝试用 SMTC 本地封面覆盖
                    isSmtcCoverActive.value = false;
                    coverUrl.value = APP_COVER_LOGO_MAP[app_id_str.includes("edge") ? "edge" : "chrome"];
                    fetchAndApplySmtcCover(newTrackInfo);
                } else if (!app_id_str.includes("bilibili") && artist !== "potplayer") {
                    if (app_id_str.includes("potplayer")) {
                        // PotPlayer 音乐模式：忽略 SMTC 封面，强制走网络获取
                        isSmtcCoverActive.value = false;
                    }
                    fetchAndApplyCover(newTrackInfo, song, artist, false, true, !app_id_str.includes("potplayer"));
                    console.log("获取封面ing");
                }

                if (!app_id_str.includes("potplayer")) {
                    // 浏览器/视频类应用使用内置 logo 封面（import 引用，打包后路径才会被 Vite 正确重写）
                    for (const [key, logo] of Object.entries(APP_COVER_LOGO_MAP)) {
                        if (app_id_str.includes(key)) {
                            // 已拿到 SMTC 封面则不再退回默认应用图标
                            if (!isSmtcCoverActive.value) {
                                coverUrl.value = logo;
                            }
                            break;
                        }
                    }
                } else {
                    if (artist == "potplayer") { // PotPlayer视频模式
                        console.log("直接使用PotPlayerLogo");
                        coverUrl.value = potplayerLogo;
                        // 清除上个封面的缓存与沉浸背景：防止 coverglass 背景残留上一首歌的模糊封面
                        blurredCoverUrl.value = '';
                        isSmtcCoverActive.value = false;
                        blurredCoverCache.clear();
                        coverCache.clear();
                    }
                }

                // 仅在 WS 不活跃时，发起 HTTP 网络歌词兜底（PotPlayer 不拉歌词，标题常驻）
                // 切换 SMTC 应用后 WS 心跳可能仍属于旧应用，此时也立即用 HTTP 兜底，保证新歌歌词及时到位
                if ((!isWsActive || appSwitched) && !isPotplayerSource.value) {
                    invoke<string>('fetch_netease_lyrics', { songName: song, artistName: artist, durationMs })
                        .then(lrc => {
                            if (appSwitched || Date.now() - lastWsLyricTime > 3000) {
                                if (lrc) {
                                    parsedLyrics.value = parseLrc(lrc);
                                    // 刚拉到歌词时，如果发现时长还是 0，立刻补救
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
            }
        } else {
            // SMTC 未检测到播放器
            if (!isWsActive) {
                setSafeTrackInfo(`${t('noSongPlaying')} - ${getPlayerName()}`);
                isPlaying.value = false;
                coverUrl.value = '';

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
        'other': t('genericMediaFull')
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
    // 兜底：去掉 .exe 后缀后直接展示包名
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

const setSafeTrackInfo = (text: string) => {
    // 1. 终极过滤：剔除所有空白、零宽字符
    if (!text || text.replace(/[\s\u200B-\u200D\uFEFF\u3000]/g, '').length === 0) return;

    // 2. 防重判定：如果和当前屏幕上的一样，或者和队列排在最后的一样，拒收
    if (text === currentTrackInfo.value && renderQueue.length === 0) return;
    if (renderQueue.length > 0 && renderQueue[renderQueue.length - 1] === text) return;

    // 3. 扔进强制渲染队列，绝不使用 clearTimeout 取消任何一句话！
    renderQueue.push(text);
    drainRenderQueue();
};

const drainRenderQueue = () => {
    // 如果正在播动画，或者队列空了，直接挂机
    if (isRendering || renderQueue.length === 0) return;

    const nextText = renderQueue.shift();
    if (!nextText || nextText === currentTrackInfo.value) {
        drainRenderQueue(); // 跳过重复，继续查下一个
        return;
    }

    // 上锁！开始渲染新文字
    isRendering = true;
    currentTrackInfo.value = nextText;

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

    // 4. 动画护城河：强制锁死 350ms！
    // 必须等 Vue 的 out-in 动画完美落幕，才允许渲染下一句！
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

    console.log('[calculateScroll]', { textWidth, containerWidth, text: currentTrackInfo.value });

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

        // 滚到底的保证：动画总时长不得超过本句展示时长。
        // 否则长句被 90px/s 钳制 / 4.5s 保底拉长后，还没滚到末尾就被下一句顶掉，
        // 表现为「有时候歌词滚不到底」。文字在展示期内即可滚完并停住结尾。
        totalDuration = Math.max(Math.min(totalDuration, displayWindowSec), 0.8);

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
                }else if (rxDiff <= 0)
                    networkStatus.value = 'error';     // 无流量变化，说明网络异常
                console.log(latency, rxDiff, txDiff);
            } catch (error) {
                console.error('延迟获取失败:', error);
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

// ==================== 窗口坐标保存/恢复（物理坐标为唯一标准，免疫 DPI/缩放） ====================
const POSITION_KEYS = {
    physX: 'nsd_island_phys_x',
    physY: 'nsd_island_phys_y',
    logX: 'nsd_island_log_x',
    logY: 'nsd_island_log_y',
    centerX: 'nsd_island_center_x',
    y: 'nsd_island_y',
    x: 'nsd_island_x',
};

// 判定坐标是否可用（拦截幽灵坐标/未就绪坐标）
const isUsablePosition = (x: number, y: number) =>
    Number.isFinite(x) && Number.isFinite(y) && x > -10000 && y > -10000 && !(x === 0 && y === 0);

// 读取已保存的物理坐标；无有效缓存返回 null
const loadSavedPhysicalPos = (): { x: number; y: number } | null => {
    const px = localStorage.getItem(POSITION_KEYS.physX);
    const py = localStorage.getItem(POSITION_KEYS.physY);
    if (px === null || py === null) return null;
    const x = parseFloat(px);
    const y = parseFloat(py);
    return isUsablePosition(x, y) ? { x: Math.round(x), y: Math.round(y) } : null;
};

// 将当前位置（物理坐标）写入缓存
const persistWindowPosition = async (win: Window) => {
    const pos = await win.outerPosition();
    if (!isUsablePosition(pos.x, pos.y)) return;
    localStorage.setItem(POSITION_KEYS.physX, String(Math.round(pos.x)));
    localStorage.setItem(POSITION_KEYS.physY, String(Math.round(pos.y)));
};

// 保存"物理中心点 + 顶部 y"（重置位置专用：窗口宽度可调整时仍保持居中）
const persistCenteredPosition = async (win: Window) => {
    const pos = await win.outerPosition();
    if (!isUsablePosition(pos.x, pos.y)) return;
    const size = await win.innerSize();
    const centerX = Math.round(pos.x + size.width / 2);
    localStorage.setItem(POSITION_KEYS.centerX, String(centerX));
    localStorage.setItem(POSITION_KEYS.y, String(Math.round(pos.y)));
};

// 读取期望的物理位置：优先"左边缘"（用户手动拖动），其次"物理中心"（重置居中，按当前宽度换算左边缘）
const loadExpectedPos = async (finalWPhysical: number): Promise<{ x: number; y: number } | null> => {
    const phys = loadSavedPhysicalPos();
    if (phys) return phys;
    const cxRaw = localStorage.getItem(POSITION_KEYS.centerX);
    const cyRaw = localStorage.getItem(POSITION_KEYS.y);
    if (cxRaw !== null && cyRaw !== null) {
        const cx = parseFloat(cxRaw);
        const cy = parseInt(cyRaw, 10);
        if (isUsablePosition(cx, cy)) {
            return { x: Math.round(cx - finalWPhysical / 2), y: cy };
        }
    }
    return null;
};

// 设置位置并验证，防止窗口句柄未就绪时 setPosition 被静默丢弃
const setPositionWithVerify = async (win: Window, x: number, y: number) => {
    const target = new PhysicalPosition(x, y);
    await win.setPosition(target);
    const actual = await win.outerPosition();
    if (Math.abs(actual.x - x) > 2 || Math.abs(actual.y - y) > 2) {
        console.warn(`⚠️ 首次定位未生效 (目标 ${x},${y}，实际 ${actual.x},${actual.y})，300ms 后重试`);
        await new Promise(resolve => setTimeout(resolve, 300));
        await win.setPosition(target);
    }
};

// 判断目标物理坐标是否落在任一显示器范围内（含 50px 容差，防止拔掉外接屏后恢复出屏）
const isPosOnAnyMonitor = async (x: number, y: number) => {
    const monitors = await availableMonitors();
    return monitors.some((m) => {
        const { position, size } = m;
        return x >= position.x - 50 && x <= position.x + size.width + 50 &&
            y >= position.y - 50 && y <= position.y + size.height + 50;
    });
};

// 调整窗口位置到正确位置
const adjustWindowPosition = async () => {
    try {
        const appWindow = getCurrentWindow();
        // 增加等待时间，确保打包环境下窗口句柄稳定
        await new Promise((resolve) => setTimeout(resolve, 200));

        const monitor = await currentMonitor();
        if (!monitor) return;

        const scaleFactor = monitor.scaleFactor;

        // 【关键】实时计算当前应有的尺寸，不使用外部 ref 变量
        const { w, h } = getBaseSize();
        const finalW = w * appScale.value;
        const finalH = h * appScale.value;

        await appWindow.setSize(new PhysicalSize(
            Math.ceil(finalW * scaleFactor),
            Math.ceil(finalH * scaleFactor)
        ));

        // 重新获取设定后的真实物理尺寸，用于精准居中
        const windowSize = await appWindow.innerSize();
        const windowWidthPhysical = windowSize.width;

        const monitorWidthPhysical = monitor.size.width;
        const monitorLeftPhysical = monitor.position.x;
        const monitorTopPhysical = monitor.position.y;

        const x = monitorLeftPhysical + (monitorWidthPhysical - windowWidthPhysical) / 2;
        const y = monitorTopPhysical + (12 * scaleFactor);

        console.log('[居中定位]', {
            monitorLeft: monitorLeftPhysical, monitorTop: monitorTopPhysical,
            monitorW: monitorWidthPhysical, windowW: windowWidthPhysical,
            scale: scaleFactor, x: Math.round(x), y: Math.round(y),
        });

        await appWindow.setPosition(new PhysicalPosition(Math.round(x), Math.round(y)));
        // 验证居中是否真正生效
        const after = await appWindow.outerPosition();
        if (Math.abs(after.x - x) > 2 || Math.abs(after.y - y) > 2) {
            console.warn(`⚠️ 居中定位未生效 (目标 ${Math.round(x)},${Math.round(y)}，实际 ${after.x},${after.y})`);
        }
    } catch (error) {
        console.error('调整窗口位置失败:', error);
    } finally {
        // 仅当灵动岛处于显示状态时才拉起透明窗口；
        // 否则空窗口会残留并拦截该区域的鼠标点击（窗体隐藏 ≠ 窗口隐藏）
        if (!isIslandVisible.value) return;
        try {
            await invoke('show_window_no_activate', { label: 'widget' });
        } catch (e) {
            console.error(e);
        }
    }
};

const onEnter = (el: Element, done: () => void) => {
    // 确保入场时窗口可以被正常点击
    getCurrentWindow().setIgnoreCursorEvents(false).catch(() => { });

    const HTMLElement = el as HTMLElement;
    HTMLElement.style.transformOrigin = 'center top';
    let start = performance.now();

    // 👈 顺应前端参数调整出场缩放的物理曲线
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

    // 终极防休眠保险：就算系统把 requestAnimationFrame 彻底冻结了
    // 只要时间一到（350ms），强行结束动画并彻底隐藏物理窗口！
    setTimeout(() => {
        if (!isFinished) {
            // 兜底：如果卡死了，强行把透明度归零，防止残留像素拦截鼠标
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

    // 1. 全局动画锁：任何变形动画期间，绝对禁止拖拽
    if (isSizeAnimating) return;

    // 2. 状态锁：音乐展开、消息通知、系统提示期间，统统禁止拖拽！
    if (isMusicExpanded.value || isMusicExpanding.value || isMsgActive.value || displaySysToast.value) {
        // 发现企图拖拽，立刻打断施法
        isMouseDown = false;
        return;
    }

    // 如果固定到了任务栏或已锁定位置，则禁止拖动
    if (isPositionLocked.value) return;

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

    // 重置位置
    const resetPositionItem = await MenuItem.new({
        text: t('resetPosition'),
        id: 'reset_position',
        action: async () => {
            try {
                // 清除所有新旧格式的坐标缓存
                localStorage.removeItem('nsd_island_phys_x');
                localStorage.removeItem('nsd_island_phys_y');
                localStorage.removeItem('nsd_island_log_x');
                localStorage.removeItem('nsd_island_log_y');
                localStorage.removeItem('nsd_island_center_x');
                localStorage.removeItem('nsd_island_y');
                localStorage.removeItem('nsd_island_x');

                await adjustWindowPosition();
                // 重置后立即把居中的位置保存，确保下次启动能精确还原
                await new Promise(resolve => setTimeout(resolve, 100));
                await persistCenteredPosition(getCurrentWindow());
                showToast(t('positionReset'));
            } catch (error) {
                console.error(error);
            }
        }
    });

    // 锁定位置菜单项
    const toggleLockItem = await MenuItem.new({
        text: isPositionLocked.value ? t('unlockCurrentLocked') : t('lock'),
        id: 'toggle_lock',
        action: () => {
            isPositionLocked.value = !isPositionLocked.value;
            localStorage.setItem('nsd_position_locked', String(isPositionLocked.value));
            // 修改这里：根据状态触发 lock 或 unlock 专属通知
            showToast(
                isPositionLocked.value ? t('positionLocked') : t('positionUnlocked'),
                isPositionLocked.value ? 'lock' : 'unlock'
            );
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

// 记录全局灵动岛是否正在执行形变动画
let isSizeAnimating = false;
let sizeAnimTimer: number | null = null;
// 程序主动移动窗口（形变动画）的结束时间戳，用于拦截滞后的 onMoved 事件
let lastProgrammaticMoveEnd = 0;

// 在顶部声明缩放变量
const appScale = ref(Number(localStorage.getItem('nsd_app_scale')) || 1.0);

// 监听缩放变化，直接修改 html 根节点的 zoom，这是 Webkit 渲染最完美的缩放方式
watch(appScale, (newScale) => {
    (document.documentElement.style as any).zoom = newScale;
}, { immediate: true });

// 灵动岛核心代码！（完美防漂移+防裁切+防打断抖动）
const animateIslandSize = async (targetWidth: number, targetHeight: number) => {
    try {
        // 核心：计算最终的缩放尺寸
        const finalWidth = targetWidth * appScale.value;
        const finalHeight = targetHeight * appScale.value;

        // 1. 触发形变前：立刻上锁
        isSizeAnimating = true;
        if (sizeAnimTimer) clearTimeout(sizeAnimTimer);

        sizeAnimTimer = window.setTimeout(() => {
            isSizeAnimating = false;
            lastProgrammaticMoveEnd = Date.now();
        }, 500);

        const appWindow = getCurrentWindow();
        const realSize = await appWindow.innerSize();
        const scaleFactor = window.devicePixelRatio;

        const realStartW = realSize.width / scaleFactor;
        const realStartH = realSize.height / scaleFactor;

        await invoke('start_island_animation', {
            startWidth: realStartW,
            startHeight: realStartH,
            targetWidth: finalWidth,    // 👈 传给 Rust 放大后的目标宽度
            targetHeight: finalHeight,  // 👈 传给 Rust 放大后的目标高度
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
    isAnimationLocked = true;   // ⚡ 上锁！宣布进入神圣不可侵犯的展开周期

    animateIslandSize(nsdBaseWidth.value + 95, nsdBaseHeight.value + 4);

    // 2. 延迟 120 毫秒后，打断缩小，直接猛烈展开
    musicExpandAnimTimer = window.setTimeout(() => {
        isMusicExpanded.value = true;
        isMusicExpanding.value = false;
        animateIslandSize(nsdMusicExpandedWidth.value, 135);

        // 3. 根据 Rust 端的弹簧衰减频率，约 400ms 后动画彻底结束，此时解锁
        setTimeout(() => {
            isAnimationLocked = false;

            // 检查：如果在展开的这 520ms 里，用户鼠标已经移走了，那就立刻补发收缩命令！
            if (isPendingCollapse) {
                isPendingCollapse = false;
                collapseMusic();
            }
        }, 400);
    }, 120);
};

// 鼠标离开灵动岛时：立刻收缩！
const handleMouseLeave = () => {
    if (!isMusicExpanded.value && !isMusicExpanding.value) return;

    // 直接呼叫收缩。如果锁着，collapseMusic 会自动把它记到账上稍后执行
    collapseMusic();
};

// 鼠标重新移入灵动岛时：立刻打断收缩企图
const handleMouseEnter = () => {
    // 如果之前移出留下了收缩案底，但动画还没播完鼠标又回来了，直接取消这个案底
    isPendingCollapse = false;
};

watch(displayMusic, (newVal: boolean) => {
    if (!newVal) {
        collapseMusic(); // 一旦音乐岛被隐藏（不管是因为轮换还是手动关了），立刻收缩
    }
});

// 引入你的默认图标作为兜底
import defaultLogo from '../assets/logo.png';
const currentMsgIcon = ref(defaultLogo);

// 浏览器/视频类应用的内置 logo 封面（必须 import 引用，Vite 打包时才会重写资源路径）
import bilibiliLogo from '../assets/bilibili-logo.png';
import edgeLogo from '../assets/edge-logo.png';
import chromeLogo from '../assets/chrome-logo.png';
import potplayerLogo from '../assets/potplayer-logo.jpg';

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

    // 记录当前实例的启动时间戳
    const bootTime = Date.now();

    // 监听窗口移动：用户拖动停止 600ms 后保存位置（trailing 防抖）
    let moveTimeout: number | null = null;

    await appWindow.onMoved(() => {
        if (moveTimeout) clearTimeout(moveTimeout);

        moveTimeout = window.setTimeout(async () => {
            try {
                // 启动前 3 秒内的定位（系统/程序动作）不保存
                if (Date.now() - bootTime < 3000) return;
                // 形变动画期间不保存
                if (isSizeAnimating || isMusicExpanding.value) return;
                // 动画刚结束的滞后窗口内不保存（Rust SetWindowPos 属程序移动）
                if (Date.now() - lastProgrammaticMoveEnd < 1500) return;

                await persistWindowPosition(appWindow);
            } catch (e) {
                console.error('保存坐标失败:', e);
            }
        }, 600);
    });

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

        if (!isMsgActive.value && !displaySysToast.value && !isMusicExpanded.value && !isMusicExpanding.value) {
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
            } else {
                const { w, h } = getBaseSize();
                animateIslandSize(w, h);
            }
        }

        // 收到设置修改后，如果此时没有展开音乐或显示通知，则立即触发形变更新外观！
        if (!isMsgActive.value && !displaySysToast.value && !isMusicExpanded.value && !isMusicExpanding.value) {
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

    // 监听 Rust 底层发来的硬核资源数据
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
            // 开启静默模式时：如果没有活跃事件，立刻隐藏；如果有，保持显示
            if (!shouldShowInQuietMode.value && isIslandVisible.value) {
                isIslandVisible.value = false;
            } else if (shouldShowInQuietMode.value && !isIslandVisible.value) {
                await invoke('show_window_no_activate', { label: 'widget' });
                isIslandVisible.value = true;
            }
        } else {
            // 关闭静默模式时，立刻恢复常驻显示
            await invoke('show_window_no_activate', { label: 'widget' });
            isIslandVisible.value = true;
        }
    });

    await listen<{ language: AppLanguage }>('control-language', (event) => {
        currentLanguage.value = event.payload.language;
    });

    // 监听控制台发来的“自动隐藏”配置变更
    await listen<{ enabled: boolean }>('control-autohide-fs', (event) => {
        isAutoHideEnabled.value = event.payload.enabled;
    });

    // 监听 Rust 发来的系统级全屏状态变化
    await listen<boolean>('fullscreen-changed', async (event) => {
        const isFullscreen = event.payload;

        // 如果没开这个功能，直接无视
        if (!isAutoHideEnabled.value) return;

        if (isFullscreen) {
            // 检测到全屏：如果灵动岛当前是显示的，把它收起来，并做个案底
            if (isIslandVisible.value) {
                wasVisibleBeforeFullscreen = true;

                // 【核心修复：听你的，直接物理拔管！】
                // 瞬间让操作系统干掉这个窗口，绝不等待任何 Vue 动画
                getCurrentWindow().hide().catch(() => { });

                // 同步 Vue 状态（虽然会触发 onLeave，但物理窗口已经没了，不会有幽灵残留）
                isIslandVisible.value = false;
            }
        } else {
            // 退出全屏：如果进全屏前它是开着的，现在把它恢复出来
            if (wasVisibleBeforeFullscreen) {
                await invoke('show_window_no_activate', { label: 'widget' });

                // 等待 40ms 让透明窗口先挂载好，再拉开幕布，防止闪烁
                setTimeout(() => {
                    isIslandVisible.value = true;
                }, 40);

                wasVisibleBeforeFullscreen = false; // 销案
            }
        }
    });

    try {
        await appWindow.innerPosition();
    } catch (e) { }

    // 在启动调整位置前，先校准初始宽高变量（仅用于首次渲染，不参与坐标计算）
    const { w, h } = getBaseSize();
    currentWidth.value = w * appScale.value;
    currentHeight.value = h * appScale.value;

    // 安全恢复窗口位置（物理坐标为唯一标准，免疫 DPI/缩放/显示器变动）
    const safeRestorePosition = async () => {
        try {
            await new Promise(resolve => setTimeout(resolve, 250));

            const appWindow = getCurrentWindow();
            const monitor = await currentMonitor();
            if (!monitor) {
                console.warn('无法获取显示器信息，执行默认居中');
                await adjustWindowPosition();
                return;
            }

            // 先确保窗口尺寸正确（setSize 只接受物理尺寸）
            const scaleFactor = monitor.scaleFactor;
            const { w: realW, h: realH } = getBaseSize();
            const finalW = realW * appScale.value;
            const finalH = realH * appScale.value;
            await appWindow.setSize(new PhysicalSize(
                Math.ceil(finalW * scaleFactor),
                Math.ceil(finalH * scaleFactor)
            ));

            // 1️⃣ 期望位置：用户拖动的左边缘物理坐标，或重置居中的物理中心（按当前宽度换算左边缘）
            const expected = await loadExpectedPos(Math.ceil(finalW * scaleFactor));
            if (expected) {
                // 显示器变动保护：保存的位置已不在任何显示器上（如拔掉外接屏）→ 居中
                if (!(await isPosOnAnyMonitor(expected.x, expected.y))) {
                    console.warn(`⚠️ 保存的位置 (${expected.x},${expected.y}) 不在任何显示器上，执行默认居中`);
                    await adjustWindowPosition();
                    return;
                }
                await setPositionWithVerify(appWindow, expected.x, expected.y);
                console.log(`✅ 位置恢复成功: x=${expected.x}, y=${expected.y}`);
                return;
            }

            // 2️⃣ 旧版逻辑坐标（一次性迁移：逻辑 × 当前缩放 = 物理）
            const lxRaw = localStorage.getItem(POSITION_KEYS.logX);
            const lyRaw = localStorage.getItem(POSITION_KEYS.logY);
            if (lxRaw !== null && lyRaw !== null) {
                const lx = parseFloat(lxRaw);
                const ly = parseFloat(lyRaw);
                if (isUsablePosition(lx, ly)) {
                    await setPositionWithVerify(appWindow, Math.round(lx * scaleFactor), Math.round(ly * scaleFactor));
                    console.log(`⚠️ 旧版逻辑坐标迁移恢复: x=${lx}, y=${ly}`);
                    return;
                }
            }

            // 3️⃣ 无任何有效缓存 → 默认居中
            console.log('无有效缓存，执行默认居中');
            await adjustWindowPosition();

        } catch (error) {
            console.error('安全恢复位置失败，执行兜底居中:', error);
            await adjustWindowPosition();
        }
    };

    // 清理旧版残留键
    if (localStorage.getItem('nsd_island_x')) {
        localStorage.removeItem('nsd_island_x');
    }

    await safeRestorePosition();

    // 检查本地记录的灵动岛开关状态
    const isWidgetEnabled = localStorage.getItem('nsd_widget_visible') !== 'false';

    // 只有在【用户开启了灵动岛】且【没开静默模式】时，启动才自动拉开灵动岛
    if (isWidgetEnabled && !isMsgModeEnabled.value) {
        await invoke('show_window_no_activate', { label: 'widget' });
        isIslandVisible.value = true;

        // 窗口显示后再校验一次位置（隐藏状态下 setPosition 可能被系统丢弃）
        setTimeout(async () => {
            try {
                const w = getCurrentWindow();
                const monitor = await currentMonitor();
                if (!monitor) return;
                const scale = monitor.scaleFactor;
                const { w: rw } = getBaseSize();
                const physW = Math.ceil(rw * appScale.value * scale);
                const expected = await loadExpectedPos(physW);
                if (!expected) return;
                const actual = await w.outerPosition();
                if (Math.abs(actual.x - expected.x) > 2 || Math.abs(actual.y - expected.y) > 2) {
                    await w.setPosition(new PhysicalPosition(expected.x, expected.y));
                    console.log(`🔄 显示后位置修正: ${actual.x},${actual.y} → ${expected.x},${expected.y}`);
                }
            } catch (e) {
                console.error('显示后位置校验失败:', e);
            }
        }, 400);
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
                if (isMsgActive.value) {
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
                    msgTitle: msgTitle.value || msgAppName.value || "新通知",
                    msgBody: msgBody.value || "",
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

    // 启动时初始化同步一次托盘流光状态
    invoke('sync_tray_menu', { glow: isGlowBorderEnabled.value });

    // 监听托盘发来的 流光边框 开关信号
    await listen('tray-toggle-glow', () => {
        isGlowBorderEnabled.value = !isGlowBorderEnabled.value;
        localStorage.setItem('nsd_glow_border', String(isGlowBorderEnabled.value));
        showToast(isGlowBorderEnabled.value ? t('glowBorderEnabled') : t('glowBorderDisabled'));
    });

    // 监听托盘发来的 重置位置 信号
    await listen('tray-reset-pos', async () => {
        try {
            // 清理本地坐标缓存
            localStorage.removeItem('nsd_island_phys_x');
            localStorage.removeItem('nsd_island_phys_y');
            localStorage.removeItem('nsd_island_log_x');
            localStorage.removeItem('nsd_island_log_y');
            localStorage.removeItem('nsd_island_center_x');
            localStorage.removeItem('nsd_island_y');
            localStorage.removeItem('nsd_island_x');

            await adjustWindowPosition();
            // 重置后立即把居中的位置保存，确保下次启动能精确还原
            await new Promise(resolve => setTimeout(resolve, 100));
            await persistCenteredPosition(getCurrentWindow());
            showToast(t('positionReset'), 'sys');
        } catch (error) {
            console.error(error);
        }
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
                // 内容兜底逻辑保持不变
                msgBody.value = res.body || (res.title === res.app_name ? t('receivedNotification') : res.title);

                currentMsgIcon.value = getAppIcon(res.app_name);

                if (!isMsgActive.value) {
                    isMsgActive.value = true;
                    animateIslandSize(nsdMsgExpandedWidth.value, 65);
                }

                if ((window as any).msgTimer) clearTimeout((window as any).msgTimer);
                (window as any).msgTimer = setTimeout(() => {
                    isMsgActive.value = false;
                    const { w, h } = getBaseSize();
                    animateIslandSize(w, h);
                }, 5000);
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
            // 1. 先让透明的 OS 窗口容器显示，此时内部 DOM 为 v-show="false"，视觉上仍是隐形的
            await invoke('show_window_no_activate', { label: 'widget' });
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
            // PotPlayer：不做歌词匹配，标题常驻显示
            if (!isPotplayerSource.value && parsedLyrics.value.length > 0) {
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
                        if (nextLyric && nextLyric !== currentTrackInfo.value) {
                            setSafeTrackInfo(nextLyric);
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
                    spectrumData.value = data;
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
</style>