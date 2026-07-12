<template>
    <transition @enter="onEnter" @leave="onLeave" :css="false">
        <div v-show="isIslandVisible" class="island-container" @mousedown="handleMouseDown" @mousemove="handleMouseMove"
            @mouseup="handleMouseUp" @mouseleave="handleMouseLeave" @mouseenter="handleMouseEnter" :style="islandStyle"
            @contextmenu="handleRightClick">

            <div class="island-core-content" :class="{ 'is-split-layout': isSplitMode }"
                :style="{ borderRadius: isExpandedSize ? '22px' : '98px' }">
                <div class="left-capsule" :style="coreContentStyle" :class="{ 'is-split': isSplitMode }">
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

                                <div v-else-if="sysToastType === 'battery-charge'"
                                    class="toast-icon battery-charge-icon">
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

                            <div v-else-if="showPomodoroText" class="pomodoro-text-box" key="pomodoro">
                                <svg viewBox="0 0 24 24" class="pomodoro-svg" fill="none" stroke="currentColor"
                                    stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <circle cx="12" cy="12" r="10"></circle>
                                    <polyline points="12 6 12 12 16 14"></polyline>
                                </svg>
                                <div class="pomodoro-info">
                                    <span class="pomodoro-time">{{ formattedIslandPomoTime }}</span>
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
                                    <div class="music-info-mask-box" ref="maskBoxRef">
                                        <div class="music-info-text single-line"
                                            :class="{ 'fade-out': isMusicExpanded }">
                                            <span class="scroll-inner" ref="textInnerRef"
                                                :class="{ 'is-scrolling': scrollDist > 0 }"
                                                :style="scrollDist > 0 ? { '--scroll-dist': scrollDist + 'px', '--scroll-duration': scrollDuration } : {}">
                                                {{ currentTrackInfo }}
                                            </span>
                                        </div>
                                        <div class="music-info-text double-line"
                                            :class="{ 'fade-in': isMusicExpanded }">
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
                                <transition name="speed-fade" mode="out-in">
                                    <div v-if="isShowingUpload" class="speed-item" key="upload">
                                        <span :class="['label', { 'high-traffic': isHighUpload }]">⬆</span>
                                        <span class="value">{{ uploadSpeed }}</span>
                                    </div>
                                    <div v-else class="speed-item" key="download">
                                        <span :class="['label', { 'high-traffic': isHighDownload }]">⬇</span>
                                        <span class="value">{{ downloadSpeed }}</span>
                                    </div>
                                </transition>
                            </div>
                        </transition>
                    </div>

                    <transition mode="out-in" @enter="onInnerEnter" @leave="onInnerLeave" :css="false">
                        <div v-if="isPomodoroExpanded" class="island-close-btn" @click.stop="closeIslandPomodoro"
                            key="close-btn">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                                stroke-linecap="round" stroke-linejoin="round">
                                <line x1="18" y1="6" x2="6" y2="18"></line>
                                <line x1="6" y1="6" x2="18" y2="18"></line>
                            </svg>
                        </div>

                        <div v-else-if="showSpectrumIndicator" class="audio-spectrum"
                            :class="{ 'is-playing': isPlaying, 'expanded': isMusicExpanded }" key="spectrum">
                            <span class="bar" v-for="(val, index) in spectrumData" :key="index"
                                :style="{ transform: `scaleY(${val})` }"></span>
                        </div>

                        <div v-else :class="['status-dot', networkStatus]" key="dot"></div>
                    </transition>
                </div>

                <transition name="pop">
                    <div class="right-circle" :style="coreContentStyle" v-if="isSplitMode"
                        @click.stop="isPomodoroExpanded = true" style="cursor: pointer;">

                        <svg class="progress-ring" viewBox="0 0 38 38" v-show="isPomodoroActive">
                            <circle class="progress-ring-circle" stroke="#ff4757" stroke-width="2" fill="transparent"
                                r="18" cx="19" cy="19" :style="{ strokeDashoffset: pomoProgressOffset }" />
                        </svg>

                        <svg viewBox="0 0 24 24" class="pomodoro-svg" fill="none" stroke="currentColor" stroke-width="2"
                            stroke-linecap="round" stroke-linejoin="round" style="position: relative; z-index: 2;">
                            <circle cx="12" cy="12" r="10"></circle>
                            <polyline points="12 6 12 12 16 14"></polyline>
                        </svg>
                    </div>
                </transition>
            </div>
        </div>
    </transition>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed, watch, nextTick, type CSSProperties } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow, currentMonitor, PhysicalPosition, LogicalPosition, PhysicalSize } from '@tauri-apps/api/window'; import { Menu, MenuItem } from '@tauri-apps/api/menu';
