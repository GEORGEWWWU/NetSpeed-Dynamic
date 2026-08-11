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
                        <div v-if="isPositionAdjusting" class="position-adjustment-box" key="position-adjustment">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                                stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                                <path d="M12 2v20M2 12h20" />
                                <circle cx="12" cy="12" r="4" />
                            </svg>
                            <span>{{ t('dragIslandToPosition') }}</span>
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

                        <div v-else-if="displayMusic" class="music-ctl-box"
                            :class="{ 'expanded': isMusicExpanded, 'has-calendar': isMusicExpanded && showExpandedCalendar }"
                            :key="'music_' + musicBoxKey">
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
                                                {{ currentTrackInfo }}
                                            </span>
                                        </transition>
                                    </div>
                                    <div class="music-info-text double-line" :class="{ 'fade-in': isMusicExpanded }">
                                        <div class="song-title">{{ currentSongName }}</div>
                                        <div class="song-artist">{{ displayedArtistName }}</div>
                                    </div>
                                </div>
                            </div>

                            <div v-if="isMusicExpanded" class="audio-spectrum embedded"
                                :class="{ 'is-playing': isPlaying }" aria-hidden="true">
                                <span class="bar" v-for="(val, index) in spectrumData" :key="`expanded-${index}`"
                                    :style="{ transform: `scaleY(${val})` }"></span>
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

                            <div v-if="isMusicExpanded && hasExpandedDetails" class="expanded-monitor-row"
                                :class="{ 'has-calendar': showExpandedCalendar }"
                                :style="{ gridTemplateColumns: `repeat(${expandedMonitorCardCount}, minmax(0, 1fr))` }">
                                <div v-if="showExpandedResource" class="expanded-resource-summary">
                                    <div class="monitor-chip">
                                        <span class="monitor-chip-label">CPU</span>
                                        <span :class="['monitor-chip-value', { 'high-usage': cpuUsage >= 85 }]">{{ cpuUsage }}%</span>
                                        <span class="monitor-chip-track"><span class="monitor-chip-fill"
                                                :class="{ 'high-usage-bg': cpuUsage >= 85 }"
                                                :style="{ width: cpuUsage + '%' }"></span></span>
                                    </div>
                                    <div class="monitor-chip">
                                        <span class="monitor-chip-label">RAM</span>
                                        <span :class="['monitor-chip-value', { 'high-usage': ramUsage >= 85 }]">{{ ramUsage }}%</span>
                                        <span class="monitor-chip-track"><span class="monitor-chip-fill"
                                                :class="{ 'high-usage-bg': ramUsage >= 85 }"
                                                :style="{ width: ramUsage + '%' }"></span></span>
                                    </div>
                                </div>
                                <div v-if="showExpandedFps" class="fps-pill compact">
                                    <span class="fps-label">FPS</span>
                                    <span class="fps-value">{{ currentFps || '—' }}</span>
                                </div>
                                <div v-if="showExpandedSpeed" class="telemetry-speed-card">
                                    <Transition name="speed-fade" mode="out-in">
                                        <span v-if="isShowingUpload" class="telemetry-speed-line" key="expanded-up">
                                            <b>↑</b><em>{{ uploadSpeed }}</em>
                                        </span>
                                        <span v-else class="telemetry-speed-line" key="expanded-down">
                                            <b>↓</b><em>{{ downloadSpeed }}</em>
                                        </span>
                                    </Transition>
                                </div>
                            </div>

                            <div v-if="isMusicExpanded && showExpandedCalendar" class="expanded-calendar-card">
                                <div class="calendar-month">{{ calendarMonth }}</div>
                                <div class="calendar-date">{{ calendarDate }}</div>
                                <div class="calendar-weekday">{{ calendarWeekday }}</div>
                                <div class="calendar-grid" aria-label="Current month calendar">
                                    <span v-for="weekday in calendarWeekdays" :key="weekday" class="calendar-grid-weekday">{{ weekday }}</span>
                                    <span v-for="day in calendarDays" :key="day.dateKey"
                                        :class="['calendar-grid-day', { 'is-today': day.isToday }]">
                                        {{ day.value }}
                                    </span>
                                </div>
                            </div>
                        </div>

                        <div v-else-if="displayMonitors" class="monitor-dashboard"
                            :class="{
                                'expanded': isMusicExpanded,
                                'has-device-row': isMusicExpanded && hasExpandedDeviceDetails,
                                'has-calendar': isMusicExpanded && !displayMusic && showExpandedCalendar,
                            }"
                            :style="isMusicExpanded
                                ? { gridTemplateColumns: `repeat(${expandedMonitorCardCount}, minmax(0, 1fr))` }
                                : undefined"
                            key="monitors">
                            <div v-if="displayResource" class="resource-box">
                                <div class="res-group">
                                    <div class="res-info-row">
                                        <span class="res-title">
                                            <svg class="status-card-icon" viewBox="0 0 24 24" width="14" height="14"
                                                fill="none" stroke="currentColor" stroke-width="2.4"
                                                stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                                                <rect x="5" y="5" width="14" height="14" rx="2" />
                                                <rect x="9" y="9" width="6" height="6" rx="1" />
                                                <path d="M9 2v3M15 2v3M9 19v3M15 19v3M2 9h3M2 15h3M19 9h3M19 15h3" />
                                            </svg>
                                            <span class="res-label">CPU</span>
                                        </span>
                                        <span class="res-value" :class="{ 'high-usage': cpuUsage >= 85 }">{{ cpuUsage
                                        }}%</span>
                                    </div>
                                    <div class="res-bar-track">
                                        <div class="res-bar-fill" :style="{ width: cpuUsage + '%' }"
                                            :class="{ 'high-usage-bg': cpuUsage >= 85 }"></div>
                                    </div>
                                </div>
                                <div class="res-group">
                                    <div class="res-info-row">
                                        <span class="res-title">
                                            <svg class="status-card-icon" viewBox="0 0 24 24" width="14" height="14"
                                                fill="none" stroke="currentColor" stroke-width="2.4"
                                                stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                                                <rect x="3" y="6" width="18" height="12" rx="2" />
                                                <path d="M7 10v4M11 10v4M15 10v4M19 10v4M7 18v3M17 18v3" />
                                            </svg>
                                            <span class="res-label">RAM</span>
                                        </span>
                                        <span class="res-value" :class="{ 'high-usage': ramUsage >= 85 }">{{ ramUsage
                                        }}%</span>
                                    </div>
                                    <div class="res-bar-track">
                                        <div class="res-bar-fill" :style="{ width: ramUsage + '%' }"
                                            :class="{ 'high-usage-bg': ramUsage >= 85 }"></div>
                                    </div>
                                </div>
                            </div>
                            <div v-if="displayFps" class="fps-pill">
                                <span class="fps-title">
                                    <svg class="status-card-icon" viewBox="0 0 24 24" width="14" height="14"
                                        fill="none" stroke="currentColor" stroke-width="2.4"
                                        stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                                        <path d="M4.3 18a9 9 0 1 1 15.4 0" />
                                        <path d="m12 15 4-5" />
                                        <circle cx="12" cy="15" r="1" fill="currentColor" stroke="none" />
                                    </svg>
                                    <span class="fps-label">FPS</span>
                                </span>
                                <span class="fps-value">{{ currentFps || '—' }}</span>
                            </div>
                            <div v-if="displayMonitorSpeed" class="monitor-speed-section"
                                :class="{ 'has-divider': displayResource || displayFps }">
                                <Transition name="speed-fade" mode="out-in">
                                    <span v-if="isShowingUpload" class="telemetry-speed-line" key="monitor-up">
                                        <svg class="status-card-icon speed-direction-icon" viewBox="0 0 24 24"
                                            width="14" height="14" fill="none" stroke="currentColor" stroke-width="2.4"
                                            stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                                            <path d="M12 19V5M6.5 10.5 12 5l5.5 5.5" />
                                        </svg>
                                        <em>{{ uploadSpeed }}</em>
                                    </span>
                                    <span v-else class="telemetry-speed-line" key="monitor-down">
                                        <svg class="status-card-icon speed-direction-icon" viewBox="0 0 24 24"
                                            width="14" height="14" fill="none" stroke="currentColor" stroke-width="2.4"
                                            stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                                            <path d="M12 5v14M6.5 13.5 12 19l5.5-5.5" />
                                        </svg>
                                        <em>{{ downloadSpeed }}</em>
                                    </span>
                                </Transition>
                            </div>
                        </div>

                        <div v-else-if="displaySpeed" class="speed-box" key="speed">
                            <Transition name="speed-fade" mode="out-in">
                                <div v-if="nsdBaseWidth >= 240" key="dual" class="speed-dual-box">
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

                    </transition>

                    <div v-if="isMusicExpanded && !displayMusic && showExpandedCalendar"
                        class="expanded-calendar-card without-music">
                        <div class="calendar-month">{{ calendarMonth }}</div>
                        <div class="calendar-date">{{ calendarDate }}</div>
                        <div class="calendar-weekday">{{ calendarWeekday }}</div>
                        <div class="calendar-grid" aria-label="Current month calendar">
                            <span v-for="weekday in calendarWeekdays" :key="weekday" class="calendar-grid-weekday">{{ weekday }}</span>
                            <span v-for="day in calendarDays" :key="day.dateKey"
                                :class="['calendar-grid-day', { 'is-today': day.isToday }]">
                                {{ day.value }}
                            </span>
                        </div>
                    </div>

                    <transition name="device-row-fade">
                        <div v-if="isMusicExpanded && hasExpandedDeviceDetails" class="device-status-row"
                            :class="{
                                'with-music': displayMusic,
                                'without-music': !displayMusic,
                                'has-calendar': showExpandedCalendar,
                                'has-monitor-row': hasExpandedDetails,
                            }"
                            :style="{ gridTemplateColumns: `repeat(${expandedDeviceCardCount}, minmax(0, 1fr))` }">
                            <div v-if="showExpandedNetwork" class="device-status-card">
                                <svg class="device-status-icon" viewBox="0 0 48 48" fill="none" aria-hidden="true">
                                    <path d="M4 18.9653C15.5888 7.9865 33.3821 8.9029 44 18.9653M38 25.799C30.268 18.067 17.732 18.067 10 25.799M32 32.3137C27.5817 27.8954 20.4183 27.8954 16 32.3137"
                                        stroke="currentColor" stroke-width="4" stroke-linecap="round"
                                        stroke-linejoin="round" />
                                    <circle cx="24" cy="37.5" r="2.5" fill="currentColor" />
                                </svg>
                                <div class="device-status-copy">
                                    <span class="device-status-label">{{ t('networkConnection') }}</span>
                                    <strong :title="deviceStatus.networkName || t('notConnected')">{{
                                        deviceStatus.networkName || t('notConnected') }}</strong>
                                    <small>{{ networkStatusDetail }}</small>
                                </div>
                            </div>

                            <div v-if="showExpandedAudioOutput" class="device-status-card">
                                <svg class="device-status-icon" viewBox="0 0 48 48" fill="none" aria-hidden="true">
                                    <path d="M36 32C40.4183 32 44 28.4183 44 24C44 19.5817 40.4183 16 36 16M12 16C7.5817 16 4 19.5817 4 24C4 28.4183 7.5817 32 12 32M12 32V16C12 9.3726 17.3726 4 24 4C30.6274 4 36 9.3726 36 16V32C36 38.6274 30.6274 44 24 44"
                                        stroke="currentColor" stroke-width="4" stroke-linecap="round"
                                        stroke-linejoin="round" />
                                </svg>
                                <div class="device-status-copy">
                                    <span class="device-status-label">{{ t('audioOutput') }}</span>
                                    <strong :title="deviceStatus.audioOutputName || t('unavailable')">{{
                                        deviceStatus.audioOutputName || t('unavailable') }}</strong>
                                    <small>{{ t('defaultOutput') }}</small>
                                </div>
                            </div>

                            <div v-if="showExpandedBluetooth" class="device-status-card">
                                <svg class="device-status-icon" viewBox="0 0 48 48" fill="none" aria-hidden="true">
                                    <path d="M12 13L34 34L23 44V4L34 14L12 35" stroke="currentColor"
                                        stroke-width="4" stroke-linecap="round" stroke-linejoin="round" />
                                </svg>
                                <div class="device-status-copy">
                                    <span class="device-status-label">{{ t('bluetoothConnection') }}</span>
                                    <strong>{{ bluetoothConnectionText }}</strong>
                                </div>
                            </div>

                            <div v-if="showExpandedOutputVolume" class="device-status-card volume-card">
                                <svg class="device-status-icon" viewBox="0 0 48 48" fill="none" aria-hidden="true">
                                    <path d="M24 6V42C17 42 11.7985 32.8391 11.7985 32.8391H6C4.8954 32.8391 4 31.9437 4 30.8391V17.0108C4 15.9062 4.8954 15.0108 6 15.0108H11.7985C11.7985 15.0108 17 6 24 6Z"
                                        stroke="currentColor" stroke-width="4" stroke-linejoin="round" />
                                    <path d="M32 15C34.454 17.1919 36 20.3791 36 24C36 27.5895 34.4807 30.7517 32 33"
                                        stroke="currentColor" stroke-width="4" stroke-linecap="round"
                                        stroke-linejoin="round" />
                                </svg>
                                <div class="device-status-copy">
                                    <span class="device-status-label">{{ t('outputVolume') }}</span>
                                    <strong>{{ outputVolumeText }}</strong>
                                    <span class="device-volume-track"><span class="device-volume-fill"
                                            :style="{ width: `${deviceStatus.outputVolume || 0}%` }"></span></span>
                                </div>
                            </div>
                        </div>
                    </transition>
                </div>

                <transition mode="out-in" @enter="onInnerEnter" @leave="onInnerLeave" :css="false">
                    <div v-if="showSpectrumIndicator && !isMusicExpanded" class="audio-spectrum"
                        :class="{ 'is-playing': isPlaying }" key="spectrum">
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
import { getCurrentWindow, currentMonitor, PhysicalPosition, LogicalPosition, PhysicalSize } from '@tauri-apps/api/window'; import { Menu, MenuItem } from '@tauri-apps/api/menu';
import { listen, emit } from '@tauri-apps/api/event';
import { t, currentLanguage, type AppLanguage } from '../i18n';

