<template>
    <BackToHome />
    <h1>HciLog</h1>
    <FileDropZone :showControls="true" :buttons="buttonOptions" :checkboxes="checkboxOptions"
        :numberInputs="numberInputs" @button-clicked="handleButtonClicked" />

    <HelpButton />
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import BackToHome from '@/components/BackToHome.vue';
import FileDropZone from '@/components/FileDropZone.vue';
import { invoke } from '@tauri-apps/api/core';
import HelpButton from '@/components/HelpButton.vue';

interface HciLogOptions {
    bluetrum_ts: boolean;
    skip_chars: number;
}

export default defineComponent({
    name: 'HciLog',
    components: {
        BackToHome,
        FileDropZone,
        HelpButton, // 注册 HelpButton 组件
    },
    setup() {
        const buttonOptions = ref([
            { label: '提交', id: 'submit' },
        ]);
        const checkboxOptions = ref([
            { label: '处理蓝讯时间戳', state: true },
        ])
        const numberInputs = ref([
            { label: '需要忽略行首的字符数', value: 0 },
        ]);

        const handleButtonClicked = async (data: {
            buttonId: string;
            filePath: string;
            checkboxes: Array<{ label: string; state: boolean }>;
            numberInputs: Array<{ label: string; value: number }>;
        }) => {
            switch (data.buttonId) {
                case 'submit':
                    try {
                        await invoke('parse_hci_log', {
                            filePath: data.filePath,
                            options: <HciLogOptions>{
                                bluetrum_ts: data.checkboxes[0].state,
                                skip_chars: data.numberInputs[0].value,
                            }
                        });
                        alert('转换完成');
                    } catch (error) {
                        console.error('提交失败:', error);
                    }
                    break;
                default:
                    break;
            }
        };

        return {
            buttonOptions,
            numberInputs,
            checkboxOptions,
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