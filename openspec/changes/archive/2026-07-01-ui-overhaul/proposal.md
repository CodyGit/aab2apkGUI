## Why

当前界面存在以下体验问题：
1. 深红配色（`#e94560`）用于链接和按钮，视觉上偏"警告/危险"感，不符合工具类应用的预期
2. 4 个卡片 section 纵向堆叠，在 900×650 窗口下信息密度低，用户需要频繁滚动
3. 空状态（未选文件）时仍然展示全部 UI，缺乏操作引导
4. 独立设置页仅有一个配置项，增加了不必要的导航跳转
5. 转换成功后反馈不够醒目（成功信息藏在日志折叠区）

需要一次统一的界面翻新，提升精致度和用户体验。

## What Changes

- **配色重构**：暗色背景 → 明亮灰白主题，链接和按钮改为蓝色系，减少视觉疲劳
- **紧凑布局**：压缩 section 间距和 padding，所有交互元素在一屏内展示，无滚动条
- **窗口缩小**：默认窗口从 900×650 调整到 ~700×500
- **Header 简化**：导航链接移除，改为齿轮图标按钮；环境状态用圆点指示灯
- **设置弹窗化**：删除 `/settings` 路由，设置改为点击齿轮触发的 overlay 弹窗
- **输出目录降级**：不再独占 section，改为转换按钮上方单行文本
- **空状态引导**：未选文件时，拖拽区更醒目，环境未就绪时突出引导信息
- **成功状态**：转换完成后自动展开日志，突出显示 APK 文件名和快捷操作
- **快捷键**：Enter 开始转换，Esc 关闭弹窗

## Capabilities

### New Capabilities

- `ui-theme`: 整体配色方案重构，从暗色主题切换到明亮灰白主题，调整按钮/链接/边框颜色
- `ui-layout`: 紧凑布局改造，压缩间距，窗口缩小，所有内容一屏展示
- `ui-workflow`: 交互流程优化，包括设置弹窗化、空状态引导、成功反馈、快捷键

### Modified Capabilities

<!-- No existing capabilities to modify. UI change only touches frontend, not backend commands. -->

## Impact

- 前端组件：App.vue、HomeView.vue、SettingsView.vue、FileDropZone.vue、KeystoreSelector.vue、DeviceConfigPanel.vue、ConvertProgress.vue、EnvCheckDialog.vue
- 新增组件：SettingsDialog.vue
- 删除：SettingsView.vue、SettingsForm.vue 可保留但不再路由
- 后端：无变更
- 窗口配置：tauri.conf.json 窗口尺寸调整
