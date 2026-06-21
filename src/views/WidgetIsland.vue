<template>
    <transition @enter="onEnter" @leave="onLeave" :css="false">
        <div v-show="isIslandVisible" :class="['island-container', { 'has-music-border': isGlowBorderEnabled }]"
            @mousedown="handleMouseDown" :style="islandStyle" @contextmenu="handleRightClick">

            <div class="rainbow-border-glow" v-if="isGlowBorderEnabled"></div>

            <div class="island-core-content" :style="coreContentStyle">
                <div class="inner-wrapper">
                    <transition @enter="onInnerEnter" @leave="onInnerLeave" :css="false">
                        <div class="music-ctl-box" v-show="isMusicCtlEnabled" :key="musicBoxKey"
                            @mouseenter="handleMusicBoxEnter" @mouseleave="handleMusicBoxLeave">

                            <div class="album-cover" :class="{ 'is-playing': isPlaying }">
                                <div class="cover-inner"
                                    :style="coverUrl ? { backgroundImage: `url(${coverUrl})`, backgroundSize: 'cover' } : {}">
                                </div>
                            </div>

                            <transition name="fade">
                                <div class="music-controls" v-show="!showInfo">
                                    <button class="ctl-btn" @click="prevTrack">
                                        <svg viewBox="0 0 24 24" fill="currentColor">
                                            <path d="M6 6h2v12H6zm3.5 6l8.5 6V6z" />
                                        </svg>
                                    </button>

                                    <button class="ctl-btn play-btn" @click="togglePlay">
                                        <svg v-if="isPlaying" viewBox="0 0 24 24" fill="currentColor">
                                            <path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z" />
                                        </svg>
                                        <svg v-else viewBox="0 0 24 24" fill="currentColor"
                                            style="transform: translateX(1px);">
                                            <path d="M8 5v14l11-7z" />
                                        </svg>
                                    </button>

                                    <button class="ctl-btn" @click="nextTrack">
                                        <svg viewBox="0 0 24 24" fill="currentColor">
                                            <path d="M6 18l8.5-6L6 6v12zM16 6v12h2V6h-2z" />
                                        </svg>
                                    </button>
                                </div>
                            </transition>

                            <transition name="fade">
                                <div class="music-info-mask-box" v-show="showInfo">
                                    <div class="music-info-text">
                                        {{ currentTrackInfo }}
                                    </div>
                                </div>
                            </transition>
                        </div>
                    </transition>

                    <transition @enter="onInnerEnter" @leave="onInnerLeave" :css="false">
                        <div class="speed-box" v-show="!isMusicCtlEnabled" key="speed">
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
                    </transition>
                </div>

                <div :class="['status-dot', networkStatus]"></div>
            </div>
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

