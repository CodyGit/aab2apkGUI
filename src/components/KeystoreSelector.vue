<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { KeystoreConfig, KeystoreEntry } from "../types";

defineProps<{
  modelValue: KeystoreConfig;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", v: KeystoreConfig): void;
}>();

const entries = ref<KeystoreEntry[]>([]);
const selectedName = ref("");
const keystoresDir = ref("");

// Add keystore dialog
const showAddDialog = ref(false);
const addSourcePath = ref("");
const addName = ref("");
const addStorePassword = ref("");
const addKeyAlias = ref("");
const addKeyPassword = ref("");
const addError = ref("");
// Password visibility
const showStorePass = ref(false);
const showKeyPass = ref(false);

async function loadEntries() {
  try {
    entries.value = await invoke<KeystoreEntry[]>("list_keystores");
    keystoresDir.value = await invoke<string>("get_keystores_dir");
    if (!selectedName.value && entries.value.length > 0) {
      const debugEntry = entries.value.find(e => e.name === "debug");
      if (debugEntry) {
        selectedName.value = "debug";
        emit("update:modelValue", debugEntry.config);
      } else {
        selectedName.value = entries.value[0].name;
        emit("update:modelValue", entries.value[0].config);
      }
    }
  } catch (e) {
    console.error("加载证书列表失败:", e);
  }
}

onMounted(loadEntries);

async function openKeystoreDir() {
  if (keystoresDir.value) {
    try {
      await invoke("reveal_in_explorer", { path: keystoresDir.value });
    } catch (e) {
      console.error("打开目录失败:", e);
    }
  }
}

function handleSelect() {
  const entry = entries.value.find(e => e.name === selectedName.value);
  if (entry) {
    emit("update:modelValue", entry.config);
  }
}

function clearAddForm() {
  addSourcePath.value = "";
  addName.value = "";
  addStorePassword.value = "";
  addKeyAlias.value = "";
  addKeyPassword.value = "";
  addError.value = "";
  showStorePass.value = false;
  showKeyPass.value = false;
}

function closeAddDialog() {
  showAddDialog.value = false;
  clearAddForm();
}

async function selectSourceFile() {
  try {
    const { open } = await import("@tauri-apps/plugin-dialog");
    const selected = await open({
      multiple: false,
      filters: [
        { name: "Keystore 文件", extensions: ["keystore", "jks"] },
        { name: "所有文件", extensions: ["*"] },
      ],
      title: "选择证书文件",
    });
    if (selected && typeof selected === "string") {
      addSourcePath.value = selected;
      const fname = selected.split(/[/\\]/).pop() || "";
      addName.value = fname.replace(/\.(keystore|jks)$/i, "");
    }
  } catch (e) {
    console.error("选择文件失败:", e);
  }
}

async function handleAddKeystore() {
  addError.value = "";
  if (!addSourcePath.value) { addError.value = "请选择证书文件"; return; }
  if (!addName.value) { addError.value = "请输入证书名称"; return; }
  if (!addStorePassword.value) { addError.value = "请输入 Store 密码"; return; }
  if (!addKeyAlias.value) { addError.value = "请输入 Key Alias"; return; }
  if (!addKeyPassword.value) { addError.value = "请输入 Key 密码"; return; }

  const savedName = addName.value;

  try {
    await invoke("add_keystore", {
      sourcePath: addSourcePath.value,
      name: savedName,
      storePassword: addStorePassword.value,
      keyAlias: addKeyAlias.value,
      keyPassword: addKeyPassword.value,
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
  <div class="keystore-selector">
    <div class="select-row">
      <select v-model="selectedName" class="select" @change="handleSelect">
        <option value="" disabled>-- 请选择证书 --</option>
        <option
          v-for="entry in entries"
          :key="entry.name"
          :value="entry.name"
        >
          {{ entry.name }}
        </option>
      </select>
      <button class="btn btn-primary btn-sm" @click="showAddDialog = true">+ 添加证书</button>
    </div>
    <div v-if="selectedName" class="info">
      <span class="detail">Alias: {{ modelValue.keyAlias }}</span>
    </div>
    <p class="dir-hint">
      <a class="dir-link" @click.prevent="openKeystoreDir">点击查看证书目录</a>
    </p>

    <!-- Add Keystore Dialog -->
    <div v-if="showAddDialog" class="overlay">
      <div class="dialog">
        <h3>添加证书</h3>
        <div class="form">
          <div class="field">
            <label>证书文件</label>
            <div class="input-row">
              <input type="text" :value="addSourcePath" readonly placeholder="选择 .keystore 文件" class="text-input" />
              <button class="btn btn-primary btn-sm" @click="selectSourceFile">浏览...</button>
            </div>
          </div>
          <div class="field">
            <label>证书名称</label>
            <input v-model="addName" type="text" placeholder="给证书起个名字" class="text-input" />
          </div>
          <div class="field">
            <label>Store 密码</label>
            <div class="input-row">
              <input v-model="addStorePassword" :type="showStorePass ? 'text' : 'password'" placeholder="请输入 Store 密码" class="text-input" />
              <button class="btn btn-icon" @click="showStorePass = !showStorePass" tabindex="-1">
                {{ showStorePass ? '🙈' : '👁' }}
              </button>
            </div>
          </div>
          <div class="field">
            <label>Key Alias</label>
            <input v-model="addKeyAlias" type="text" placeholder="请输入 Key Alias" class="text-input" />
          </div>
          <div class="field">
            <label>Key 密码</label>
            <div class="input-row">
              <input v-model="addKeyPassword" :type="showKeyPass ? 'text' : 'password'" placeholder="请输入 Key 密码" class="text-input" />
              <button class="btn btn-icon" @click="showKeyPass = !showKeyPass" tabindex="-1">
                {{ showKeyPass ? '🙈' : '👁' }}
              </button>
            </div>
          </div>
          <p v-if="addError" class="error-text">{{ addError }}</p>
        </div>
        <div class="actions">
          <button class="btn btn-cancel" @click="closeAddDialog">取消</button>
          <button class="btn btn-primary" @click="handleAddKeystore">添加</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.keystore-selector {
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
.info {
  display: flex;
  gap: 16px;
}
.detail {
  font-size: 12px;
  color: var(--text-dim);
}
.dir-hint {
  font-size: 11px;
  color: var(--text-dim);
  word-break: break-all;
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
.btn-icon {
  padding: 4px 8px;
  background: none;
  border: none;
  cursor: pointer;
  font-size: 14px;
  line-height: 1;
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
  max-width: 480px;
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
.input-row {
  display: flex;
  gap: 8px;
}
.input-row .text-input {
  flex: 1;
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
