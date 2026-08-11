<template>
    <div class="panel-container">
        <div v-if="themeMode === 'coverglass'" class="coverglass-bg-container">
            <div class="coverglass-bg-image" :style="coverUrl ? { backgroundImage: `url(${coverUrl})` } : {}"></div>
            <div class="coverglass-blur-layer"></div>
            <div class="coverglass-noise-layer"></div>
        </div>
        <div class="custom-titlebar">
            <div data-tauri-drag-region class="titlebar-drag-area"></div>

            <div class="titlebar-controls">
                <button class="titlebar-btn" @click="minimizeWindow">
                    <svg viewBox="0 0 12 12" fill="currentColor">
                        <rect x="1" y="5" width="10" height="1.5" rx="0.5" />
                    </svg>
                </button>
                <button class="titlebar-btn close-btn" @click="closeWindow">
                    <svg viewBox="0 0 12 12" stroke="currentColor" stroke-width="1.2" stroke-linecap="round">
                        <path d="M2.5 2.5L9.5 9.5M9.5 2.5L2.5 9.5" />
                    </svg>
                </button>
            </div>
        </div>

        <header class="panel-header">
            <div class="brand">
                <img src="../assets/logo.png" class="logo-icon">
                <div>
                    <h1>NetSpeed Dynamic Pro · Wei_XingYU Edition</h1>
                    <p class="subtitle">{{ t('appSubtitle') }} v{{ appVersion }}</p>
                </div>
            </div>

            <div class="header-controls">
                <button class="dynamicset-btn" :class="{ 'is-active': isDynamicSet }" @click="toggleDynamicSet">
                    {{ t('personalizeCenter') }}
                </button>
                <span class="control-separator"></span>

                <span class="status-badge" :class="{ 'is-active': isWidgetVisible }">
                    {{ isWidgetVisible ? t('enabled') : t('disabled') }}
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
                        <h3>{{ t('realtimeStatus') }}</h3>
                        <button class="stats-toggle-btn" @click="toggleRightPanel">
                            {{ rightPanel === 'settings' ? t('trafficStats') : t('exit') }}
                        </button>
                    </div>
                    <div class="speed-monitor">
                        <div class="speed-item">
                            <span class="arrow up">
                                <Upload theme="outline" :size="20" :stroke-width="3" />
                            </span>
                            <div class="speed-info">
                                <span class="label">{{ t('uploadSpeed') }}</span>
                                <span class="value">{{ uploadSpeed }}</span>
                            </div>
                        </div>
                        <div class="speed-item">
                            <span class="arrow down">
                                <Download theme="outline" :size="20" :stroke-width="3" />
                            </span>
                            <div class="speed-info">
                                <span class="label">{{ t('downloadSpeed') }}</span>
                                <span class="value">{{ downloadSpeed }}</span>
                            </div>
                        </div>
                    </div>
                    <div ref="chartRef" class="mini-chart"></div>
                </div>

                <div class="card settings-card" v-if="rightPanel === 'settings'">
                    <h3>{{ t('consoleSettings') }}</h3>
                    <div class="setting-item flex-row-item" :class="{ 'is-dropdown-open': isThemeModeDropdownOpen }">
                        <div class="item-meta">
                            <span class="item-title">{{ t('themeColor') }}</span>
                            <span class="item-desc">{{ t('themeColorDesc') }}</span>
                        </div>

                        <div class="custom-dropdown" tabindex="0" @blur="isThemeModeDropdownOpen = false">
                            <div class="dropdown-trigger" style="width: 110px;"
                                @click="isThemeModeDropdownOpen = !isThemeModeDropdownOpen">
                                <div class="current-item">
                                    <template v-if="themeMode === 'light'">{{ t('lightMode') }}</template>
                                    <template v-else-if="themeMode === 'dark'">{{ t('darkMode') }}</template>
                                    <template v-else-if="themeMode === 'coverglass'">{{ t('coverglassMode')
                                        }}</template>
                                    <template v-else-if="themeMode === 'system'">{{ t('systemMode') }}</template>
                                </div>
                                <svg viewBox="0 0 24 24" class="arrow-icon"
                                    :class="{ 'is-open': isThemeModeDropdownOpen }">
                                    <path d="M7 10l5 5 5-5" fill="none" stroke="currentColor" stroke-width="2"
                                        stroke-linecap="round" />
                                </svg>
                            </div>

                            <transition name="dropdown">
                                <div class="dropdown-menu" v-show="isThemeModeDropdownOpen" style="width: 100%;">
                                    <div class="dropdown-item" :class="{ 'is-active': themeMode === 'light' }"
                                        @click="handleSelectThemeMode('light')">
                                        {{ t('lightMode') }}
                                    </div>
                                    <div class="dropdown-item" :class="{ 'is-active': themeMode === 'dark' }"
                                        @click="handleSelectThemeMode('dark')">
                                        {{ t('darkMode') }}
                                    </div>
                                    <div class="dropdown-item" :class="{ 'is-active': themeMode === 'coverglass' }"
                                        @click="handleSelectThemeMode('coverglass')">
                                        {{ t('coverglassMode') }}
                                    </div>
                                    <div class="dropdown-item" :class="{ 'is-active': themeMode === 'system' }"
                                        @click="handleSelectThemeMode('system')">
                                        {{ t('systemMode') }}
                                    </div>
                                </div>
                            </transition>
                        </div>
                    </div>
                    <div class="setting-item">
                        <div class="item-meta">
                            <span class="item-title">{{ t('autoStart') }}</span>
                            <span class="item-desc">{{ t('autoStartDesc') }}</span>
                        </div>
                        <label class="switch">
                            <input type="checkbox" v-model="autoStart" @change="toggleAutoStart">
                            <span class="slider"></span>
                        </label>
                    </div>
                    <div class="setting-item slider-item">
                        <div class="item-meta" style="width: 100%;">
                            <span class="item-title">{{ t('islandOpacity') }}</span>
                            <span class="item-desc">{{ t('islandOpacityDesc') }} ({{ opacity }}%)</span>
                        </div>
                        <input type="range" min="0" max="100" v-model="opacity" class="range-input" />
                    </div>
                </div>

                <template v-else>
                    <div class="card stats-card">
                        <div class="card-header-row">
                            <h3>{{ t('trafficStats') }}</h3>

                            <div class="custom-dropdown" tabindex="0" @blur="isStatChartDropdownOpen = false">
                                <div class="dropdown-trigger" style="width: 90px;"
                                    @click="isStatChartDropdownOpen = !isStatChartDropdownOpen">
                                    <div class="current-item">
                                        <template v-if="statChartType === 'bar'">{{ t('barChart') }}</template>
                                        <template v-else-if="statChartType === 'line'">{{ t('lineChart') }}</template>
                                    </div>
                                    <svg viewBox="0 0 24 24" class="arrow-icon"
                                        :class="{ 'is-open': isStatChartDropdownOpen }">
                                        <path d="M7 10l5 5 5-5" fill="none" stroke="currentColor" stroke-width="2"
                                            stroke-linecap="round" />
                                    </svg>
                                </div>

                                <transition name="dropdown">
                                    <div class="dropdown-menu" v-show="isStatChartDropdownOpen" style="width: 100%;">
                                        <div class="dropdown-item" :class="{ 'is-active': statChartType === 'bar' }"
                                            @click="handleSelectStatChart('bar')">
                                            {{ t('barChart') }}
                                        </div>
                                        <div class="dropdown-item" :class="{ 'is-active': statChartType === 'line' }"
                                            @click="handleSelectStatChart('line')">
                                            {{ t('lineChart') }}
                                        </div>
                                    </div>
                                </transition>
                            </div>
                        </div>
                        <div class="stats-overview">
                            <div class="stat-box">
                                <span class="stat-label">{{ t('totalUpload') }}</span>
                                <span class="stat-val">{{ formatBytesValue(totalUpload) }} {{
                                    formatBytesUnit(totalUpload) }}</span>
                                <span class="stat-unit"></span>
                            </div>
                            <div class="stat-box">
                                <span class="stat-label">{{ t('totalDownload') }}</span>
                                <span class="stat-val">{{ formatBytesValue(totalDownload) }} {{
                                    formatBytesUnit(totalDownload) }}</span>
                            </div>
                            <div class="stat-box">
                                <span class="stat-label">{{ t('monthTraffic') }}</span>
                                <span class="stat-val">{{ formatBytesValue(monthTraffic) }} {{
                                    formatBytesUnit(monthTraffic) }}</span>
                            </div>
                        </div>
                        <div ref="statsChartRef" class="stats-chart-container"></div>
                    </div>
                </template>

                <div class="dynamicset-grid bottom-grid-card card-pager-outer">
                    <div class="pager-viewport">
                        <div class="pager-track"
                            :style="{ transform: currentPage === 0 ? 'translateX(0)' : 'translateX(-50%)' }">

                            <div class="pager-page">
                                <div class="set-item" :class="{ 'is-dropdown-open': isLanguageDropdownOpen }">
                                    <div class="set-item-meta">
                                        <span class="set-item-title">{{ t('language') }}</span>
                                        <span class="set-item-desc">{{ t('languageDesc') }}</span>
                                    </div>
                                    <div class="custom-dropdown" tabindex="0" @blur="isLanguageDropdownOpen = false">
                                        <div class="dropdown-trigger"
                                            @click="isLanguageDropdownOpen = !isLanguageDropdownOpen">
                                            <div class="current-item">{{ t(currentLanguage === 'zh-CN' ?
                                                'simplifiedChinese' :
                                                'english') }}</div>
                                            <svg viewBox="0 0 24 24" class="arrow-icon"
                                                :class="{ 'is-open': isLanguageDropdownOpen }">
                                                <path d="M7 10l5 5 5-5" fill="none" stroke="currentColor"
                                                    stroke-width="2" stroke-linecap="round" />
                                            </svg>
                                        </div>

                                        <transition name="dropdown">
                                            <div class="dropdown-menu" v-show="isLanguageDropdownOpen">
                                                <div v-for="option in languageOptions" :key="option.value"
                                                    class="dropdown-item"
                                                    :class="{ 'is-active': currentLanguage === option.value }"
                                                    @click="handleSelectLanguage(option.value)">
                                                    {{ t(option.labelKey) }}
                                                </div>
                                            </div>
                                        </transition>
                                    </div>
                                </div>
                                <div class="set-item" :class="{ 'is-dropdown-open': isPlayerDropdownOpen }">
                                    <div class="set-item-meta">
                                        <span class="set-item-title">{{ t('targetMediaPlatform') }}</span>
                                        <span class="set-item-desc">{{ t('targetMediaPlatformDesc') }}</span>
                                    </div>
                                    <div class="custom-dropdown" tabindex="0" @blur="isPlayerDropdownOpen = false">
                                        <div class="dropdown-trigger"
                                            @click="isPlayerDropdownOpen = !isPlayerDropdownOpen">
                                            <div class="current-item">
                                                <template v-if="targetPlayer === 'netease'"><img
                                                        src="../assets/musci163.svg" class="platform-icon"> {{
                                                            t('netease') }}</template>
                                                <template v-else-if="targetPlayer === 'spotify'"><img
                                                        src="../assets/Spotify.svg" class="platform-icon">
                                                    Spotify</template>
                                                <template v-else-if="targetPlayer === 'apple'"><img
                                                        src="../assets/applemusic.svg" class="platform-icon">
                                                    Apple</template>
                                                <template v-else-if="targetPlayer === 'qqmusic'"><img
                                                        src="../assets/qqmusic.svg" class="platform-icon"> {{
                                                            t('qqMusic') }}</template>
                                                <template v-else-if="targetPlayer === 'kugou'"><img
                                                        src="../assets/kugou.svg" class="platform-icon"> {{
                                                            t('kugouMusic') }}</template>
                                                <template v-else-if="targetPlayer === 'echo'"><img
                                                        src="../assets/echomusic.ico" class="platform-icon">
                                                    EchoMusic</template>
                                                <template v-else-if="targetPlayer === 'lx-music'"><img
                                                        src="../assets/lxmusic.png" class="platform-icon"> {{
                                                            t('lxMusic') }}</template>
                                                <template v-else-if="targetPlayer === 'other'">
                                                    <svg viewBox="0 0 24 24" class="platform-icon" fill="currentColor">
                                                        <path
                                                            d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 14.5v-9l6 4.5-6 4.5z" />
                                                    </svg>
                                                    {{ t('genericMedia') }}
                                                </template>
                                            </div>
                                            <svg viewBox="0 0 24 24" class="arrow-icon"
                                                :class="{ 'is-open': isPlayerDropdownOpen }">
                                                <path d="M7 10l5 5 5-5" fill="none" stroke="currentColor"
                                                    stroke-width="2" stroke-linecap="round" />
                                            </svg>
                                        </div>

                                        <transition name="dropdown">
                                            <div class="dropdown-menu" v-show="isPlayerDropdownOpen">
                                                <div class="dropdown-item"
                                                    :class="{ 'is-active': targetPlayer === 'netease' }"
                                                    @click="handleSelectPlayer('netease')">
                                                    <img src="../assets/musci163.svg" class="platform-icon"> {{
                                                        t('netease') }}
                                                </div>
                                                <div class="dropdown-item"
                                                    :class="{ 'is-active': targetPlayer === 'spotify' }"
                                                    @click="handleSelectPlayer('spotify')">
                                                    <img src="../assets/Spotify.svg" class="platform-icon"> Spotify
                                                </div>
                                                <div class="dropdown-item"
                                                    :class="{ 'is-active': targetPlayer === 'apple' }"
                                                    @click="handleSelectPlayer('apple')">
                                                    <img src="../assets/applemusic.svg" class="platform-icon"> Apple
                                                </div>
                                                <div class="dropdown-item"
                                                    :class="{ 'is-active': targetPlayer === 'qqmusic' }"
                                                    @click="handleSelectPlayer('qqmusic')">
                                                    <img src="../assets/qqmusic.svg" class="platform-icon"> {{
                                                        t('qqMusic') }}
                                                </div>
                                                <div class="dropdown-item"
                                                    :class="{ 'is-active': targetPlayer === 'kugou' }"
                                                    @click="handleSelectPlayer('kugou')">
                                                    <img src="../assets/kugou.svg" class="platform-icon"> {{
                                                        t('kugouMusic') }}
                                                </div>
                                                <div class="dropdown-item"
                                                    :class="{ 'is-active': targetPlayer === 'echo' }"
                                                    @click="handleSelectPlayer('echo')">
                                                    <img src="../assets/echomusic.ico" class="platform-icon"> EchoMusic
                                                </div>
                                                <div class="dropdown-item"
                                                    :class="{ 'is-active': targetPlayer === 'lx-music' }"
                                                    @click="handleSelectPlayer('lx-music')">
                                                    <img src="../assets/lxmusic.png" class="platform-icon"> {{
                                                        t('lxMusic') }}
                                                </div>
                                                <div class="dropdown-item"
                                                    :class="{ 'is-active': targetPlayer === 'other' }"
                                                    @click="handleSelectPlayer('other')">
                                                    <svg viewBox="0 0 24 24" class="platform-icon" fill="currentColor">
                                                        <path
                                                            d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 14.5v-9l6 4.5-6 4.5z" />
                                                    </svg>
                                                    {{ t('otherMediaControl') }}
                                                </div>
                                            </div>
                                        </transition>
                                    </div>
                                </div>
                                <div class="set-item">
                                    <div class="set-item-meta">
                                        <span class="set-item-title">{{ t('mediaController') }}</span>
                                        <span class="set-item-desc">{{ t('mediaControllerDesc') }}</span>
                                    </div>
                                    <label class="switch">
                                        <input type="checkbox" v-model="enableMusicCtrl">
                                        <span class="slider"></span>
                                    </label>
                                </div>
                                <div class="set-item">
                                    <div class="set-item-meta">
                                        <span class="set-item-title">{{ t('messageNotifications') }}</span>
                                        <span class="set-item-desc">{{ t('messageNotificationsDesc') }}</span>
                                    </div>
                                    <label class="switch">
                                        <input type="checkbox" v-model="enableMsgNotify" @change="toggleMsgNotify">
                                        <span class="slider"></span>
                                    </label>
                                </div>
                                <div class="set-item">
                                    <div class="set-item-meta">
                                        <span class="set-item-title">{{ t('quietMode') }}</span>
                                        <span class="set-item-desc">{{ t('quietModeDesc') }}</span>
                                    </div>
                                    <label class="switch">
                                        <input type="checkbox" v-model="msgModeEnabled" @change="toggleMsgMode">
                                        <span class="slider"></span>
                                    </label>
                                </div>
                                <div class="set-item">
                                    <div class="set-item-meta">
                                        <span class="set-item-title">{{ t('fullscreenAutoHide') }}</span>
                                        <span class="set-item-desc">{{ t('fullscreenAutoHideDesc') }}</span>
                                    </div>
                                    <label class="switch">
                                        <input type="checkbox" v-model="autoHideFullscreen" @change="toggleAutoHide">
                                        <span class="slider"></span>
                                    </label>
                                </div>
                            </div>

                            <div class="pager-page">
                                <!--第二页设置项-->
                                <div class="set-item">
                                    <div class="set-item-meta">
                                        <span class="set-item-title">{{ t('systemResourceMonitor') }}</span>
                                        <span class="set-item-desc">{{ t('systemResourceMonitorDesc') }}</span>
                                    </div>
                                    <label class="switch">
                                        <input type="checkbox" v-model="enableSysResource" @change="toggleSysResource">
                                        <span class="slider"></span>
                                    </label>
                                </div>
                                <div class="set-item">
                                    <div class="set-item-meta">
                                        <span class="set-item-title">{{ t('fpsMonitor') }}</span>
                                        <span class="set-item-desc">{{ t('fpsMonitorDesc') }}</span>
                                    </div>
                                    <label class="switch">
                                        <input type="checkbox" v-model="enableFps" @change="toggleFps">
                                        <span class="slider"></span>
                                    </label>
                                </div>
                                <div class="set-item display-pref-item"
                                    :class="{ 'is-dropdown-open': isDisplayPreferencesOpen }">
                                    <div class="set-item-meta">
                                        <span class="set-item-title">{{ t('displayContent') }}</span>
                                        <span class="set-item-desc">{{ t('displayContentDesc') }}</span>
                                    </div>
                                    <div class="custom-dropdown" tabindex="0"
                                        @focusout="handleDisplayPreferencesFocusOut">
                                        <div class="dropdown-trigger display-pref-trigger"
                                            @click="isDisplayPreferencesOpen = !isDisplayPreferencesOpen">
                                            <div class="current-item">{{ t('configure') }}</div>
                                            <svg viewBox="0 0 24 24" class="arrow-icon"
                                                :class="{ 'is-open': isDisplayPreferencesOpen }">
                                                <path d="M7 10l5 5 5-5" fill="none" stroke="currentColor"
                                                    stroke-width="2" stroke-linecap="round" />
                                            </svg>
                                        </div>
                                        <transition name="dropdown">
                                            <div v-if="isDisplayPreferencesOpen" class="dropdown-menu display-pref-menu">
                                                <div class="display-pref-group">
                                                    <span class="display-pref-heading">{{ t('collapsedState') }}</span>
                                                    <div class="display-pref-options">
                                                        <label class="display-pref-option">
                                                            <input type="checkbox" v-model="displayPreferences.collapsed.music"
                                                                @change="updateDisplayPreferences">
                                                            <span>{{ t('mediaShort') }}</span>
                                                        </label>
                                                        <label class="display-pref-option">
                                                            <input type="checkbox" v-model="displayPreferences.collapsed.resource"
                                                                @change="updateDisplayPreferences">
                                                            <span>{{ t('resourceShort') }}</span>
                                                        </label>
                                                        <label class="display-pref-option">
                                                            <input type="checkbox" v-model="displayPreferences.collapsed.fps"
                                                                @change="updateDisplayPreferences">
                                                            <span>{{ t('fpsShort') }}</span>
                                                        </label>
                                                        <label class="display-pref-option">
                                                            <input type="checkbox" v-model="displayPreferences.collapsed.speed"
                                                                @change="updateDisplayPreferences">
                                                            <span>{{ t('speedShort') }}</span>
                                                        </label>
                                                    </div>
                                                </div>
                                                <div class="display-pref-divider"></div>
                                                <div class="display-pref-group">
                                                    <span class="display-pref-heading">{{ t('expandedState') }}</span>
                                                    <div class="display-pref-options">
                                                        <label class="display-pref-option">
                                                            <input type="checkbox" v-model="displayPreferences.expanded.music"
                                                                @change="updateDisplayPreferences">
                                                            <span>{{ t('mediaShort') }}</span>
                                                        </label>
                                                        <label class="display-pref-option">
                                                            <input type="checkbox" v-model="displayPreferences.expanded.resource"
                                                                @change="updateDisplayPreferences">
                                                            <span>{{ t('resourceShort') }}</span>
                                                        </label>
                                                        <label class="display-pref-option">
                                                            <input type="checkbox" v-model="displayPreferences.expanded.fps"
                                                                @change="updateDisplayPreferences">
                                                            <span>{{ t('fpsShort') }}</span>
                                                        </label>
                                                        <label class="display-pref-option">
                                                            <input type="checkbox" v-model="displayPreferences.expanded.speed"
                                                                @change="updateDisplayPreferences">
                                                            <span>{{ t('speedShort') }}</span>
                                                        </label>
                                                        <label class="display-pref-option">
                                                            <input type="checkbox" v-model="displayPreferences.expanded.network"
                                                                @change="updateDisplayPreferences">
                                                            <span>{{ t('networkConnectionShort') }}</span>
                                                        </label>
                                                        <label class="display-pref-option">
                                                            <input type="checkbox" v-model="displayPreferences.expanded.audioOutput"
                                                                @change="updateDisplayPreferences">
                                                            <span>{{ t('audioOutputShort') }}</span>
                                                        </label>
                                                        <label class="display-pref-option">
                                                            <input type="checkbox" v-model="displayPreferences.expanded.bluetooth"
                                                                @change="updateDisplayPreferences">
                                                            <span>{{ t('bluetoothShort') }}</span>
                                                        </label>
                                                        <label class="display-pref-option">
                                                            <input type="checkbox" v-model="displayPreferences.expanded.outputVolume"
                                                                @change="updateDisplayPreferences">
                                                            <span>{{ t('outputVolumeShort') }}</span>
                                                        </label>
                                                        <label class="display-pref-option">
                                                            <input type="checkbox" v-model="displayPreferences.expanded.calendar"
                                                                @change="updateDisplayPreferences">
                                                            <span>{{ t('calendarShort') }}</span>
                                                        </label>
                                                    </div>
                                                </div>
                                            </div>
                                        </transition>
                                    </div>
                                </div>
                            </div>

                        </div>
                    </div>

                    <div class="pagination-capsule" @click="togglePage">
                        <div class="page-dot" :class="{ active: currentPage === 0 }"></div>
                        <div class="page-dot" :class="{ active: currentPage === 1 }"></div>
                    </div>
                </div>
            </template>

            <template v-else>
                <DynamicSet @show-plugin-dialog="handlePluginDialog" />
            </template>
        </div>

        <footer class="panel-footer">
            <div class="ft_left">
                <span>&copy; 2026 <button class="openmywebsite" @click="openMywebsite">Wei_XingYU</button> Edition</span>
                <span>NSD Wei_XingYU v{{ appVersion }}</span>
            </div>
            <div class="ft_right">
                <span class="action-link"
                    :style="{ opacity: isChecking ? 0.5 : 1, pointerEvents: isChecking ? 'none' : 'auto', position: 'relative' }"
                    @click="checkUpdate">
                    <span v-if="hasNewVersion" class="update-dot"></span>
                    {{ isChecking ? t('checking') : (hasNewVersion ? t('newVersionDetected') : t('checkUpdate')) }}
                </span>
            </div>
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
                        <button v-if="dialog.isConfirm" class="btn btn-secondary" @click="closeDialog">{{ t('cancel')
                            }}</button>
                        <button class="btn btn-primary" @click="handleDialogConfirm">{{ t('confirm') }}</button>
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
import Upload from '@icon-park/vue-next/es/icons/Upload';
import Download from '@icon-park/vue-next/es/icons/Download';
import DynamicSet from '../components/DynamicSet.vue';
import { t, currentLanguage, setLanguage, languageOptions, type AppLanguage } from '../i18n';

