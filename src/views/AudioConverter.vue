<template>
    <BackToHome />
    <h1>音频格式转换</h1>

    <!-- 文件选择区域 -->
    <FileDropZone :showControls="true" :buttons="buttonOptions" :checkboxes="checkboxOptions"
        :numberInputs="numberInputs" @button-clicked="handleButtonClicked" />
    <!-- 输入流配置弹窗 -->
    <div v-if="showInputDialog" class="modal-overlay">
        <div class="modal">
            <h3>输入流配置</h3>
            <p>检测到文件为 .opus/.raw/.pcm 或无后缀，请指定输入流类型和参数。</p>

            <!-- 流类型选择 -->
            <div class="input-row">
                <label>输入流类型：</label>
                <select v-model="inputStreamType">
                    <option value="opus">OPUS 流</option>
                    <option value="pcm">PCM 流</option>
                </select>
            </div>

            <!-- OPUS 配置 -->
            <div v-if="inputStreamType === 'opus'" class="config-section">
                <h4>OPUS 流参数</h4>
                <div class="input-row">
                    <label>采样率 (Hz):</label>
                    <select v-model="inputOpusConfig.sampleRate">
                        <option value="8000">8000</option>
                        <option value="12000">12000</option>
                        <option value="16000">16000</option>
                        <option value="24000">24000</option>
                        <option value="48000">48000</option>
                    </select>
                </div>
                <div class="input-row">
                    <label>比特率 (bps):</label>
                    <select v-model="inputOpusConfig.bitrate">
                        <option value="6000">6000</option>
                        <option value="8000">8000</option>
                        <option value="16000">16000</option>
                        <option value="24000">24000</option>
                        <option value="32000">32000</option>
                        <option value="48000">48000</option>
                        <option value="64000">64000</option>
                    </select>
                </div>
                <div class="input-row">
                    <label>帧长 (ms):</label>
                    <select v-model="inputOpusConfig.frameSize">
                        <option value="2.5">2.5</option>
                        <option value="5">5</option>
                        <option value="10">10</option>
                        <option value="20">20</option>
                        <option value="40">40</option>
                        <option value="60">60</option>
                    </select>
                </div>
            </div>

            <!-- PCM 配置 -->
            <div v-if="inputStreamType === 'pcm'" class="config-section">
                <h4>PCM 流参数</h4>
                <div class="input-row">
                    <label>采样率 (Hz):</label>
                    <select v-model="inputPcmConfig.sampleRate">
                        <option value="8000">8000</option>
                        <option value="11025">11025</option>
                        <option value="16000">16000</option>
                        <option value="22050">22050</option>
                        <option value="32000">32000</option>
                        <option value="44100">44100</option>
                        <option value="48000">48000</option>
                    </select>
                </div>
                <div class="input-row">
                    <label>样点格式：</label>
                    <select v-model="inputPcmConfig.sampleFormat">
                        <option v-for="fmt in pcmFormats" :key="fmt" :value="fmt">{{ fmt }}</option>
                    </select>
                </div>
            </div>

            <!-- 按钮 -->
            <div class="modal-buttons">
                <button @click="cancelInputConfig" class="btn-cancel">取消</button>
                <button @click="confirmInputConfig" class="btn-confirm">确定并开始转换</button>
            </div>
        </div>
    </div>

    <!-- 输出配置区域 -->
    <div class="output-config" v-if="showOutputConfig">
        <h3>输出配置</h3>

        <!-- 音频格式选择 -->
        <div class="format-selection">
            <label for="audioFormat">输出格式:</label>
            <select id="audioFormat" v-model="selectedFormat" @change="onFormatChange">
                <option value="wav">WAV</option>
                <option value="opus">OPUS</option>
                <option value="raw">RAW</option>
            </select>
        </div>

        <!-- 动态配置项 -->
        <div class="dynamic-config">
            <!-- OPUS 配置 -->
            <div v-if="selectedFormat === 'opus'" class="format-config">
                <div class="config-row">
                    <div class="config-item">
                        <label>采样率 (Hz):</label>
                        <select v-model="opusConfig.sampleRate">
                            <option value="8000">8000</option>
                            <option value="12000">12000</option>
                            <option value="16000">16000</option>
                            <option value="24000">24000</option>
                            <option value="48000">48000</option>
                        </select>
                    </div>
                    <div class="config-item">
                        <label>码率 (bps):</label>
                        <select v-model="opusConfig.bitrate">
                            <option value="6000">6000</option>
                            <option value="8000">8000</option>
                            <option value="16000">16000</option>
                            <option value="24000">24000</option>
                            <option value="32000">32000</option>
                            <option value="48000">48000</option>
                            <option value="64000">64000</option>
                            <option value="96000">96000</option>
                            <option value="128000">128000</option>
                            <option value="192000">192000</option>
                            <option value="256000">256000</option>
                            <option value="auto">auto</option>
                        </select>
                    </div>
                </div>
                <div class="config-row">
                    <div class="config-item">
                        <label>帧长 (ms):</label>
                        <select v-model="opusConfig.frameSize">
                            <option value="2.5">2.5</option>
                            <option value="5">5</option>
                            <option value="10">10</option>
                            <option value="20">20</option>
                            <option value="40">40</option>
                            <option value="60">60</option>
                        </select>
                    </div>
                    <div class="config-item">
                        <label>VBR:</label>
                        <select v-model="opusConfig.vbr">
                            <option value="on">开启</option>
                            <option value="off">关闭</option>
                        </select>
                    </div>
                </div>
            </div>

            <!-- WAV 配置 -->
            <div v-if="selectedFormat === 'wav'" class="format-config">
                <div class="config-row">
                    <div class="config-item">
                        <label>采样率 (Hz):</label>
                        <select v-model="wavConfig.sampleRate">
                            <option value="8000">8000</option>
                            <option value="16000">16000</option>
                            <option value="32000">32000</option>
                            <option value="48000">48000</option>
                            <option value="other">其他</option>
                        </select>
                    </div>
                    <div class="config-item" v-if="wavConfig.sampleRate === 'other'">
                        <label>自定义采样率:</label>
                        <input type="number" v-model="wavConfig.customSampleRate" placeholder="输入采样率" />
                    </div>
                </div>
                <div class="config-row">
                    <div class="config-item">
                        <label>编码:</label>
                        <select v-model="wavConfig.encoding">
                            <option value="s16">S16 (16位有符号)</option>
                            <option value="s24">S24 (24位有符号)</option>
                            <option value="s32">S32 (32位有符号)</option>
                            <option value="u8">U8 (8位无符号)</option>
                        </select>
                    </div>
                </div>
            </div>

            <!-- RAW 配置 -->
            <div v-if="selectedFormat === 'raw'" class="format-config">
                <div class="config-row">
                    <div class="config-item">
                        <label>采样率 (Hz):</label>
                        <select v-model="rawConfig.sampleRate">
                            <option value="8000">8000</option>
                            <option value="16000">16000</option>
                            <option value="32000">32000</option>
                            <option value="48000">48000</option>
                            <option value="other">其他</option>
                        </select>
                    </div>
                    <div class="config-item" v-if="rawConfig.sampleRate === 'other'">
                        <label>自定义采样率:</label>
                        <input type="number" v-model="rawConfig.customSampleRate" placeholder="输入采样率" />
                    </div>
                </div>
                <div class="config-row">
                    <div class="config-item">
                        <label>编码:</label>
                        <select v-model="rawConfig.encoding">
                            <option value="s16">S16 (16位有符号)</option>
                            <option value="s24">S24 (24位有符号)</option>
                            <option value="s32">S32 (32位有符号)</option>
                            <option value="u8">U8 (8位无符号)</option>
                        </select>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import BackToHome from '@/components/BackToHome.vue';
