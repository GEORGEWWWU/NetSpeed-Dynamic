<template>
    <div class="gallery-layout-wrapper">
        <div class="gallery-header">
            <div class="title-wrapper">
                <h1 class="art-title">Live<span class="art-accent">Active</span><span class="art-dot">.</span></h1>
                <div class="title-sub-row">
                    <span class="title-badge">实时活动枢纽</span>
                    <span class="subtitle-divider">/</span>
                    <span class="subtitle-text">选择模块 · 激活体验</span>
                </div>
            </div>
        </div>

        <transition name="fade">
            <button v-show="canScrollLeft" class="scroll-btn scroll-btn-left" @click="scrollToLeft">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                    stroke-linejoin="round">
                    <polyline points="15 18 9 12 15 6"></polyline>
                </svg>
            </button>
        </transition>

        <transition name="fade">
            <button v-show="canScrollRight" class="scroll-btn scroll-btn-right" @click="scrollToRight">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                    stroke-linejoin="round">
                    <polyline points="9 18 15 12 9 6"></polyline>
                </svg>
            </button>
        </transition>

        <div class="gallery-track" ref="trackRef" @scroll="checkScroll"
            :class="{ 'mask-left': canScrollLeft, 'mask-right': canScrollRight }">

            <div v-for="item in activities" :key="item.id" class="gallery-card"
                :class="{ 'is-active': activeId === item.id }" :style="{ '--accent-color': item.accent }"
                @click="handleCardClick(item.id)">

                <div class="card-hero">
                    <div class="hero-top-row">
                        <div class="pro-icon" v-html="item.icon"></div>
                        <div v-if="item.id === 'pomodoro'" class="pomodoro-controls" @click.stop>
                            <button class="pomo-btn reset-btn" @click="handleResetBtn">重置</button>
                            <button class="pomo-btn toggle-btn" :class="{ 'is-running': item.enabled }"
                                @click="handleToggleBtn(item)">
                                {{ item.enabled ? '暂停' : (isStarted ? '继续' : '启动') }}
                            </button>
                        </div>
                        <label v-else class="custom-switch" @click.stop>
                            <input type="checkbox" v-model="item.enabled" :disabled="item.disable">
                            <span class="slider"></span>
                        </label>
                    </div>
                    <div class="hero-text">
                        <h3>{{ item.title }}</h3>
                        <p>{{ item.desc }}</p>
                    </div>
                </div>

                <div class="card-body" v-if="activeId === item.id">
                    <transition name="fade-up" appear>
                        <div class="body-content">
                            <template v-if="item.id === 'pomodoro'">
                                <div class="pro-setting-item mt-10"
                                    style="flex-direction: column; align-items: flex-start; gap: 12px; border-bottom: none; padding-bottom: 0;">
                                    <div class="pro-meta">
                                        <span class="pro-title">专注时长</span>
                                    </div>

                                    <div
                                        style="width: 100%; display: flex; justify-content: space-between; align-items: center;">
                                        <div v-if="!isEditingPomodoro" class="time-display"
                                            style="font-size: 28px; font-weight: 800; color: var(--accent-color); padding: 0; text-align: left;">
                                            {{ formattedPomodoroTime }}
                                        </div>

                                        <div v-else class="time-input-group" style="padding: 0; gap: 6px;">
                                            <input type="text" v-model="inputH" class="time-input"
                                                style="width: 48px; font-size: 16px; padding: 4px;" placeholder="时" />
                                            <span class="time-separator" style="font-size: 18px;">:</span>
                                            <input type="text" v-model="inputM" class="time-input"
                                                style="width: 48px; font-size: 16px; padding: 4px;" placeholder="分" />
                                            <span class="time-separator" style="font-size: 18px;">:</span>
                                            <input type="text" v-model="inputS" class="time-input"
                                                style="width: 48px; font-size: 16px; padding: 4px;" placeholder="秒" />
                                        </div>

                                        <button v-if="!isEditingPomodoro" class="time-edit-btn"
                                            style="padding: 4px 12px;" :disabled="isStarted"
                                            @click="startEditTime">设置</button>
                                        <button v-else class="time-edit-btn" style="padding: 4px 12px;"
                                            @click="saveTime">保存</button>
                                    </div>
                                </div>

                                <div style="border-bottom: 1px dashed var(--control-border); margin: 6px 0;"></div>

                                <div class="pro-setting-item" style="padding-top: 6px;">
                                    <div class="pro-meta">
                                        <span class="pro-title">免打扰模式</span>
                                    </div>
                                    <label class="custom-switch mini"><input type="checkbox"><span
                                            class="slider"></span></label>
                                </div>
                            </template>

                            <template v-else-if="item.id === 'flight'">
                                <div class="pro-input-group mt-10">
                                    <input type="text" class="custom-input" placeholder="输入航班号 (如 MU5137)" />
                                </div>
                                <div class="pro-input-group">
                                    <input type="date" class="custom-input" />
                                </div>
                            </template>

                            <template v-else-if="item.id === 'sports'">
                                <div class="pro-input-group mt-10">
                                    <div class="custom-select-wrapper">
                                        <select class="custom-input custom-select">
                                            <option>英格兰足球超级联赛 (EPL)</option>
                                            <option>NBA 篮球职业联赛</option>
                                        </select>
                                    </div>
                                </div>
                                <div class="pro-input-group">
                                    <input type="text" class="custom-input" placeholder="关注队伍 / Match ID" />
                                </div>
                            </template>

                            <template v-else-if="item.id === 'obs'">
                                <div class="pro-input-group mt-10">
                                    <input type="text" class="custom-input" placeholder="WebSocket 端口 (缺省 4455)" />
                                </div>
                                <div class="pro-input-group">
                                    <input type="password" class="custom-input" placeholder="连接密码鉴权" />
                                </div>
                            </template>

                            <template v-else>
                                <div class="pro-coming-soon">
                                    <div class="loader-line"></div>
                                    <p>模块部署中 SYSTEM_PENDING</p>
                                </div>
                            </template>
                        </div>
                    </transition>
                </div>
            </div>

            <div class="spacer"></div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted, nextTick, computed } from 'vue';