// 默认读取本地保存的状态（如果没有保存过，默认是开启 true）
const isWidgetVisible = ref(localStorage.getItem('nsd_widget_visible') !== 'false');
const autoStart = ref(false);
const opacity = ref(Number(localStorage.getItem('nsd_island_opacity') || '100'));

const savedTheme = localStorage.getItem('nsd_theme_mode') || 'light';
const themeMode = ref(['light', 'dark', 'coverglass', 'system'].includes(savedTheme) ? savedTheme : 'light');

const coverUrl = ref('');
const coverCache = new Map<string, string>();
const currentTrackInfo = ref('');
let coverTimer: number | null = null;

const uploadSpeed = ref('0 B/s');
const downloadSpeed = ref('0 B/s');

const appVersion = ref('2.5.0');

const isDynamicSet = ref(false);

const currentPage = ref(0);
const togglePage = () => {
    currentPage.value = currentPage.value === 0 ? 1 : 0;
};

const isChecking = ref(false);
const hasNewVersion = ref(false);

const enableFps = ref(localStorage.getItem('nsd_fps_monitor') === 'true');

// 添加切换方法
const toggleFps = async () => {
    localStorage.setItem('nsd_fps_monitor', String(enableFps.value));
    await emit('control-fps-monitor', { enabled: enableFps.value });
};