import FileDropZone from '@/components/FileDropZone.vue';
import { invoke } from '@tauri-apps/api/core';

interface OpusConfig {
    sampleRate: string;
    bitrate: string;
    frameSize: string;
    vbr: string;
}

interface WavConfig {
    sampleRate: string;
    customSampleRate: number;
    encoding: string;
}

interface RawConfig {
    sampleRate: string;
    customSampleRate: number;
    encoding: string;
}

interface AudioConvertOptions {
    format: string;
    config: OpusConfig | WavConfig | RawConfig;
}

export default defineComponent({
    name: 'AudioConverter',
    components: {
        BackToHome,
        FileDropZone,
    },
    setup() {
        // ===== 输入流配置弹窗控制 =====
        const showInputDialog = ref(false);
        const inputStreamType = ref<'opus' | 'pcm'>('opus');

        // OPUS 输入流参数
        const inputOpusConfig = ref({
            sampleRate: '16000',
            bitrate: '16000',
            frameSize: '20', // 注意：这里应为字符串以匹配 select
        });

        // PCM 输入流参数
        const inputPcmConfig = ref({
            sampleRate: '16000',
            sampleFormat: 's16le',
        });

        // 支持的 PCM 格式
        const pcmFormats = ['s16le', 's24le', 's32le', 'u8', 'f32le'];

        // 检查是否需要输入参数
        const checkNeedInputConfig = (filePath: string): boolean => {
            const ext = filePath.split('.').pop()?.toLowerCase();
            return ['opus', 'raw', 'pcm', ''].includes(ext ?? '');
        };

        // ===== 输出配置 =====
        const selectedFormat = ref('opus');
        const showOutputConfig = ref(true);

        // 不同格式的配置
        const opusConfig = ref<OpusConfig>({
            sampleRate: '16000',
            bitrate: '16000',
            frameSize: '20',
            vbr: 'off'
        });

        const wavConfig = ref<WavConfig>({
            sampleRate: '16000',
            customSampleRate: 16000,
            encoding: 's16'
        });

        const rawConfig = ref<RawConfig>({
            sampleRate: '16000',
            customSampleRate: 16000,
            encoding: 's16'
        });

        const buttonOptions = ref([
            { label: '开始转换', id: 'convert' },
        ]);

        // 这些是 FileDropZone 需要的，但我们主要使用自定义配置区域
        const checkboxOptions = ref([]);
        const numberInputs = ref([]);

        const onFormatChange = () => {
            console.log('音频格式已更改为:', selectedFormat.value);
        };

        const getCurrentConfig = () => {
            switch (selectedFormat.value) {
                case 'opus':
                    return opusConfig.value;
                case 'wav':
                    return {
                        ...wavConfig.value,
                        sampleRate: wavConfig.value.sampleRate === 'other'
                            ? wavConfig.value.customSampleRate.toString()
                            : wavConfig.value.sampleRate
                    };
                case 'raw':
                    return {
                        ...rawConfig.value,
                        sampleRate: rawConfig.value.sampleRate === 'other'
                            ? rawConfig.value.customSampleRate.toString()
                            : rawConfig.value.sampleRate
                    };
                default:
                    return {};
            }
        };

        const handleButtonClicked = async (data: {
            buttonId: string;
            filePath: string;
            checkboxes: Array<{ label: string; state: boolean }>;
            numberInputs: Array<{ label: string; value: number }>;
        }) => {
            if (data.buttonId !== 'convert') return;

            if (!data.filePath) {
                alert('请先选择音频文件');
                return;
            }

            const requiresInputConfig = checkNeedInputConfig(data.filePath);

            if (requiresInputConfig) {
                // 弹出输入配置对话框
                showInputDialog.value = true;

                // 这里我们返回，等待用户在对话框中确认
                // 但我们不能在这里 await，所以需要把后续逻辑移到确认按钮
                return;
            }

            // 不需要输入配置，直接转换
            await performConversion(data.filePath, null);
        };

        // 执行转换（抽离为独立函数）
        const performConversion = async (
            filePath: string,
            inputConfig: { type: 'opus' | 'pcm'; config: any } | null
        ) => {
            try {
                const options: AudioConvertOptions = {
                    format: selectedFormat.value,
                    config: getCurrentConfig() as any,
                };

                console.log('转换参数:', {
                    filePath,
                    inputConfig, // 传给后端的输入流信息
                    options,
                });

                await invoke('convert_audio', {
                    filePath,
                    options,
                    inputConfig, // 假设后端 API 支持 inputConfig 参数
                });
                alert('音频转换完成！');
            } catch (error) {
                console.error('转换失败:', error);
                alert(`转换失败: ${error}`);
            } finally {
                showInputDialog.value = false;
            }
        };

        // 输入配置对话框的确认处理
        const confirmInputConfig = () => {
            const config =
                inputStreamType.value === 'opus'
                    ? {
                        type: 'opus' as const,
                        config: {
                            sampleRate: parseInt(inputOpusConfig.value.sampleRate),
                            bitrate: parseInt(inputOpusConfig.value.bitrate),
                            frameSize: parseInt(inputOpusConfig.value.frameSize),
                        },
                    }
                    : {
                        type: 'pcm' as const,
                        config: {
                            sampleRate: parseInt(inputPcmConfig.value.sampleRate),
                            sampleFormat: inputPcmConfig.value.sampleFormat,
                        },
                    };

            // 关闭弹窗并执行转换
            performConversion(filePathForConversion.value, config);
        };

        // 用于保存待转换的文件路径
        const filePathForConversion = ref('');

        // 取消输入配置
        const cancelInputConfig = () => {
            showInputDialog.value = false;
        };

        return {
            // 输出配置
            selectedFormat,
            showOutputConfig,
            opusConfig,
            wavConfig,
            rawConfig,
            buttonOptions,
            checkboxOptions,
            numberInputs,
            onFormatChange,
            handleButtonClicked,

            // 输入配置弹窗
            showInputDialog,
            inputStreamType,
            inputOpusConfig,
            inputPcmConfig,
            pcmFormats,
            confirmInputConfig,
            cancelInputConfig,
            filePathForConversion,
        };
    }
});
</script>

