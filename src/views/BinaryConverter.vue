<template>
    <div class="binary-converter">
        <BackToHome />
        <h1>进制转换与二进制位选择工具</h1>
        <p class="subtitle">支持十六进制、十进制、八进制和二进制之间的转换</p>

        <div class="converter-card">
            <div class="converter-grid">
                <div class="input-group">
                    <label class="input-label">十六进制 (0x)</label>
                    <div class="input-wrapper">
                        <span class="prefix">0x</span>
                        <input v-model="inputValues.hex" @input="handleHexInput"
                            @keypress="validateInput($event, 'hex')" placeholder="输入十六进制数值" class="converter-input">
                    </div>
                    <div class="input-hint">允许字符: 0-9, A-F</div>
                </div>

                <div class="input-group">
                    <label class="input-label">十进制</label>
                    <div class="input-wrapper">
                        <input v-model="inputValues.dec" @input="handleDecInput"
                            @keypress="validateInput($event, 'dec')" placeholder="输入十进制数值"
                            class="converter-input full-input">
                    </div>
                    <div class="input-hint">允许字符: 0-9</div>
                </div>

                <div class="input-group">
                    <label class="input-label">八进制 (0o)</label>
                    <div class="input-wrapper">
                        <span class="prefix">0o</span>
                        <input v-model="inputValues.oct" @input="handleOctInput"
                            @keypress="validateInput($event, 'oct')" placeholder="输入八进制数值" class="converter-input">
                    </div>
                    <div class="input-hint">允许字符: 0-7</div>
                </div>

                <div class="input-group">
                    <label class="input-label">二进制 (0b)</label>
                    <div class="input-wrapper">
                        <span class="prefix">0b</span>
                        <input v-model="inputValues.bin" @input="handleBinInput"
                            @keypress="validateInput($event, 'bin')" placeholder="输入二进制数值" class="converter-input">
                    </div>
                    <div class="input-hint">允许字符: 0, 1</div>
                </div>
            </div>
        </div>

        <div class="binary-display-card">
            <h2>二进制位展开 (每行16位)</h2>
            <div class="bit-container">
                <!-- 按行分组显示二进制位 -->
                <div v-for="(group, groupIndex) in bitGroups" :key="'group-' + groupIndex" class="bit-group">
                    <!-- 二进制位显示 -->
                    <div class="bit-row bit-value-row">
                        <span v-for="(bit, bitIndex) in group.bits" :key="'bit-' + groupIndex + '-' + bitIndex" class="bit"
                            :class="{ selected: isBitSelected(group.startIndex + bitIndex) }"
                            @mousedown="startSelection(group.startIndex + bitIndex)"
                            @mouseenter="dragSelection(group.startIndex + bitIndex)" @mouseup="endSelection">
                            {{ bit }}
                        </span>
                    </div>

                    <!-- 位号标记 -->
                    <div class="bit-row">
                        <span v-for="(_bit, bitIndex) in group.bits" :key="'label-' + groupIndex + '-' + bitIndex"
                            class="bit-label">
                            {{ totalBits - (group.startIndex + bitIndex) - 1 }}
                        </span>
                    </div>
                </div>
            </div>

            <div v-if="selectedBitsValue" class="selection-info">
                <div class="value-display">
                    <div class="value-title">选中位的值</div>
                    <div class="value-content">{{ selectedBitsValue.dec }} (十进制)</div>
                </div>
                <div class="value-display">
                    <div class="value-title">十六进制</div>
                    <div class="value-content">0x{{ selectedBitsValue.hex }}</div>
                </div>
                <div class="value-display">
                    <div class="value-title">二进制</div>
                    <div class="value-content">0b{{ selectedBitsValue.bin }}</div>
                </div>
                <div class="value-display">
                    <div class="value-title">位范围</div>
                    <div class="value-content">[{{ selectionRange[0] }}:{{ selectionRange[1] }}] ({{ selectedBitsCount
                        }}位)</div>
                </div>
            </div>
        </div>

        <div class="instructions-card">
            <h3>使用说明</h3>
            <ul>
                <li>在任意进制输入框中输入数值，其他进制会自动转换</li>
                <li>二进制区域会显示当前数值的二进制位展开，每行显示16位</li>
                <li><span class="highlight">鼠标拖动</span>可以选择连续的二进制位</li>
                <li>选中的位会以<span class="highlight">蓝色高亮</span>显示</li>
                <li>选中区域下方会显示选中位对应的数值</li>
                <li>位号标记从右向左，从0开始（最右边为最低位）</li>
                <li>超过16位的数据会自动换行显示</li>
            </ul>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import BackToHome from '@/components/BackToHome.vue'