// 音乐控制平台切换功能
const targetPlayer = ref(localStorage.getItem('nsd_target_player') || 'netease');

const setTargetPlayer = async (player: string) => {
    targetPlayer.value = player;
    localStorage.setItem('nsd_target_player', player); // 本地记忆化
    try {
        await invoke('set_target_player', { player }); // 秒发给 Rust 立即生效
    } catch (e) {
        console.error('切换平台失败', e);
    }
};

// 利用 Canvas 一次性生成静态模糊图，彻底拯救 GPU
const bakeBlurImage = (url: string): Promise<string> => {
    return new Promise((resolve) => {
        const img = new Image();
        // 防止跨域导致 Canvas 污染
        if (url.startsWith('http')) {
            img.crossOrigin = 'anonymous';
        }

        img.onload = () => {
            const canvas = document.createElement('canvas');
            // 降低物理分辨率，进一步榨干性能
            canvas.width = 120;
            canvas.height = 120;
            const ctx = canvas.getContext('2d');
            if (!ctx) return resolve(url);

            // 在 JS 层面算一次模糊
            ctx.filter = 'blur(10px)';
            // 往外围多画一点，裁掉模糊产生的白边
            ctx.drawImage(img, -10, -10, 140, 140);

            try {
                // 导出为画质 0.6 的 jpeg，直接变成一张静态死图
                resolve(canvas.toDataURL('image/jpeg', 0.6));
            } catch (e) {
                // 如果遇到 Tauri asset 协议严格跨域，退回原图
                resolve(url);
            }
        };
        img.onerror = () => resolve(url);
        img.src = url;
    });
};