import { emit, listen } from '@tauri-apps/api/event';

const activities = ref([
    {
        id: 'pomodoro',
        icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><polyline points="12 6 12 12 16 14"></polyline></svg>',
        title: '专注番茄钟',
        desc: '沉浸工作时间管理',
        accent: '#ff4757',
        enabled: localStorage.getItem('nsd_pomodoro_active') === 'true',
        disable: false
    },
    {
        id: 'flight',
        icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M17.8 19.2L16 11l3.5-3.5C21 6 21.5 4 21.5 4c0 0-2 .5-3.5 2L14.5 9.5 6 7.5l-2 2 6 3-3.5 3.5-2.5-.5-2 2 3.5 1.5 1.5 3.5 2-2-.5-2.5L14 14l3.8 5.2z"></path></svg>',
        title: '航班实时追踪',
        desc: '延误与登机动态',
        accent: '#ff4757',
        enabled: false,
        disable: true
    },
    {
        id: 'sports',
        icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><path d="M12 2a14.5 14.5 0 0 0 0 20"></path><path d="M2 12h20"></path></svg>',
        title: '赛事比分看板',
        desc: '桌面实时球赛比分',
        accent: '#ff4757',
        enabled: false,
        disable: true
    },
    {
        id: 'obs',
        icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"></circle><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path></svg>',
        title: '直播录屏监控',
        desc: 'OBS 状态与防闭麦',
        accent: '#ff4757',
        enabled: false,
        disable: true
    },
    {
        id: 'printer',
        icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="6 9 6 2 18 2 18 9"></polyline><path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2"></path><rect x="6" y="14" width="12" height="8"></rect></svg>',
        title: '打印机队列',
        desc: '批量打印进度状态',
        accent: '#ff4757',
        enabled: false,
        disable: true
    },
]);

