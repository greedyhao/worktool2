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
      <div v-for="(row, rowIndex) in registerRows" :key="rowIndex" class="register-row">
        <div v-for="(reg, regIndex) in row" :key="regIndex" class="register-item">
          <strong>{{ reg.name }}:</strong> {{ reg.value }}
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import BackToHome from '@/components/BackToHome.vue';
import FileDropZone from '@/components/FileDropZone.vue';

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

    // 寄存器名称
    const registerNames = [
      "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
      "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
      "t5", "t6",
    ];

    // 将寄存器数据分组，每行 4 个
    const registerRows = computed(() => {
      if (!cpuRegs.value) return [];

      const rows = [];
      for (let i = 0; i < cpuRegs.value.regs.length; i += 4) {
        const row = registerNames.slice(i, i + 4).map((name, index) => ({
          name,
          value: cpuRegs.value!.regs[i + index],
        }));
        rows.push(row);
      }
      return rows;
    });

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
      registerRows,
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

.register-row {
  display: flex;
  gap: 20px;
  margin-bottom: 10px;
}

.register-item {
  flex: 1;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 5px;
  background-color: #f9f9f9;
}
</style>