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
      <!-- 动态生成复选框，横向排列并自动换行 -->
      <div class="checkbox-container-wrapper" :class="{ collapsed: isCollapsed }">
        <div class="checkbox-row" ref="checkboxRow">
          <div
            v-for="(checkbox, index) in checkboxes"
            :key="`checkbox-${index}`"
            class="checkbox-container"
            :class="{ active: checkbox.state }"
            @click="toggleCheckbox(checkbox)"
          >
            {{ checkbox.label }}
          </div>
        </div>
      </div>

      <!-- 展开/折叠按钮，仅在行数超过 3 行时显示 -->
      <button
        v-if="showToggleButton"
        class="toggle-button"
        @click="toggleCollapse"
      >
        {{ isCollapsed ? '展开' : '折叠' }}
      </button>

      <!-- 动态生成数字输入框 -->
      <div class="number-input-row">
        <div
          v-for="(input, index) in numberInputs"
          :key="`number-input-${index}`"
          class="number-input-container"
        >
          <label>{{ input.label }}</label>
          <input
            type="number"
            v-model="input.value"
            @input="handleNumberInputChange(input)"
          />
        </div>
      </div>

      <!-- 动态生成按键，横向排列 -->
      <div class="button-row">
        <button
          v-for="(button, index) in buttons"
          :key="`button-${index}`"
          @click="handleButtonClick(button)"
        >
          {{ button.label }}
        </button>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, watch, nextTick } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { getCurrentWebview } from '@tauri-apps/api/webview';

export default defineComponent({
  name: 'FileDropZone',
  emits: ['file-selected', 'button-clicked'], // 定义事件
  props: {
    showControls: {
      type: Boolean,
      default: false,
    },
    checkboxes: {
      type: Array as () => Array<{ label: string; state: boolean }>,
      default: () => [],
    },
    buttons: {
      type: Array as () => Array<{ label: string; id: string }>,
      default: () => [],
    },
    numberInputs: {
      type: Array as () => Array<{ label: string; value: number }>,
      default: () => [],
    },
  },
  setup(props, { emit }) {
    const filePath = ref(''); // 文件路径
    const isDragOver = ref(false); // 是否在拖拽状态
    const dropMessage = ref('释放文件以选择'); // 拖拽时的提示信息
    const isCollapsed = ref(true); // 是否折叠
    const checkboxRow = ref<HTMLElement | null>(null); // 复选框容器引用
    const showToggleButton = ref(false); // 是否显示展开/折叠按钮

    // 计算复选框的行数
    const calculateRowCount = () => {
      if (checkboxRow.value) {
        const rowHeight = 50; // 每行的近似高度（根据实际样式调整）
        const totalHeight = checkboxRow.value.clientHeight;
        return Math.ceil(totalHeight / rowHeight);
      }
      return 0;
    };

    // 更新是否显示展开/折叠按钮
    const updateToggleButtonVisibility = () => {
      nextTick(() => {
        const rowCount = calculateRowCount();
        showToggleButton.value = rowCount > 3;
      });
    };

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

    // 处理按键点击
    const handleButtonClick = (button: { label: string; id: string }) => {
      const checkboxStates = props.checkboxes.map((checkbox) => ({
        label: checkbox.label,
        state: checkbox.state,
      }));

      const numberInputValues = props.numberInputs.map((input) => ({
        label: input.label,
        value: input.value,
      }));

      emit('button-clicked', {
        buttonId: button.id,
        filePath: filePath.value,
        checkboxes: checkboxStates,
        numberInputs: numberInputValues,
      });
    };

    // 切换复选框状态
    const toggleCheckbox = (checkbox: { label: string; state: boolean }) => {
      checkbox.state = !checkbox.state;
    };

    // 切换折叠状态
    const toggleCollapse = () => {
      isCollapsed.value = !isCollapsed.value;
    };

    // 处理数字输入框的输入事件
    const handleNumberInputChange = (input: { label: string; value: number }) => {
      console.log(`Input ${input.label} changed to ${input.value}`);
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

      // 初始化时更新按钮可见性
      updateToggleButtonVisibility();
    });

    // 监听复选框数量变化
    watch(
      () => props.checkboxes.length,
      () => {
        updateToggleButtonVisibility();
      }
    );

    return {
      filePath,
      isDragOver,
      dropMessage,
      isCollapsed,
      checkboxRow,
      showToggleButton,
      handleClick,
      handleButtonClick,
      toggleCheckbox,
      toggleCollapse,
      handleNumberInputChange,
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

/* 复选框容器 */
.checkbox-container-wrapper {
  max-height: 150px; /* 3 行的高度 */
  overflow: hidden;
  transition: max-height 0.3s ease;
}

.checkbox-container-wrapper.collapsed {
  max-height: 150px; /* 折叠时的高度 */
}

.checkbox-container-wrapper:not(.collapsed) {
  max-height: none; /* 移除高度限制 */
  overflow: auto; /* 允许滚动 */
}

/* 复选框横向排列并自动换行 */
.checkbox-row {
  display: flex;
  flex-wrap: wrap; /* 允许换行 */
  gap: 10px; /* 复选框之间的间距 */
  margin-bottom: 20px;
}

.checkbox-container {
  flex: 1 1 auto; /* 允许复选框根据内容宽度调整 */
  min-width: 150px; /* 每个复选框的最小宽度 */
  max-width: 200px; /* 每个复选框的最大宽度 */
  padding: 10px;
  border: 1px solid #ccc;
  border-radius: 5px;
  text-align: center;
  cursor: pointer;
  transition: background-color 0.3s, border-color 0.3s;
}

.checkbox-container.active {
  background-color: #4caf50; /* 选中时的背景色 */
  color: white; /* 选中时的文字颜色 */
  border-color: #4caf50; /* 选中时的边框颜色 */
}

/* 展开/折叠按钮 */
.toggle-button {
  display: block;
  width: 100%;
  padding: 10px;
  background-color: #f0f0f0;
  border: 1px solid #ccc;
  border-radius: 5px;
  text-align: center;
  cursor: pointer;
  margin-bottom: 20px;
}

.toggle-button:hover {
  background-color: #ddd;
}

/* 按键横向排列 */
.button-row {
  display: flex;
  gap: 10px; /* 按键之间的间距 */
}

button {
  padding: 10px 20px;
  background-color: #4caf50;
  color: white;
  border: none;
  border-radius: 5px;
  cursor: pointer;
  flex: 1; /* 按键均匀分布 */
}

button:hover {
  background-color: #45a049;
}

/* 数字输入框样式 */
.number-input-row {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  margin-bottom: 20px;
}

.number-input-container {
  flex: 1 1 auto;
  min-width: 150px;
  max-width: 200px;
  padding: 10px;
  border: 1px solid #ccc;
  border-radius: 5px;
  text-align: center;
}

.number-input-container label {
  display: block;
  margin-bottom: 5px;
}

.number-input-container input {
  width: 100%;
  padding: 5px;
  border: 1px solid #ccc;
  border-radius: 3px;
}
</style>