// 启动或暂停番茄钟
const handleToggleBtn = async (item: any) => {
    if (item.id === 'pomodoro') {
        if (isEditingPomodoro.value) {
            saveTime();
        }

        // 核心修正：显式切换状态
        const nextState = !item.enabled;
        item.enabled = nextState;

        if (nextState) {
            isStarted.value = true;
            localStorage.setItem('nsd_pomodoro_started', 'true');
            localStorage.setItem('nsd_pomodoro_active', 'true');
            localStorage.setItem('nsd_pomodoro_visible', 'true'); // 确保岛端可见
        } else {
            // 仅仅是暂停，不重置生命周期
            localStorage.setItem('nsd_pomodoro_active', 'false');
        }

        // 发送给灵动岛
        await emit('live-activity-toggle', {
            id: item.id,
            enabled: nextState
        });
    } else {
        item.enabled = !item.enabled;
        await emit('live-activity-toggle', {
            id: item.id,
            enabled: item.enabled
        });
    }
};

// 重置番茄钟 (彻底清空上下文，防止二次卡死)
const handleResetBtn = async () => {
    const pomoItem = activities.value.find(a => a.id === 'pomodoro');
    if (pomoItem) {
        pomoItem.enabled = false;
        isStarted.value = false;

        // 斩草除根式清空本地存储
        localStorage.setItem('nsd_pomodoro_active', 'false');
        localStorage.setItem('nsd_pomodoro_started', 'false');
        localStorage.setItem('nsd_pomodoro_visible', 'false');

        const savedTime = Number(localStorage.getItem('nsd_pomodoro_time')) || 1500;
        pomodoroTime.value = savedTime;

        await emit('live-activity-toggle', {
            id: 'pomodoro',
            enabled: false,
            time: savedTime,
            isReset: true
        });
    }
};
// 番茄钟时间设置逻辑
// 永久记忆化：后台依然存总秒数，默认 25 分钟（1500秒）
const pomodoroTime = ref(Number(localStorage.getItem('nsd_pomodoro_time')) || 1500);
const isEditingPomodoro = ref(false);
const isStarted = ref(localStorage.getItem('nsd_pomodoro_started') === 'true');

const inputH = ref('00');
const inputM = ref('25');
const inputS = ref('00');

// 展示态：把后台的总秒数，换算成 XX:XX:XX 展示
const formattedPomodoroTime = computed(() => {
    const h = Math.floor(pomodoroTime.value / 3600).toString().padStart(2, '0');
    const m = Math.floor((pomodoroTime.value % 3600) / 60).toString().padStart(2, '0');
    const s = (pomodoroTime.value % 60).toString().padStart(2, '0');
    return `${h}:${m}:${s}`;
});

// 纯正的 Vue 实时过滤函数：只留数字，且最多3位
const filterTimeInput = (val: string) => {
    const onlyNum = val.replace(/[^\d]/g, ''); // 擦除所有非数字（包括负号、e等）
    return onlyNum.slice(0, 3); // 死死锁死 3 位数
};

// 用 Vue 的 watch 替代原生 oninput，彻底解决失去焦点变成 9999 的 Bug
watch(inputH, (newVal) => { inputH.value = filterTimeInput(newVal); });
watch(inputM, (newVal) => { inputM.value = filterTimeInput(newVal); });
watch(inputS, (newVal) => { inputS.value = filterTimeInput(newVal); });

// 点击设置：把总秒数拆分给三个输入框
const startEditTime = () => {
    inputH.value = Math.floor(pomodoroTime.value / 3600).toString();
    inputM.value = Math.floor((pomodoroTime.value % 3600) / 60).toString();
    inputS.value = (pomodoroTime.value % 60).toString();
    isEditingPomodoro.value = true;
};

