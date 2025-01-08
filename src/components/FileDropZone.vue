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
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { getCurrentWebview } from '@tauri-apps/api/webview';

export default defineComponent({
  name: 'FileDropZone',
  emits: ['file-selected'], // 定义事件
  setup(_, { emit }) {
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
</style>