<style scoped>
/* 模态框背景 */
.modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 2000;
}

/* 模态框主体 */
.modal {
    background: white;
    padding: 20px;
    border-radius: 10px;
    width: 90%;
    max-width: 500px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
}

.modal h3 {
    margin-top: 0;
    color: #333;
}

.modal p {
    color: #666;
    font-size: 0.9rem;
    margin-bottom: 15px;
}

.input-row {
    margin-bottom: 15px;
    display: flex;
    align-items: center;
    gap: 10px;
}

.input-row label {
    flex: 0 0 90px;
    text-align: right;
    font-weight: bold;
    color: #555;
}

.input-row select {
    flex: 1;
    padding: 8px;
    border: 1px solid #ccc;
    border-radius: 5px;
}

.config-section {
    border: 1px solid #eee;
    padding: 15px;
    border-radius: 8px;
    background-color: #f9f9f9;
    margin-bottom: 15px;
}

.config-section h4 {
    margin-top: 0;
    margin-bottom: 10px;
    color: #333;
}

.modal-buttons {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 20px;
}

.btn-cancel {
    padding: 8px 16px;
    background: #ccc;
    border: none;
    border-radius: 5px;
    cursor: pointer;
}

.btn-cancel:hover {
    background: #bbb;
}

.btn-confirm {
    padding: 8px 16px;
    background: #4caf50;
    color: white;
    border: none;
    border-radius: 5px;
    cursor: pointer;
}

