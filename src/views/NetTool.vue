<template>
  <div class="app">
    <BackToHome />
    <h1>NetTool</h1>
    <form @submit.prevent="startTest">
      <div class="form-group">
        <label for="mode">Mode:</label>
        <select id="mode" v-model="config.mode">
          <option value="tcp">TCP</option>
          <option value="udp">UDP</option>
        </select>
      </div>
      <div class="form-group">
        <label for="role">Role:</label>
        <select id="role" v-model="config.role">
          <option value="client">Client</option>
          <option value="server">Server</option>
        </select>
      </div>
      <div class="form-group">
        <label for="address">Server Address:</label>
        <input id="address" v-model="config.address" placeholder="127.0.0.1" />
      </div>
      <div class="form-group">
        <label for="port">Port:</label>
        <input id="port" v-model="config.port" type="number" placeholder="8080" />
      </div>
      <div class="form-group">
        <label for="interval">Update Interval (seconds):</label>
        <input id="interval" v-model="config.interval" type="number" placeholder="5" />
      </div>
      <div class="form-group">
        <label for="duration">Test Duration (seconds):</label>
        <input id="duration" :value="config.duration === null ? '' : config.duration"
          @input="updateDuration(($event.target as HTMLInputElement).value)" type="number" placeholder="null" />
      </div>
      <button type="submit" class="btn-start">Start Test</button>
    </form>
    <button @click="stopTest" class="btn-stop">Stop Test</button>
    <div class="log-containerc">
      <textarea 
        readonly 
        class="log-box"
        ref="logBox"
        :value="logContent"
        wrap="off"
      ></textarea>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, reactive, onMounted, onUnmounted, nextTick  } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import BackToHome from '@/components/BackToHome.vue';

interface NetToolConfig {
  mode: string;
  role: string;
  address: string;
  port: number;
  interval: number;
  duration: number | null;
}

interface RateInfo {
  delta_bytes: number;
  rate: number;
}

export default defineComponent({
  name: 'NetTool',
  components: {
    BackToHome,
  },
  setup() {
    const rate = ref(0.0);
    const delta_bytes = ref(0);
    const config = reactive<NetToolConfig>({
      mode: 'tcp',
      role: 'client',
      address: '127.0.0.1',
      port: 8080,
      interval: 5,
      duration: null, // 默认不设置持续时间
    });

    let unlisten: () => void;

    const logContent = ref('');
    const logBox = ref<HTMLTextAreaElement | null>(null);
  
    // 更新日志内容并自动滚动到底部
    const updateLog = (entry: string) => {
      logContent.value += `${new Date().toLocaleTimeString()}: ${entry}\n`;
      nextTick(() => {
        if (logBox.value) {
          logBox.value.scrollTop = logBox.value.scrollHeight;
        }
      });
    };

    // 监听速率更新事件
    onMounted(async () => {
      unlisten = await listen<RateInfo>('rate-update', (event) => {
        delta_bytes.value = event.payload.delta_bytes;
        rate.value = event.payload.rate;
        console.log('Received rate update:', event.payload);
        updateLog(`Total Bytes: ${(delta_bytes.value / 1000).toFixed(2)}KB - Rate: ${rate.value.toFixed(2)}KB/s`);
      });
    });

    // 组件卸载时取消监听
    onUnmounted(() => {
      if (unlisten) {
        unlisten();
      }
    });

    // 启动测试
    const startTest = async () => {
      try {
        await invoke('nettool_start_test', {
          config: config,
        });
      } catch (error) {
        console.error('Failed to start test:', error);
      }
    };

    // 停止测试
    const stopTest = async () => {
      try {
        await invoke('nettool_stop_test');
      } catch (error) {
        console.error('Failed to stop test:', error);
      }
    };

    // 新增处理duration输入的方法
    const updateDuration = (value: string) => {
      if (value === '') {
        config.duration = null;
      } else {
        const num = Number(value);
        config.duration = isNaN(num) ? null : num;
      }
    };

    return {
      rate,
      delta_bytes,
      logContent,
      logBox,
      config,
      startTest,
      stopTest,
      updateDuration,
    };
  },
});
</script>

<style scoped>
.app {
  text-align: center;
  padding: 20px;
  padding-top: 80px;
  /* 为返回按钮留出空间 */
}

.form-group {
  margin-bottom: 15px;
}

label {
  display: inline-block;
  width: 150px;
  text-align: right;
  margin-right: 10px;
}

input,
select {
  width: 200px;
  padding: 5px;
  border: 1px solid #ccc;
  border-radius: 4px;
}

button {
  padding: 10px 20px;
  margin: 10px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 16px;
}

.btn-start {
  background-color: #4caf50;
  color: white;
}

.btn-stop {
  background-color: #f44336;
  color: white;
}

.rate-display {
  margin-top: 20px;
  font-size: 18px;
  font-weight: bold;
}

/* 新版日志显示样式 */
.log-container {
  margin: 20px auto;
  max-width: 90%;
  border-radius: 8px;
  background: #1e1e1e;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.log-box {
  width: 100%;
  height: 60vh;
  min-height: 300px;
  padding: 15px;
  color: #d4d4d4;
  font-family: 'Consolas', monospace;
  font-size: 14px;
  line-height: 1.5;
  border: none;
  background: transparent;
  resize: none;
  white-space: pre;
  overflow-y: auto;
  scroll-behavior: smooth;
}

.log-box:focus {
  outline: none;
}

/* 滚动条样式 */
.log-box::-webkit-scrollbar {
  width: 8px;
}

.log-box::-webkit-scrollbar-track {
  background: #2d2d2d;
  border-radius: 4px;
}

.log-box::-webkit-scrollbar-thumb {
  background: #4d4d4d;
  border-radius: 4px;
}

.log-box::-webkit-scrollbar-thumb:hover {
  background: #707070;
}
</style>