const syncMusicCover = async () => {
    // 只有在开启沉浸模式时才请求，节省性能
    if (themeMode.value !== 'coverglass') return;
    try {
        const res = await invoke<[string, string, boolean] | null>('fetch_netease_music_info');
        if (res) {
            const [song, artist] = res;
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

                        // 等待 Canvas 烘焙完成，拿到一张纯静态的模糊 Base64 图片
                        const bakedImage = await bakeBlurImage(realCoverUrl);

                        coverUrl.value = bakedImage;

                        if (coverCache.size > 50) coverCache.clear();
                        // 缓存直接存烤好的静态图，下次切回来连烤都不用烤了
                        coverCache.set(newTrackInfo, bakedImage);
                    } catch (coverErr) {
                        coverUrl.value = '';
                    }
                }
            }
        } else {
            // 没检测到播放时清空封面
            currentTrackInfo.value = '';
            coverUrl.value = '';
        }
    } catch (err) {
        console.error('沉浸模式封面同步失败:', err);
    }
};

// 监听主题变化，动态启停轮询定时器
watch(themeMode, (newMode) => {
    if (newMode === 'coverglass') {
        syncMusicCover(); // 立即获取一次
        if (!coverTimer) {
            // 每 2 秒刷新一次，和灵动岛的频率保持一致
            coverTimer = window.setInterval(syncMusicCover, 2000);
        }
    } else {
        if (coverTimer) {
            clearInterval(coverTimer);
            coverTimer = null;
        }
    }
}, { immediate: true });

