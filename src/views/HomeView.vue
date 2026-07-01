<script setup lang="ts">
import { onMounted, onUnmounted, inject } from "vue";
import { useConvertStore } from "../stores/convert";
import { useSettingsStore } from "../stores/settings";
import { useEnvStore } from "../stores/env";
import FileDropZone from "../components/FileDropZone.vue";
import KeystoreSelector from "../components/KeystoreSelector.vue";
import DeviceConfigPanel from "../components/DeviceConfigPanel.vue";
import ConvertProgress from "../components/ConvertProgress.vue";
import { invoke } from "@tauri-apps/api/core";

const convertStore = useConvertStore();
const settingsStore = useSettingsStore();
const envStore = useEnvStore();

const showSettings = inject<() => void>("showSettings", () => {});

onMounted(async () => {
  await settingsStore.load();
  convertStore.deviceSpec = settingsStore.lastDeviceTemplate;
  window.addEventListener("global-enter", handleGlobalEnter);
});

onUnmounted(() => {
  window.removeEventListener("global-enter", handleGlobalEnter);
});

function handleGlobalEnter() {
  if (canConvert()) handleConvert();
}

async function handleFileSelected(path: string, name: string) {
  convertStore.setAabPath(path, name);
  const parentDir = path.replace(/[/\\][^/\\]*$/, "");
  convertStore.outputDir = parentDir;
}

async function handleSelectOutputDir() {
  try {
    const { open } = await import("@tauri-apps/plugin-dialog");
    const dir = await open({ directory: true, multiple: false, title: "选择输出目录" });
    if (dir && typeof dir === "string") {
      convertStore.outputDir = dir;
    }
  } catch (e) {
    console.error("选择目录失败:", e);
  }
}

function canConvert() {
  return convertStore.aabPath && envStore.allOk && !convertStore.converting;
}

async function handleConvert() {
  if (!canConvert()) return;
  settingsStore.lastKeystoreConfig = { ...convertStore.keystoreConfig };
  settingsStore.lastDeviceTemplate = convertStore.deviceSpec;
  await settingsStore.save();
  await convertStore.startConvert();
}

async function openDir(path: string) {
  try { await invoke("reveal_in_explorer", { path }); } catch {}
}
</script>

<template>
  <div class="home">
    <!-- File Input -->
    <section class="section">
      <FileDropZone
        :file-path="convertStore.aabPath"
        :file-name="convertStore.aabFileName"
        accept=".aab"
        @file-selected="handleFileSelected"
      />
    </section>

    <!-- Keystore + Device Config -->
    <section class="section">
      <div class="side-by-side">
        <div class="half">
          <div class="label">签名证书</div>
          <KeystoreSelector v-model="convertStore.keystoreConfig" />
        </div>
        <div class="half">
          <div class="label">设备配置</div>
          <DeviceConfigPanel v-model="convertStore.deviceSpec" />
        </div>
      </div>
    </section>

    <!-- Output line + Convert -->
    <section class="section">
      <div class="output-line">
        <span class="output-label">输出至:</span>
        <input
          type="text"
          :value="convertStore.outputDir"
          :title="convertStore.outputDir"
          placeholder="与 AAB 文件相同目录"
          readonly
          class="output-input"
        />
        <button class="btn-text" @click="handleSelectOutputDir">浏览...</button>
      </div>
      <button
        class="btn btn-primary btn-large"
        :disabled="!canConvert()"
        @click="handleConvert"
      >
        {{ convertStore.converting ? "转换中..." : "开始转换" }}
      </button>
      <p v-if="!envStore.allOk" class="hint-text">
        环境未就绪，请
        <a class="link" @click.prevent="showSettings()">配置 bundletool</a>
        或检查 Java 环境
      </p>
      <p v-else-if="!convertStore.aabPath" class="hint-text">
        请选择 AAB 文件后开始转换
      </p>
    </section>

    <!-- Progress -->
    <ConvertProgress
      v-if="convertStore.logs.length > 0 || convertStore.result"
      :logs="convertStore.logs"
      :result="convertStore.result"
      :error="convertStore.error"
      @open-dir="openDir"
    />
  </div>
</template>

<style scoped>
.section {
  margin-bottom: 10px;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 12px;
}
.label {
  font-size: 12px;
  color: var(--text-dim);
  margin-bottom: 8px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  font-weight: 500;
}
.side-by-side {
  display: flex;
  gap: 20px;
}
.half {
  flex: 1;
  min-width: 0;
}
.output-line {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 12px;
}
.output-label {
  font-size: 12px;
  color: var(--text-dim);
  white-space: nowrap;
}
.output-input {
  flex: 1;
  padding: 6px 10px;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text);
  font-size: 12px;
  text-overflow: ellipsis;
}
.output-input:focus {
  outline: none;
  border-color: var(--link);
}
.btn-text {
  background: none;
  border: none;
  color: var(--link);
  cursor: pointer;
  font-size: 12px;
  white-space: nowrap;
}
.btn-text:hover {
  text-decoration: underline;
}
.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  transition: opacity 0.15s;
}
.btn-primary {
  background: var(--primary);
  color: white;
}
.btn-primary:hover:not(:disabled) {
  opacity: 0.9;
}
.btn-primary:disabled {
  opacity: 0.35;
  cursor: not-allowed;
}
.btn-secondary {
  background: var(--bg);
  color: var(--text);
  border: 1px solid var(--border);
}
.btn-large {
  width: 100%;
  padding: 10px;
  font-size: 15px;
  border-radius: 8px;
}
.hint-text {
  color: var(--text-dim);
  font-size: 11px;
  margin-top: 8px;
  text-align: center;
}
.link {
  color: var(--link);
  cursor: pointer;
  text-decoration: underline;
}
</style>
