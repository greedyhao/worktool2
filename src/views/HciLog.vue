<template>
    <BackToHome />
    <h1>HciLog</h1>
    <FileDropZone :showControls="true" :buttons="buttonOptions" @button-clicked="handleButtonClicked" />
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import BackToHome from '@/components/BackToHome.vue';
import FileDropZone from '@/components/FileDropZone.vue';
import { invoke } from '@tauri-apps/api/core';

export default defineComponent({
    name: 'HciLog',
    components: {
        BackToHome, // 注册组件
        FileDropZone,
    },
    setup() {
        const buttonOptions = ref([
            { label: '提交', id: 'submit' },
        ]);

        const handleButtonClicked = async (data: {
            buttonId: string;
            filePath: string;
            checkboxes: Array<{ label: string; state: boolean }>;
        }) => {
            switch (data.buttonId) {
                case 'submit':
                    try {
                        await invoke('parse_hci_log', {
                            filePath: data.filePath,
                        });
                    } catch (error) {
                        console.error('提交失败:', error);
                    }
                    break;
                default:
                    break;
            }
        }
        return {
            buttonOptions,
            handleButtonClicked,
        };
    }
});
</script>

<style scoped>
.app {
    text-align: center;
    padding: 20px;
    padding-top: 80px;
    /* 为返回按钮留出空间 */
}
</style>