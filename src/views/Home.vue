<template>
    <div class="home">
        <div class="version-info">
            <p>版本号: {{ version }}</p>
            <p v-if="isDebug">编译时间: {{ buildTime }}</p>
        </div>
        <h1>欢迎使用开发辅助工具</h1>
        <div class="grid-container">
            <router-link to="/HardfaultLog" class="grid-item">异常信息处理</router-link>
            <router-link to="/AnalyzeThread" class="grid-item">线程分析</router-link>
            <router-link to="/HciLog" class="grid-item">HCI日志转换</router-link>
            <router-link to="/MemoryTrace" class="grid-item">内存泄漏分析</router-link>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    name: 'Home',
    data() {
        return {
            version: __APP_VERSION__, // 版本号
            isDebug: __BUILD_MODE__ === 'development', // 是否是 debug 模式
            buildTime: __BUILD_TIME__, // 编译时间
        };
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
</style>