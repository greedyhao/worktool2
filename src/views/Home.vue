<template>
    <div class="home">
        <div class="version-info">
            <button class="version-button" :class="{ 'new-version': hasNewVersion }" @click="checkForUpdates">
                <div class="progress-bar" :style="{ width: downloadProgress + '%' }"></div>
                版本号: {{ version }}
                <span v-if="hasNewVersion" class="new-badge">New</span>
            </button>
            <p v-if="isDebug">编译时间: {{ buildTime }}</p>
            <a href="#" class="changelog-link" @click.prevent="openChangelog">查看完整更新日志</a>
        </div>
        <h1>欢迎使用开发辅助工具</h1>
        <div class="grid-container">
            <router-link v-if="isDesktop" to="/ExceptionLog" class="grid-item">异常信息处理</router-link>
            <router-link v-if="isDesktop" to="/AnalyzeThread" class="grid-item">线程分析</router-link>
            <router-link v-if="isDesktop" to="/HciLog" class="grid-item">HCI日志转换</router-link>
            <router-link v-if="isDesktop" to="/BinaryConverter" class="grid-item">二进制转换</router-link>
            <router-link v-if="isDesktop" to="/MemoryTrace" class="grid-item">内存泄漏分析</router-link>
            <!-- <router-link v-if="isDesktop" to="/AudioConverter" class="grid-item">音频格式转换</router-link> -->
            <router-link to="/NetworkSpeedTest" class="grid-item">网络工具</router-link>
            <router-link to="/Settings" class="grid-item">设置</router-link>
        </div>
        <HelpButton />
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { check, Update } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { invoke } from '@tauri-apps/api/core';
import { load } from '@tauri-apps/plugin-store';
import { open } from '@tauri-apps/plugin-shell';
import HelpButton from '@/components/HelpButton.vue';

export default defineComponent({
    name: 'Home',
    components: {
        HelpButton,
    },
    data() {
        return {
            version: __APP_VERSION__,
            isDebug: __BUILD_MODE__ === 'development',
            buildTime: __BUILD_TIME__,
            hasNewVersion: false,
            isDesktop: false,
            autoUpdate: true,
            updateLog: '',
            downloadProgress: 0,
            isUpdating: false,
        };
    },
    methods: {
        async checkForUpdates() {
            if (!this.isDesktop || this.isUpdating) return;
            await this.handleUpdateCheck(true);
        },

        async handleUpdateCheck(isManual: boolean) {
            try {
                const update = await check();
                if (!update) {
                    isManual && alert('当前已是最新版本。');
                    this.hasNewVersion = false; // 清除更新标记
                    return;
                }

                this.hasNewVersion = true; // 始终标记有新版本

                if (!this.autoUpdate && !isManual) {
                    return;
                }
                // 自动更新逻辑（仅在autoUpdate=true且非手动时触发）
                if (this.autoUpdate && !isManual) {
                    await this.performUpdate(update, false);
                }
                // 手动更新或关闭自动更新时的逻辑
                else if (isManual) {
                    await this.performUpdate(update, isManual);
                }
            } catch (error) {
                console.error('检查更新失败:', error);
                isManual && alert('检查更新失败，请稍后重试。');
            }
        },

        async performUpdate(update: Update, _isManual: boolean) {
            this.isUpdating = true;
            this.downloadProgress = 0;

            try {
                alert(`更新内容：${update.body}`);

                let downloaded = 0;
                let contentLength = 0;

                await update.downloadAndInstall((event) => {
                    switch (event.event) {
                        case 'Started':
                            contentLength = event.data.contentLength ?? 0;
                            downloaded = 0;
                            break;
                        case 'Progress':
                            downloaded += event.data.chunkLength;
                            this.downloadProgress = contentLength > 0
                                ? Math.min((downloaded / contentLength) * 100, 100)
                                : 0;
                            break;
                    }
                });

                this.updateLog = `版本: ${update.version}\n发布日期: ${update.date}\n更新内容: ${update.body}`;
                await relaunch();
            } finally {
                this.isUpdating = false;
                this.downloadProgress = 0;
            }
        },

        async loadAutoUpdateSetting() {
            const store = await load('store.json');
            const val = await store.get<{ value: boolean }>('autoUpdate');
            this.autoUpdate = val?.value ?? true;
        },
        
        async openChangelog() {
            await open('https://gitee.com/haozhu1997/worktool2/blob/main/doc/changelog.md');
        }
    },
    async mounted() {
        await this.loadAutoUpdateSetting();
        const platform = await invoke<string>('get_platform');
        this.isDesktop = ['windows', 'macos', 'linux'].includes(platform);

        if (this.isDesktop) {
            this.handleUpdateCheck(false);
        }
    },
});
</script>

<style scoped>
.home {
    text-align: center;
    padding: 20px;
}

.grid-container {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 20px;
    padding: 20px;
}

.grid-item {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 150px;
    background-color: #f0f0f0;
    border-radius: 10px;
    text-decoration: none;
    color: #333;
    font-size: 18px;
    transition: background-color 0.3s;
}

.grid-item:hover {
    background-color: #ddd;
}

.version-button {
    position: relative;
    padding: 10px 20px;
    font-size: 16px;
    border: none;
    border-radius: 5px;
    background-color: #007bff;
    color: white;
    cursor: pointer;
    transition: background-color 0.3s;
}

.version-button.new-version {
    background-color: #28a745;
}

.version-button:hover {
    background-color: #0056b3;
}

.new-badge {
    position: absolute;
    top: -8px;
    /* 调整到按钮外边缘 */
    right: -8px;
    /* 调整到按钮外边缘 */
    background-color: red;
    color: white;
    font-size: 12px;
    padding: 2px 6px;
    border-radius: 50%;
    z-index: 1;
    /* 确保在进度条上方 */
}

.progress-bar {
    position: absolute;
    bottom: 0;
    left: 0;
    height: 2px;
    background: rgba(255, 255, 255, 0.8);
    transition: width 0.3s ease;
    z-index: 0;
    /* 保持在底层 */
}

.changelog-link {
    display: block;
    margin-top: 8px;
    color: #666;
    font-size: 14px;
    text-decoration: none;
    cursor: pointer;
    transition: color 0.2s;
}

.changelog-link:hover {
    color: #007bff;
    text-decoration: underline;
}
</style>