// 点击保存：把输入的 999:999:999 换算成总秒数存起来
const saveTime = () => {
    const h = Number(inputH.value) || 0;
    const m = Number(inputM.value) || 0;
    const s = Number(inputS.value) || 0;
    const totalSeconds = h * 3600 + m * 60 + s;
    pomodoroTime.value = totalSeconds;
    localStorage.setItem('nsd_pomodoro_time', totalSeconds.toString());
    isEditingPomodoro.value = false;
    // 如果是用户修改了时间设置，打断它先前的状态，并把新时间推给灵动岛
    isStarted.value = false;
    localStorage.setItem('nsd_pomodoro_started', 'false');
    emit('live-activity-toggle', { id: 'pomodoro', enabled: false, time: totalSeconds });
};

const activeId = ref('pomodoro');
const trackRef = ref<HTMLElement | null>(null);

// 滚动按钮可见性状态
const canScrollLeft = ref(false);
const canScrollRight = ref(true);

// 检查当前滚动位置并更新按钮状态
const checkScroll = () => {
    if (!trackRef.value) return;
    const { scrollLeft, clientWidth } = trackRef.value;

    canScrollLeft.value = scrollLeft > 1;

    // 获取所有卡片，按最后一张卡片的位置来判断
    const cards = trackRef.value.querySelectorAll('.gallery-card');
    if (cards.length > 0) {
        const lastCard = cards[cards.length - 1] as HTMLElement;
        // 最后一个卡片的右边缘物理位置（加上一点 padding 缓冲）
        const lastCardRightEdge = lastCard.offsetLeft + lastCard.offsetWidth + 24;

        // 如果当前视口的右侧边界还没碰到最后一张卡片的右边缘，才显示右滑按钮
        canScrollRight.value = Math.ceil(scrollLeft + clientWidth) < lastCardRightEdge;
    } else {
        canScrollRight.value = false;
    }
};

// 滚动到最左侧：直接激活第一个卡片并居中
const scrollToLeft = () => {
    if (activities.value.length > 0) {
        activateAndCenter(activities.value[0].id);
    }
};

// 滚动到最右侧：直接激活最后一个卡片并居中
const scrollToRight = () => {
    if (activities.value.length > 0) {
        activateAndCenter(activities.value[activities.value.length - 1].id);
    }
};

// 提取出一个共用的居中激活方法，使用纯数学预判
const activateAndCenter = (id: string) => {
    activeId.value = id;

    const container = trackRef.value;
    if (!container) return;

    const index = activities.value.findIndex(item => item.id === id);
    if (index === -1) return;

    // 预判计算
    const finalOffsetLeft = 4 + (index * 220);
    const finalActiveWidth = 320;

    // 计算居中偏移量，并确保最左侧安全边界不落空
    let targetScrollLeft = finalOffsetLeft - (container.clientWidth / 2) + (finalActiveWidth / 2);
    targetScrollLeft = Math.max(0, targetScrollLeft);

    container.scrollTo({ left: targetScrollLeft, behavior: 'smooth' });
};

// 点击任意卡片：激活并居中
const handleCardClick = (id: string) => {
    activateAndCenter(id);
};

onMounted(async () => {
    // 监听灵动岛倒计时跑完后发来的退场广播
    await listen<{ id: string, enabled: boolean, isReset?: boolean }>('live-activity-toggle', (event) => {
        if (event.payload.id === 'pomodoro' && event.payload.isReset) {
            const pomoItem = activities.value.find(a => a.id === 'pomodoro');
            if (pomoItem) {
                pomoItem.enabled = false; // 修正：强制确保 Vue 列表状态为 false
            }

            isStarted.value = false;
            // 彻底同步清除本地脏数据，防止 Ctrl+R 刷新后变“暂停”
            localStorage.setItem('nsd_pomodoro_active', 'false');
            localStorage.setItem('nsd_pomodoro_started', 'false');
            localStorage.setItem('nsd_pomodoro_visible', 'false');

            pomodoroTime.value = Number(localStorage.getItem('nsd_pomodoro_time')) || 1500;
        }
    });

    nextTick(() => { checkScroll(); });
    window.addEventListener('resize', checkScroll);
});

