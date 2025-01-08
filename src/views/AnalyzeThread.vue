<template>
    <div class="analyze-thread">
        <BackToHome />
        <h1>线程分析</h1>
        <FileDropZone :showControls="true" :buttons="buttonOptions" @button-clicked="handleButtonClicked" />
        <div class="iframe-container">
            <iframe ref="iframe" :srcdoc="plotHtml" style="width: 100%; height: 100%; border: none;"></iframe>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, onUnmounted } from 'vue';
import BackToHome from '@/components/BackToHome.vue';
import { invoke } from '@tauri-apps/api/core';
import FileDropZone from '@/components/FileDropZone.vue';

export default defineComponent({
    name: 'AnalyzeThread',
    components: {
        BackToHome,
        FileDropZone,
    },
    setup() {
        const checkboxOptions = ref([]);
        const buttonOptions = ref([
            { label: '预处理', id: 'preprocess' },
            { label: '提交', id: 'submit' },
        ]);

        const iframe = ref<HTMLIFrameElement | null>(null); // iframe 引用
        const plotHtml = ref(''); // iframe 的内容

        // 调整 iframe 大小
        const resizeIframe = () => {
            if (iframe.value) {
                // 强制重新加载 iframe
                iframe.value.srcdoc = iframe.value.srcdoc;
            }
        };

        // 监听窗口大小变化
        onMounted(async () => {
            // 获取 plot HTML 内容
            plotHtml.value = await invoke('generate_plot');
            plotHtml.value = `<script src="/js/plotly-2.12.1.min.js"><\/script>` + plotHtml.value;

            // 监听窗口大小变化
            window.addEventListener('resize', resizeIframe);
            resizeIframe(); // 初始化时调整大小
        });

        // 移除事件监听
        onUnmounted(() => {
            window.removeEventListener('resize', resizeIframe);
        });

        const handleButtonClicked = async (data: {
            buttonId: string;
            filePath: string;
            checkboxes: Array<{ label: string; state: boolean }>;
        }) => {
            switch (data.buttonId) {
                case 'preprocess':
                    try {
                        // 调用 Rust 后端处理异常日志
                        const result = await invoke('analyze_thread_preprocess', {
                            inputFile: data.filePath,
                            outputFile: data.filePath + '.out.txt',
                        });

                        console.log('预处理成功:', result);
                        // // 处理成功的结果
                        // if (result) {
                        //     console.log('预处理成功:', result);
                        //     alert('预处理成功，结果已保存到文件');
                        // }
                    } catch (error) {
                        console.error('处理异常日志失败:', error);
                        alert('处理异常日志失败，请检查文件地址是否正确');
                    }
                    break;
                case 'submit':
                default:
                    console.log('未知操作:', data);
            }
        };

        return {
            buttonOptions,
            iframe,
            plotHtml,
            handleButtonClicked,
        };
    },
});
</script>

<style scoped>
.analyze-thread {
    padding: 20px;
    position: relative;
}

.iframe-container {
    width: 100%;
    height: calc(100vh - 100px);
    /* 减去标题和返回按钮的高度 */
    overflow: hidden;
    /* 隐藏溢出内容 */
}
</style>