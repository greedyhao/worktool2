<template>
    <BackToHome />
    <h1>MemoryTrace</h1>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import BackToHome from '@/components/BackToHome.vue';

export default defineComponent({
    name: 'MemoryTrace',
    components: {
        BackToHome, // 注册组件
    },
});
</script>

<style scoped>
.app {
    text-align: center;
    padding: 20px;
    padding-top: 80px; /* 为返回按钮留出空间 */
}
</style>