onUnmounted(() => {
    window.removeEventListener('resize', checkScroll);
});
</script>

<style scoped>
/* 框架布局*/
.gallery-layout-wrapper {
    display: flex;
    flex-direction: column;
    width: 100%;
    flex: 1;
    min-height: 0;
    overflow: hidden;
    position: relative;
}

.gallery-header {
    flex-shrink: 0;
    margin-bottom: 24px;
}

/* 标题区域 — 艺术字排版 */
.title-wrapper {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding-left: 4px;
}

/* 艺术主标题 */
.art-title {
    font-size: 60px;
    font-weight: 900;
    letter-spacing: -3px;
    line-height: 1;
    margin: 0;
    color: var(--h1-color, #1a1a1a);
    display: flex;
    align-items: baseline;
    user-select: none;
}

.art-accent {
    -webkit-text-fill-color: transparent;
    -webkit-text-stroke: 1.5px var(--h1-color, #1a1a1a);
    background: none;
}

.art-dot {
    color: #ef4444;
    font-size: 64px;
    line-height: 0.8;
    margin-left: 1px;
}

/* 副标题行 */
.title-sub-row {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-top: 2px;
    flex-wrap: wrap;
}

.title-badge {
    font-size: 11px;
    font-weight: 800;
    letter-spacing: 1.5px;
    color: var(--h1-color, #333);
    background: var(--control-bg, rgba(0, 0, 0, 0.04));
    padding: 4px 14px;
    border-radius: 100px;
    border: 1px solid var(--control-border, rgba(0, 0, 0, 0.06));
}

.subtitle-divider {
    font-size: 11px;
    color: var(--control-border, #ddd);
    font-weight: 300;
}

.subtitle-text {
    font-size: 12px;
    font-weight: 600;
    color: var(--subtitle-color, #888);
    letter-spacing: 0.3px;
}

/* 横向控制按钮 */
.scroll-btn {
    position: absolute;
    top: 280px;
    transform: translateY(-50%);
    z-index: 10;
    width: 44px;
    height: 44px;
    border-radius: 50%;
    background: var(--card-bg, #ffffff);
    border: 1px solid var(--control-border);
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--item-title-color);
    transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
}

.scroll-btn svg {
    width: 20px;
    height: 20px;
}

.scroll-btn:hover {
    transform: translateY(-50%) scale(1.1);
    border-color: var(--accent-color, #ccc);
    box-shadow: 0 6px 20px rgba(0, 0, 0, 0.12);
}

.scroll-btn-left {
    left: 4px;
}

.scroll-btn-right {
    right: 4px;
}

/* 横向容器 */
.gallery-track {
    display: flex;
    gap: 20px;
    overflow-x: auto;
    padding: 10px 50vw 30px 4px;
    align-items: stretch;
    flex-grow: 1;
    min-height: 0;
    scroll-behavior: smooth;
    scrollbar-width: none;
    -ms-overflow-style: none;
}

.gallery-track::-webkit-scrollbar {
    display: none;
}

/* 注册原生的 CSS 属性，赋予它们可过渡的能力 */
@property --mask-left-size {
    syntax: '<length>';
    initial-value: 0px;
    inherits: false;
}

@property --mask-right-size {
    syntax: '<length>';
    initial-value: 0px;
    inherits: false;
}

/* 容器边缘平滑渐罩 */
.gallery-track {
    /* 显式声明基础变量 */
    --mask-left-size: 0px;
    --mask-right-size: 0px;

    /* 统一使用单条渐变公式，完全由变量控制左右两侧的淡出区域 */
    -webkit-mask-image: linear-gradient(to right,
            transparent 0%,
            #000 var(--mask-left-size),
            #000 calc(100% - var(--mask-right-size)),
            transparent 100%);
    mask-image: linear-gradient(to right,
            transparent 0%,
            #000 var(--mask-left-size),
            #000 calc(100% - var(--mask-right-size)),
            transparent 100%);

    /* 核心：让这两个长度变量支持平滑过渡（0.4s 可以根据喜好调整） */
    transition: --mask-left-size 0.4s cubic-bezier(0.16, 1, 0.3, 1),
        --mask-right-size 0.4s cubic-bezier(0.16, 1, 0.3, 1);
}

/* 当需要显示左遮罩时，左边缘淡出宽度从 0px 平滑变大到 60px */
.gallery-track.mask-left {
    --mask-left-size: 60px;
}

/* 当需要显示右遮罩时，右边缘淡出宽度从 0px 平滑变大到 60px */
.gallery-track.mask-right {
    --mask-right-size: 60px;
}

.spacer {
    width: 50vw;
    flex-shrink: 0;
}

/* 专业工业化 Bento 卡片设计 */
.gallery-card {
    flex-shrink: 0;
    width: 200px;
    background: var(--card-bg);
    border: 1px solid var(--control-border);
    border-radius: 20px;
    cursor: pointer;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    position: relative;
    transform: scale(0.96);
    opacity: 0.45;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.02);
    transition: width 0.5s cubic-bezier(0.16, 1, 0.3, 1),
        transform 0.5s cubic-bezier(0.16, 1, 0.3, 1),
        opacity 0.3s ease,
        box-shadow 0.5s cubic-bezier(0.16, 1, 0.3, 1),
        border-color 0.5s cubic-bezier(0.16, 1, 0.3, 1);
}

.gallery-card:hover {
    opacity: 0.85;
}

.gallery-card.is-active {
    width: 320px;
    transform: scale(1);
    opacity: 1;
    cursor: default;
    border-color: var(--accent-color);
    box-shadow: 0 20px 40px -10px rgba(0, 0, 0, 0.1),
        0 0 0 1px var(--accent-color) inset;
}

:global(.dark-theme) .gallery-card.is-active {
    box-shadow: 0 20px 40px -10px rgba(0, 0, 0, 0.5),
        0 0 20px -5px var(--accent-color);
    background: linear-gradient(180deg, var(--control-bg) 0%, var(--card-bg) 100%);
}

/* 卡片头部 */
.card-hero {
    padding: 24px;
    display: flex;
    flex-direction: column;
    height: 140px;
    flex-shrink: 0;
    position: relative;
}

.hero-top-row {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: auto;
}

.pro-icon {
    width: 36px;
    height: 36px;
    color: var(--item-title-color);
    transition: all 0.4s ease;
}

.is-active .pro-icon {
    color: var(--accent-color);
    transform: scale(1.1);
}

.hero-text h3 {
    margin: 0;
    font-size: 18px;
    font-weight: 800;
    color: var(--h1-color);
    letter-spacing: -0.5px;
    transition: color 0.3s;
}

.hero-text p {
    margin: 6px 0 0 0;
    font-size: 12px;
    font-weight: 600;
    color: var(--subtitle-color);
    text-transform: uppercase;
    letter-spacing: 0.5px;
}

/* 卡片控制域 (展开区域) */
.card-body {
    padding: 0 24px 24px 24px;
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    overflow-y: auto;
}

.body-content {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
}

.fade-up-enter-active {
    transition: all 0.4s cubic-bezier(0.16, 1, 0.3, 1);
    transition-delay: 0.1s;
}

.fade-up-enter-from {
    opacity: 0;
    transform: translateY(15px);
}

/* 完全自定义 UI 控件 */

/* 1. 自定义 Switch 开关 */
.custom-switch {
    position: relative;
    display: inline-block;
    width: 44px;
    height: 24px;
    flex-shrink: 0;
}

.custom-switch.mini {
    width: 36px;
    height: 20px;
}

.custom-switch input {
    opacity: 0;
    width: 0;
    height: 0;
}

.custom-switch .slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: var(--control-border, #e2e8f0);
    border-radius: 24px;
    transition: background-color 0.3s ease;
}

.custom-switch .slider:before {
    position: absolute;
    content: "";
    height: 18px;
    width: 18px;
    left: 3px;
    bottom: 3px;
    background-color: #ffffff;
    border-radius: 50%;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.15);
    transition: transform 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

.custom-switch.mini .slider:before {
    height: 14px;
    width: 14px;
    left: 3px;
    bottom: 3px;
}

.custom-switch input:checked+.slider {
    background-color: var(--accent-color, #10b981);
}

.custom-switch input:checked+.slider:before {
    transform: translateX(20px);
}

.custom-switch.mini input:checked+.slider:before {
    transform: translateX(16px);
}

/* 处理 Switch 禁用状态下的指针效果 */
.custom-switch input:disabled+.slider {
    cursor: not-allowed;
    opacity: 0.6;
}

.custom-switch input:disabled {
    cursor: not-allowed;
}

/* 2. 通用 Input 基础样式 */
.pro-input-group {
    margin-bottom: 16px;
}

.custom-input {
    -webkit-appearance: none;
    appearance: none;
    width: 100%;
    background: rgba(0, 0, 0, 0.03);
    border: 1px solid var(--control-border, #e2e8f0);
    border-radius: 8px;
    padding: 12px 14px;
    font-size: 13px;
    font-weight: 600;
    color: var(--h1-color, #333);
    outline: none;
    transition: all 0.2s ease;
    box-sizing: border-box;
    /* ⬇️ 新增：防止内容溢出 */
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

:global(.dark-theme) .custom-input {
    background: rgba(255, 255, 255, 0.05);
}

.custom-input::placeholder {
    color: var(--item-desc-color, #999);
    font-weight: 500;
}

.custom-input:focus {
    border-color: var(--accent-color);
    box-shadow: 0 0 0 3px rgba(0, 0, 0, 0.05);
    background: transparent;
}

/* 3. 自定义 Select (利用 Wrapper 伪造下拉箭头) */
.custom-select-wrapper {
    position: relative;
    width: 100%;
}

.custom-select {
    padding-right: 36px;
    /* 给箭头留出空间 */
    cursor: pointer;
}

/* 自定义 SVG 箭头 */
.custom-select-wrapper::after {
    content: "";
    position: absolute;
    right: 14px;
    top: 50%;
    transform: translateY(-50%);
    width: 12px;
    height: 12px;
    pointer-events: none;
    background-image: url('data:image/svg+xml;utf8,<svg viewBox="0 0 24 24" fill="none" stroke="%23666666" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="6 9 12 15 18 9"></polyline></svg>');
    background-size: contain;
    background-repeat: no-repeat;
    opacity: 0.6;
}

/* 4. 自定义 Range Slider (滑块) */
.custom-range {
    -webkit-appearance: none;
    appearance: none;
    width: 100%;
    height: 6px;
    background: var(--control-border, #e2e8f0);
    border-radius: 4px;
    outline: none;
    margin-top: 4px;
    margin-bottom: 8px;
}

/* Chrome/Safari 轨道和滑块 */
.custom-range::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: var(--accent-color, #333);
    cursor: pointer;
    border: 3px solid #fff;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.2);
    transition: transform 0.2s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

.custom-range::-webkit-slider-thumb:hover {
    transform: scale(1.15);
}

/* Firefox 轨道和滑块 */
.custom-range::-moz-range-thumb {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: var(--accent-color, #333);
    cursor: pointer;
    border: 3px solid #fff;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.2);
}

.custom-range::-moz-range-track {
    background: var(--control-border, #e2e8f0);
    border-radius: 4px;
}

/* 排版辅助类 */
.pro-setting-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    gap: 12px;
    padding: 14px 0;
    border-bottom: 1px dashed var(--control-border);
}

.pro-setting-item:last-child {
    border-bottom: none;
}

.pro-meta {
    display: flex;
    flex-direction: column;
    gap: 4px;
    flex-shrink: 0;
    max-width: 100%;
}

.pro-title {
    font-size: 13px;
    font-weight: 700;
    color: var(--item-title-color);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.pro-desc {
    font-size: 11px;
    color: var(--item-desc-color);
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.mt-10 {
    margin-top: 10px;
}

.pro-coming-soon {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    justify-content: center;
    height: 100%;
    padding: 20px 0;
}

.loader-line {
    width: 40px;
    height: 3px;
    background: var(--control-border);
    position: relative;
    overflow: hidden;
    margin-bottom: 12px;
}

.loader-line::after {
    content: '';
    position: absolute;
    left: 0;
    top: 0;
    height: 100%;
    width: 15px;
    background: var(--item-title-color);
    animation: loader-slide 1.5s infinite ease-in-out;
}

@keyframes loader-slide {
    0% {
        transform: translateX(-15px);
    }

    100% {
        transform: translateX(40px);
    }
}

.pro-coming-soon p {
    font-size: 11px;
    font-weight: 700;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    color: var(--item-desc-color);
    margin: 0;
    letter-spacing: 0.5px;
}

/* 边缘按钮显隐过渡动画 */
.fade-enter-active,
.fade-leave-active {
    transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
    opacity: 0;
}

/* 番茄钟时间设置相关样式 */
.time-edit-btn {
    background: transparent;
    border: 1px solid var(--control-border);
    color: var(--item-title-color);
    border-radius: 6px;
    padding: 2px 10px;
    font-size: 11px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
    outline: none;
}

.time-edit-btn:hover {
    background: var(--control-border);
}

/* 当番茄钟运行时，禁用设置按钮的样式 */
.time-edit-btn:disabled {
    cursor: not-allowed;
    opacity: 0.5;
    background: transparent;
    border-color: var(--control-border);
    color: var(--item-desc-color);
}

.time-display {
    font-size: 22px;
    font-weight: 700;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    color: var(--item-title-color);
    text-align: center;
    letter-spacing: 2px;
    padding: 4px 0;
}

.time-input-group {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 4px 0;
}

.time-input {
    width: 44px;
    background: rgba(0, 0, 0, 0.03);
    border: 1px solid var(--control-border);
    border-radius: 8px;
    padding: 6px 4px;
    text-align: center;
    font-size: 14px;
    font-weight: bold;
    color: var(--h1-color);
    outline: none;
    transition: all 0.2s ease;
}

.time-input:focus {
    border-color: var(--accent-color);
    background: transparent;
}

:global(.dark-theme) .time-input {
    background: rgba(255, 255, 255, 0.05);
}

.time-separator {
    font-weight: bold;
    color: var(--item-title-color);
}

/* 隐藏输入框自带的上下箭头 */
.time-input::-webkit-outer-spin-button,
.time-input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
}

.time-input[type=number] {
    -moz-appearance: textfield;
    appearance: textfield;
}

/* 番茄钟右上角新按钮样式 */
.pomodoro-controls {
    display: flex;
    gap: 6px;
    align-items: center;
}

.pomo-btn {
    padding: 4px 10px;
    font-size: 12px;
    font-weight: 700;
    border-radius: 6px;
    cursor: pointer;
    border: 1px solid var(--control-border);
    background: var(--card-bg);
    color: var(--item-title-color);
    transition: all 0.2s ease;
    outline: none;
}

.pomo-btn:hover {
    background: var(--control-border);
}

/* 核心修正：当处于“启动”状态（未运行，没有 .is-running 类）时高亮为大红色 */
.toggle-btn:not(.is-running) {
    background: var(--accent-color, #ff4757);
    border-color: var(--accent-color, #ff4757);
    color: #fff;
}

.toggle-btn:not(.is-running):hover {
    opacity: 0.9;
}

/* 当点击变成“暂停”状态（正在运行，带有 .is-running 类）时，变回普通浅色按钮 */
.toggle-btn.is-running {
    background: var(--card-bg);
    border-color: var(--control-border);
    color: var(--item-title-color);
}
</style>