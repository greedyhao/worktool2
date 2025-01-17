<template>
    <div class="home">
        <div class="version-info">
            <button class="version-button" :class="{ 'new-version': hasNewVersion }" @click="checkForUpdates">
                版本号: {{ version }}
                <span v-if="hasNewVersion" class="new-badge">New</span>
            </button>
            <p v-if="isDebug">编译时间: {{ buildTime }}</p>
        </div>
        <h1>欢迎使用开发辅助工具</h1>
        <div class="grid-container">
            <router-link v-if="isDesktop" to="/ExceptionLog" class="grid-item">异常信息处理</router-link>
            <router-link v-if="isDesktop" to="/AnalyzeThread" class="grid-item">线程分析</router-link>
            <router-link v-if="isDesktop" to="/HciLog" class="grid-item">HCI日志转换</router-link>
            <router-link v-if="isDesktop" to="/MemoryTrace" class="grid-item">内存泄漏分析</router-link>
            <router-link to="/NetTool" class="grid-item">网络工具</router-link>
            <router-link to="/Settings" class="grid-item">设置</router-link>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { invoke } from '@tauri-apps/api/core';
import { load } from '@tauri-apps/plugin-store';

export default defineComponent({
    name: 'Home',
    data() {
        return {
            version: __APP_VERSION__, // 版本号
            isDebug: __BUILD_MODE__ === 'development', // 是否是 debug 模式
            buildTime: __BUILD_TIME__, // 编译时间
            hasNewVersion: false, // 是否有新版本
            isDesktop: false,
            autoUpdate: true, // 默认自动更新
            updateLog: '', // 更新日志
        };
    },
    methods: {
        async checkForUpdates() {
            if (this.isDesktop) {
                try {
                    const update = await check();
                    if (update) {
                        this.hasNewVersion = true;
                        console.log(
                            `发现新版本 ${update.version}，发布日期：${update.date}，更新内容：${update.body}`
                        );

                        // 用户确认后，开始下载并安装更新
                        let downloaded = 0;
                        let contentLength = 0;

                        await update.downloadAndInstall((event) => {
                            switch (event.event) {
                                case 'Started':
                                    contentLength = event.data.contentLength ?? 0;
                                    console.log(`开始下载，总大小：${event.data.contentLength} 字节`);
                                    break;
                                case 'Progress':
                                    downloaded += event.data.chunkLength;
                                    console.log(`已下载 ${downloaded} 字节，总大小：${contentLength}`);
                                    break;
                                case 'Finished':
                                    console.log('下载完成');
                                    break;
                            }
                        });

                        console.log('更新已安装');
                        this.updateLog = `版本: ${update.version}\n发布日期: ${update.date}\n更新内容: ${update.body}`;
                        await relaunch(); // 重启应用
                    } else {
                        alert('当前已是最新版本。');
                    }
                } catch (error) {
                    console.error('检查更新失败:', error);
                    alert('检查更新失败，请稍后重试。');
                }
            }
        },
        async loadAutoUpdateSetting() {
            const store = await load('store.json');
            const val = await store.get<{ value: boolean }>('autoUpdate');
            this.autoUpdate = val?.value ?? true;
        },
    },
    async mounted() {
        await this.loadAutoUpdateSetting();

        // 进入页面时自动检查更新
        if (this.isDesktop && this.autoUpdate) {
            try {
                const update = await check();
                if (update) {
                    this.hasNewVersion = true;
                    console.log(
                        `发现新版本 ${update.version}，发布日期：${update.date}，更新内容：${update.body}`
                    );

                    // 自动下载并安装更新
                    await update.downloadAndInstall();
                    this.updateLog = `版本: ${update.version}\n发布日期: ${update.date}\n更新内容: ${update.body}`;
                    await relaunch();
                }
            } catch (error) {
                console.error('检查更新失败:', error);
            }
        }

        const platform = <string>await invoke('get_platform');
        this.isDesktop = ['windows', 'macos', 'linux'].includes(platform);

        // 显示更新日志
        if (this.updateLog) {
            alert(`更新日志:\n${this.updateLog}`);
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
    top: -10px;
    right: -10px;
    background-color: red;
    color: white;
    font-size: 12px;
    padding: 2px 6px;
    border-radius: 50%;
}
</style>