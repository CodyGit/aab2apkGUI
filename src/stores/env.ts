import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { EnvStatus } from "../types";

export const useEnvStore = defineStore("env", () => {
  const javaOk = ref(false);
  const javaVersion = ref("");
  const bundletoolOk = ref(false);
  const bundletoolPath = ref("");

  const allOk = computed(() => javaOk.value && bundletoolOk.value);

  async function checkEnvironment() {
    try {
      const status: EnvStatus = await invoke("env_check");
      javaOk.value = status.javaOk;
      javaVersion.value = status.javaVersion;
      bundletoolOk.value = status.bundletoolOk;
      bundletoolPath.value = status.bundletoolPath;
    } catch (e) {
      console.error("环境检测失败:", e);
    }
  }

  return { javaOk, javaVersion, bundletoolOk, bundletoolPath, allOk, checkEnvironment };
});