const coreContentStyle = computed(() => {
    // 这里的背景色应该跟着主题走，保持是不透明的（或者很高的不透明度），用来遮挡中间的流光
    if (islandTheme.value === 'white') {
        return {
            backgroundColor: '#ffffff'
        };
    }
    return {
        backgroundColor: '#000000' // 黑色主题时，核心区域为纯黑
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
let isClickingToggle = false;
// 【修改】流光边框默认状态完全镜像音乐控制器（只要音乐控制器开着它就开，关了就一起关）
const isGlowBorderEnabled = ref(isMusicCtlEnabled.value);

const coverUrl = ref('');
const coverCache = new Map<string, string>();

const togglePlay = async () => {
    isPlaying.value = !isPlaying.value;
    isClickingToggle = true;
    try {
        await invoke('control_system_media', { action: 'play_pause' });
    } catch (err) {
        console.error(err);
    }
    setTimeout(() => {
        isClickingToggle = false;
    }, 1500);
};

const prevTrack = async () => {
    await invoke('control_system_media', { action: 'prev' });
};

const nextTrack = async () => {
    await invoke('control_system_media', { action: 'next' });
};

// 核心同步函数：塞入到你的 fetchSpeedStats 同一频次的定时器中
// 核心同步函数：塞入到你的 fetchSpeedStats 同一频次的定时器中
const syncMusicStatus = async () => {
    try {
        // 1. 调用 Rust 提取网易云标题 [歌名, 歌手, 是否在播放]
        const res = await invoke<[string, string, boolean] | null>('fetch_netease_music_info');

        if (res) {
            const [song, artist, playing] = res;

            // 拼接新的歌曲信息
            const newTrackInfo = `${song} - ${artist}`;

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

            if (!isClickingToggle) {
                isPlaying.value = playing;
            }
        } else {
            // 没检测到播放时，清空状态
            currentTrackInfo.value = '未在播放歌曲 - 网易云音乐';
            isPlaying.value = false;
            coverUrl.value = ''; // 没歌时清空，显示默认的优美渐变色
        }
    } catch (err) {
        console.error('音乐信息获取失败:', err);
    }
};

const showInfo = ref(false);
const currentTrackInfo = ref('未在播放歌曲 - 未知歌手'); // 默认显示内容
let hideControlsTimer: number | null = null;

// 启动倒计时隐藏控件
const startHideTimer = () => {
    stopHideTimer();
    hideControlsTimer = window.setTimeout(() => {
        showInfo.value = true;
    }, 800);
};

const stopHideTimer = () => {
    if (hideControlsTimer) {
        clearTimeout(hideControlsTimer);
        hideControlsTimer = null;
    }
};

// 定义一个用于强制刷新的 key
const musicBoxKey = ref(0);

// 鼠标进入整个音乐控制区域
const handleMusicBoxEnter = () => {
    stopHideTimer();     // 清除可能正在倒计时的隐藏任务
    showInfo.value = false; // 立刻切换为显示控制按钮
};

// 鼠标彻底离开整个音乐控制区域
const handleMusicBoxLeave = () => {
    startHideTimer();    // 离开后，再开启倒计时恢复为歌曲名称显示
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

    // 创建“重置位置”菜单项
    const resetPositionItem = await MenuItem.new({
        text: '重置位置',
        id: 'reset_position',
        action: () => {
            // 点击后直接调用你原本写好的位置调整逻辑
            adjustWindowPosition().catch(console.error);
        }
    });

    const toggleGlowBorderItem = await MenuItem.new({
        text: isGlowBorderEnabled.value ? '关闭流光边框' : '开启流光边框',
        id: 'toggle_glow_border',
        enabled: isMusicCtlEnabled.value, // 核心约束：只有在音乐控制器开着的前提下，菜单才允许点击切换
        action: () => {
            isGlowBorderEnabled.value = !isGlowBorderEnabled.value;
        }
    });

    // 创建“关闭”菜单项
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
    await menu.append(toggleGlowBorderItem);
    await menu.append(resetPositionItem);
    await menu.append(closeItem); // 两个菜单项会上下排列

    // 4. 弹出菜单
    try {
        await menu.popup(position);
    } catch (error) {
        console.error('菜单弹出失败:', error);
    }
};

const onInnerEnter = (el: Element, done: () => void) => {
    const htmlEl = el as HTMLElement;
    let start = performance.now();

    if (htmlEl.classList.contains('music-ctl-box')) {
        const freq = 2.0;
        const decay = 10.5;
        const duration = 600;

        // 【关键防御】：在动画开跑前，死死按住初始状态，不给浏览器任何闪烁的机会
        htmlEl.style.transform = `translateY(20px)`;
        htmlEl.style.opacity = '0';

        const animate = (time: number) => {
            let t = (time - start) / 1000;
            let progress = (time - start) / duration;

            let spring = 1 - Math.cos(freq * t * 2 * Math.PI) * Math.exp(-decay * t);
            let opacity = Math.min(1, progress * 4);
            let y = 20 * (1 - spring);

            htmlEl.style.transform = `translateY(${y}px)`;
            htmlEl.style.opacity = opacity.toString();

            if (progress < 1) {
                requestAnimationFrame(animate);
            } else {
                htmlEl.style.transform = 'none';
                htmlEl.style.opacity = '1';
                done();

                // 【新增核心调用】控制箱入场弹簧动画完全结束后，开启 1s 倒计时隐藏控件
                startHideTimer();
            }
        };
        requestAnimationFrame(animate);
    } else {
        const duration = 200;
        // 【关键防御】：网速盒入场前也做初始化拦截
        htmlEl.style.opacity = '0';
        htmlEl.style.transform = 'none';

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
    }
};

const onInnerLeave = (el: Element, done: () => void) => {
    const htmlEl = el as HTMLElement;
    let start = performance.now();
    const duration = 150;

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

onMounted(async () => {
    document.addEventListener('contextmenu', (e) => {
        e.preventDefault();
    }, { capture: true }); // 使用捕获阶段，确保先于 Tauri 底层拦截

    // 【修改】统一合并后的音乐控制器状态监听器
    await listen<{ enabled: boolean }>('control-music-ctl', (event) => {
        const isEnabled = event.payload.enabled;
        isMusicCtlEnabled.value = isEnabled;

        // 核心联动：只要触发了开关指令，流光边框无视之前的状态，强行跟随最新开关状态
        isGlowBorderEnabled.value = isEnabled;

        if (isEnabled) {
            showInfo.value = false;
            musicBoxKey.value++;
            stopHideTimer();
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

    // 监听来自控制台的音乐控制器状态同步指令
    await listen<{ enabled: boolean }>('control-music-ctl', (event) => {
        const isEnabled = event.payload.enabled;
        isMusicCtlEnabled.value = isEnabled;

        // 【新增】同样加入联动逻辑
        if (localStorage.getItem('nsd_glow_border') === null) {
            isGlowBorderEnabled.value = isEnabled;
        }
    });

    await adjustWindowPosition();

    // 先显示透明的 Tauri 窗口，再触发 Vue 的灵动岛入场弹簧动画
    await getCurrentWindow().show();
    isIslandVisible.value = true;

    fetchSpeedStats();
    checkNetworkLatency();

    // 在你原有的每秒刷新定时器中，顺带执行音乐同步
    speedTimer = setInterval(() => {
        fetchSpeedStats();
        if (isMusicCtlEnabled.value) {
            syncMusicStatus(); // 当音乐控制器启用时，每秒顺带检查网易云状态
        }
    }, 1000) as unknown as number;

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

    startHideTimer();
});

onUnmounted(() => {
    clearInterval(speedTimer);
    clearInterval(pingTimer);
    stopHideTimer();
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

/* 1. 外层包裹层：负责裁切多余的流光，并提供呼吸扩散的高斯模糊效果 */
.island-container {
    position: absolute;
    top: 0;
    left: 0;
    width: 100% !important;
    height: 100% !important;
    border-radius: 100px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2px;
    /* 这里的 2px 决定了流光边框的粗细 */
    user-select: none;
    -webkit-user-select: none;
    overflow: hidden;
    /* 牢牢把流光裁剪成一条圆角线 */
    background: transparent;
    transition: background 0.4s ease;
}

/* 2. 隐藏在底层的巨大旋转渐变层 */
.rainbow-border-glow {
    position: absolute;
    /* 关键：把它的尺寸设定为远超出容器的巨大正方形，解决大方块旋转露角的问题 */
    width: 300px;
    height: 300px;
    top: calc(50% - 150px);
    left: calc(50% - 150px);
    z-index: 1;
    background: conic-gradient(from 0deg,
            #ff3b30, #ff9500, #ffcc00, #4cd964, #5ac8fa, #007aff, #5856d6, #ff3b30);
    animation: rainbow-rotate 3s linear infinite;
    will-change: transform;
}

/* 3. 核心遮罩内容块：挡在旋转渐变层的上方 */
.island-core-content {
    position: relative;
    z-index: 2;
    /* 压在流光上面 */
    width: 100%;
    height: 100%;
    border-radius: 98px;
    /* 稍微比外层缩小一点 */
    backdrop-filter: blur(20px) !important;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 14px;
    overflow: hidden;
}

/* 4. 顺时针匀速旋转 */
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

/* 核心改动：增加统一的内部绝对定位平替包裹层 */
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

/* 封面内部绑定背景图的 div */
.cover-inner {
    width: 100%;
    height: 100%;
    background-position: center;
    background-repeat: no-repeat;
    background-size: cover;
    transition: background-image 0.3s ease;
    /* 切换封面时平滑淡入 */
}

/* 正在播放时的旋转动画 */
.is-playing .cover-inner {
    animation: rotate 8s linear infinite;
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
    left: 24px;
    /* 紧贴 24px 宽的专辑封面 */
    right: 20px;
    /* 给右侧网络指示灯留出安全间距 */
    height: 100%;
    display: flex;
    align-items: center;
    overflow: hidden;
    padding-left: 8px;
    -webkit-app-region: no-drag;
    /* 允许在文本区域触发 hover */
    transform: translateY(-1px);

    /* 核心过渡遮罩：右侧文字溢出边缘呈渐隐效果 */
    mask-image: linear-gradient(to right, #000 75%, transparent 100%);
    -webkit-mask-image: linear-gradient(to right, #000 75%, transparent 100%);
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
</style>