const isIslandVisible = ref(false);
const isMenuOpen = ref(false);

// 统一监听灵动岛显隐，无论因为什么原因出现/消失，都立刻向控制台汇报
watch(isIslandVisible, (newVal) => {
    emit('island-status-sync', { visible: newVal });
});

// 记录全屏自动隐藏开关状态
const isAutoHideEnabled = ref(localStorage.getItem('nsd_autohide_fs') === 'true');
const isFullscreenAppActive = ref(false);
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
const isPositionAdjusting = ref(false);

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
// 除了拖动/切歌时的即时校准，播放中每 25 秒再向真实播放器时间线校准一次。
const AUTO_LYRIC_CALIBRATION_INTERVAL_MS = 25_000;
let lastAutoLyricCalibrationAt = 0;
// 酷狗切歌时，SMTC 标题比底部时间更早刷新。先等待底栏切换完成，
// 再采用第一条有效的真实时间，避免把上一首歌的时间误套到新歌上。
const kugouTimelineReady = ref(true);
let kugouTrackChangedAt = 0;
// 歌词防吞字与队列控制
const lyricQueue = ref<string[]>([]);
let lastLyricChangeTime = 0;
let currentMatchedIndex = -1;

// 对齐播放器进度，并在检测到明显跳播时丢弃旧歌词队列。
const alignLyricPosition = (positionMs: number, forceQueueReset = false) => {
    if (!Number.isFinite(positionMs)) return;

    const nextPosition = Math.max(0, positionMs);
    const isSeek = Math.abs(nextPosition - localPositionMs.value) > 1500;
    localPositionMs.value = nextPosition;

    if (forceQueueReset || isSeek) {
        lyricQueue.value = [];
        currentMatchedIndex = -1;
        lastLyricChangeTime = 0;
    }
};

const hasUsableSystemTimeline = (positionMs: number, durationMs: number) =>
    Number.isFinite(positionMs) && positionMs >= 0 && (durationMs > 0 || positionMs > 0);

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

// 律动频谱
const spectrumData = ref([0.35, 0.35, 0.35, 0.35, 0.35]);
let spectrumTimer: number;

// 封面url
const coverUrl = ref('');
const coverCache = new Map<string, string>();

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

// 实时FPS功能相关
const enableFps = ref(localStorage.getItem('nsd_fps_monitor') === 'true');
const pluginFps = ref(0);
const desktopFps = ref(0);
const lastPluginFpsAt = ref(0);
let desktopFpsTimer: number | null = null;

// 游戏/前台 3D 程序有数据时优先显示插件采集值；
// 暂时没有游戏帧时，回退到 Windows 桌面合成器的实际刷新率，避免把“无游戏数据”误显示为 0 FPS。
const currentFps = computed(() => {
    const pluginValueIsFresh = isFullscreenAppActive.value &&
        pluginFps.value > 0 && Date.now() - lastPluginFpsAt.value < 2500;
    return pluginValueIsFresh ? pluginFps.value : desktopFps.value;
});

const stopDesktopFpsSampler = () => {
    if (desktopFpsTimer !== null) {
        clearInterval(desktopFpsTimer);
        desktopFpsTimer = null;
    }
    desktopFps.value = 0;
};

const startDesktopFpsSampler = () => {
    stopDesktopFpsSampler();
    const refreshDesktopFps = async () => {
        if (!enableFps.value) return;
        try {
            const fps = await invoke<number>('get_desktop_fps');
            if (Number.isFinite(fps) && fps > 0) desktopFps.value = Math.round(fps);
        } catch (error) {
            console.error('读取 Windows 桌面帧率失败:', error);
        }

        if (pluginFps.value > 0 && Date.now() - lastPluginFpsAt.value >= 2500) {
            pluginFps.value = 0;
            lastPluginFpsAt.value = 0;
        }
    };

    refreshDesktopFps();
    desktopFpsTimer = window.setInterval(refreshDesktopFps, 1000);
};