import { listen, emit } from '@tauri-apps/api/event';

const isIslandVisible = ref(false);
const isPomodoroActive = ref(localStorage.getItem('nsd_pomodoro_active') === 'true');
const isPomodoroVisible = ref(localStorage.getItem('nsd_pomodoro_visible') === 'true');
const islandPomoTime = ref(Number(localStorage.getItem('nsd_pomodoro_time')) || 1500);
const islandPomoTotalTime = ref(Number(localStorage.getItem('nsd_pomodoro_time')) || 1500);
let pomoCountdownTimer: number | null = null;
const isMenuOpen = ref(false);

// 计算环形进度条的 stroke 偏移量
// 半半径 r=18，周长 C ≈ 113.1
const pomoProgressOffset = computed(() => {
    const C = 113.1;
    if (islandPomoTotalTime.value <= 0) return 0; // 进度走完时完全消失（偏移量为 0）
    const ratio = islandPomoTime.value / islandPomoTotalTime.value;
    // 顺时针减少消失：当时间变少时，offset 逐渐变大，使得实线部分顺时针缩回
    return C * (1 - ratio);
});

// 将秒数转化为 XX:XX 格式的计算属性
const formattedIslandPomoTime = computed(() => {
    const m = Math.floor(islandPomoTime.value / 60).toString().padStart(2, '0');
    const s = (islandPomoTime.value % 60).toString().padStart(2, '0');
    return `${m}:${s}`;
});
// 核心倒计时定时器管理逻辑
const startPomoCountdown = () => {
    if (pomoCountdownTimer) clearInterval(pomoCountdownTimer);

    if (islandPomoTime.value <= 0) {
        cleanUpPomoExit();
        return;
    }

    pomoCountdownTimer = window.setInterval(() => {
        islandPomoTime.value--;

        if (islandPomoTime.value <= 0) {
            clearInterval(pomoCountdownTimer!);
            pomoCountdownTimer = null;
            cleanUpPomoExit();
        }
    }, 1000);
};
// 提取出干净的退出清理函数，确保多端强同步
const cleanUpPomoExit = () => {
    isPomodoroActive.value = false;
    isPomodoroVisible.value = false;
    isPomodoroExpanded.value = false;

    // 重置时间为预设时长
    const defaultTime = Number(localStorage.getItem('nsd_pomodoro_time')) || 1500;
    islandPomoTime.value = defaultTime;
    islandPomoTotalTime.value = defaultTime;

    // 清空本地标记
    localStorage.setItem('nsd_pomodoro_visible', 'false');
    localStorage.setItem('nsd_pomodoro_active', 'false');
    localStorage.setItem('nsd_pomodoro_started', 'false');

    showToast('专注结束，休息一下吧！', 'app');

    // 关键核心：向全应用广播重置信号（LiveActive.vue 会接住这个信号并把按钮洗白）
    emit('live-activity-toggle', { id: 'pomodoro', enabled: false, isReset: true });

    if (!isMsgActive.value && !displaySysToast.value && !isMusicExpanded.value && !isMusicExpanding.value) {
        const { w, h } = getBaseSize();
        animateIslandSize(w, h);
    }
};
const stopPomoCountdown = () => {
    if (pomoCountdownTimer) {
        clearInterval(pomoCountdownTimer);
        pomoCountdownTimer = null;
    }
};
// 这是唯一能真正关闭实时活动全岛模式的杀手锏
const closeIslandPomodoro = () => {
    // 仅仅退出全岛展开模式，恢复到分离模式
    isPomodoroExpanded.value = false;
    // 触发尺寸动画恢复
    if (!isMsgActive.value && !displaySysToast.value && !isMusicExpanded.value && !isMusicExpanding.value) {
        const { w, h } = getBaseSize();
        animateIslandSize(w, h);
    }
};

