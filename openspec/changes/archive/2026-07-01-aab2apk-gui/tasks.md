## 1. 项目初始化

- [x] 1.1 使用 `npm create tauri-app` 初始化 Tauri v2 + Vue 3 + TypeScript 项目
- [x] 1.2 安装前端依赖：pinia、vue-router
- [x] 1.3 配置 Vite 构建选项和路径别名
- [x] 1.4 在 src-tauri 中嵌入 debug.keystore 资源文件

## 2. 后端：环境检测

- [x] 2.1 实现 `check_java` 命令：检测 PATH 中 java 是否存在且版本 >= 11
- [x] 2.2 实现 `check_bundletool` 命令：检测用户配置的 bundletool.jar 路径是否有效
- [x] 2.3 实现 `env_check` 组合命令：返回 java_ok、java_version、bundletool_ok、bundletool_path

## 3. 后端：配置管理

- [x] 3.1 定义 AppSettings 结构体（bundletool_path, default_output_dir, last_keystore_config, last_device_template, language）
- [x] 3.2 实现 `load_settings` 命令：从 app data dir 读取 config.json
- [x] 3.3 实现 `save_settings` 命令：写入 config.json
- [x] 3.4 实现首次启动时使用默认值创建配置文件

## 4. 后端：证书操作

- [x] 4.1 实现 `list_keystore_info` 命令：调用 keytool -list 读取别名和有效期
- [x] 4.2 实现 `validate_keystore` 命令：验证密码和别名是否有效
- [x] 4.3 实现 `parse_keystore_config` 命令：解析 keystore JSON 配置文件

## 5. 后端：AAB 转换引擎

- [x] 5.1 实现 bundletool 命令构建：build-apks 参数组装（bundle, output, ks, ks-pass, ks-key-alias, key-pass, mode/device-spec）
- [x] 5.2 实现异步命令执行：Command::new("java") 调用 bundletool，逐行捕获 stdout/stderr
- [x] 5.3 实现进度推送：通过 Tauri Event 实时推送执行日志到前端
- [x] 5.4 实现 .apks 解压：解析 zip 提取 universal.apk 或 split APKs
- [x] 5.5 实现 `convert_aab` 命令：整合上述步骤的完整转换命令

## 6. 后端：文件信息读取

- [x] 6.1 实现 `read_aab_info` 命令：通过 bundletool dump manifest 读取 AAB 信息
- [x] 6.2 实现 `read_apk_info` 命令：通过 bundletool dump manifest 读取 APK 信息
- [x] 6.3 实现返回结构化的文件信息（包名、版本、SDK、权限列表）

## 7. 前端：应用框架搭建

- [x] 7.1 创建 Vue Router 路由配置（/ 和 /settings）
- [x] 7.2 创建 Pinia store：convertStore（转换状态）、envStore（环境状态）、settingsStore（设置状态）
- [x] 7.3 创建 App.vue 根布局：导航栏、路由出口、环境检测弹窗嵌入
- [x] 7.4 定义 TypeScript 类型接口（AppSettings, KeystoreConfig, DeviceSpec, FileInfo 等）

## 8. 前端：主转换页面 (HomeView)

- [x] 8.1 实现 FileDropZone 组件：拖拽/点击选择 .aab 文件，显示文件路径和基本信息
- [x] 8.2 实现 KeystoreSelector 组件：默认证书/自定义选择切换，密码和别名字段，证书信息展示
- [x] 8.3 实现 DeviceConfigPanel 组件：预设模板下拉菜单 + 自定义 JSON 编辑区
- [x] 8.4 实现输出目录选择区域：浏览选择目录，显示当前路径
- [x] 8.5 实现 ConvertProgress 组件：转换日志实时展示、进度指示器、结果状态
- [x] 8.6 实现"开始转换"按钮及其禁用逻辑（文件已选 + 环境可用）
- [x] 8.7 实现转换完成后"打开所在目录"功能

## 9. 前端：环境检测弹窗 (EnvCheckDialog)

- [x] 9.1 实现环境状态摘要显示（Java ✓/✗、bundletool ✓/✗）
- [x] 9.2 实现 Java 缺失时的下载指引（链接 + 说明文案）
- [x] 9.3 实现 bundletool 缺失时的下载指引（链接 + 说明文案）
- [x] 9.4 实现"稍后处理"和"前往设置"按钮

## 10. 前端：设置页面 (SettingsView)

- [x] 10.1 实现 SettingsForm 组件：bundletool.jar 路径配置（文件浏览器选择）
- [x] 10.2 实现默认输出目录配置
- [x] 10.3 实现配置保存和读取的 IPC 调用封装
- [x] 10.4 实现环境重新检测按钮

## 11. 前端：文件信息查看

- [x] 11.1 实现 FileInfoPanel 组件：以卡片/表格展示包名、版本、SDK、权限等
- [x] 11.2 支持独立的"查看 APK 信息"入口（选择 APK 文件查看信息）

## 12. 集成与收尾

- [x] 12.1 端到端流程联调：环境检测 → 选择 AAB → 配置证书 → 转换 → 打开目录
- [x] 12.2 错误处理和用户提示完善（各失败场景的用户友好提示）
- [x] 12.3 中文文案审核和统一

## 13. 打包配置

- [x] 13.1 配置 Tauri bundler：应用名、图标、版本号、identifier
- [x] 13.2 配置 Windows 打包（.msi + .exe）
- [x] 13.3 配置 macOS 打包（.dmg）
- [x] 13.4 配置 Linux 打包（.deb + .AppImage）
