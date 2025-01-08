<template>
  <div class="drop-zone-container">
    <div class="drop-zone" @click="handleClick">
      <p v-if="!filePath">将文件拖放到此处，或点击选择文件</p>
      <p v-else>已选择文件：{{ filePath }}</p>
    </div>

    <!-- 拖拽时的覆盖层 -->
    <div v-if="isDragOver" class="drop-overlay">
      <p>{{ dropMessage }}</p>
    </div>

    <!-- 添加的控件区域 -->
    <div v-if="showControls" class="controls">
      <!-- 动态生成复选框 -->
      <div v-for="(checkbox, index) in checkboxes" :key="index" class="checkbox-container">
        <label>
          <input type="checkbox" v-model="checkbox.state" /> {{ checkbox.label }}
        </label>
      </div>

      <!-- 提交按钮 -->
      <button @click="handleButtonClick">提交</button>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { getCurrentWebview } from '@tauri-apps/api/webview';

export default defineComponent({
  name: 'FileDropZone',
  emits: ['file-selected', 'controls-submitted'], // 定义事件
  props: {
    showControls: {
      type: Boolean,
      default: false,
    },
    checkboxes: {
      type: Array as () => Array<{ label: string; state: boolean }>,
      default: () => [],
    },
  },
  setup(props, { emit }) {
    const filePath = ref(''); // 文件路径
    const isDragOver = ref(false); // 是否在拖拽状态
    const dropMessage = ref('释放文件以选择'); // 拖拽时的提示信息

    // 处理文件选择
    const handleClick = async () => {
      const selected = await open({
        multiple: false,
        filters: [{ name: '日志文件', extensions: ['txt', 'log'] }, { name: '所有文件', extensions: ['*'] }],
      });
      if (selected) {
        filePath.value = selected; // 获取文件路径
        emit('file-selected', filePath.value); // 触发事件
      }
    };

    // 处理按钮点击
    const handleButtonClick = () => {
      const checkboxStates = props.checkboxes.map((checkbox) => ({
        label: checkbox.label,
        state: checkbox.state,
      }));
      emit('controls-submitted', {
        filePath: filePath.value,
        checkboxes: checkboxStates,
      });
    };

    // 监听文件拖放事件
    onMounted(async () => {
      const webview = getCurrentWebview();
      await webview.onDragDropEvent((event) => {
        if (event.payload.type === 'over') {
          // 拖拽到窗口上方时
          isDragOver.value = true;
        } else if (event.payload.type === 'drop') {
          // 拖拽完成时
          isDragOver.value = false;
          const files = event.payload.paths;
          if (files && files.length > 0) {
            filePath.value = files[0]; // 获取文件路径
            emit('file-selected', filePath.value); // 触发事件
          }
        } else {
          // 拖拽取消时
          isDragOver.value = false;
        }
      });
    });

    return {
      filePath,
      isDragOver,
      dropMessage,
      handleClick,
      handleButtonClick,
    };
  },
});
</script>

<style scoped>
.drop-zone-container {
  position: relative;
}

.drop-zone {
  border: 2px dashed #ccc;
  border-radius: 10px;
  padding: 20px;
  text-align: center;
  cursor: pointer;
  margin-bottom: 20px;
  transition: border-color 0.3s, background-color 0.3s;
}

.drop-zone:hover {
  border-color: #4caf50;
  background-color: #f9f9f9;
}

.drop-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(255, 255, 255, 0.9);
  /* 半透明白色背景 */
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
  /* 确保覆盖层在最上层 */
  border: 4px dashed #4caf50;
  /* 拖拽时的边框 */
}

.drop-overlay p {
  font-size: 1.5rem;
  color: #333;
}

.controls {
  margin-top: 20px;
}

.checkbox-container {
  margin-bottom: 10px;
}

button {
  padding: 10px 20px;
  background-color: #4caf50;
  color: white;
  border: none;
  border-radius: 5px;
  cursor: pointer;
}

button:hover {
  background-color: #45a049;
}
</style>