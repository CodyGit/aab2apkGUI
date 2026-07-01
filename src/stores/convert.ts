import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { ConvertProgress, ConvertResult, KeystoreConfig } from "../types";

export const useConvertStore = defineStore("convert", () => {
  const aabPath = ref("");
  const aabFileName = ref("");
  const outputDir = ref("");
  const keystoreConfig = ref<KeystoreConfig>({
    keystorePath: "",
    storePassword: "",
    keyAlias: "",
    keyPassword: "",
  });
  const deviceSpec = ref<string>("");
  const converting = ref(false);
  const logs = ref<string[]>([]);
  const result = ref<ConvertResult | null>(null);
  const error = ref<string>("");

  function setAabPath(path: string, name: string) {
    aabPath.value = path;
    aabFileName.value = name;
  }

  async function startConvert() {
    converting.value = true;
    logs.value = [];
    error.value = "";
    result.value = null;

    // Listen for progress events
    const unlisten = await listen<ConvertProgress>("convert-progress", (event) => {
      logs.value.push(event.payload.line);
      if (event.payload.done) {
        converting.value = false;
      }
    });

    try {
      const res: ConvertResult = await invoke("convert_aab", {
        aabPath: aabPath.value,
        outputDir: outputDir.value,
        keystoreConfig: keystoreConfig.value,
        deviceSpec: deviceSpec.value || null,
      });
      result.value = res;
    } catch (e) {
      error.value = String(e);
      converting.value = false;
    } finally {
      unlisten();
    }
  }

  return {
    aabPath,
    aabFileName,
    outputDir,
    keystoreConfig,
    deviceSpec,
    converting,
    logs,
    result,
    error,
    setAabPath,
    startConvert,
  };
});
