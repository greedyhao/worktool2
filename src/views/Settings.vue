<template>
    <BackToHome />
    <h1>Settings</h1>
    <div class="setting-item">
        <label for="auto-update">自动更新</label>
        <input type="checkbox" id="auto-update" v-model="autoUpdate" @change="saveSettings" />
    </div>
    <div class="setting-item">
        <a :href="sourceCodeUrl" target="_blank">查看项目源码</a>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue';
import BackToHome from '@/components/BackToHome.vue';
import { load } from '@tauri-apps/plugin-store';

export default defineComponent({
    name: 'Settings',
    components: {
        BackToHome,
    },
    setup() {
        const autoUpdate = ref(false);
        const sourceCodeUrl = 'https://github.com/greedyhao/worktool2'; // 替换为你的项目源码地址

        // 加载保存的设置
        onMounted(async () => {
            const store = await load('store.json');
            const val = await store.get<{ value: boolean }>('autoUpdate');
            autoUpdate.value = val?.value ?? true;
            console.log(val);
        });

        // 保存设置
        const saveSettings = async () => {
            const store = await load('store.json');
            await store.set('autoUpdate', { value: autoUpdate.value });
        };

        return {
            autoUpdate,
            sourceCodeUrl,
            saveSettings,
        };
    },
});
</script>

<style scoped>
.app {
    text-align: center;
    padding: 20px;
    padding-top: 80px;
    /* 为返回按钮留出空间 */
}

.setting-item {
    margin: 20px 0;
}

.setting-item label {
    margin-right: 10px;
}

.setting-item a {
    color: #42b983;
    text-decoration: none;
}

.setting-item a:hover {
    text-decoration: underline;
}
</style>