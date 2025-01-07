<template>
    <div class="exception-log">
      <BackToHome />
      <h1>异常日志处理</h1>
      <FileDropZone @file-selected="handleFileSelected" />
      <div class="form-container">
        <div class="form-group">
          <label for="file-path">异常日志文件地址：</label>
          <input
            id="file-path"
            v-model="filePath"
            type="text"
            placeholder="请输入文件地址"
            readonly
          />
        </div>
        <button @click="handleProcess">处理</button>
      </div>
  
      <div v-if="cpuRegs" class="result-container">
        <h2>{{ cpuRegs.header }}</h2>
        <div class="registers">
          <div v-for="(value, index) in cpuRegs.regs" :key="index" class="register-item">
            <strong>寄存器 {{ index }}:</strong> {{ value }}
          </div>
        </div>
      </div>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent, ref } from 'vue';
  import { invoke } from '@tauri-apps/api/core';
  import BackToHome from '@/components/BackToHome.vue';
  import FileDropZone from '@/components/FileDropZone.vue'; // 引入拖拽组件
  
  interface CPURegs {
    regs: string[];
    header: string;
  }
  
  export default defineComponent({
    name: 'ExceptionLog',
    components: {
      BackToHome,
      FileDropZone,
    },
    setup() {
      const filePath = ref(''); // 文件地址
      const cpuRegs = ref<CPURegs | null>(null); // CPU 寄存器组数据
  
      // 处理文件选择
      const handleFileSelected = (path: string) => {
        filePath.value = path;
      };
  
      // 处理按钮点击事件
      const handleProcess = async () => {
        if (!filePath.value) {
          alert('请选择文件');
          return;
        }
  
        try {
          // 调用 Rust 后端处理异常日志
          const result = await invoke<CPURegs>('process_exception_log', {
            filePath: filePath.value,
          });
  
          // 将结果保存到 cpuRegs
          cpuRegs.value = result;
        } catch (error) {
          console.error('处理异常日志失败:', error);
          alert('处理异常日志失败，请检查文件地址是否正确');
        }
      };
  
      return {
        filePath,
        cpuRegs,
        handleFileSelected,
        handleProcess,
      };
    },
  });
  </script>
  
  <style scoped>
  .exception-log {
    padding: 20px;
    position: relative;
  }
  
  .form-container {
    max-width: 400px;
    margin: 0 auto;
    text-align: left;
  }
  
  .form-group {
    margin-bottom: 15px;
  }
  
  label {
    display: block;
    margin-bottom: 5px;
    font-weight: bold;
  }
  
  input {
    width: 100%;
    padding: 8px;
    box-sizing: border-box;
  }
  
  button {
    padding: 10px 20px;
    background-color: #4caf50;
    color: white;
    border: none;
    cursor: pointer;
  }
  
  button:hover {
    background-color: #45a049;
  }
  
  .result-container {
    margin-top: 20px;
    text-align: left;
  }
  
  .registers {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 10px;
  }
  
  .register-item {
    padding: 10px;
    border: 1px solid #ddd;
    border-radius: 5px;
    background-color: #f9f9f9;
  }
  </style>