// 当前数值
const currentValue = ref(0n)

// 输入框的值
const inputValues = ref({
    hex: '',
    dec: '',
    oct: '',
    bin: ''
})

// 位选择状态
const isSelecting = ref(false)
const selectionStart = ref<number | null>(null)
const selectionEnd = ref<number | null>(null)
const selectedBitsValue = ref<{
    dec: string
    hex: string
    bin: string
} | null>(null)

// 计算二进制字符串
const binStr = computed(() => {
    if (currentValue.value === 0n) return '0'
    return currentValue.value.toString(2)
})

// 计算总位数（补零到最近的16的倍数）
const totalBits = computed(() => {
    const length = binStr.value.length
    return Math.ceil(length / 16) * 16
})

// 计算补零后的二进制位数组
const bits = computed(() => {
    // 补零到总位数
    let padded = binStr.value.padStart(totalBits.value, '0')
    return padded.split('')
})

// 将二进制位按每16位分组
const bitGroups = computed(() => {
    const groups = []
    const bitsArray = bits.value

    for (let i = 0; i < bitsArray.length; i += 16) {
        const groupBits = bitsArray.slice(i, i + 16)
        groups.push({
            bits: groupBits,
            startIndex: i
        })
    }

    return groups
})

// 计算选中范围
const selectionRange = computed(() => {
    if (selectionStart.value === null || selectionEnd.value === null) return [0, 0]

    const startIndex = Math.min(selectionStart.value, selectionEnd.value)
    const endIndex = Math.max(selectionStart.value, selectionEnd.value)

    // 位号是从右向左递增（从0开始）
    const startBit = totalBits.value - startIndex - 1
    const endBit = totalBits.value - endIndex - 1

    return [Math.max(startBit, endBit), Math.min(startBit, endBit)]
})

// 计算选中位数
const selectedBitsCount = computed(() => {
    if (selectionStart.value === null || selectionEnd.value === null) return 0
    return Math.abs(selectionStart.value - selectionEnd.value) + 1
})

// 输入验证：只允许对应进制的字符
const validateInput = (event: KeyboardEvent, type: 'hex' | 'dec' | 'oct' | 'bin') => {
    const char = event.key.toLowerCase()

    // 允许控制键
    if (event.ctrlKey || event.metaKey ||
        ['Backspace', 'Delete', 'Tab', 'Enter', 'ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown'].includes(event.key)) {
        return
    }

    let validChars: string[] = []
    switch (type) {
        case 'hex':
            validChars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f']
            break
        case 'dec':
            validChars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
            break
        case 'oct':
            validChars = ['0', '1', '2', '3', '4', '5', '6', '7']
            break
        case 'bin':
            validChars = ['0', '1']
            break
    }

    if (!validChars.includes(char)) {
        event.preventDefault()
    }
}

// 处理十六进制输入
const handleHexInput = (event: Event) => {
    const target = event.target as HTMLInputElement
    const value = target.value
    updateValue(value, 16)
}

// 处理十进制输入
const handleDecInput = (event: Event) => {
    const target = event.target as HTMLInputElement
    const value = target.value
    updateValue(value, 10)
}

// 处理八进制输入
const handleOctInput = (event: Event) => {
    const target = event.target as HTMLInputElement
    const value = target.value
    updateValue(value, 8)
}

// 处理二进制输入
const handleBinInput = (event: Event) => {
    const target = event.target as HTMLInputElement
    const value = target.value
    updateValue(value, 2)
}

// 清理输入值：移除无效字符
const cleanInputValue = (value: string, type: 'hex' | 'dec' | 'oct' | 'bin'): string => {
    let validPattern: RegExp
    switch (type) {
        case 'hex':
            validPattern = /[^0-9a-fA-F]/g
            break
        case 'dec':
            validPattern = /[^0-9]/g
            break
        case 'oct':
            validPattern = /[^0-7]/g
            break
        case 'bin':
            validPattern = /[^01]/g
            break
        default:
            return value
    }
    return value.replace(validPattern, '')
}