.btn-confirm:hover {
    background: #45a049;
}

.app {
    text-align: center;
    padding: 20px;
    padding-top: 80px;
}

.output-config {
    max-width: 800px;
    margin: 20px auto;
    padding: 20px;
    border: 1px solid #ddd;
    border-radius: 10px;
    background-color: #f9f9f9;
}

.output-config h3 {
    margin-bottom: 20px;
    color: #333;
}

.format-selection {
    margin-bottom: 30px;
}

.format-selection label {
    display: inline-block;
    margin-right: 10px;
    font-weight: bold;
}

.format-selection select {
    padding: 8px 12px;
    border: 1px solid #ccc;
    border-radius: 5px;
    font-size: 16px;
    min-width: 120px;
}

.dynamic-config {
    margin-top: 20px;
}

.format-config {
    padding: 20px;
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    background-color: white;
}

.config-row {
    display: flex;
    flex-wrap: wrap;
    gap: 20px;
    margin-bottom: 15px;
}

.config-item {
    flex: 1;
    min-width: 200px;
    max-width: 300px;
}

.config-item label {
    display: block;
    margin-bottom: 5px;
    font-weight: bold;
    color: #555;
}

.config-item select,
.config-item input {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid #ccc;
    border-radius: 5px;
    font-size: 14px;
    box-sizing: border-box;
}

.config-item select:focus,
.config-item input:focus {
    outline: none;
    border-color: #4caf50;
    box-shadow: 0 0 5px rgba(76, 175, 80, 0.3);
}

.config-item input[type="number"] {
    text-align: right;
}

/* 响应式设计 */
@media (max-width: 600px) {
    .config-row {
        flex-direction: column;
    }

    .config-item {
        max-width: none;
    }
}
</style>