// 记录番茄钟小球是否被用户临时点击展开
const isPomodoroExpanded = ref(false);
// 当有实时活动，且当前不是大弹窗状态(没收到消息/没展开音乐)时，启动分割模式
const isSplitMode = computed(() => {
    if (isMsgActive.value || displaySysToast.value || isMusicExpanded.value || isMusicExpanding.value) {
        return false;
    }
    // 替换为 isPomodoroVisible
    return isPomodoroVisible.value && isMusicCtlEnabled.value && !isPomodoroExpanded.value;
});
// 用于判断左侧主区域是否显示番茄钟文本
const showPomodoroText = computed(() => {
    if (isMsgActive.value || displaySysToast.value || isMusicExpanded.value || isMusicExpanding.value) {
        return false;
    }
    // 替换为 isPomodoroVisible
    if (isPomodoroVisible.value && !isMusicCtlEnabled.value) {
        return true;
    }
    // 替换为 isPomodoroVisible
    if (isPomodoroVisible.value && isMusicCtlEnabled.value && isPomodoroExpanded.value) {
        return true;
    }
    if (isExpandedSize.value) {
        return true;
    }
    return false;
});

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
        animateIslandSize(250, 38);
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

// 1. 瞬间判定当前是否处于大窗口状态
const isExpandedSize = computed(() => isMusicExpanded.value || isMsgActive.value);

// 2. 外层容器：状态一变，立马切成目标圆角
const islandStyle = computed<CSSProperties>(() => {
    const linear = islandOpacity.value / 100;
    const alpha = Math.pow(linear, 1 / 2.2);

    const currentBgColor = isSplitMode.value
        ? 'transparent'
        : (islandTheme.value === 'white' ? `rgba(255, 255, 255, ${alpha})` : `rgba(0, 0, 0, ${alpha})`);

    const baseStyle = {
        backgroundColor: currentBgColor,
        color: islandTheme.value === 'white' ? '#000000' : '#ffffff'
    };

    return {
        ...baseStyle,
        width: '100vw',
        height: '100vh',
        // 只要展开就是 24px，收起就是 100px
        borderRadius: isExpandedSize.value ? '24px' : '100px',
        position: 'relative',
    };
});

