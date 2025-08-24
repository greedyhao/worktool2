<!-- src/views/NetworkSpeedTest.vue -->
<template>
  <div class="app">
    <BackToHome />
    <h1>网络测速</h1>
    <div class="main-container">
      <!-- 配置面板 -->
      <div class="config-panel">
        <h2>测速配置</h2>
        <form @submit.prevent="startTest">
          <div class="form-group">
            <label>协议:</label>
            <select v-model="config.protocol">
              <option value="WebSocket">WebSocket</option>
              <option value="TCP">TCP</option>
              <option value="UDP">UDP</option>
            </select>
          </div>
          <div class="form-group">
            <label>测试模式:</label>
            <select v-model="config.mode">
              <option value="UploadOnly">仅测试设备上行速度</option>
              <!-- <option value="DownloadOnly">仅测试设备下行速度</option> -->
            </select>
          </div>
          <div class="form-group">
            <label>监听端口:</label>
            <input v-model.number="config.port" type="number" min="1" max="65535" />
          </div>
          <div class="form-group">
            <label>状态刷新间隔 (ms):</label>
            <input v-model.number="config.refreshInterval" type="number" min="100" max="10000" />
          </div>
          <div class="form-group">
            <label>测试数据包大小 (KB):</label>
            <input
              v-model.number="config.payloadSizeKb"
              type="number"
              min="1"
              max="10240"
            />
          </div>
          <div class="button-group">
            <button type="submit" :disabled="isTesting">
              {{ isTesting ? '停止服务器' : '启动服务器' }}
            </button>
          </div>
        </form>
      </div>
      <!-- 实时数据面板 -->
      <div class="data-panel">
        <h2>实时数据</h2>
        <div class="current-results">
          <div
            v-for="(result, index) in displayedResults"
            :key="index"
            class="result-item"
            :class="getResultClass(result)"
          >
            <h3>{{ result.protocol }} - {{ result.clientAddress }}</h3>
            <div class="metrics">
              <div class="metric">
                <span class="label">测试模式:</span>
                <span class="value">{{ result.mode === 'UploadOnly' ? '设备上行' : '设备下行' }}</span>
              </div>
              <div class="metric">
                <span class="label">速度:</span>
                <span class="value">{{ formatSpeed(result.speedMbps) }}</span>
              </div>
              <div class="metric message">
                <span class="label">状态:</span>
                <span class="value">{{ result.message }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <!-- 图表面板 -->
    <div class="chart-panel">
      <h2>历史数据图表</h2>
      <div ref="chartContainer" class="chart-container"></div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted, onUnmounted, nextTick } from 'vue'
import * as echarts from 'echarts'
import BackToHome from '../components/BackToHome.vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event' // 导入事件监听

// 配置数据
const config = reactive({
  protocol: 'WebSocket',
  mode: 'UploadOnly', // 默认模式
  port: 8765,
  refreshInterval: 1000,
  payloadSizeKb: 1024 // 默认 1MB
})
// 状态
const isTesting = ref(false)
const displayedResults = ref([]) // 显示最新的几条结果
const allResults = ref([]) // 存储所有接收到的结果
const chart = ref(null)
const chartContainer = ref(null)
let unlisten = null // 用于取消事件监听

// 图表数据结构
const chartData = reactive({
  timestamps: [],
  series: {
    speed: [] // 单一速度序列
  }
})

// 初始化图表
const initChart = () => {
  if (chartContainer.value) {
    chart.value = echarts.init(chartContainer.value)
    updateChart()
  }
}

// 更新图表
const updateChart = () => {
  if (!chart.value) return
  const option = {
    title: {
      text: '设备网络速度趋势图'
    },
    tooltip: {
      trigger: 'axis'
    },
    legend: {
       data: ['设备速度 KB/s']
    },
    xAxis: {
      type: 'category',
      data: chartData.timestamps // 修正：使用 data
    },
    yAxis: {
      type: 'value',
      name: '速度 (KB/s)'
    },
    series: [
      {
        name: '设备速度 KB/s',
        type: 'line',
        data: chartData.series.speed, // 修正：使用 data
        smooth: true,
        itemStyle: { color: '#4CAF50' },
        lineStyle: { color: '#4CAF50' }
      }
    ]
  }
  chart.value.setOption(option, true)
}

