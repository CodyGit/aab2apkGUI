<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

const props = defineProps<{
  filePath: string;
  fileName: string;
  accept: string;
}>();

const emit = defineEmits<{
  (e: "file-selected", path: string, name: string): void;
}>();

const dragging = ref(false);
const zoneRef = ref<HTMLElement | null>(null);
let unlisten: UnlistenFn | null = null;

onMounted(async () => {
  // Listen for Tauri native drag-drop events
  unlisten = await listen<{ paths: string[] }>("tauri://drag-drop", (event) => {
    dragging.value = false;
    const files = event.payload.paths;
    if (files && files.length > 0) {
      const path = files[0];
      const ext = path.split(".").pop()?.toLowerCase();
      if (ext === props.accept.replace(".", "")) {
        const name = path.split(/[/\\]/).pop() || path;
        emit("file-selected", path, name);
      }
    }
  });

  // Listen for Tauri drag hover/leave events
  await listen("tauri://drag-enter", () => {
    dragging.value = true;
  });
  await listen("tauri://drag-leave", () => {
    dragging.value = false;
  });

  // Also listen for drop on the zone element
  await nextTick();
  zoneRef.value?.addEventListener("dragover", (e) => { e.preventDefault(); dragging.value = true; });
  zoneRef.value?.addEventListener("dragleave", () => { dragging.value = false; });
  zoneRef.value?.addEventListener("drop", (e) => { e.preventDefault(); });
});

onUnmounted(() => {
  if (unlisten) unlisten();
});

async function handleClick() {
  try {
    const { open } = await import("@tauri-apps/plugin-dialog");
    const selected = await open({
      multiple: false,
      filters: [{ name: "AAB files", extensions: ["aab"] }],
      title: "选择 AAB 文件",
    });
    if (selected && typeof selected === "string") {
      const name = selected.split(/[/\\]/).pop() || selected;
      emit("file-selected", selected, name);
    }
  } catch (e) {
    console.error("选择文件失败:", e);
  }
}
</script>

<template>
  <div
    ref="zoneRef"
    class="drop-zone"
    :class="{ dragging, filled: !!filePath }"
    @click="handleClick"
  >
    <div v-if="filePath" class="file-selected">
      <span class="file-icon">📦</span>
      <span class="file-name">{{ fileName }}</span>
      <span class="file-path">{{ filePath }}</span>
    </div>
    <div v-else class="drop-placeholder">
      <span class="drop-icon">↓</span>
      <span>拖拽 .aab 文件到此处，或点击选择文件</span>
    </div>
  </div>
</template>

<style scoped>
.drop-zone {
  border: 2px dashed var(--border);
  border-radius: 8px;
  padding: 24px;
  text-align: center;
  cursor: pointer;
  transition: all 0.2s;
  min-height: 80px;
  display: flex;
  align-items: center;
  justify-content: center;
}
.drop-zone:hover,
.drop-zone.dragging {
  border-color: var(--primary);
  background: var(--hover-bg);
}
.drop-zone.filled {
  border-style: solid;
  border-color: var(--success);
}
.file-selected {
  display: flex;
  flex-direction: column;
  gap: 4px;
  align-items: center;
}
.file-icon {
  font-size: 28px;
}
.file-name {
  font-size: 15px;
  font-weight: 600;
}
.file-path {
  font-size: 12px;
  color: var(--text-dim);
  word-break: break-all;
}
.drop-placeholder {
  display: flex;
  flex-direction: column;
  gap: 8px;
  align-items: center;
  color: var(--text-dim);
  font-size: 13px;
}
.drop-icon {
  font-size: 32px;
  color: var(--accent);
}
</style>