// 更新数值
const updateValue = (newValue: string, base: number) => {
    try {
        // 确定输入类型并清理输入值
        let inputType: 'hex' | 'dec' | 'oct' | 'bin'
        switch (base) {
            case 16: inputType = 'hex'; break
            case 10: inputType = 'dec'; break
            case 8: inputType = 'oct'; break
            case 2: inputType = 'bin'; break
            default: return
        }

        // 清理输入值
        let cleanValue = cleanInputValue(newValue.trim().toUpperCase(), inputType)

        // 去除前缀
        if (base === 16 && cleanValue.startsWith('0X')) {
            cleanValue = cleanValue.substring(2)
        } else if (base === 8 && cleanValue.startsWith('0O')) {
            cleanValue = cleanValue.substring(2)
        } else if (base === 2 && cleanValue.startsWith('0B')) {
            cleanValue = cleanValue.substring(2)
        }

        // 空值处理
        if (cleanValue === '') {
            currentValue.value = 0n
            inputValues.value = { hex: '', dec: '', oct: '', bin: '' }
            resetSelection()
            return
        }

        // 转换为BigInt
        let newVal: bigint
        if (base === 10) {
            newVal = BigInt(cleanValue)
        } else {
            // 使用parseInt转换为十进制，再转BigInt
            const parsed = parseInt(cleanValue, base)
            if (isNaN(parsed)) {
                return // 无效输入，不更新
            }
            newVal = BigInt(parsed)
        }

        // 检查是否为负数
        if (newVal < 0n) {
            return // 不支持负数
        }

        currentValue.value = newVal

        // 更新其他输入框的值（只更新非当前输入的字段）
        const newValues = {
            hex: newVal.toString(16).toUpperCase(),
            dec: newVal.toString(10),
            oct: newVal.toString(8),
            bin: newVal.toString(2)
        }

        // 更新除当前输入框外的其他输入框
        if (inputType !== 'hex') inputValues.value.hex = newValues.hex
        if (inputType !== 'dec') inputValues.value.dec = newValues.dec
        if (inputType !== 'oct') inputValues.value.oct = newValues.oct
        if (inputType !== 'bin') inputValues.value.bin = newValues.bin

        // 清除选择状态
        resetSelection()
    } catch (e) {
        console.error('转换错误:', e)
        // 发生错误时不更新值，保持当前状态
    }
}

// 位选择交互
const startSelection = (index: number) => {
    isSelecting.value = true
    selectionStart.value = index
    selectionEnd.value = index
}

const dragSelection = (index: number) => {
    if (isSelecting.value) {
        selectionEnd.value = index
    }
}

const endSelection = () => {
    if (!isSelecting.value) return
    isSelecting.value = false
    calculateSelectedValue()
}

// 重置选择
const resetSelection = () => {
    isSelecting.value = false
    selectionStart.value = null
    selectionEnd.value = null
    selectedBitsValue.value = null
}

// 计算选中位的值
const calculateSelectedValue = () => {
    if (selectionStart.value === null || selectionEnd.value === null) return

    const start = Math.min(selectionStart.value, selectionEnd.value)
    const end = Math.max(selectionStart.value, selectionEnd.value)

    // 提取选中的二进制位
    const selectedBits = bits.value.slice(start, end + 1).join('')

    // 转换为BigInt
    if (selectedBits === '') return

    const value = BigInt('0b' + selectedBits)

    selectedBitsValue.value = {
        dec: value.toString(10),
        hex: value.toString(16).toUpperCase(),
        bin: selectedBits
    }
}

// 检查位是否被选中
const isBitSelected = (index: number): boolean => {
    if (selectionStart.value === null || selectionEnd.value === null) return false

    const start = Math.min(selectionStart.value, selectionEnd.value)
    const end = Math.max(selectionStart.value, selectionEnd.value)

    return index >= start && index <= end
}

// 初始化
onMounted(() => {
    updateValue('255', 10)
})
</script>

<style scoped>
.binary-converter {
    padding: 20px;
    position: relative;
    max-width: 1200px;
    margin: 0 auto;
}

