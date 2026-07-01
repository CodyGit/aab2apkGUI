import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { AppSettings, KeystoreConfig } from "../types";

const defaultKeystoreConfig: KeystoreConfig = {
  keystorePath: "",
  storePassword: "",
  keyAlias: "",
  keyPassword: "",
};

export const useSettingsStore = defineStore("settings", () => {
  const bundletoolJarPath = ref("");
  const defaultOutputDir = ref("");
  const lastKeystoreConfig = ref<KeystoreConfig>({ ...defaultKeystoreConfig });
  const lastDeviceTemplate = ref("universal");
  const loading = ref(false);

  async function load() {
    loading.value = true;
    try {
      const s: AppSettings = await invoke("load_settings");
      bundletoolJarPath.value = s.bundletoolJarPath;
      defaultOutputDir.value = s.defaultOutputDir;
      lastKeystoreConfig.value = s.lastKeystoreConfig;
      lastDeviceTemplate.value = s.lastDeviceTemplate;
    } catch (e) {
      console.error("加载设置失败:", e);
    } finally {
      loading.value = false;
    }
  }

  async function save() {
    try {
      await invoke("save_settings", {
        settings: {
          bundletoolJarPath: bundletoolJarPath.value,
          defaultOutputDir: defaultOutputDir.value,
          lastKeystoreConfig: lastKeystoreConfig.value,
          lastDeviceTemplate: lastDeviceTemplate.value,
          language: "zh-CN",
        },
      });
      return true;
    } catch (e) {
      console.error("保存设置失败:", e);
      return false;
    }
  }

  return { bundletoolJarPath, defaultOutputDir, lastKeystoreConfig, lastDeviceTemplate, loading, load, save };
});