// 记录最后一次接收到真实 WS 歌词的时间
let lastWsLyricTime = 0;
// 进度与播放状态分别记时，不能让普通歌词消息阻止系统播放器校准时间轴。
let lastWsProgressTime = 0;
let lastWsPlaybackTime = 0;

// WebSocket 状态管理
const isWsConnected = ref(false);
let unlistenWsStatus: (() => void) | null = null;

// WebSocket 实时歌词监听
let unlistenWs: (() => void) | null = null;

const initWebSocket = async () => {
    try {
        // 必须先挂载监听器，再去呼叫 Rust 连接！
        // 因为本地 WS 连接是毫秒级的，如果先 invoke 再 listen，Vue 会完美错过连接成功的初始信号！
        if (!unlistenWsStatus) {
            unlistenWsStatus = await listen('websocket-status', (event: any) => {
                isWsConnected.value = event.payload;
                if (isWsConnected.value) {
                    parsedLyrics.value = []; // 连上 WS 后，立刻清空可能残存的网络歌词，防止打架
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
                        lyricUnavailable.value = false;
                        isMediaActive.value = true; // 强制激活音乐卡片展示
                        isPlaying.value = true;     // 强制启动 50ms 歌词比对定时器

                        parsedLyrics.value = payload.lyrics.map((l: any) => ({
                            time: l.time,
                            text: l.text
                        }));
                        lyricQueue.value = [];
                        currentMatchedIndex = -1;
                        lastLyricChangeTime = 0; // 重置时间锁，允许立刻显示第一句歌词

                        // 有些歌词服务会在 init 中直接附带当前进度，优先立即对齐。
                        if (typeof payload.position === 'number') {
                            lastWsProgressTime = Date.now();
                            alignLyricPosition(payload.position, true);
                        } else {
                            // init 没有进度时，立即向 Windows 媒体会话读取当前位置。
                            // 不能等待普通的 2 秒轮询，否则歌词会先从第 1 句开始跑。
                            const initReceivedAt = Date.now();
                            invoke<[string, string, boolean, number, number] | null>('fetch_netease_music_info')
                                .then((mediaInfo) => {
                                    if (!mediaInfo || lastWsProgressTime > initReceivedAt) return;
                                    const [, , , positionMs, durationMs] = mediaInfo;
                                    if (hasUsableSystemTimeline(positionMs, durationMs)) {
                                        alignLyricPosition(positionMs, true);
                                    }
                                })
                                .catch((err) => console.error('歌词初始进度同步失败:', err));
                        }

                        return;
                    }

                    // 收到实时进度 (progress)
                    if (payload.type === 'progress') {
                        lastWsLyricTime = Date.now();
                        isMediaActive.value = true; // 心跳保活

                        if (typeof payload.position === 'number') {
                            lastWsProgressTime = Date.now();
                            // 修正：绝不能无脑覆盖，而是跟 HTTP 逻辑一样，误差大于 500ms 时才校准
                            // 这样才能让 50ms 定时器里的 `localPositionMs.value += delta` 完美发挥顺滑推算的作用！
                            if (Math.abs(payload.position - localPositionMs.value) > 500) {
                                alignLyricPosition(payload.position);
                            }
                        }
                        return;
                    }

                    // 收到播放状态 (playback)
                    if (payload.type === 'playback') {
                        lastWsLyricTime = Date.now();
                        lastWsPlaybackTime = Date.now();
                        if (payload.status === 'playing') {
                            isPlaying.value = true;
                            isMediaActive.value = true;
                        } else if (payload.status === 'paused') {
                            isPlaying.value = false;
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
                    lyricUnavailable.value = false;
                    isMediaActive.value = true;
                    isPlaying.value = true;
                    parsedLyrics.value = [];
                    lyricQueue.value = [];
                    setSafeTrackInfo(lyricText.trim());
                }
            });
        }

        // 47290 仅属于 JustSolo 等外部歌词服务器。网易云已有本机真实进度桥接，
        // 不应在未安装 JustSolo 时制造“歌词任务出错”的无效连接。
        if ((localStorage.getItem('nsd_target_player') || 'netease') === 'other') {
            await invoke('start_websocket_lyrics', { url: "ws://127.0.0.1:47290/" });
        }

    } catch (err) {
        console.error("WebSocket 启动失败:", err);
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

type DisplayPreferenceGroup = {
    music: boolean;
    resource: boolean;
    fps: boolean;
    speed: boolean;
};

type ExpandedDisplayPreferenceGroup = DisplayPreferenceGroup & {
    network: boolean;
    audioOutput: boolean;
    bluetooth: boolean;
    outputVolume: boolean;
    calendar: boolean;
};

type DisplayPreferences = {
    collapsed: DisplayPreferenceGroup;
    expanded: ExpandedDisplayPreferenceGroup;
};

type DeviceStatus = {
    networkName: string | null;
    networkType: string | null;
    networkSignal: number | null;
    audioOutputName: string | null;
    outputVolume: number | null;
    outputMuted: boolean;
    bluetoothConnectedCount: number | null;
};

const readDisplayPreference = (key: string) => localStorage.getItem(key) !== 'false';
const displayPreferences = ref<DisplayPreferences>({
    collapsed: {
        music: readDisplayPreference('nsd_display_collapsed_music'),
        resource: readDisplayPreference('nsd_display_collapsed_resource'),
        fps: readDisplayPreference('nsd_display_collapsed_fps'),
        speed: readDisplayPreference('nsd_display_collapsed_speed'),
    },
    expanded: {
        music: readDisplayPreference('nsd_display_expanded_music'),
        resource: readDisplayPreference('nsd_display_expanded_resource'),
        fps: readDisplayPreference('nsd_display_expanded_fps'),
        speed: readDisplayPreference('nsd_display_expanded_speed'),
        network: readDisplayPreference('nsd_display_expanded_network'),
        audioOutput: readDisplayPreference('nsd_display_expanded_audioOutput'),
        bluetooth: readDisplayPreference('nsd_display_expanded_bluetooth'),
        outputVolume: readDisplayPreference('nsd_display_expanded_outputVolume'),
        calendar: readDisplayPreference('nsd_display_expanded_calendar'),
    },
});

const deviceStatus = ref<DeviceStatus>({
    networkName: null,
    networkType: null,
    networkSignal: null,
    audioOutputName: null,
    outputVolume: null,
    outputMuted: false,
    bluetoothConnectedCount: null,
});
let isDeviceStatusRefreshing = false;

const refreshDeviceStatus = async () => {
    if (isDeviceStatusRefreshing || !isMusicExpanded.value || !hasExpandedDeviceDetails.value) return;
    isDeviceStatusRefreshing = true;
    try {
        deviceStatus.value = await invoke<DeviceStatus>('get_device_status');
    } catch (error) {
        console.error('读取系统设备状态失败:', error);
    } finally {
        isDeviceStatusRefreshing = false;
    }
};

// 功能开关只负责采集数据；显示偏好独立决定收起和展开时的内容。
const canShowPrimaryContent = computed(() => !isMsgActive.value && !displaySysToast.value);
const isMusicAvailable = computed(() => isMusicCtlEnabled.value && isMediaActive.value);
const showCollapsedResource = computed(() => enableSysResource.value && displayPreferences.value.collapsed.resource);
const showCollapsedFps = computed(() => enableFps.value && displayPreferences.value.collapsed.fps);
const showCollapsedSpeed = computed(() => displayPreferences.value.collapsed.speed);
const showCollapsedMusic = computed(() =>
    isMusicAvailable.value && displayPreferences.value.collapsed.music
);
const showExpandedMusic = computed(() => isMusicAvailable.value && displayPreferences.value.expanded.music);
const showExpandedResource = computed(() => enableSysResource.value && displayPreferences.value.expanded.resource);
const showExpandedFps = computed(() => enableFps.value && displayPreferences.value.expanded.fps);
const showExpandedSpeed = computed(() => displayPreferences.value.expanded.speed);
const showExpandedNetwork = computed(() => displayPreferences.value.expanded.network);
const showExpandedAudioOutput = computed(() => displayPreferences.value.expanded.audioOutput);
const showExpandedBluetooth = computed(() => displayPreferences.value.expanded.bluetooth);
const showExpandedOutputVolume = computed(() => displayPreferences.value.expanded.outputVolume);
const showExpandedCalendar = computed(() => displayPreferences.value.expanded.calendar);
const calendarNow = ref(new Date());
let calendarRefreshTimer: number | null = null;
const calendarMonth = computed(() => new Intl.DateTimeFormat('en-US', { month: 'short' })
    .format(calendarNow.value).toUpperCase());
const calendarDate = computed(() => calendarNow.value.getDate());
const calendarWeekday = computed(() => new Intl.DateTimeFormat('en-US', { weekday: 'long' })
    .format(calendarNow.value).toUpperCase());
const calendarStart = computed(() => {
    const start = new Date(calendarNow.value);
    start.setHours(0, 0, 0, 0);
    start.setDate(start.getDate() - start.getDay());
    return start;
});
const calendarWeekdays = computed(() => Array.from({ length: 7 }, (_, index) => {
    const date = new Date(calendarStart.value);
    date.setDate(date.getDate() + index);
    return new Intl.DateTimeFormat('en-US', { weekday: 'short' }).format(date).slice(0, 1).toUpperCase();
}));
const calendarDays = computed(() => {
    const now = calendarNow.value;
    return Array.from({ length: 14 }, (_, index) => {
        const date = new Date(calendarStart.value);
        date.setDate(date.getDate() + index);
        return {
            value: date.getDate(),
            isToday: date.toDateString() === now.toDateString(),
            dateKey: date.toISOString().slice(0, 10),
        };
    });
});
const expandedDeviceCardCount = computed(() =>
    Number(showExpandedNetwork.value) + Number(showExpandedAudioOutput.value) +
    Number(showExpandedBluetooth.value) + Number(showExpandedOutputVolume.value)
);
const expandedMonitorCardCount = computed(() =>
    Number(showExpandedResource.value) * 2 + Number(showExpandedFps.value) + Number(showExpandedSpeed.value)
);
const hasExpandedDeviceDetails = computed(() => expandedDeviceCardCount.value > 0);
const hasExpandedMonitors = computed(() => showExpandedResource.value || showExpandedFps.value);
const hasExpandedDetails = computed(() => hasExpandedMonitors.value || showExpandedSpeed.value);
const canExpandCard = computed(() => canShowPrimaryContent.value &&
    (showExpandedMusic.value || showExpandedResource.value || showExpandedFps.value ||
        showExpandedSpeed.value || hasExpandedDeviceDetails.value || showExpandedCalendar.value));

const networkStatusDetail = computed(() => {
    if (!deviceStatus.value.networkName) return t('unavailable');
    const parts = [deviceStatus.value.networkType];
    if (deviceStatus.value.networkSignal !== null) {
        parts.push(`${deviceStatus.value.networkSignal}%`);
    }
    return parts.filter(Boolean).join(' · ');
});

const outputVolumeText = computed(() => {
    if (deviceStatus.value.outputMuted) return t('muted');
    if (deviceStatus.value.outputVolume === null) return t('unavailable');
    return `${deviceStatus.value.outputVolume}%`;
});

const bluetoothConnectionText = computed(() => {
    if (deviceStatus.value.bluetoothConnectedCount === null) return t('unavailable');
    return `${deviceStatus.value.bluetoothConnectedCount}${t('devicesConnected')}`;
});

const displayMusic = computed(() => canShowPrimaryContent.value &&
    (isMusicExpanded.value ? showExpandedMusic.value : showCollapsedMusic.value));
const selectedResourceForState = computed(() =>
    isMusicExpanded.value ? showExpandedResource.value : showCollapsedResource.value
);
const selectedFpsForState = computed(() =>
    isMusicExpanded.value ? showExpandedFps.value : showCollapsedFps.value
);
const selectedSpeedForState = computed(() =>
    isMusicExpanded.value ? showExpandedSpeed.value : showCollapsedSpeed.value
);
const displayResource = computed(() => canShowPrimaryContent.value && !displayMusic.value && selectedResourceForState.value);
const displayFps = computed(() => canShowPrimaryContent.value && !displayMusic.value && selectedFpsForState.value);
const displayMonitorSpeed = computed(() => canShowPrimaryContent.value && !displayMusic.value &&
    selectedSpeedForState.value && (displayResource.value || displayFps.value));
const displayMonitors = computed(() => displayResource.value || displayFps.value || displayMonitorSpeed.value);
const displaySpeed = computed(() => canShowPrimaryContent.value && !displayMusic.value &&
    selectedSpeedForState.value && !displayResource.value && !displayFps.value);

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
        displayMusic.value &&
        !isMsgActive.value &&
        blurredCoverUrl.value;
});

// 辅助函数：获取当前状态应该拥有的默认大小
const getBaseSize = () => {
    if (displayMusic.value) return { w: nsdMusicBaseWidth.value, h: Math.max(nsdBaseHeight.value + 8, 42) };
    if (displayMonitors.value) {
        const visibleSegmentUnits = Number(displayResource.value) * 2 +
            Number(displayFps.value) + Number(displayMonitorSpeed.value) * 1.15;
        const monitorWidth = Math.max(nsdBaseWidth.value, 50 + visibleSegmentUnits * 80);
        return { w: monitorWidth, h: Math.max(nsdBaseHeight.value + 8, 42) };
    }
    if (displaySpeed.value) return { w: nsdBaseWidth.value, h: nsdBaseHeight.value };
    return { w: nsdMusicBaseWidth.value, h: Math.max(nsdBaseHeight.value + 8, 42) };
};

const getExpandedSize = () => {
    if (showExpandedMusic.value) {
        const detailSegmentUnits = Number(showExpandedResource.value) * 2 +
            Number(showExpandedFps.value) + Number(showExpandedSpeed.value) * 1.15;
        const expandedContentRows = Number(hasExpandedDetails.value) + Number(hasExpandedDeviceDetails.value);
        return {
            w: Math.max(nsdMusicExpandedWidth.value, 110 + detailSegmentUnits * 70,
                80 + expandedDeviceCardCount.value * 100, showExpandedCalendar.value ? 840 : 0),
            h: Math.max(124 + expandedContentRows * 53,
                showExpandedCalendar.value ? 250 : 0),
        };
    }

    const visibleSegmentUnits = Number(showExpandedResource.value) * 2 +
        Number(showExpandedFps.value) + Number(showExpandedSpeed.value) * 1.15;
    const expandedContentRows = Number(hasExpandedDetails.value) + Number(hasExpandedDeviceDetails.value);
    const compactContentHeight = expandedContentRows > 0
        ? 20 + expandedContentRows * 81 + Math.max(0, expandedContentRows - 1) * 8
        : 0;
    return {
        w: Math.max(nsdBaseWidth.value, 60 + visibleSegmentUnits * 90,
            80 + expandedDeviceCardCount.value * 100,
            showExpandedCalendar.value && (hasExpandedDetails.value || hasExpandedDeviceDetails.value) ? 740 : 0),
        h: Math.max(nsdBaseHeight.value + 30, compactContentHeight,
            showExpandedCalendar.value ? 190 : 0),
    };
};

// 监听内容切换，触发丝滑动画过渡
watch([displaySpeed, displayMusic, displayResource, displayFps, displayMonitorSpeed, displayMonitors], () => {
    // 只有在没有被临时弹窗（消息、音乐展开）霸占时，才执行基础大小切换
    if (!isMsgActive.value && !displaySysToast.value && !isMusicExpanded.value && !isMusicExpanding.value) {
        const { w, h } = getBaseSize();
        animateIslandSize(w, h);
    }
});

watch([showExpandedMusic, showExpandedResource, showExpandedFps, showExpandedSpeed,
    showExpandedNetwork, showExpandedAudioOutput, showExpandedBluetooth, showExpandedOutputVolume, showExpandedCalendar], () => {
    if (isMusicExpanded.value) {
        if (!canExpandCard.value) {
            collapseMusic();
            return;
        }
        const { w, h } = getExpandedSize();
        animateIslandSize(w, h);
        refreshDeviceStatus();
        refreshDeviceStatus();
    }
});

// 专门用于控制右侧常驻指示灯的独立计算属性（完全不受消息通知打断）
const showSpectrumIndicator = computed(() => {
    return displayMusic.value;
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

// 核心同步函数：负责获取状态并智能降级
const syncMusicStatus = async () => {
    try {
        const res = await invoke<[string, string, boolean, number, number] | null>('fetch_netease_music_info');

        // 判定过去 3 秒内是否有活跃的本地 WebSocket 推送
        const isWsActive = (Date.now() - lastWsLyricTime < 3000);
        const hasRecentWsProgress = (Date.now() - lastWsProgressTime < 3000);
        const hasRecentWsPlayback = (Date.now() - lastWsPlaybackTime < 3000);
        const hasWsLyricsTimeline = isWsConnected.value && parsedLyrics.value.length > 0;

        if (res) {
            const [song, artist, playing, positionMs, durationMs] = res;

            // 完整 WS 歌词时间轴存在时，不能让不兼容播放器返回的 false 覆盖歌词服务状态。
            // 暂停/恢复仍由 WS 的 playback 消息负责；没有 WS 歌词时才回退到 SMTC。
            if (!hasRecentWsPlayback && !hasWsLyricsTimeline && !isWsActive) {
                isPlaying.value = playing;
            }
            if (!isMediaActive.value) isMediaActive.value = true;
            isFirstMediaCheck = false;
            isNewlyEnabled = false;

            currentSongName.value = song;
            currentArtistName.value = artist || t('unknownArtist');
            const newTrackInfo = artist ? `${song} - ${artist}` : song;
            const targetPlayer = localStorage.getItem('nsd_target_player') || 'netease';
            const isKugouPlayer = targetPlayer === 'kugou';
            const supportsPeriodicLyricCalibration = targetPlayer === 'netease' || isKugouPlayer;
            const hasConfirmedTimeline = !isKugouPlayer || kugouTimelineReady.value;
            const now = Date.now();

            // 真实时间轴每 25 秒强制校准一次，防止本地 50ms 推算因掉帧、OCR 误差或
            // 播放器内部缓冲而逐渐偏离。切歌保护尚未完成时不能使用酷狗的时间。
            if (supportsPeriodicLyricCalibration && hasConfirmedTimeline &&
                currentBaseInfo.value === newTrackInfo &&
                hasUsableSystemTimeline(positionMs, durationMs) &&
                now - lastAutoLyricCalibrationAt >= AUTO_LYRIC_CALIBRATION_INTERVAL_MS) {
                alignLyricPosition(positionMs, Math.abs(positionMs - localPositionMs.value) > 1500);
                lastAutoLyricCalibrationAt = now;
            }

            // WS 只推歌词、不推进度时，仍持续使用 SMTC 校准；同时支持从中段播放和拖动进度条。
            if ((!isKugouPlayer || kugouTimelineReady.value) &&
                !hasRecentWsProgress && hasUsableSystemTimeline(positionMs, durationMs) &&
                Math.abs(positionMs - localPositionMs.value) > 800) {
                alignLyricPosition(positionMs - (playing ? 250 : 0));
            }

            if (currentBaseInfo.value !== newTrackInfo) {
                currentBaseInfo.value = newTrackInfo;
                lyricUnavailable.value = false;
                lastAutoLyricCalibrationAt = now;

                // 只有 WS 自己带完整歌词时才交给它管理歌词；仅提供播放器进度的桥接仍共用网络歌词逻辑。
                if (!hasWsLyricsTimeline) {
                    if (isKugouPlayer) {
                        kugouTimelineReady.value = false;
                        kugouTrackChangedAt = Date.now();
                        alignLyricPosition(0, true);
                        // 不采用切歌这一帧的时间：此时酷狗底栏可能仍显示上一首歌。
                        window.setTimeout(syncMusicStatus, 700);
                    } else if (hasUsableSystemTimeline(positionMs, durationMs)) {
                        alignLyricPosition(positionMs, true);
                    }

                    setSafeTrackInfo(newTrackInfo);
                    parsedLyrics.value = [];
                    lyricQueue.value = [];
                    currentMatchedIndex = -1;
                    lastLyricChangeTime = performance.now() + 2000;
                }

                if (coverCache.has(newTrackInfo)) {
                    coverUrl.value = coverCache.get(newTrackInfo)!;
                    blurredCoverUrl.value = blurredCoverCache.get(newTrackInfo) || '';
                } else {
                    invoke<string>('get_random_cover_url', { songName: song, artistName: artist })
                        .then(async url => {
                            coverUrl.value = url;
                            if (coverCache.size > 50) {
                                coverCache.clear();
                                blurredCoverCache.clear();
                            }
                            coverCache.set(newTrackInfo, url);
                            const bakedImage = await bakeBlurImage(url);
                            blurredCoverUrl.value = bakedImage;
                            blurredCoverCache.set(newTrackInfo, bakedImage);
                        }).catch(() => {
                            coverUrl.value = '';
                            blurredCoverUrl.value = '';
                        });
                }

                // WS 只有完整歌词列表时才跳过网络歌词；播放器进度桥接不能阻止歌词加载。
                if (!hasWsLyricsTimeline) {
                    invoke<string>('fetch_netease_lyrics', { songName: song, artistName: artist, durationMs })
                        .then(lrc => {
                            if (currentBaseInfo.value !== newTrackInfo ||
                                (isWsConnected.value && parsedLyrics.value.length > 0)) {
                                return;
                            }

                            const lyrics = lrc ? parseLrc(lrc) : [];
                            if (lyrics.length > 0) {
                                lyricUnavailable.value = false;
                                parsedLyrics.value = lyrics;
                            } else {
                                lyricUnavailable.value = true;
                                setSafeTrackInfo(`${newTrackInfo} · 未找到歌词`);
                            }
                        }).catch(() => { });
                }
            } else if (isKugouPlayer && !kugouTimelineReady.value) {
                const elapsedSinceTrackChange = Date.now() - kugouTrackChangedAt;
                // 至少等待 900ms，避开旧曲底栏残留；之后第一条有效 OCR 时间即可启动。
                if (hasUsableSystemTimeline(positionMs, durationMs) && elapsedSinceTrackChange >= 900) {
                    alignLyricPosition(positionMs, true);
                    kugouTimelineReady.value = true;
                } else if (elapsedSinceTrackChange >= 5000) {
                    // OCR 偶发识别失败不能让歌词永久停在歌名：先从开头恢复，
                    // 后续一旦读到真实时间，常规校准会立刻纠正。
                    alignLyricPosition(0, true);
                    kugouTimelineReady.value = true;
                } else {
                    window.setTimeout(syncMusicStatus, 500);
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

// 定义一个用于强制刷新的 key
const musicBoxKey = ref(0);

// 定义双行文本所需的单独变量
const currentSongName = ref(t('noSongPlaying'));
const currentArtistName = ref(getPlayerName());
const currentTrackInfo = ref(`${t('noSongPlaying')} - ${getPlayerName()}`);
const lyricUnavailable = ref(false);
const displayedArtistName = computed(() => lyricUnavailable.value
    ? `${currentArtistName.value} · 未找到歌词`
    : currentArtistName.value);

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

// 核心计算函数：判断文本是否超出容器，并动态调整滚动速度和时长
const calculateScroll = () => {
    if (!textInnerRef.value || !maskBoxRef.value) return;

    // 展开状态下不执行滚动
    if (isMusicExpanded.value) {
        scrollDist.value = 0;
        return;
    }

    const textWidth = textInnerRef.value.getBoundingClientRect().width;
    const containerWidth = maskBoxRef.value.clientWidth;

    // 关键修正：因为 CSS mask 从 75% 处开始渐变遮挡
    // 我们必须以这 75% 的“绝对清晰安全区”作为计算基准
    const safeWidth = containerWidth * 0.75;

    // 只要文字超过了安全区，哪怕还没超出整个物理盒子，也必须开始滚动！
    if (textWidth > safeWidth) {
        // 计算滚动距离：把文字的末尾准确无误地拖进安全区，外加 5px 的微小呼吸空间
        // 这样既不会挡住结尾，也不会像之前那样盲目多滚几十个像素
        scrollDist.value = Math.ceil(textWidth - safeWidth + 5);

        // 按照 30px/s 的速度阅读，计算纯移动时间
        const timeToMove = scrollDist.value / 30;

        // 将首尾各停留的 1s 左右（基于20%占比计算）融入总时长中，确保匀速
        const totalDuration = timeToMove / 0.6;

        scrollDuration.value = `${Math.max(totalDuration, 4.5)}s`;
    } else {
        scrollDist.value = 0;
    }
};

// 核心修复 2：监听数组必须带上 displayMusic，并在 nextTick 后加上微小延迟，防止 v-else-if 导致宽度拿到 0
watch([currentTrackInfo, displayMusic, isMusicExpanded], async () => {
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
            showToast(t('networkDisconnected'), 'sys');
        } else if (newStatus === 'good' && oldStatus === 'error') {
            showToast(t('networkRestored'), 'sys');
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

            const WINDOW_INIT_WIDTH = currentWidth.value;   // 默认 260
            const WINDOW_INIT_HEIGHT = currentHeight.value; // 默认 42
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

        // 核心修复：检查此时到底是不是该隐藏？
        // 如果在动画超时期间，灵动岛又被呼出了，绝对不能执行 hide！
        if (!isIslandVisible.value) {
            getCurrentWindow().hide().catch(console.error);
        }
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
                // 清空记忆坐标，让它下次启动时重新计算默认居中
                localStorage.removeItem('nsd_island_center_x'); // 更新为新键名
                localStorage.removeItem('nsd_island_y');
                localStorage.removeItem('nsd_island_x'); // 顺手抹掉旧版本的残留

                await adjustWindowPosition();
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

// 音乐控制器展开方法
const expandMusic = () => {
    if (isMusicExpanded.value || isMusicExpanding.value) return;

    isMusicExpanding.value = true;
    isPendingCollapse = false;  // 重置待办任务
    isAnimationLocked = true;   // ⚡ 上锁！宣布进入神圣不可侵犯的展开周期

    animateIslandSize(nsdBaseWidth.value + 95, nsdBaseHeight.value + 4);

    // 2. 延迟 120 毫秒后，打断缩小，直接猛烈展开
    musicExpandAnimTimer = window.setTimeout(() => {
        isMusicExpanded.value = true;
        isMusicExpanding.value = false;
        const { w, h } = getExpandedSize();
        animateIslandSize(w, h);

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

// 鼠标移入灵动岛时：取消待收缩状态并自动展开音乐控制器
const handleMouseEnter = () => {
    if (isPositionAdjusting.value) return;

    // 如果之前移出留下了收缩案底，但动画还没播完鼠标又回来了，直接取消这个案底
    isPendingCollapse = false;

    if (canExpandCard.value) {
        expandMusic();
    }
};

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
    const appWindow = getCurrentWindow();
    calendarRefreshTimer = window.setInterval(() => {
        calendarNow.value = new Date();
    }, 60_000);

    // 监听窗口移动并保存坐标 (带有 300ms 防抖，防止疯狂写入)
    let moveTimeout: number | null = null;
    await appWindow.onMoved(({ payload }) => {
        if (moveTimeout) clearTimeout(moveTimeout);
        // 注意这里加上了 async，因为要等待获取真实尺寸
        moveTimeout = window.setTimeout(async () => {
            try {
                // 核心修复：获取当前窗口的真实宽度，计算出中心点 X 坐标
                const size = await appWindow.innerSize();
                const centerX = payload.x + (size.width / 2);

                // 存入专用的 center_x 键中
                localStorage.setItem('nsd_island_center_x', centerX.toString());
                localStorage.setItem('nsd_island_y', payload.y.toString());
            } catch (e) {
                console.error('保存坐标失败:', e);
            }
        }, 300);
    });

    window.addEventListener('blur', collapseMusic);

    document.addEventListener('contextmenu', (e) => {
        e.preventDefault();
    }, { capture: true }); // 使用捕获阶段，确保先于 Tauri 底层拦截

    // 音乐控制器状态监听器
    await listen<{ enabled: boolean }>('control-music-ctl', (event) => {
        const isEnabled = event.payload.enabled;
        isMusicCtlEnabled.value = isEnabled;
        if (isEnabled) {
            initWebSocket();
            if (localStorage.getItem('nsd_glow_border') === null) {
                isGlowBorderEnabled.value = true;
                localStorage.setItem('nsd_glow_border', 'true');
            }
            isMediaActive.value = true;
            isNewlyEnabled = true;
            showInfo.value = false;
            musicBoxKey.value++;
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
                const { w, h } = getExpandedSize();
                animateIslandSize(w, h);
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
    });

    await listen<DisplayPreferences>('control-display-preferences', (event) => {
        displayPreferences.value = {
            collapsed: { ...event.payload.collapsed },
            expanded: { ...event.payload.expanded },
        };

        Object.entries(event.payload.collapsed).forEach(([feature, enabled]) => {
            localStorage.setItem(`nsd_display_collapsed_${feature}`, String(enabled));
        });
        Object.entries(event.payload.expanded).forEach(([feature, enabled]) => {
            localStorage.setItem(`nsd_display_expanded_${feature}`, String(enabled));
        });
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
        isFullscreenAppActive.value = isFullscreen;

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

    // 在启动调整位置前，根据当前的实际状态，校准初始宽高
    const { w, h } = getBaseSize();
    currentWidth.value = w * appScale.value;
    currentHeight.value = h * appScale.value;

    // 优先读取本地缓存的自定义坐标
    const savedCenterX = localStorage.getItem('nsd_island_center_x');
    const savedY = localStorage.getItem('nsd_island_y');

    // 兼容处理：清理旧版有缺陷的 x 坐标，防止干扰
    if (!savedCenterX && localStorage.getItem('nsd_island_x')) {
        localStorage.removeItem('nsd_island_x');
        await adjustWindowPosition();
    } else if (savedCenterX !== null && savedY !== null) {
        try {
            const scaleFactor = window.devicePixelRatio;
            const targetPhysicalWidth = Math.ceil(currentWidth.value * scaleFactor);
            const targetPhysicalHeight = Math.ceil(currentHeight.value * scaleFactor);

            // 必须先设置高宽，再移动位置，防止形变抖动
            await appWindow.setSize(new PhysicalSize(targetPhysicalWidth, targetPhysicalHeight));

            // 核心还原算法：中心点 X 坐标 - (当前物理宽度 / 2) = 真正的左上角 X 坐标
            const restoreX = Math.round(parseFloat(savedCenterX) - (targetPhysicalWidth / 2));
            const restoreY = parseInt(savedY, 10);

            await appWindow.setPosition(new PhysicalPosition(restoreX, restoreY));
        } catch (error) {
            // 如果发生缩放比例错乱或越界，兜底使用默认居中算法
            await adjustWindowPosition();
        }
    } else {
        await adjustWindowPosition();
    }

    // 检查本地记录的灵动岛开关状态
    const isWidgetEnabled = localStorage.getItem('nsd_widget_visible') !== 'false';

    // 只有在【用户开启了灵动岛】且【没开静默模式】时，启动才自动拉开灵动岛
    if (isWidgetEnabled && !isMsgModeEnabled.value) {
        await invoke('show_window_no_activate', { label: 'widget' });
        isIslandVisible.value = true;
    }

    fetchSpeedStats();
    checkNetworkLatency();

    // 启动网速和硬件显示轮换定时器 (每 5 秒切换一次)
    speedCycleTimer = window.setInterval(() => {
        const isCompactSpeedVisible = displayMonitorSpeed.value ||
            (isMusicExpanded.value && displayMusic.value && showExpandedSpeed.value);
        if (isCompactSpeedVisible || (displaySpeed.value && nsdBaseWidth.value < 240)) {
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

    // 接收来自控制台的 FPS 指令
    await listen<{ enabled: boolean }>('control-fps-monitor', (event) => {
        enableFps.value = event.payload.enabled;
        if (enableFps.value) {
            startDesktopFpsSampler();
            invoke('toggle_fps_plugin', { enable: true }).catch(console.error);
        } else {
            pluginFps.value = 0;
            lastPluginFpsAt.value = 0;
            stopDesktopFpsSampler();
            invoke('toggle_fps_plugin', { enable: false }).catch(console.error);
        }
    });

    // 个性化中心的位置调整：开始时解锁，应用或顶部居中后重新固定。
    await listen<{ action: 'start' | 'apply' | 'center' }>('adjust-island-position', async ({ payload }) => {
        try {
            if (payload.action === 'start') {
                isPositionLocked.value = false;
                isPositionAdjusting.value = true;
                localStorage.setItem('nsd_position_locked', 'false');
                collapseMusic();
                animateIslandSize(300, 48);
                await appWindow.show();
                await appWindow.setFocus();
                return;
            }

            if (payload.action === 'center') {
                localStorage.removeItem('nsd_island_center_x');
                localStorage.removeItem('nsd_island_y');
                await adjustWindowPosition();
            }

            isPositionAdjusting.value = false;
            isPositionLocked.value = true;
            localStorage.setItem('nsd_position_locked', 'true');
            const { w, h } = getBaseSize();
            animateIslandSize(w, h);
            showToast(payload.action === 'center' ? t('positionReset') : t('positionLocked'), 'lock');
        } catch (error) {
            console.error('更新灵动岛位置失败:', error);
        }
    });

    // 监听后端发来的高频 UDP FPS 信号
    await listen<{ fps: number }>('fps-event', (event) => {
        if (Number.isFinite(event.payload.fps) && event.payload.fps > 0) {
            pluginFps.value = Math.round(event.payload.fps);
            lastPluginFpsAt.value = Date.now();
        }
    });

    // 本地已记住开关时，每次程序重启都必须真正重启采集器，
    // 否则只会恢复“已开启”的界面状态，FPS 会一直停在初始值。
    if (enableFps.value) {
        startDesktopFpsSampler();
        invoke('toggle_fps_plugin', { enable: true }).catch(console.error);
    }

    // 在你原有的每秒刷新定时器中，顺带执行音乐同步
    // 1. 高频定时器：专门负责网速和硬件监控（每 500ms ~ 1000ms 刷新一次）
    speedTimer = setInterval(async () => {
        // 刷新网速
        fetchSpeedStats();

        // 设备信息仅在展开且至少启用一张设备卡时读取；800ms 一次并防止请求重叠。
        refreshDeviceStatus();

        // 实时同步给任务栏插件
        syncToTaskbar();
    }, 800) as unknown as number;


    // 2. 中频定时器：专门负责音乐状态同步（每 2000ms 刷新一次即可）
    musicTimer = setInterval(() => {
        invoke<boolean>('is_music_controller_enabled')
            .then((enabled) => {
                isMusicCtlEnabled.value = enabled;
                if (enabled) {
                    syncMusicStatus();
                }
            })
            .catch(console.error);
    }, 2000);

    // 启动后立即读取一次当前媒体会话，避免等待下一轮定时器或错过首次同步事件。
    invoke<boolean>('is_music_controller_enabled')
        .then((enabled) => {
            isMusicCtlEnabled.value = enabled;
            if (enabled) syncMusicStatus();
        })
        .catch(console.error);


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

        if (isPlaying.value) {
            const shouldAdvanceKugouLyrics =
                (localStorage.getItem('nsd_target_player') || 'netease') !== 'kugou' ||
                kugouTimelineReady.value;

            // 1. 播放状态下，本地时钟疯狂往前推算
            if (shouldAdvanceKugouLyrics) {
                localPositionMs.value += delta;
            }

            // 2. 毫秒级歌词匹配与队列逻辑 (解决快节奏吞字、闪烁消失问题)
            if (shouldAdvanceKugouLyrics && parsedLyrics.value.length > 0) {
                let matchedIndex = -1;

                // 找出当前时间进度应该播放哪一句
                for (let i = 0; i < parsedLyrics.value.length; i++) {
                    // 抢跑 550ms：完美抵消 150ms 叠化动画 + 100ms 滤镜模糊 + 听觉视觉生理时差
                    if (parsedLyrics.value[i].time <= localPositionMs.value + 550 - (nsdLyricDelay.value * 1000)) {
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

            // 3. 原有的频谱逻辑保持不变
            if (showSpectrumIndicator.value) {
                try {
                    const data = await invoke<number[]>('get_audio_spectrum');
                    spectrumData.value = data;
                } catch (err) {
                    // 忽略错误，防止刷屏
                }
            }
        } else {
            // 没在播放时，让柱子平滑回落到最低点
            spectrumData.value = [0.35, 0.35, 0.35, 0.35, 0.35];
        }
    }, 50) as unknown as number;

    // 软件启动时，如果媒体控制是开启的，立刻连接 WebSocket
    if (isMusicCtlEnabled.value) {
        initWebSocket();
    }

    // 初始化时触发一次计算
    setTimeout(() => {
        calculateScroll();
    }, 700);
});

onUnmounted(() => {
    stopWebSocket();
    stopDesktopFpsSampler();
    if (calendarRefreshTimer) clearInterval(calendarRefreshTimer);
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
    transform: translateZ(0);
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

.play-btn,
.play-btn:hover,
.play-btn:active {
    background: transparent !important;
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

.music-ctl-box.expanded::before {
    content: '';
    position: absolute;
    top: 14px;
    left: 16px;
    right: 270px;
    height: 118px;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 20px;
    background: linear-gradient(145deg, rgba(255, 255, 255, 0.09), rgba(255, 255, 255, 0.025));
    box-sizing: border-box;
    pointer-events: none;
}

.music-ctl-box.expanded:not(.has-calendar)::before {
    right: 16px;
    height: 94px;
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
    height: 62px;
    margin-top: 22px !important;
    margin-left: 28px !important;
    border: none;
    z-index: 1;
}

/* 封面：覆盖掉上面的 transition: all，只保留变形和圆角的过渡 */
.album-cover {
    transition: transform 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.2), border-radius 0.3s ease !important;
}

.music-ctl-box.expanded .album-cover {
    width: 62px !important;
    height: 62px !important;
    border-radius: 12px !important;
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
    left: 108px !important;
    right: 290px !important;
    display: flex !important;
    align-items: center !important;
    justify-content: flex-start !important;
    transition: none !important;
}

.music-ctl-box.expanded:not(.has-calendar) .music-info-mask-box {
    right: 70px !important;
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
    font-size: 18px;
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
    font-size: 14px;
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
    top: 88px;
    left: calc((100% - 266px) / 2 + 8px);
    transform: translateX(-50%);
    width: calc(100% - 286px);
    display: flex;
    justify-content: center;
    gap: 34px;
    z-index: 1;
}

.music-ctl-box.expanded:not(.has-calendar) .music-controls {
    top: 75px;
    left: 50%;
    width: 100%;
}

.music-ctl-box.expanded .ctl-btn svg {
    width: 22px;
    height: 22px;
}

.music-ctl-box.expanded .play-btn svg {
    width: 28px;
    height: 28px;
}

/* 展开媒体卡片底部的组合监控摘要 */
.expanded-monitor-row {
    position: absolute;
    left: 16px;
    right: 16px;
    bottom: 8px;
    height: 47px;
    display: grid;
    align-items: stretch;
    gap: 8px;
    padding: 0;
    box-sizing: border-box;
    -webkit-app-region: no-drag;
}

.expanded-monitor-row.has-calendar {
    right: 270px;
}

.expanded-calendar-card {
    position: absolute;
    top: 10px;
    right: 12px;
    width: 242px;
    min-height: 230px;
    padding: 12px 16px 10px;
    border: 1px solid rgba(182, 224, 238, 0.15);
    border-radius: 12px;
    background: linear-gradient(145deg, rgba(255, 255, 255, 0.11), rgba(255, 255, 255, 0.045));
    box-sizing: border-box;
    color: #fff;
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', sans-serif;
    font-variant-numeric: tabular-nums;
    text-align: center;
    -webkit-app-region: no-drag;
}

.expanded-calendar-card.without-music {
    top: 10px;
    min-height: 170px;
}

.calendar-month,
.calendar-weekday {
    color: #b6e0ee;
    font-size: 11px;
    font-weight: 800;
    letter-spacing: 0.55px;
    line-height: 1;
}

.calendar-date {
    margin: 3px 0 2px;
    font-size: 40px;
    font-weight: 780;
    letter-spacing: -1px;
    line-height: 0.92;
}

.calendar-weekday {
    color: rgba(255, 255, 255, 0.62);
    font-size: 10px;
    letter-spacing: 0.35px;
}

.calendar-grid {
    display: grid;
    grid-template-columns: repeat(7, minmax(0, 1fr));
    gap: 8px 5px;
    margin-top: 12px;
    font-size: 12px;
    font-weight: 700;
    line-height: 1;
}

.calendar-grid-weekday {
    color: rgba(255, 255, 255, 0.48);
    font-size: 9px;
}

.calendar-grid-day {
    display: grid;
    width: 21px;
    height: 21px;
    place-items: center;
    border-radius: 50%;
}

.calendar-grid-day.is-today {
    background: #79d8ef;
    color: #071217;
    box-shadow: 0 0 7px rgba(121, 216, 239, 0.42);
}

.expanded-resource-summary {
    grid-column: span 2;
    min-width: 0;
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    align-items: stretch;
    gap: 8px;
}

.monitor-chip {
    flex: 1;
    min-width: 0;
    display: grid;
    grid-template-columns: auto auto;
    grid-template-rows: auto 3px;
    align-items: center;
    column-gap: 4px;
    row-gap: 4px;
    padding: 5px 8px;
    border-radius: 9px;
    border: 1px solid rgba(255, 255, 255, 0.09);
    background: linear-gradient(145deg, rgba(255, 255, 255, 0.09), rgba(255, 255, 255, 0.045));
    box-sizing: border-box;
}

.monitor-chip-label,
.monitor-chip-value,
.fps-label,
.fps-value {
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', sans-serif;
    font-variant-numeric: tabular-nums;
    line-height: 1;
}

.monitor-chip-label {
    font-size: 9px;
    font-weight: 800;
    opacity: 0.62;
}

.monitor-chip-value {
    justify-self: end;
    font-size: 12px;
    font-weight: 750;
}

.monitor-chip-track {
    grid-column: 1 / -1;
    height: 3px;
    overflow: hidden;
    border-radius: 999px;
    background: rgba(255, 255, 255, 0.14);
}

.monitor-chip-fill {
    display: block;
    height: 100%;
    border-radius: inherit;
    background: currentColor;
    transition: width 0.35s ease;
}

.fps-pill {
    flex: 0 0 auto;
    min-width: 58px;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 0 9px;
    border-radius: 9px;
    background: rgba(150, 150, 150, 0.16);
    box-sizing: border-box;
}

.fps-pill.compact {
    width: 100%;
    min-width: 0;
    padding: 0 8px;
    border-radius: 9px;
    border: 1px solid rgba(255, 255, 255, 0.09);
    background: linear-gradient(145deg, rgba(255, 255, 255, 0.09), rgba(255, 255, 255, 0.045));
}

.fps-label {
    font-size: 9px;
    font-weight: 800;
    opacity: 0.62;
    letter-spacing: 0.3px;
}

.fps-value {
    font-size: 13px;
    font-weight: 780;
}

.telemetry-speed-card,
.monitor-speed-section {
    min-width: 0;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    gap: 0;
    box-sizing: border-box;
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', sans-serif;
    font-variant-numeric: tabular-nums;
}

.telemetry-speed-card {
    width: 100%;
    padding: 4px 8px;
    border-radius: 9px;
    border: 1px solid rgba(255, 255, 255, 0.09);
    background: linear-gradient(145deg, rgba(255, 255, 255, 0.09), rgba(255, 255, 255, 0.045));
}

.telemetry-speed-line {
    display: grid;
    grid-template-columns: 14px minmax(0, 1fr);
    align-items: center;
    gap: 3px;
    width: 100%;
    max-width: 100%;
    font-size: 9px;
    font-weight: 650;
    line-height: 1;
}

.speed-direction-icon {
    justify-self: center;
    opacity: 0.62;
}

.telemetry-speed-line em {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-style: normal;
}

/* 位置调整模式：在拖动时覆盖普通内容，避免鼠标移入触发灵动岛展开。 */
.position-adjustment-box {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    font-size: 12px;
    font-weight: 600;
    white-space: nowrap;
    user-select: none;
}

.position-adjustment-box svg {
    width: 17px;
    height: 17px;
    flex: 0 0 17px;
    opacity: 0.8;
}

/* 展开态设备信息：保持现有监控卡不动，仅追加第二层状态卡。 */
.device-status-row {
    position: absolute;
    left: 16px;
    right: 16px;
    height: 42px;
    display: flex;
    align-items: stretch;
    gap: 6px;
    z-index: 4;
    -webkit-app-region: no-drag;
}

.device-status-row.with-music {
    bottom: 61px;
    height: 47px;
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 8px;
}

.device-status-row.with-music:not(.has-monitor-row) {
    bottom: 8px;
}

.device-status-row.without-music {
    left: 12px;
    right: 12px;
    bottom: 10px;
    height: 81px;
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 8px;
}

.device-status-row.has-calendar {
    right: 270px;
}

.device-status-row.without-music.has-calendar {
    right: 260px;
}

.device-status-card {
    flex: 1 1 0;
    min-width: 0;
    display: flex;
    align-items: center;
    gap: 7px;
    padding: 5px 8px;
    border: 1px solid rgba(255, 255, 255, 0.09);
    border-radius: 9px;
    background: linear-gradient(145deg, rgba(255, 255, 255, 0.09), rgba(255, 255, 255, 0.045));
    box-sizing: border-box;
    overflow: hidden;
}

.device-status-icon {
    width: 21px;
    height: 21px;
    flex: 0 0 21px;
    opacity: 0.92;
}

.device-status-copy {
    min-width: 0;
    flex: 1;
    display: grid;
    grid-template-rows: auto auto auto;
    align-content: center;
    gap: 2px;
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', sans-serif;
    line-height: 1;
}

.device-status-label {
    overflow: hidden;
    color: currentColor;
    font-size: 7px;
    font-weight: 750;
    opacity: 0.55;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.device-status-copy strong {
    min-width: 0;
    overflow: hidden;
    font-size: 9.5px;
    font-weight: 720;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.device-status-copy small {
    min-width: 0;
    overflow: hidden;
    font-size: 7px;
    font-weight: 580;
    opacity: 0.55;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.device-volume-track {
    width: 100%;
    height: 3px;
    overflow: hidden;
    border-radius: 999px;
    background: rgba(255, 255, 255, 0.14);
}

.device-volume-fill {
    display: block;
    height: 100%;
    border-radius: inherit;
    background: #34c759;
    transition: width 0.25s ease;
}

.device-row-fade-enter-active,
.device-row-fade-leave-active {
    transition: opacity 0.18s ease, transform 0.18s ease;
}

.device-row-fade-enter-from,
.device-row-fade-leave-to {
    opacity: 0;
    transform: translateY(4px);
}

.music-ctl-box.expanded .audio-spectrum.embedded {
    position: absolute;
    right: 32px !important;
    top: 28px !important;
    transform: scale(1.15);
    z-index: 3;
    pointer-events: none;
    /* 把 all 换成具体的属性，防止抖动 */
    transition: opacity 0.3s ease, transform 0.3s ease !important;
}

.music-ctl-box.expanded.has-calendar .audio-spectrum.embedded {
    right: 286px !important;
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
.monitor-dashboard {
    position: absolute;
    inset: 3px 2px;
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 0 3px;
    border-radius: 14px;
    border: none;
    background: transparent;
    box-sizing: border-box;
    -webkit-app-region: no-drag;
    overflow: hidden;
}

.monitor-dashboard .fps-pill:only-child {
    flex: 1;
    margin: 0;
}

.monitor-dashboard .fps-pill {
    flex: 0.9 1 0;
    min-width: 0;
    height: 100%;
    padding: 0 8px;
    border: 1px solid rgba(255, 255, 255, 0.065);
    border-radius: 12px;
    background: linear-gradient(145deg, rgba(255, 255, 255, 0.075), rgba(255, 255, 255, 0.035));
}

.monitor-dashboard .resource-box + .fps-pill {
    border-left: 1px solid rgba(255, 255, 255, 0.065);
}

.monitor-dashboard .monitor-speed-section {
    flex: 1.28 1 0;
    height: 100%;
    padding: 0 9px;
    border: 1px solid rgba(255, 255, 255, 0.065);
    border-radius: 12px;
    background: linear-gradient(145deg, rgba(255, 255, 255, 0.075), rgba(255, 255, 255, 0.035));
}

.monitor-dashboard .monitor-speed-section.has-divider {
    border-left: 1px solid rgba(255, 255, 255, 0.065);
}

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

.monitor-dashboard .resource-box {
    position: relative;
    flex: 2 1 0;
    min-width: 0;
    padding-right: 0;
    gap: 5px;
}

/* 单个资源组 (CPU/RAM) */
.res-group {
    flex: 1 1 0%;
    min-width: 0;
    display: flex;
    flex-direction: column;
    /* 改为纵向两行布局 */
    justify-content: center;
    gap: 5px;
    /* 上下排间距 */
}

.monitor-dashboard .res-group {
    height: 100%;
    padding: 5px 9px;
    border: 1px solid rgba(255, 255, 255, 0.065);
    border-radius: 12px;
    background: linear-gradient(145deg, rgba(255, 255, 255, 0.075), rgba(255, 255, 255, 0.035));
    box-sizing: border-box;
}

.monitor-dashboard .res-group + .res-group {
    border-left: 1px solid rgba(255, 255, 255, 0.065);
}

/* 第一行的文字容器 (标签 + 数值) */
.res-info-row {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    gap: 8px;
    /* 底部对齐，让文字重心更稳 */
    width: 100%;
}

/* 标签 (CPU/RAM) */
.res-label {
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', sans-serif;
    font-size: 9px;
    font-weight: 800;
    opacity: 0.75;
    color: currentColor;
    background: transparent;
    padding: 0;
    line-height: 1;
    white-space: nowrap;
    flex-shrink: 0;
}

/* 进度条轨道 */
.res-bar-track {
    width: 100%;
    /* 占满下面一整行 */
    height: 3px;
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
    white-space: nowrap;
    flex-shrink: 0;
}

.monitor-dashboard.expanded {
    inset: 8px 5px;
    display: grid;
    gap: 6px;
    padding: 0 4px;
    border-radius: 14px;
}

.monitor-dashboard.expanded.has-device-row {
    inset: 10px 12px 99px;
}

.monitor-dashboard.expanded.has-calendar {
    right: 260px;
}

.monitor-dashboard.expanded.has-calendar:not(.has-device-row) {
    inset: 10px 260px 99px 12px;
}

.monitor-dashboard.expanded .resource-box {
    grid-column: span 2;
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 6px;
}

.monitor-dashboard.expanded .fps-pill,
.monitor-dashboard.expanded .monitor-speed-section {
    width: 100%;
}

.res-title,
.fps-title {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    min-width: 0;
}

.status-card-icon {
    flex: 0 0 auto;
    opacity: 0.68;
}

.monitor-dashboard.expanded .res-group,
.monitor-dashboard.expanded .fps-pill,
.monitor-dashboard.expanded .monitor-speed-section {
    height: 100%;
    padding: 7px 8px;
    border-radius: 11px;
    border-color: rgba(255, 255, 255, 0.085);
    background: linear-gradient(145deg, rgba(255, 255, 255, 0.085), rgba(255, 255, 255, 0.035));
}

.monitor-dashboard.expanded .res-info-row {
    gap: 4px;
}

.monitor-dashboard.expanded .res-label,
.monitor-dashboard.expanded .fps-label {
    font-size: 10px;
}

.monitor-dashboard.expanded .res-value,
.monitor-dashboard.expanded .fps-value {
    font-size: 15px;
}

.monitor-dashboard.expanded .telemetry-speed-line {
    grid-template-columns: 14px minmax(0, 1fr);
    gap: 5px;
    font-size: 11px;
}

.monitor-dashboard.expanded .telemetry-speed-line b {
    font-size: 8px;
}

/* 高负载告警态 (>=85%) */
.high-usage {
    color: #b6170f !important;
}

.high-usage-bg {
    background: #b6170f !important;
}

/* 亮色主题适配 (可选，如果全局 currentColor 处理得当可省略) */
:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .res-label {
    background: transparent;
}

:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .res-bar-track {
    background: rgba(0, 0, 0, 0.1);
}

:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .expanded-monitor-row {
    background: transparent;
}

:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .monitor-chip,
:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .fps-pill.compact,
:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .telemetry-speed-card {
    background: linear-gradient(145deg, rgba(0, 0, 0, 0.07), rgba(0, 0, 0, 0.035));
    border-color: rgba(0, 0, 0, 0.08);
}

:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .device-status-card {
    background: linear-gradient(145deg, rgba(0, 0, 0, 0.07), rgba(0, 0, 0, 0.035));
    border-color: rgba(0, 0, 0, 0.08);
}

:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .device-volume-track {
    background: rgba(0, 0, 0, 0.13);
}

:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .monitor-chip-track {
    background: rgba(0, 0, 0, 0.13);
}

:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .monitor-dashboard {
    background: rgba(0, 0, 0, 0.035);
    border-color: rgba(0, 0, 0, 0.08);
}

:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .monitor-dashboard .res-group,
:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .monitor-dashboard .fps-pill,
:deep(.island-container[style*="background-color: rgba(255, 255, 255"]) .monitor-dashboard .monitor-speed-section {
    background: transparent;
    border-color: rgba(0, 0, 0, 0.1);
}

.speed-dual-box {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
}

.speed-single-box {
    display: flex;
    align-items: center;
    width: 100%;
}
</style>
