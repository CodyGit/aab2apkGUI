## 1. 配色重构

- [x] 1.1 更新 App.vue 中的 CSS 变量（--bg, --surface, --primary, --accent, --text, --text-dim, --border, --success, --error）
- [x] 1.2 统一检查所有组件，确认按钮/链接/边框/文字使用了新的 CSS 变量

## 2. Header 简化

- [x] 2.1 移除 `<nav>` 导航链接（"转换"、"设置"）
- [x] 2.2 左侧改为应用图标 + 名称，右侧改为环境状态圆点（●/○）+ 齿轮按钮
- [x] 2.3 Header 高度从 48px 压缩到 36px
- [x] 2.4 更新拖拽区域（-webkit-app-region: drag/nodrag）

## 3. 设置弹窗化

- [x] 3.1 创建 SettingsDialog.vue 弹窗组件（嵌入 SettingsForm 内容）
- [x] 3.2 在 App.vue 中管理设置弹窗状态，齿轮按钮触发
- [x] 3.3 从路由中移除 /settings，删除 SettingsView.vue
- [x] 3.4 重新检测环境按钮放入弹窗

## 4. 紧凑布局

- [x] 4.1 压缩 HomeView section 间距（20px → 10px）和 padding（16px → 12px）
- [x] 4.2 调整组件内部间距
- [x] 4.3 确保 700×500 窗口内所有内容一屏可见，无滚动条

## 5. 输出目录降级

- [x] 5.1 移除输出目录独立 section
- [x] 5.2 改为转换按钮上方单行"输出至: /path [浏览]"
- [x] 5.3 路径过长时使用 text-overflow: ellipsis 截断

## 6. 窗口尺寸调整

- [x] 6.1 修改 tauri.conf.json：width 700, height 500, minWidth 600, minHeight 420

## 7. 空状态引导

- [x] 7.1 文件未选择时拖拽区加大图标和引导文字
- [x] 7.2 环境未就绪时在转换按钮下方显示引导信息（含跳转设置链接）

## 8. 成功状态

- [x] 8.1 转换成功后自动展开日志区域
- [x] 8.2 在日志顶部插入成功提示条（绿色背景，"✓ 转换完成！"）
- [x] 8.3 APK 文件名以蓝色链接显示，点击打开目录

## 9. 快捷键

- [x] 9.1 App.vue 根级别监听 keydown：Enter 触发转换，Esc 关闭弹窗
- [x] 9.2 各弹窗统一 emit close 事件供全局 Esc 处理
- [x] 9.3 Enter 仅当文件已选且环境就绪时触发
