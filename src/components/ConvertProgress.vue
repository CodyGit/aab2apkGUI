<script setup lang="ts">
import { ref, watch, nextTick } from "vue";
import type { ConvertResult } from "../types";

const props = defineProps<{
  logs: string[];
  result: ConvertResult | null;
  error: string;
}>();

const emit = defineEmits<{
  (e: "open-dir", path: string): void;
}>();

const logExpanded = ref(false);
const logEnd = ref<HTMLElement | null>(null);

// Auto-expand on success
watch(() => props.result, (r) => {
  if (r?.success) logExpanded.value = true;
});

watch(
  () => props.logs.length,
  async () => {
    await nextTick();
    logEnd.value?.scrollIntoView({ behavior: "smooth" });
  }
);

function toggleLogs() {
  logExpanded.value = !logExpanded.value;
}
</script>

<template>
  <section class="section">
    <div class="log-header" @click="toggleLogs">
      <span class="arrow">{{ logExpanded ? '▼' : '▶' }}</span>
      <span class="label">转换日志</span>
      <span class="log-count">{{ logs.length }} 条</span>
    </div>

    <!-- Success banner -->
    <div v-if="result?.success" class="success-banner">
      <span class="success-icon">✓</span>
      <span>转换完成！</span>
      <template v-for="f in result.apkFiles" :key="f">
        <a class="apk-link" @click.prevent="emit('open-dir', f)">{{ f.split(/[/\\]/).pop() }}</a>
      </template>
    </div>

    <div v-if="logExpanded" class="log-box">
      <pre v-for="(line, i) in logs" :key="i" class="log-line">{{ line }}</pre>
      <div ref="logEnd" />
    </div>

    <div v-if="error" class="result-error">
      <span class="error-icon">✗</span>
      <pre class="error-msg">{{ error }}</pre>
    </div>
  </section>
</template>

<style scoped>
.section {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 10px 12px;
}
.log-header {
  display: flex;
  align-items: center;
  gap: 6px;
  cursor: pointer;
  user-select: none;
}
.label {
  font-size: 12px;
  color: var(--text-dim);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  font-weight: 500;
}
.arrow {
  font-size: 9px;
  color: var(--text-dim);
}
.log-count {
  font-size: 10px;
  color: var(--text-dim);
  margin-left: auto;
}
.success-banner {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 10px;
  padding: 8px 12px;
  background: #f0fdf4;
  border: 1px solid #bbf7d0;
  border-radius: 6px;
  font-size: 13px;
  color: var(--success);
  flex-wrap: wrap;
}
.success-icon {
  font-weight: 700;
  font-size: 14px;
}
.apk-link {
  color: var(--link);
  cursor: pointer;
  text-decoration: underline;
  text-underline-offset: 2px;
  font-size: 12px;
  word-break: break-all;
}
.log-box {
  background: #f9fafb;
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 10px;
  max-height: 180px;
  overflow-y: auto;
  font-size: 11px;
  font-family: "Consolas", "Courier New", monospace;
  margin-top: 8px;
}
.log-line {
  white-space: pre-wrap;
  word-break: break-all;
  margin: 0;
  color: var(--text-dim);
  line-height: 1.5;
}
.result-error {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  margin-top: 8px;
  padding: 8px 12px;
  background: #fef2f2;
  border: 1px solid #fecaca;
  border-radius: 6px;
}
.error-icon {
  color: var(--error);
  font-weight: 700;
  font-size: 14px;
}
.error-msg {
  font-size: 11px;
  color: var(--error);
  white-space: pre-wrap;
  word-break: break-all;
  margin: 0;
}
</style>
