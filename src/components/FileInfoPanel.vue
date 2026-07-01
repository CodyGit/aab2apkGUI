<script setup lang="ts">
import type { AppFileInfo } from "../types";

defineProps<{
  info: AppFileInfo;
  title: string;
}>();
</script>

<template>
  <div class="file-info">
    <h3>{{ title }}</h3>
    <table>
      <tr v-if="info.packageName">
        <td class="label">包名</td>
        <td class="value">{{ info.packageName }}</td>
      </tr>
      <tr v-if="info.versionName">
        <td class="label">版本名称</td>
        <td class="value">{{ info.versionName }}</td>
      </tr>
      <tr v-if="info.versionCode">
        <td class="label">版本号</td>
        <td class="value">{{ info.versionCode }}</td>
      </tr>
      <tr v-if="info.minSdkVersion">
        <td class="label">最低 SDK</td>
        <td class="value">{{ info.minSdkVersion }}</td>
      </tr>
      <tr v-if="info.targetSdkVersion">
        <td class="label">目标 SDK</td>
        <td class="value">{{ info.targetSdkVersion }}</td>
      </tr>
      <tr v-if="info.permissions.length > 0">
        <td class="label">权限</td>
        <td class="value">
          <span v-for="p in info.permissions" :key="p" class="perm-tag">{{ p }}</span>
        </td>
      </tr>
    </table>
  </div>
</template>

<style scoped>
.file-info {
  margin-top: 12px;
  background: rgba(15, 52, 96, 0.3);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 10px 14px;
}
.file-info h3 {
  font-size: 12px;
  color: var(--text-dim);
  margin-bottom: 8px;
}
table {
  width: 100%;
  font-size: 12px;
  border-collapse: collapse;
}
tr {
  border-bottom: 1px solid rgba(51, 51, 51, 0.5);
}
tr:last-child {
  border-bottom: none;
}
td {
  padding: 4px 0;
}
.label {
  color: var(--text-dim);
  width: 80px;
  white-space: nowrap;
}
.value {
  color: var(--text);
  word-break: break-all;
}
.perm-tag {
  display: inline-block;
  background: var(--primary);
  padding: 1px 6px;
  border-radius: 3px;
  margin: 1px 2px;
  font-size: 10px;
}
</style>
