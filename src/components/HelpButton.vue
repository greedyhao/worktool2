<template>
    <!-- 将按钮固定在右下角 -->
    <button class="help-button" @click="showHelp = true">帮助</button>

    <!-- 模态框部分 -->
    <div v-if="showHelp" class="modal" @click.self="closeModal">
        <div class="modal-content">
            <span class="close" @click="closeModal">&times;</span>
            <div v-html="renderedMarkdown"></div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue';
import { marked } from 'marked'; // 导入 marked

export default defineComponent({
    name: 'HelpButton',
    props: {
        markdownFile: {
            type: String,
            required: true,
        },
    },
    setup(props) {
        const showHelp = ref(false);
        const renderedMarkdown = ref('');

        // 关闭模态框的方法
        const closeModal = () => {
            showHelp.value = false;
        };

        onMounted(async () => {
            try {
                // 动态加载 Markdown 文件
                const response = await fetch(`./src/assets/help/${props.markdownFile}.md`);
                const markdownText = await response.text();

                // 替换自定义标记为实际路径
                const processedMarkdown = markdownText.replace(
                    /!\[([\p{L}\s\d]*)\]\(~images[\\\/](.*?)\)/gu, // 支持空内容和中文
                    (_match, alt, src) => {
                        // console.log('匹配到的内容:', match, 'alt:', alt, 'src:', src); // 打印匹配结果
                        // 替换为实际路径
                        const normalizedSrc = src.replace(/\\/g, '/'); // 统一路径分隔符
                        return `![${alt}](/src/assets/images/${normalizedSrc})`;
                    }
                );

                // console.log('替换后的 Markdown 内容:', processedMarkdown); // 打印替换后的内容

                // 将 Markdown 转换为 HTML
                const html = await marked(processedMarkdown);
                renderedMarkdown.value = html;
            } catch (error) {
                console.error('加载帮助文档失败:', error);
            }
        });

        return {
            showHelp,
            renderedMarkdown,
            closeModal, // 返回关闭模态框的方法
        };
    },
});
</script>

<style scoped>
/* 帮助按钮样式 */
.help-button {
    position: fixed; /* 固定在页面 */
    bottom: 20px; /* 距离底部 20px */
    right: 20px; /* 距离右侧 20px */
    padding: 10px 20px;
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 5px;
    cursor: pointer;
    z-index: 1000; /* 确保按钮在最上层 */
}

.help-button:hover {
    background-color: #0056b3;
}

/* 模态框样式 */
.modal {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1001; /* 确保模态框在按钮之上 */
}

.modal-content {
    background-color: white;
    padding: 20px;
    border-radius: 5px;
    max-width: 600px;
    max-height: 80vh;
    overflow-y: auto;
    position: relative;
}

.close {
    position: absolute;
    top: 10px;
    right: 10px;
    font-size: 24px;
    cursor: pointer;
}
</style>