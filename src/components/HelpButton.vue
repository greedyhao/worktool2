<template>
    <!-- 将按钮固定在右下角 -->
    <button class="help-button" @click="redirectToHelp">?</button>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { open } from '@tauri-apps/plugin-shell';

export default defineComponent({
    name: 'HelpButton',
    setup() {
        // 固定的帮助链接
        const helpUrl = 'https://gitee.com/haozhu1997/worktool2/blob/main/doc/help.md';

        // 使用 Tauri 的安全方式打开链接
        const redirectToHelp = async () => {
            await open(helpUrl); // 在新标签页中打开链接
        };

        return {
            redirectToHelp,
        };
    },
});
</script>

<style scoped>
/* 帮助按钮样式 */
.help-button {
    position: fixed; /* 固定在页面 */
    bottom: 20px; /* 距离底部 20px */
    right: 20px; /* 距离右侧 20px */
    width: 40px; /* 设置按钮宽度 */
    height: 40px; /* 设置按钮高度 */
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 50%; /* 圆形按钮 */
    font-size: 20px; /* 问号大小 */
    cursor: pointer;
    z-index: 1000; /* 确保按钮在最上层 */
    display: flex;
    justify-content: center;
    align-items: center;
}

.help-button:hover {
    background-color: #0056b3;
}
</style>