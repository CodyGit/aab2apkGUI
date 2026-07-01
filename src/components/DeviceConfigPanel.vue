<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface DeviceConfigEntry {
  name: string;
  config: string;
}

defineProps<{
  modelValue: string;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", v: string): void;
}>();

const entries = ref<DeviceConfigEntry[]>([]);
const selectedName = ref("");
const configsDir = ref("");

const showAddDialog = ref(false);
const addName = ref("");
const addConfig = ref("");
const addError = ref("");

async function loadEntries() {
  try {
    let all = await invoke<DeviceConfigEntry[]>("list_device_configs");
    all.sort((a, b) => {
      if (a.name.toLowerCase() === "universal") return -1;
      if (b.name.toLowerCase() === "universal") return 1;
      return a.name.localeCompare(b.name);
    });
    entries.value = all;
    configsDir.value = await invoke<string>("get_device_configs_dir");
    if (!selectedName.value && entries.value.length > 0) {
      selectedName.value = entries.value[0].name;
      emit("update:modelValue", entries.value[0].config);
    }
  } catch (e) {
    console.error("加载设备配置列表失败:", e);
  }
}

onMounted(loadEntries);

function handleSelect() {
  const entry = entries.value.find(e => e.name === selectedName.value);
  if (entry) {
    emit("update:modelValue", entry.config);
  }
}

function currentConfigDetail(): string | null {
  const entry = entries.value.find(e => e.name === selectedName.value);
  if (!entry) return null;
  try {
    const obj = JSON.parse(entry.config);
    if (obj.mode === "universal") return "通用模式 — 生成单个 APK";
    const parts: string[] = [];
    if (obj.supportedAbis) parts.push("ABI: " + obj.supportedAbis.join(", "));
    if (obj.screenDensity) parts.push("DPI: " + obj.screenDensity);
    if (obj.sdkVersion) parts.push("SDK: " + obj.sdkVersion);
    return parts.join(" | ") || entry.config;
  } catch {
    return entry.config;
  }
}

async function openConfigsDir() {
  if (configsDir.value) {
    try {
      await invoke("reveal_in_explorer", { path: configsDir.value });
    } catch (e) {
      console.error("打开目录失败:", e);
    }
  }
}

function clearAddForm() {
  addName.value = "";
  addConfig.value = "";
  addError.value = "";
}

function closeAddDialog() {
  showAddDialog.value = false;
  clearAddForm();
}

async function handleAddConfig() {
  addError.value = "";
  if (!addName.value) { addError.value = "请输入配置名称"; return; }
  if (!addConfig.value.trim()) { addError.value = "请输入 JSON 配置"; return; }

  const savedName = addName.value;
  try {
    await invoke("add_device_config", {
      name: savedName,
      config: addConfig.value,
    });
    closeAddDialog();
    await loadEntries();
    selectedName.value = savedName;
    handleSelect();
  } catch (e) {
    addError.value = String(e);
  }
}
</script>

<template>
  <div class="device-config">
    <div class="select-row">
      <select v-model="selectedName" class="select" @change="handleSelect">
        <option value="" disabled>-- 请选择设备配置 --</option>
        <option
          v-for="entry in entries"
          :key="entry.name"
          :value="entry.name"
        >
          {{ entry.name }}
        </option>
      </select>
      <button class="btn btn-primary btn-sm" @click="showAddDialog = true">+ 添加配置</button>
    </div>
    <div v-if="currentConfigDetail()" class="detail">
      {{ currentConfigDetail() }}
    </div>
    <p class="dir-hint">
      <a class="dir-link" @click.prevent="openConfigsDir">点击查看配置目录</a>
    </p>

    <!-- Add dialog -->
    <div v-if="showAddDialog" class="overlay">
      <div class="dialog">
        <h3>添加设备配置</h3>
        <div class="form">
          <div class="field">
            <label>配置名称</label>
            <input v-model="addName" type="text" placeholder="给配置起个名字" class="text-input" />
          </div>
          <div class="field">
            <label>JSON 配置</label>
            <textarea
              v-model="addConfig"
              class="textarea"
              placeholder='{"supportedAbis":["arm64-v8a"],"sdkVersion":31}'
              rows="6"
            ></textarea>
          </div>
          <p v-if="addError" class="error-text">{{ addError }}</p>
        </div>
        <div class="actions">
          <button class="btn btn-cancel" @click="closeAddDialog">取消</button>
          <button class="btn btn-primary" @click="handleAddConfig">添加</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.device-config {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.select-row {
  display: flex;
  gap: 8px;
}
.select {
  flex: 1;
  padding: 8px 12px;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text);
  font-size: 13px;
}
.select:focus {
  outline: none;
  border-color: var(--accent);
}
.detail {
  font-size: 12px;
  color: var(--text-dim);
}
.dir-hint {
  font-size: 11px;
  color: var(--text-dim);
}
.dir-link {
  color: var(--link);
  cursor: pointer;
  text-decoration: underline;
  text-underline-offset: 2px;
}
.dir-link:hover {
  color: var(--text);
}
.btn {
  padding: 8px 12px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 12px;
  white-space: nowrap;
}
.btn-sm {
  padding: 6px 10px;
}
.btn-primary {
  background: var(--accent);
  color: white;
}
.btn-secondary {
  background: var(--primary);
  color: var(--text);
}

/* Dialog */
.overlay {
  position: fixed;
  top: 0; left: 0; right: 0; bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}
.dialog {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 24px;
  max-width: 520px;
  width: 90%;
}
.dialog h3 {
  font-size: 16px;
  margin-bottom: 16px;
}
.form {
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.field label {
  display: block;
  font-size: 12px;
  color: var(--text-dim);
  margin-bottom: 4px;
}
.text-input {
  width: 100%;
  padding: 8px 10px;
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
.textarea {
  width: 100%;
  padding: 10px 12px;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text);
  font-size: 12px;
  font-family: "Consolas", "Courier New", monospace;
  resize: vertical;
}
.textarea:focus {
  outline: none;
  border-color: var(--accent);
}
.actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  margin-top: 16px;
}
.error-text {
  color: var(--error);
  font-size: 12px;
}
</style>
