<template>
    <BackToHome />
    <h1>线程分析</h1>
    <div class="iframe-container">
        <iframe ref="iframe" :srcdoc="plotHtml" style="width: 100%; height: 500px; border: none;"></iframe>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import BackToHome from '@/components/BackToHome.vue';
import { invoke } from '@tauri-apps/api/core';

export default defineComponent({
    name: 'AnalyzeThread',
    components: {
        BackToHome, // 注册组件
    },
    mounted() {
        // 监听窗口大小变化
        window.addEventListener('resize', this.refreshIframe);
    },
    beforeDestroy() {
        // 移除事件监听
        window.removeEventListener('resize', this.refreshIframe);
    },
    methods: {
        refreshIframe() {
            const iframe = this.$refs.iframe;
            if (iframe) {
                // 强制刷新 iframe
                iframe.src = iframe.src;
            }
        }
    },
    data() {
        return {
            plotHtml: ''
        }
    },
    async mounted() {
        this.plotHtml = await invoke('generate_plot')

        this.plotHtml = `<script src="/js/plotly-2.12.1.min.js"><\/script>` + this.plotHtml
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