// 3. 内层核心：永远比外层小 2px
const coreContentStyle = computed(() => {
    const linear = islandOpacity.value / 100;
    const alpha = Math.pow(linear, 1 / 2.2);

    // 展开 22px，收起 98px
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

// 律动频谱
const spectrumData = ref([0.35, 0.35, 0.35, 0.35, 0.35]);
let spectrumTimer: number;

// 封面url
const coverUrl = ref('');
const coverCache = new Map<string, string>();

// 记录是否开启了置于任务栏
const isPinnedToTaskbar = ref(localStorage.getItem('nsd_pin_taskbar') === 'true');
// 记录是否锁定了位置，并存到本地
const isPositionLocked = ref(localStorage.getItem('nsd_position_locked') === 'true');
// 记录消息模式开关状态
const isMsgModeEnabled = ref(localStorage.getItem('nsd_msg_mode') === 'true');

// 使用计算属性智能判断当前该显示谁
const displaySpeed = computed(() => !isMsgActive.value && !displaySysToast.value && !isMusicCtlEnabled.value && !isPomodoroVisible.value);
const displayMusic = computed(() => !isMsgActive.value && !displaySysToast.value && isMusicCtlEnabled.value && !showPomodoroText.value);

// 辅助函数：获取当前状态应该拥有的默认大小
const getBaseSize = () => {
    if (isPomodoroVisible.value) return { w: 250, h: 38 };
    if (displaySpeed.value) return { w: 150, h: 34 };
    return { w: 250, h: 38 };
};

// 监听内容切换，触发丝滑动画过渡
watch([displaySpeed, displayMusic, showPomodoroText], () => {
    if (!isMsgActive.value && !displaySysToast.value && !isMusicExpanded.value && !isMusicExpanding.value) {
        const { w, h } = getBaseSize();
        animateIslandSize(w, h);
    }
});

// 专门用于控制右侧常驻指示灯的独立计算属性（完全不受消息通知打断）
const showSpectrumIndicator = computed(() => {
    return isMusicCtlEnabled.value;
});

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

            // 新增这两行为了展开后的双行显示分别赋值
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
    const map: Record<string, string> = {
        'netease': '网易云音乐',
        'spotify': 'Spotify',
        'apple': 'Apple Music',
        'qqmusic': 'QQ音乐',
        'kugou': '酷狗音乐',
        'echo': 'Echo Music',
        'lx-music': '洛雪音乐',
        'other': '通用媒体'
    };
    return map[key] || '未知平台';
};

// 定义一个用于强制刷新的 key
const musicBoxKey = ref(0);

// 定义双行文本所需的单独变量
const currentSongName = ref('未在播放歌曲');
const currentArtistName = ref(getPlayerName());
const currentTrackInfo = ref(`未在播放歌曲 - ${getPlayerName()}`);

// 音乐滚动相关变量
const maskBoxRef = ref<HTMLElement | null>(null);
const textInnerRef = ref<HTMLElement | null>(null);
const scrollDist = ref(0);
const scrollDuration = ref('0s');

// 核心计算函数：判断文本是否超出容器，并动态调整滚动速度和时长
const calculateScroll = () => {
    if (!textInnerRef.value || !maskBoxRef.value) return;

    // 展开状态下不执行滚动
    if (isMusicExpanded.value) {
        scrollDist.value = 0;
        return;
    }

    // 使用 getBoundingClientRect() 获取无视父级限制的真实渲染宽度
    const textWidth = textInnerRef.value.getBoundingClientRect().width;
    const containerWidth = maskBoxRef.value.clientWidth;

    if (textWidth > containerWidth) {
        // 使用 Math.ceil() 强制取整，绝对不允许出现小数像素！
        scrollDist.value = Math.ceil(textWidth - containerWidth + 20);

        // 按照 30px/s 的速度阅读，计算纯移动时间
        const timeToMove = scrollDist.value / 30;

        // 将首尾各停留的 1s 左右（基于20%占比计算）融入总时长中，确保匀速
        const totalDuration = timeToMove / 0.6;

        scrollDuration.value = `${Math.max(totalDuration, 4.5)}s`;
    } else {
        scrollDist.value = 0;
    }
};

// 监听数组必须带上 displayMusic，并在 nextTick 后加上微小延迟，防止 v-else-if 导致宽度拿到 0
watch([currentTrackInfo, displayMusic, isMusicExpanded, isSplitMode], async () => {
    await nextTick();
    setTimeout(() => {
        if (displayMusic.value) {
            calculateScroll();
        } else {
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

// 监听网络状态变化，触发系统通知
watch(networkStatus, (newStatus, oldStatus) => {
    // 忽略初始化时的变化，确保是真的状态翻转
    if (oldStatus && oldStatus !== newStatus) {
        if (newStatus === 'error') {
            showToast('网络连接已断开', 'sys');
        } else if (newStatus === 'good' && oldStatus === 'error') {
            showToast('网络已恢复连接', 'sys');
        }
    }
});

// 调整窗口位置到正确位置
const adjustWindowPosition = async () => {
    try {
        const appWindow = getCurrentWindow();
        await new Promise((resolve) => setTimeout(resolve, 150));
        const monitor = await currentMonitor();

        if (monitor) {
            const scaleFactor = window.devicePixelRatio;

            const WINDOW_INIT_WIDTH = currentWidth.value;
            const WINDOW_INIT_HEIGHT = currentHeight.value;
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

    // 如果音乐灵动岛正在展开或已完全展开，强制禁止呼出右键菜单
    if (isMusicExpanded.value || isMusicExpanding.value || isMsgActive.value || displaySysToast.value) {
        return;
    }

    // 打开控制台
    const openSettingsItem = await MenuItem.new({
        text: '打开控制台',
        id: 'open_settings',
        action: async () => {
            await emit('open-settings-panel');
            showToast('已打开控制台');
        }
    });

    // 重置位置
    const resetPositionItem = await MenuItem.new({
        text: isPinnedToTaskbar.value ? '重置位置 (已锁定)' : '重置位置',
        id: 'reset_position',
        enabled: !isPinnedToTaskbar.value,
        action: async () => {
            try {
                await adjustWindowPosition();
                showToast('已重置位置');
            } catch (error) {
                console.error(error);
            }
        }
    });

    // 锁定位置菜单项
    const toggleLockItem = await MenuItem.new({
        text: isPositionLocked.value ? '解锁 (当前已锁定)' : '锁定',
        id: 'toggle_lock',
        enabled: !isPinnedToTaskbar.value,
        action: () => {
            isPositionLocked.value = !isPositionLocked.value;
            localStorage.setItem('nsd_position_locked', String(isPositionLocked.value));
            // 修改这里：根据状态触发 lock 或 unlock 专属通知
            showToast(
                isPositionLocked.value ? '锁定位置成功' : '位置已解锁',
                isPositionLocked.value ? 'lock' : 'unlock'
            );
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

// 灵动岛核心代码！（完美防漂移+防裁切+防打断抖动）
const animateIslandSize = async (targetWidth: number, targetHeight: number) => {
    try {
        // 1. 触发形变前：立刻上锁
        isSizeAnimating = true;
        if (sizeAnimTimer) clearTimeout(sizeAnimTimer);

        // 2. 设定 500ms 后自动解锁（覆盖大多数弹簧动画的持续时间）
        sizeAnimTimer = window.setTimeout(() => {
            isSizeAnimating = false;
        }, 500);

        const appWindow = getCurrentWindow();
        const realSize = await appWindow.innerSize();
        const scaleFactor = window.devicePixelRatio;

        const realStartW = realSize.width / scaleFactor;
        const realStartH = realSize.height / scaleFactor;

        await invoke('start_island_animation', {
            startWidth: realStartW,
            startHeight: realStartH,
            targetWidth: targetWidth,
            targetHeight: targetHeight,
            isPinned: isPinnedToTaskbar.value
        });
    } catch (err) {
        console.error('呼叫 Rust 动画失败:', err);
        // 如果调用失败，安全起见立刻解锁，防止死锁
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

    // 1. 弹性按压动画 (先微微变小)
    animateIslandSize(245, 34);

    // 2. 延迟 120 毫秒后，打断缩小，直接猛烈展开
    musicExpandAnimTimer = window.setTimeout(() => {
        isMusicExpanded.value = true;
        isMusicExpanding.value = false;
        animateIslandSize(320, 115);

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

// 鼠标离开灵动岛时
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

// 图标映射器
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

    // 音乐控制器状态监听器
    await listen<{ enabled: boolean }>('control-music-ctl', (event) => {
        const isEnabled = event.payload.enabled;
        isMusicCtlEnabled.value = isEnabled;

        if (isEnabled) {
            showInfo.value = false;
            musicBoxKey.value++;
        }
    });

    // 监听系统底层事件（音量、电源）
    await listen<string>('system-event', (event) => {
        showToast(event.payload, 'sys');
    });

    // 监听电量显示
    await listen<{ state: 'charging' | 'discharging', percent: number }>('battery-event', (event) => {
        const { state, percent } = event.payload;

        if (state === 'charging') {
            showToast(`已接入电源，当前电量 ${percent}%`, 'battery-charge');
        } else if (state === 'discharging' && percent <= 20) {
            // 这里还可以加入防抖：只在刚掉到 20%、10%、5% 等关键节点触发一次，避免疯狂弹窗
            showToast(`电池电量低，剩余 ${percent}%`, 'battery-low');
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

            // 通知控制台恢复开关状态，让主面板的开关同步变绿（开启）
            await emit('island-status-sync', { visible: true });
        }
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
                isIslandVisible.value = false; // 这会自然触发你的弹簧收缩动画并隐藏窗口
            }
        } else {
            // 退出全屏：如果进全屏前它是开着的，现在把它恢复出来
            if (wasVisibleBeforeFullscreen) {
                await getCurrentWindow().show();

                // 等待 40ms 让透明窗口先挂载好，再拉开幕布，防止闪烁（复用你之前的完美体验逻辑）
                setTimeout(() => {
                    isIslandVisible.value = true;
                }, 40);

                wasVisibleBeforeFullscreen = false; // 销案
            }
        }
    });

    // 监听实时活动控制台指令 (优化版)
    await listen<{ id: string, enabled: boolean, time?: number, isReset?: boolean }>('live-activity-toggle', async (event) => {
        if (event.payload.id === 'pomodoro') {
            isPomodoroActive.value = event.payload.enabled;
            if (event.payload.time !== undefined) {
                islandPomoTime.value = event.payload.time;
                islandPomoTotalTime.value = event.payload.time;
            }
            if (event.payload.enabled) {
                // 启动/继续：确立可见性，并跑秒
                isPomodoroVisible.value = true;
                localStorage.setItem('nsd_pomodoro_visible', 'true');
                startPomoCountdown();
            } else {
                // 无论是暂停还是重置，都先停表
                stopPomoCountdown();
                // 果是控制台传过来的“重置”信号，果断执行彻底退场
                if (event.payload.isReset) {
                    isPomodoroVisible.value = false;      // 剥夺可见性
                    isPomodoroExpanded.value = false;     // 退出全岛展开模式
                    localStorage.setItem('nsd_pomodoro_visible', 'false');
                } else {
                    // 如果仅仅是普通的“暂停”，则保持可见性，只在未启动时归位时间
                    if (!event.payload.isReset && localStorage.getItem('nsd_pomodoro_started') === 'false') {
                        islandPomoTime.value = Number(localStorage.getItem('nsd_pomodoro_time')) || 1500;
                        islandPomoTotalTime.value = islandPomoTime.value; // 👈 新增：同步恢复时的总时长
                    }
                }
            }
        }
        // 刷新尺寸：如果是重置，此时 getBaseSize() 内部的 isPomodoroVisible 已经是 false 了，
        // 它会自动判断并计算出你启动番茄钟之前的岛屿尺寸（如网速模式或音乐模式），并执行丝滑缩回动画。
        if (!isMsgActive.value && !displaySysToast.value && !isMusicExpanded.value && !isMusicExpanding.value) {
            const { w, h } = getBaseSize();
            animateIslandSize(w, h);
        }
    });

    // 初始化位置追踪
    const appWindow = getCurrentWindow();
    try {
        await appWindow.innerPosition();
    } catch (e) { }

    // 在启动调整位置前，根据当前的实际状态，校准初始宽高
    const { w, h } = getBaseSize();
    currentWidth.value = w;
    currentHeight.value = h;

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

    fetchSpeedStats();
    checkNetworkLatency();

    // 启动网速和硬件显示轮换定时器 (每 5 秒切换一次)
    speedCycleTimer = window.setInterval(() => {
        // 网速轮换
        if (displaySpeed.value) {
            isShowingUpload.value = !isShowingUpload.value;
        }
    }, 5000);

    // 在你原有的每秒刷新定时器中，顺带执行音乐同步
    // 1. 高频定时器：专门负责网速和硬件监控（每 500ms ~ 1000ms 刷新一次）
    speedTimer = setInterval(async () => {
        // 强置顶逻辑
        if (isPinnedToTaskbar.value && isIslandVisible.value && !isMenuOpen.value) {
            invoke('force_window_topmost').catch(() => { });
        }

        // 刷新网速
        fetchSpeedStats();
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
                msgTitle.value = (res.title && res.title !== res.app_name) ? res.title : '新通知';
                // 单独把程序名存起来
                msgAppName.value = res.app_name;
                // 内容兜底逻辑保持不变
                msgBody.value = res.body || (res.title === res.app_name ? '收到一条新通知' : res.title);

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
                    const { w, h } = getBaseSize();
                    animateIslandSize(w, h);
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

    // 高频频谱拉取 (大约 20 帧/秒)
    spectrumTimer = setInterval(async () => {
        if (isPlaying.value && showSpectrumIndicator.value) {
            try {
                const data = await invoke<number[]>('get_audio_spectrum');
                spectrumData.value = data;
            } catch (err) {
                // 忽略错误，防止刷屏
            }
        } else {
            // 没在播放时，让柱子平滑回落到最低点
            spectrumData.value = [0.35, 0.35, 0.35, 0.35, 0.35];
        }
    }, 50) as unknown as number;

    // 初始化时触发一次计算
    setTimeout(() => {
        calculateScroll();
    }, 700);

    // 初始化检测：如果是开机或重新挂载，且原本就是运行状态，岛端直接开跑倒计时
    if (isPomodoroActive.value) {
        startPomoCountdown();
    }
});

onUnmounted(() => {
    window.removeEventListener('blur', collapseMusic);
    clearInterval(speedTimer);
    clearInterval(pingTimer);
    clearInterval(musicTimer);
    clearInterval(notifyTimer);
    clearInterval(spectrumTimer);
    if (speedCycleTimer) clearInterval(speedCycleTimer);
    if (pomoCountdownTimer) clearInterval(pomoCountdownTimer);
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

/* 外层包裹层：负责裁切多余的流光 */
.island-container {
    margin: 0 auto;
    border-radius: 100px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0px;
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

/* 核心遮罩内容块：挡在旋转渐变层的上方 */
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
    width: 26px;
    height: 26px;
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
    right: 18px;
    height: 100%;
    display: flex;
    align-items: center;
    overflow: hidden;
    padding-left: 0;
    -webkit-app-region: no-drag;
    transform: translateY(-1px) translateX(-0.5px);
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
    gap: 2px;
    height: 12px;
    padding-right: 2px;
}

/* 暂停状态下的竖线（统一高度） */
.audio-spectrum .bar {
    width: 2px;
    height: 18px;
    background-color: #b6e0ee;
    border-radius: 3px;
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
    backface-visibility: hidden;
    transform: translateZ(0);
    -webkit-font-smoothing: antialiased;
    transform-style: preserve-3d;
}

/* 挂载动画 */
.scroll-inner.is-scrolling {
    animation: scroll-ping-pong var(--scroll-duration) linear infinite alternate;
}

/* 滚动动画帧：利用 0-20% 和 80-100% 的区间实现两端停留 */
@keyframes scroll-ping-pong {

    0%,
    20% {
        transform: translateX(0);
    }

    80%,
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

/* 实时活动：分离式灵动岛布局 */
.island-core-content {
    /* 剥夺外层容器的背景，转变为完全透明的弹性容器 */
    padding: 0 !important;
    background: transparent !important;
}

/* 左侧主胶囊：继承了原本 CoreContent 的一切表现 */
.left-capsule {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    height: 100%;
    padding: 0 14px;
    position: relative;
    overflow: hidden;
    /* 防止音乐文字溢出切口 */
    transition: width 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

/* 分离模式下，左侧胶囊让出 */
.left-capsule.is-split {
    width: calc(100% - 44px);
}

/* 右侧独立实时小球 */
.right-circle {
    position: absolute;
    right: 0;
    width: 38px;
    height: 38px;
    border-radius: 50% !important;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.05);
}

/* 小球弹出动画 */
.pop-enter-active,
.pop-leave-active {
    transition: all 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

.pop-enter-from,
.pop-leave-to {
    opacity: 0;
    transform: scale(0.4) translateX(-30px);
    /* 从左侧胶囊内部挤出来 */
}

/* 番茄钟文本排版 */
.pomodoro-text-box {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    height: 100%;
    transform: translateX(-5px) !important;
}

.pomodoro-info {
    display: flex;
    flex-direction: row;
    align-items: center;
}

.pomodoro-time {
    font-size: 18px;
    font-weight: bold;
    color: #ff4757;
    font-variant-numeric: tabular-nums;
    letter-spacing: 0.5px;
    transform: translateY(-0.5px);
}

.pomodoro-svg {
    width: 24px;
    height: 24px;
    color: #ff4757;
}

/* 实时活动全岛关闭按钮 */
.island-close-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 30px;
    height: 30px;
    color: #888;
    cursor: pointer;
    transition: all 0.2s ease;
    border-radius: 50%;
    z-index: 10;
    transform: translateX(8px) !important;
}

.island-close-btn:hover {
    color: #ff4757;
    background-color: rgba(255, 71, 87, 0.15);
    transform: scale(1.15);
}

.island-close-btn svg {
    width: 20px;
    height: 20px;
}

/* 番茄钟进度条样式 */
.progress-ring {
    position: absolute;
    top: 0;
    left: 0;
    width: 38px;
    height: 38px;
    /* 核心修复：-90deg 让起点保持在正上方，scaleX(-1) 实现水平镜像，使其由逆时针变为顺时针流动 */
    transform: rotate(-90deg) scaleY(-1);
    z-index: 1;
}

.progress-ring-circle {
    stroke-dasharray: 113.1;
    stroke-linecap: round;
    transition: stroke-dashoffset 1s linear;
}
</style>