// 媒体平台下拉菜单的状态与方法
const isPlayerDropdownOpen = ref(false);
const handleSelectPlayer = (player: string) => {
    setTargetPlayer(player);
    isPlayerDropdownOpen.value = false;
};

// 数据统计图表类型控制状态与方法
const isStatChartDropdownOpen = ref(false);
const handleSelectStatChart = (type: 'bar' | 'line') => {
    statChartType.value = type;
    isStatChartDropdownOpen.value = false; // 自动收起下拉框
    updateStatsChart(); // 触发图表刷新更新图表类型
};

// 控制台主题选择下拉菜单状态与方法
const isThemeModeDropdownOpen = ref(false);
const handleSelectThemeMode = (mode: string) => {
    themeMode.value = mode;                 // 更新响应式变量
    isThemeModeDropdownOpen.value = false;  // 自动收起下拉框
    handleThemeChange();                    // 复用原有的处理逻辑（保存本地并应用主题）
};

const isLanguageDropdownOpen = ref(false);
const handleSelectLanguage = async (language: AppLanguage) => {
    setLanguage(language);
    isLanguageDropdownOpen.value = false;
    updateStatsChart();
    await emit('control-language', { language });
};

// 灵动岛设置相关的 UI 状态绑定
const enableMusicCtrl = ref(localStorage.getItem('nsd_music_ctrl') === 'true');
const enableMsgNotify = ref(localStorage.getItem('nsd_msg_notify') === 'true');
const msgModeEnabled = ref(localStorage.getItem('nsd_msg_mode') === 'true');
const autoHideFullscreen = ref(localStorage.getItem('nsd_autohide_fs') === 'true');
const enableSysResource = ref(localStorage.getItem('nsd_sys_resource') === 'true');
const isDisplayPreferencesOpen = ref(false);

const handleDisplayPreferencesFocusOut = (event: FocusEvent) => {
    const container = event.currentTarget as HTMLElement;
    const nextTarget = event.relatedTarget as Node | null;
    if (!nextTarget || !container.contains(nextTarget)) {
        isDisplayPreferencesOpen.value = false;
    }
};

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

const updateDisplayPreferences = async () => {
    const preferences: DisplayPreferences = {
        collapsed: { ...displayPreferences.value.collapsed },
        expanded: { ...displayPreferences.value.expanded },
    };

    Object.entries(preferences.collapsed).forEach(([feature, enabled]) => {
        localStorage.setItem(`nsd_display_collapsed_${feature}`, String(enabled));
    });
    Object.entries(preferences.expanded).forEach(([feature, enabled]) => {
        localStorage.setItem(`nsd_display_expanded_${feature}`, String(enabled));
    });

    await emit('control-display-preferences', preferences);
};

// 切换系统资源监控
const toggleSysResource = async () => {
    localStorage.setItem('nsd_sys_resource', String(enableSysResource.value));
    await emit('control-sys-resource', { enabled: enableSysResource.value });
};

// 切换消息模式
const toggleMsgMode = async () => {
    // 如果开启静默模式，则强制开启消息通知并同步本地存储
    if (msgModeEnabled.value) { enableMsgNotify.value = true; toggleMsgNotify(); }

    localStorage.setItem('nsd_msg_mode', String(msgModeEnabled.value));
    await emit('control-msg-mode', { enabled: msgModeEnabled.value });
};

// 新增切换保存方法
const toggleMsgNotify = () => {
    localStorage.setItem('nsd_msg_notify', String(enableMsgNotify.value));
};

// 切换灵动岛设置
const toggleDynamicSet = () => {
    isDynamicSet.value = !isDynamicSet.value;
};

