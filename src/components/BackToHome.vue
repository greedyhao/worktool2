<template>
    <router-link to="/" class="back-to-home">
      <span>← 返回首页</span>
    </router-link>
  </template>
  
  <script lang="ts">
  import { defineComponent } from 'vue';
  
  export default defineComponent({
    name: 'BackToHome',
  });
  </script>
  
  <style scoped>
  .back-to-home {
    position: fixed; /* 固定在页面左上角 */
    top: 20px;
    left: 20px;
    padding: 10px 20px;
    background-color: #4CAF50; /* 绿色背景 */
    color: white; /* 白色文字 */
    border-radius: 5px;
    text-decoration: none;
    font-size: 16px;
    transition: background-color 0.3s, transform 0.2s;
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2); /* 添加阴影 */
  }
  
  .back-to-home:hover {
    background-color: #45a049; /* 悬停时背景色变深 */
    transform: translateY(-2px); /* 悬停时轻微上移 */
  }
  
  .back-to-home:active {
    transform: translateY(0); /* 点击时恢复原位 */
  }
  </style>