h1 {
    text-align: center;
    color: #333;
    margin-bottom: 10px;
    font-size: 2rem;
}

.subtitle {
    text-align: center;
    color: #666;
    margin-bottom: 30px;
    font-size: 1.1rem;
}

.converter-card,
.binary-display-card,
.instructions-card {
    background-color: #f9f9f9;
    border: 1px solid #ddd;
    border-radius: 10px;
    padding: 20px;
    margin-bottom: 20px;
}

.converter-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 20px;
}

.input-group {
    display: flex;
    flex-direction: column;
}

.input-label {
    font-weight: 600;
    margin-bottom: 8px;
    color: #333;
    font-size: 0.9rem;
}

.input-wrapper {
    display: flex;
    align-items: center;
    background-color: white;
    border: 1px solid #ddd;
    border-radius: 5px;
    overflow: hidden;
    transition: border-color 0.3s;
}

.input-wrapper:focus-within {
    border-color: #007bff;
    box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
}

.prefix {
    background-color: #e9ecef;
    padding: 10px 12px;
    font-weight: bold;
    color: #495057;
    font-family: monospace;
    border-right: 1px solid #ddd;
}

.converter-input {
    flex: 1;
    padding: 10px 12px;
    border: none;
    outline: none;
    font-size: 1rem;
    font-family: monospace;
}

.full-input {
    width: 100%;
}

.binary-display-card h2 {
    color: #333;
    margin-bottom: 20px;
    font-size: 1.3rem;
}

.bit-container {
    background-color: white;
    border: 1px solid #ddd;
    border-radius: 8px;
    padding: 15px;
}

.bit-group {
    margin-bottom: 15px;
}

.bit-row {
    display: flex;
    margin-bottom: 5px;
    justify-content: center;
}

.bit-value-row {
    margin-bottom: 8px;
}

.bit {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin: 0 1px;
    background-color: #f8f9fa;
    border: 1px solid #dee2e6;
    border-radius: 4px;
    font-weight: bold;
    cursor: pointer;
    transition: all 0.2s;
    user-select: none;
    font-family: monospace;
}

.bit:hover {
    background-color: #e9ecef;
    transform: translateY(-1px);
}

.bit.selected {
    background-color: #007bff;
    color: white;
    border-color: #007bff;
    box-shadow: 0 2px 4px rgba(0, 123, 255, 0.3);
}

.bit-label {
    color: #6c757d;
    font-size: 0.75rem;
    width: 32px;
    height: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin: 0 1px;
    font-family: monospace;
}

.selection-info {
    margin-top: 20px;
    padding: 15px;
    background-color: rgba(0, 123, 255, 0.1);
    border: 1px solid rgba(0, 123, 255, 0.2);
    border-radius: 8px;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 15px;
}

.value-display {
    background-color: white;
    padding: 12px;
    border-radius: 6px;
    border: 1px solid #e9ecef;
}

.value-title {
    font-size: 0.85rem;
    color: #6c757d;
    margin-bottom: 5px;
    font-weight: 600;
}

.value-content {
    font-size: 1rem;
    font-weight: bold;
    color: #333;
    word-break: break-all;
    font-family: monospace;
}

.instructions-card h3 {
    color: #333;
    margin-bottom: 15px;
    font-size: 1.2rem;
}

.instructions-card ul {
    text-align: left;
    padding-left: 20px;
    color: #555;
    line-height: 1.6;
}

.instructions-card li {
    margin-bottom: 8px;
}

.highlight {
    color: #007bff;
    font-weight: bold;
}

.converter-input:invalid {
    border-color: #dc3545;
    box-shadow: 0 0 0 2px rgba(220, 53, 69, 0.25);
}

.input-wrapper.error {
    border-color: #dc3545;
    box-shadow: 0 0 0 2px rgba(220, 53, 69, 0.25);
}

@media (max-width: 768px) {
    .converter-grid {
        grid-template-columns: 1fr;
    }

    .bit {
        width: 28px;
        height: 28px;
        font-size: 0.9rem;
    }

    .bit-label {
        width: 28px;
        font-size: 0.7rem;
    }

    .selection-info {
        grid-template-columns: 1fr;
    }
}
</style>