// 切换自动隐藏
const toggleAutoHide = async () => {
    localStorage.setItem('nsd_autohide_fs', String(autoHideFullscreen.value));
    await emit('control-autohide-fs', { enabled: autoHideFullscreen.value });
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
        legend: { data: [t('trafficLegendUpload'), t('trafficLegendDownload')], textStyle: { color: textColor }, top: 0 },
        grid: { top: 30, left: '2%', right: '2%', bottom: '0%', containLabel: true },
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
                name: t('trafficLegendUpload'),
                type: statChartType.value,
                smooth: true,
                data: upData,
                itemStyle: { color: getChartColors().line },
                barMaxWidth: 15
            },
            {
                name: t('trafficLegendDownload'),
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
        showDialog(t('settingFailedTitle'), t('autoStartFailed'));
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

// 处理插件缺失的弹窗逻辑
const handlePluginDialog = () => {
    showDialog(
        '组件缺失提示',
        '未检测到任务栏组件。请重新安装本项目 Release 中的完整安装包后再试。是否打开下载页？',
        true, // true 表示这是一个需要“确定/取消”的双按钮弹窗
        () => {
            // 用户点击确定后，自动跳转最新 Release 页面
            openUrl('https://github.com/wei-xingyu/NetSpeed-Dynamic/releases/latest');
        }
    );
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
    openUrl('https://github.com/wei-xingyu/NetSpeed-Dynamic');
}

// 新增：静默检查更新（后台偷偷查，不弹窗，报错了也不干扰用户）
const silentCheckUpdate = async () => {
    try {
        const localVersionStr = await getVersion();
        const response = await fetch('https://api.github.com/repos/wei-xingyu/NetSpeed-Dynamic/releases/latest', {
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

        const response = await fetch('https://api.github.com/repos/wei-xingyu/NetSpeed-Dynamic/releases/latest', {
            method: 'GET',
            headers: {
                'Accept': 'application/vnd.github.v3+json',
                'User-Agent': 'Tauri-App-NetSpeed-Dynamic'
            },
            signal: controller.signal
        });

        clearTimeout(timeoutId);

        if (response.status === 404) {
            showDialog(t('checkUpdateTitle'), t('updateNotFound'));
            return;
        }

        if (!response.ok) {
            showDialog(t('checkUpdateTitle'), t('updateCheckFailed'));
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
                t('newVersionTitle'),
                t('newVersionMessage', { remote: remoteVersionStr, local: localVersionStr }),
                true,
                () => {
                    openUrl(data.html_url);
                    hasNewVersion.value = false; // 用户点击去更新后，消掉红点并恢复文字
                }
            );
        } else {
            hasNewVersion.value = false;
            showDialog(t('tipTitle'), t('latestVersion'));
        }
    } catch (error: any) {
        console.error('检查更新时出错:', error);
        // 👇 精准识别是不是超时导致的
        if (error.name === 'AbortError') {
            showDialog(t('networkTimeoutTitle'), t('networkTimeoutMessage'));
        } else {
            showDialog(t('networkErrorTitle'), t('networkErrorMessage'));
        }
    } finally {
        isChecking.value = false; // 👈 无论成功失败，最后都恢复状态
    }
};

const applyTheme = () => {
    const root = document.documentElement;
    if (themeMode.value === 'dark' || themeMode.value === 'coverglass') {
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

watch(currentLanguage, () => {
    updateStatsChart();
});

watch(opacity, async (newVal) => {
    localStorage.setItem('nsd_island_opacity', newVal.toString());
    await emit('control-island-opacity', { opacity: newVal });
});

// 添加监听器，将状态同步给灵动岛
watch(enableMusicCtrl, async (newVal) => {
    localStorage.setItem('nsd_music_ctrl', newVal.toString());
    await invoke('set_music_controller_enabled', { enabled: newVal });
    await emit('control-music-ctl', { enabled: newVal });
    console.log('音乐控制器状态切换为:', newVal);
});

onMounted(async () => {
    // 告诉 Rust 上次绑定的目标是谁
    await invoke('set_target_player', { player: targetPlayer.value }).catch(() => { });
    await invoke('set_music_controller_enabled', { enabled: enableMusicCtrl.value }).catch(() => { });
    // 预览或应用重启后主动同步开关状态，避免灵动岛遗漏首次事件。
    await emit('control-music-ctl', { enabled: enableMusicCtrl.value });

    // 启动时检测并恢复任务栏组件的状态，实现自动启动
    if (localStorage.getItem('nsd_taskbar_plugin') === 'true') {
        invoke('toggle_taskbar_plugin', { enable: true }).catch(() => {
            // 如果启动失败（比如文件丢了），自动把开关状态重置为关闭
            localStorage.setItem('nsd_taskbar_plugin', 'false');
        });
    }

    silentCheckUpdate();

    window.addEventListener('contextmenu', (e) => {
        e.preventDefault();
    }, { capture: true });

    applyTheme();
    systemThemeMedia = window.matchMedia('(prefers-color-scheme: dark)');
    systemThemeMedia.addEventListener('change', handleSystemThemeUpdate);

    initChart();
    fetchSpeedStats();
    speedTimer = setInterval(fetchSpeedStats, 1000) as unknown as number;
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

    // 监听来自灵动岛右键菜单的“打开控制台”信号
    await listen('open-settings-panel', async () => {
        const appWindow = getCurrentWindow();
        await appWindow.show();        // 确保窗口显示
        await appWindow.unminimize();  // 如果最小化了，就恢复
        await appWindow.setFocus();    // 强制抢占焦点弹到最前面
    });

    await listen<{ visible: boolean }>('island-status-sync', (event) => {
        isWidgetVisible.value = event.payload.visible;
    });

    const savedState = localStorage.getItem('nsd_widget_visible') !== 'false';
    isWidgetVisible.value = savedState;
    if (savedState) {
        await emit('control-island-visibility', { show: true });
    }
});

onUnmounted(() => {
    clearInterval(speedTimer);
    chartInstance?.dispose();
    statsChartInstance?.dispose();
    systemThemeMedia?.removeEventListener('change', handleSystemThemeUpdate);
    localStorage.setItem('nsd_traffic_stats', JSON.stringify(trafficData.value));
    if (coverTimer) clearInterval(coverTimer);
});

const toggleWidget = async () => {
    const nextState = !isWidgetVisible.value;
    // 保存开关状态到本地存储
    localStorage.setItem('nsd_widget_visible', String(nextState));
    await emit('control-island-visibility', { show: nextState });
    isWidgetVisible.value = nextState;
};

// 控制窗口功能
const minimizeWindow = async () => {
    await getCurrentWindow().minimize();
};
const closeWindow = async () => {
    await getCurrentWindow().hide();
};
</script>

<style scoped>
/*提取出的颜色变量层*/
:global(:root) {
    --bg-body: #f8fafc;
    --text-body: #1e293b;
    --h1-color: #0f172a;
    --subtitle-color: #798089;
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
    --item-title-active-color: #f8fafc8c;
    --tag-dev-bg: #f1f5f9;
    --tag-dev-color: #64748b;
    --item-desc-color: #898f99df;
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
    --btn-sec-list-bg: #ebebeb;
    --btn-sec-color: #64748b;
    --btn-sec-border: #e2e8f0;
    --btn-sec-hover-bg: #e2e8f0;
    --btn-sec-hover-color: #ffffff;
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
    --bg-body: #1e1f1f;
    --text-body: #cbd5e1;
    --h1-color: #f8fafc;
    --subtitle-color: #a5aeba;
    --control-bg: #292b2ea9;
    --control-border: #383c41;
    --status-badge-inactive: #64748b;
    --status-badge-active: #f8fafc;
    --divider-border: #334155;
    --card-bg: #292b2e95;
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
    --item-title-active-color: #f8fafc8c;
    --tag-dev-bg: #334155;
    --tag-dev-color: #94a3b8;
    --item-desc-color: #898f99df;
    --slider-bg: #3e4247;
    --slider-checked-bg: #5d646d;
    --slider-disabled-bg: #334155;
    --range-bg: #42474e;
    --range-thumb-bg: #1e293b;
    --range-thumb-border: #60a5fa;
    --range-thumb-shadow: rgba(0, 0, 0, 0.5);
    --footer-text: #8b8f96aa;
    --overlay-bg: rgba(0, 0, 0, 0.6);
    --modal-bg: #292b2e;
    --modal-border: #383c41;
    --modal-h4: #f8fafc;
    --modal-p: #94a3b8;
    --btn-sec-bg: #1a1a1a;
    --btn-sec-list-bg: #202020;
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
    padding: 36px 32px 16px 32px;
    max-width: 900px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    min-height: calc(100vh - 52px);
    position: relative;
}

.panel-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
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
    margin-bottom: 16px;
}

.main-content {
    display: grid;
    grid-template-columns: 1fr 1.3fr;
    gap: 16px;
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
    padding: 16px 20px;
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
    gap: 12px;
    margin-bottom: 16px;
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
    padding: 12px 0;
    border-bottom: 1px solid var(--chart-border);
}

.setting-item:last-child {
    border-bottom: none;
    padding-bottom: 0;
}

.slider-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 10px;
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

.item-title-sec {
    height: 22px;
    font-size: 14px;
    font-weight: 600;
    color: var(--item-title-color);
    opacity: 0.8;
    display: flex;
    align-items: center;
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
    margin-top: 25px;
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

.ft_left {
    display: flex;
    flex-direction: row;
    justify-content: left;
    align-items: center;
    gap: 10px;
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

.ft_right {
    display: flex;
    flex-direction: row;
    justify-content: right;
    align-items: center;
    gap: 13px;
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
    align-content: start !important;
    grid-template-columns: 1fr 1fr 1fr;
    row-gap: 5px;
    background: var(--card-bg);
    border: 1px solid var(--card-border);
    border-radius: 20px;
    box-shadow: 0 4px 20px -2px var(--card-shadow);
    overflow: visible;
}

.dynamicset-grid::-webkit-scrollbar {
    width: 5px;
}

.dynamicset-grid::-webkit-scrollbar-track {
    background: transparent;
    margin: 12px 0;
}

.dynamicset-grid::-webkit-scrollbar-thumb {
    background-color: var(--slider-bg);
    border-radius: 10px;
}

.dynamicset-grid::-webkit-scrollbar-thumb:hover {
    background-color: var(--slider-checked-bg);
}

/* 设置项：去掉独立背景和边框，融入容器 */
.set-item {
    background: transparent;
    border: none;
    margin: 0;
    border-radius: 0;
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 70px;
    padding: 0 16px;
    box-sizing: border-box;
}

.disabled-set-item {
    opacity: 0.6;
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






/* 数据统计模块样式 */
.card-header-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
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
    margin-bottom: 8px;
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
    height: 65px;
    box-sizing: border-box;
    position: relative;
}

.stat-label {
    font-size: 12px;
    color: var(--item-desc-color);
    font-weight: 500;
    flex-shrink: 0;
    transform: translateY(-4px);
}

.stat-val {
    font-size: 16px;
    font-weight: 700;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    color: var(--speed-value);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 100%;
    flex-shrink: 0;
    transform: translateY(-2px);
}

.stats-chart-container {
    width: 100%;
    flex-grow: 1;
    min-height: 110px;
    border-top: 1px solid var(--chart-border);
    padding-top: 10px;
}


/* 常规设置 - 标题与开关缝合样式 */

/* 标题行横向排列，垂直居中 */
.combo-title-row {
    display: flex;
    align-items: center;
    gap: 8px;
}

/* 分割线样式 */
.title-separator {
    color: var(--control-border);
    font-size: 14px;
    opacity: 0.8;
}

/* 缩小的迷你开关，靠左对齐紧贴着标题 */
.mini-switch {
    transform: scale(0.65);
    transform-origin: left center;
    margin: 0;
}

input:disabled+.slider {
    cursor: not-allowed;
    opacity: 0.5;
}

/* =========================================
   自定义下拉选择器（优化紧凑版）
   ========================================= */

/* 核心修复 1：当下拉菜单打开时，强行将该条目的层级提升到最顶层，击穿 transform 的层级压制 */
.setting-item.is-dropdown-open,
.set-item.is-dropdown-open {
    position: relative;
    z-index: 8888;
    /* 打开下拉菜单时，将整行提升至高层级 */
}

.set-item-meta {
    display: flex;
    flex-direction: column;
    gap: 4px;
    flex-shrink: 0;
    /* 核心修复 2：强制左侧文字不收缩，防止被挤压 */
    min-width: 0;
}

.custom-dropdown {
    position: relative;
    outline: none;
}

/* 优化触发器：固定紧凑宽度，防止横向无限拉长 */
.dropdown-trigger {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 6px;
    padding: 6px 10px;
    background: var(--select-bg);
    border: 1px solid var(--select-border);
    border-radius: 8px;
    cursor: pointer;
    width: 105px;
    /* 固定紧凑宽度 */
    box-sizing: border-box;
    transition: all 0.2s ease;
}

.dropdown-trigger:hover {
    border-color: var(--slider-checked-bg);
}

.current-item {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    font-weight: 600;
    color: var(--select-text);
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
    /* 安全防爆：超出打点 */
}

.current-item .platform-icon {
    width: 14px;
    height: 14px;
    object-fit: contain;
    border-radius: 3px;
    transform: translateY(1px);
}

.arrow-icon {
    width: 12px;
    height: 12px;
    color: var(--item-desc-color);
    transition: transform 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    flex-shrink: 0;
    transform: translateY(1px);
}

.arrow-icon.is-open {
    transform: rotate(180deg);
}

/* 下拉菜单面板调整 */
.dropdown-menu {
    position: absolute;
    top: calc(100% + 6px);
    right: 0;
    width: 116px;
    background: var(--modal-bg);
    border: 1px solid var(--modal-border);
    border-radius: 10px;
    padding: 4px;
    box-shadow: 0 10px 25px var(--card-shadow-hover);
    z-index: 9000;
    display: flex;
    flex-direction: column;
    gap: 2px;
    max-height: 140px;
    overflow-y: auto;
}

.display-pref-trigger {
    width: 82px;
}

.display-pref-menu {
    top: auto;
    right: 0;
    bottom: calc(100% + 8px);
    width: 330px;
    max-height: none;
    padding: 10px;
    gap: 9px;
    overflow: visible;
}

.display-pref-group {
    display: flex;
    flex-direction: column;
    gap: 7px;
}

.display-pref-heading {
    color: var(--item-desc-color);
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.2px;
}

.display-pref-options {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 8px;
}

.display-pref-option {
    position: relative;
    cursor: pointer;
}

.display-pref-option input {
    position: absolute;
    opacity: 0;
    pointer-events: none;
}

.display-pref-option span {
    min-height: 30px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0 7px;
    border-radius: 8px;
    border: 1px solid var(--select-border);
    background: var(--select-bg);
    color: var(--item-desc-color);
    font-size: 11px;
    font-weight: 650;
    transition: all 0.18s ease;
    box-sizing: border-box;
}

.display-pref-option:hover span {
    border-color: var(--slider-checked-bg);
}

.display-pref-option input:checked + span {
    color: var(--btn-pri-color);
    background: var(--btn-pri-bg);
    border-color: var(--btn-pri-border);
    box-shadow: 0 2px 7px var(--btn-pri-shadow-hover);
}

.display-pref-divider {
    height: 1px;
    background: var(--control-border);
    opacity: 0.7;
}

/* 隐藏原生粗糙的滚动条，替换为你主题风格的细线条 */
.dropdown-menu::-webkit-scrollbar {
    width: 4px;
}

.dropdown-menu::-webkit-scrollbar-track {
    background: transparent;
    margin: 4px 0;
    /* 让滚动条上下留点空隙，更好看 */
}

.dropdown-menu::-webkit-scrollbar-thumb {
    background-color: var(--slider-bg);
    border-radius: 4px;
}

.dropdown-menu::-webkit-scrollbar-thumb:hover {
    background-color: var(--slider-checked-bg);
}

.dropdown-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 8px;
    font-size: 12px;
    font-weight: 600;
    color: var(--text-body);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
}

.dropdown-item .platform-icon {
    width: 14px;
    height: 14px;
    object-fit: contain;
    border-radius: 3px;
    opacity: 0.8;
    transform: translateY(1px);
}

.dropdown-item:hover {
    background: var(--btn-sec-list-bg);
}

.dropdown-item.is-active {
    background: var(--btn-sec-list-bg);
}

.dropdown-item.is-active .platform-icon {
    opacity: 1;
}

/* 下拉动画保持不变 */
.dropdown-enter-active,
.dropdown-leave-active {
    transition: all 0.15s cubic-bezier(0.4, 0, 0.2, 1);
    transform-origin: top right;
}

.dropdown-enter-from,
.dropdown-leave-to {
    opacity: 0;
    transform: scaleY(0.95) translateY(-4px);
}

.bottom-grid-card {
    grid-column: 1 / -1;
    max-height: none;
    overflow: visible;
}

/* 留白页面居中 */
.blank-dynamic-page {
    grid-column: 1 / -1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 300px;
    color: var(--item-desc-color);
}

/* 新增的自定义标题栏样式 */
.custom-titlebar {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 32px;
    display: flex;
    justify-content: flex-end;
    align-items: center;
    z-index: 9999;
    border-top-left-radius: inherit;
    border-top-right-radius: inherit;
}

/* 占据剩余空间作为拖拽区 */
.titlebar-drag-area {
    flex-grow: 1;
    height: 100%;
    -webkit-app-region: drag;
}

.titlebar-controls {
    display: flex;
    height: 100%;
    -webkit-app-region: no-drag;
}

.titlebar-btn {
    background: transparent;
    border: none;
    color: var(--text-body);
    width: 45px;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: background-color 0.2s ease, color 0.2s ease;
}

.titlebar-btn svg {
    width: 11px;
    height: 11px;
    opacity: 0.8;
}

.titlebar-btn:hover {
    background-color: var(--btn-sec-bg);
}

.titlebar-btn:hover svg {
    opacity: 1;
}

.close-btn:hover {
    background-color: #ff4757 !important;
    color: #ffffff !important;
}

/* 颜色选择器专用预览小方块 */
.color-preview-icon {
    width: 14px;
    height: 14px;
    border-radius: 3px;
    flex-shrink: 0;
    box-sizing: border-box;
    transition: all 0.2s ease;
    transform: translateY(1px);
}

/* 暗色示例方块 */
.color-preview-icon.theme-black {
    background-color: #1a1a1a;
}

/* 亮色示例方块 */
.color-preview-icon.theme-white {
    background-color: #f5f5f5;
}

.dropdown-item .color-preview-icon {
    opacity: 0.8;
}

.dropdown-item:hover .color-preview-icon,
.dropdown-item.is-active .color-preview-icon {
    opacity: 1;
}

/* 沉浸模式 (Coverglass) 专属四层背景 */
.coverglass-bg-container {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: 0;
    pointer-events: none;
    /* 绝对不能挡住鼠标点击事件 */
    overflow: hidden;
}

/* 纯净的静态背景图层 */
.coverglass-bg-image {
    position: fixed;
    top: -5%;
    left: -5%;
    width: 110%;
    height: 110%;
    background-size: cover;
    background-position: center;
    opacity: 0.3;
    transition: background-image 0.8s ease;
    transform: translateZ(0);
}

/* 高斯模糊层 (使用 backdrop-filter 性能最佳) */
.coverglass-blur-layer {
    display: none;
}

/* SVG 高级细粒度噪点层 (模拟磨砂玻璃的光学漫反射) */
.coverglass-noise-layer {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    opacity: 0.08;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='256' height='256'%3E%3Cfilter id='noise'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='2.5' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noise)'/%3E%3C/svg%3E");
    background-repeat: repeat;
    background-size: 150px 150px;
}

.panel-header,
.divider,
.main-content,
.panel-footer {
    position: relative;
    z-index: 1;
}

/* 外层卡片保留原有宽高与布局，覆盖 display 使得内部 Slider 正常响应 */
.card-pager-outer {
    position: relative !important;
    padding: 0 !important;
    display: block !important;
    overflow: visible !important;
    /* 允许菜单向下悬浮弹出 */
}

/* 内嵌视口：不改变任何内外边距，仅做左右水平裁切与层级提升 */
.pager-viewport {
    width: 100%;
    position: relative;
    z-index: 5;
    /* 显示内容配置面板向上展开，预留完整空间，避免被分页视口裁掉。 */
    clip-path: inset(-320px 0 -250px 0);
}

/* 快速响应的贝塞尔曲线平滑轨道 */
.pager-track {
    display: flex;
    width: 200%;
    transition: transform 0.3s cubic-bezier(0.25, 1, 0.5, 1);
}

/* 每一页保持 3 列网格布局 */
.pager-page {
    width: 50%;
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    row-gap: 5px;
    padding: 0 4px;
    box-sizing: border-box;
    transform: translateY(5px);
}

/* 正下方悬浮翻页小胶囊 */
.pagination-capsule {
    position: absolute;
    bottom: 4px;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 2px 8px;
    height: 14px;
    border-radius: 10px;
    cursor: pointer;
    z-index: 4;
    background: rgba(0, 0, 0, 0.08);
    border: 1px solid rgba(0, 0, 0, 0.05);
    backdrop-filter: blur(6px);
    -webkit-backdrop-filter: blur(6px);
    transition: all 0.2s cubic-bezier(0.25, 1, 0.5, 1);
}

:global(.dark-theme) .pagination-capsule {
    background: rgba(255, 255, 255, 0.12);
    border: 1px solid rgba(255, 255, 255, 0.08);
}

.pagination-capsule:hover {
    background: rgba(0, 0, 0, 0.15);
    transform: translateX(-50%) scale(1.08);
}

:global(.dark-theme) .pagination-capsule:hover {
    background: rgba(255, 255, 255, 0.22);
}

/* 胶囊内的小圆球指示器 */
.page-dot {
    width: 5px;
    height: 5px;
    border-radius: 50%;
    background-color: var(--text-body);
    opacity: 0.25;
    transition: all 0.3s cubic-bezier(0.25, 1, 0.5, 1);
}

.page-dot.active {
    opacity: 0.9;
    width: 12px;
    border-radius: 3px;
}
</style>
