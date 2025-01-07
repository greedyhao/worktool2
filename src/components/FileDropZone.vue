<template>
  <div class="drop-zone" @dragover.prevent="handleDragOver" @drop.prevent="handleDrop" @click="handleClick">
    <p v-if="!filePath">将文件拖放到此处，或点击选择文件</p>
    <p v-else>已选择文件：{{ filePath }}</p>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';

export default defineComponent({
  name: 'FileDropZone',
  emits: ['file-selected'], // 定义事件
  setup(props, { emit }) {
    const filePath = ref(''); // 文件路径

    // 处理文件拖放
    const handleDrop = async (event: DragEvent) => {
      const files = event.dataTransfer?.files;
      if (files && files.length > 0) {
        const file = files[0];
        filePath.value = file.path; // 获取文件路径
        emit('file-selected', filePath.value); // 触发事件
      }
    };

    // 处理文件选择
    const handleClick = async () => {
      const selected = await open({
        multiple: false,
        filters: [{ name: '日志文件', extensions: ['log'] }],
      });
      if (typeof selected === 'string') {
        filePath.value = selected; // 获取文件路径
        emit('file-selected', filePath.value); // 触发事件
      }
    };

    return {
      filePath,
      handleDrop,
      handleClick,
    };
  },
});
</script>

<style scoped>
.drop-zone {
  border: 2px dashed #ccc;
  border-radius: 10px;
  padding: 20px;
  text-align: center;
  cursor: pointer;
  margin-bottom: 20px;
}

.drop-zone:hover {
  border-color: #4caf50;
  background-color: #f9f9f9;
}
</style>