// 启动/停止服务器
const startTest = async () => {
  if (isTesting.value) {
    await stopTest()
    return
  }
  try {
    isTesting.value = true
    await invoke('start_speed_test', {
      config: {
        protocol: config.protocol,
        mode: config.mode,
        port: config.port,
        refreshInterval: config.refreshInterval,
        payloadSizeKb: config.payloadSizeKb
      }
    })
    console.log('测速服务器已启动')
    displayedResults.value = []
    allResults.value = []
    resetChartData()
  } catch (error) {
    console.error('启动测速服务器失败:', error)
    isTesting.value = false
  }
}

// 停止服务器
const stopTest = async () => {
  try {
    await invoke('stop_speed_test')
    isTesting.value = false
    console.log('测速服务器已停止')
    addResultToList({
      protocol: config.protocol,
      mode: config.mode,
      clientAddress: "Server",
      speedMbps: 0,
      latencyMs: 0,
      message: "服务器已停止",
      timestamp: Date.now()
    })
  } catch (error) {
    console.error('停止测速服务器失败:', error)
  }
}

// 格式化速度
const formatSpeed = (speed) => {
  if (speed < 0) return 'N/A'
  return `${speed.toFixed(2)} KB/s`
}

// 格式化延迟
const formatLatency = (latency) => {
  if (latency < 0) return 'N/A'
  return `${latency.toFixed(2)} ms`
}

// 获取结果项的 CSS 类
const getResultClass = (result) => {
  if (result.message.includes('失败') || result.message.includes('错误')) return 'result-error'
  if (result.message.includes('启动')) return 'result-info'
  if (result.message.includes('完成')) return 'result-success'
  return 'result-default'
}

// 重置图表数据
const resetChartData = () => {
  chartData.timestamps = []
  chartData.series.speed = []
  nextTick(() => {
    updateChart()
  })
}

// 添加数据到图表
const addDataToChart = (result) => {
  if (result.speedMbps >= 0) {
    const timestamp = new Date(result.timestamp).toLocaleTimeString()
    if (chartData.timestamps.length >= 50) {
      chartData.timestamps.shift()
      chartData.series.speed.shift()
    }
    chartData.timestamps.push(timestamp)
    chartData.series.speed.push(result.speedMbps >= 0 ? result.speedMbps : 0)
    nextTick(() => {
      updateChart()
    })
  }
}

// 添加结果到显示列表
const addResultToList = (result) => {
  allResults.value.push(result)
  if (displayedResults.value.length >= 10) {
    displayedResults.value.shift()
  }
  displayedResults.value.push(result)
  addDataToChart(result)
}

onMounted(async () => {
  initChart()
  window.addEventListener('resize', () => {
    if (chart.value) {
      chart.value.resize()
    }
  })

  // 监听来自 Rust 的事件
  try {
    unlisten = await listen('speed-test-result', (event) => {
      console.log('Received speed test result:', event.payload);
      addResultToList(event.payload); // event.payload 就是 SpeedTestResult 结构
    });
    console.log('Event listener registered.');
  } catch (error) {
    console.error('Failed to register event listener:', error);
  }
})

onUnmounted(() => {
  if (unlisten) {
    unlisten(); // 取消事件监听
    console.log('Event listener unregistered.');
  }
  if (chart.value) {
    chart.value.dispose()
  }
  if (isTesting.value) {
    stopTest() // 确保在组件卸载时停止服务器
  }
})
</script>


<style scoped>
.app {
  text-align: center;
  padding: 20px;
  padding-top: 80px;
  font-family: Arial, sans-serif;
  background-color: #f0f2f5;
  min-height: 100vh;
  box-sizing: border-box;
}

h1 {
  color: #333;
  margin-bottom: 30px;
}

.main-container {
  display: grid;
  grid-template-columns: 1fr 2fr;
  gap: 20px;
  margin-bottom: 30px;
  align-items: start;
}

