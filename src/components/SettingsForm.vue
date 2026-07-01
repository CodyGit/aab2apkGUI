<script setup lang="ts">
defineProps<{
  bundletoolPath: string;
}>();

const emit = defineEmits<{
  (e: "update:bundletoolPath", v: string): void;
}>();

async function selectBundletool() {
  try {
    const { open } = await import("@tauri-apps/plugin-dialog");
    const selected = await open({
      multiple: false,
      filters: [
        { name: "JAR files", extensions: ["jar"] },
        { name: "All files", extensions: ["*"] },
      ],
      title: "选择 bundletool.jar",
    });
    if (selected && typeof selected === "string") {
      emit("update:bundletoolPath", selected);
    }
  } catch (e) {
    console.error("选择文件失败:", e);
  }
}
</script>

<template>
  <div class="settings-form">
    <div class="field">
      <label>bundletool.jar 路径</label>
      <div class="input-row">
        <input type="text" :value="bundletoolPath" readonly placeholder="未配置" class="text-input" />
        <button class="btn btn-secondary" @click="selectBundletool">浏览...</button>
      </div>
      <p class="hint">下载地址: <a href="https://github.com/google/bundletool/releases" target="_blank" class="link">GitHub Releases</a></p>
    </div>
  </div>
</template>

<style scoped>
.settings-form {
  display: flex;
  flex-direction: column;
  gap: 20px;
}
.field label {
  display: block;
  font-size: 13px;
  margin-bottom: 6px;
  color: var(--text-dim);
}
.input-row {
  display: flex;
  gap: 8px;
}
.text-input {
  flex: 1;
  padding: 8px 12px;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text);
  font-size: 13px;
}
.text-input:focus {
  outline: none;
  border-color: var(--accent);
}
.btn {
  padding: 8px 12px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 12px;
}
.btn-secondary {
  background: var(--primary);
  color: var(--text);
}
.hint {
  font-size: 11px;
  color: var(--text-dim);
  margin-top: 4px;
}
.link {
  color: var(--accent);
  text-decoration: none;
}
</style>