@media (max-width: 768px) {
  .main-container {
    grid-template-columns: 1fr;
  }
}

.config-panel,
.chart-panel {
  background: white;
  padding: 20px;
  border-radius: 10px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  text-align: left;
}

.config-panel h2,
.chart-panel h2 {
  margin-top: 0;
  color: #333;
  border-bottom: 2px solid #4CAF50;
  padding-bottom: 10px;
}

.form-group {
  margin-bottom: 20px;
}

.form-group label {
  display: block;
  margin-bottom: 8px;
  font-weight: bold;
  color: #555;
}

.form-group input,
.form-group select {
  width: 100%;
  padding: 10px;
  border: 2px solid #ddd;
  border-radius: 6px;
  box-sizing: border-box;
  font-size: 14px;
  transition: border-color 0.3s;
}

.form-group input:focus,
.form-group select:focus {
  outline: none;
  border-color: #4CAF50;
}

.checkbox-group label {
  font-weight: normal;
  display: flex;
  align-items: center;
  cursor: pointer;
}

.checkbox-group input[type="checkbox"] {
  width: auto;
  margin-right: 10px;
}

.button-group {
  display: flex;
  gap: 15px;
  margin-top: 30px;
}

.button-group button {
  flex: 1;
  padding: 12px 20px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 16px;
  font-weight: bold;
  transition: all 0.3s;
}

.button-group button:first-child {
  background: #4CAF50;
  color: white;
}

.button-group button:first-child:hover:not(:disabled) {
  background: #45a049;
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(76, 175, 80, 0.3);
}

.button-group button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none !important;
  box-shadow: none !important;
}

.data-panel {
  background: white;
  padding: 20px;
  border-radius: 10px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  text-align: left;
  /* 添加下面两行来控制 data-panel 本身的高度 */
  display: flex;
  flex-direction: column;
  height: 100%; /* 或者根据需要设置一个具体的高度，例如 calc(100vh - 200px) */
}

.data-panel h2 {
  margin-top: 0;
  color: #333;
  border-bottom: 2px solid #4CAF50;
  padding-bottom: 10px;
  flex-shrink: 0; /* 防止标题在空间不足时被压缩 */
}

.current-results {
  display: flex;
  flex-direction: column;
  gap: 15px;
  /* --- 新增的样式 --- */
  max-height: 400px; /* 设置一个合适的最大高度 */
  overflow-y: auto;   /* 超出时显示垂直滚动条 */
  padding-right: 5px; /* 可选：给滚动条留点空间 */
  /* ------------------ */
  /* 移除或注释掉下面这行，因为滚动由 overflow-y: auto 控制 */
  /* overflow: hidden; */
}

.result-item {
  background: #f8f9fa;
  padding: 20px;
  border-radius: 8px;
  border-left: 4px solid #ddd;
  transition: transform 0.2s, box-shadow 0.2s;
}

.result-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.result-item.result-success {
  border-left-color: #4CAF50;
}

.result-item.result-error {
  border-left-color: #f44336;
  background-color: #ffebee;
}

.result-item.result-info {
  border-left-color: #2196F3;
  background-color: #e3f2fd;
}

.result-item.result-default {
  border-left-color: #FF9800;
}

.result-item h3 {
  margin: 0 0 15px 0;
  color: #333;
  font-size: 1.1rem;
}

.metrics {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.metric {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 6px 0;
  border-bottom: 1px solid #eee;
}

.metric:last-child {
  border-bottom: none;
}

.metric.message {
  border-bottom: none;
  padding-top: 10px;
}

.metric .label {
  font-weight: 600;
  color: #666;
  flex: 1;
  text-align: left;
}

.metric .value {
  font-weight: bold;
  font-size: 1em;
  flex: 1;
  text-align: right;
}

.result-item.result-success .metric .value {
  color: #4CAF50;
}

.result-item.result-error .metric .value {
  color: #f44336;
}

.result-item.result-info .metric .value {
  color: #2196F3;
}

.result-item.result-default .metric .value {
  color: #FF9800;
}

.chart-container {
  width: 100%;
  height: 400px;
}
</style>
