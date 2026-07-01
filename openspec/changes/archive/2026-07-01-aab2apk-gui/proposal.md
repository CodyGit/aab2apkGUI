## Why

Android App Bundle (AAB) 是 Google Play 强制要求的发布格式，但开发者在测试、分发内测版本、或从第三方渠道下载 AAB 后，无法直接安装到设备上。目前市面上的转换工具要么是命令行工具（如 bundletool），要么是在线服务（上传文件有隐私风险），缺少一个开箱即用的跨平台桌面 GUI 工具。本工具填补这个空白，让开发者和普通用户都能轻松将 AAB 转换为可安装的 APK。

## What Changes

- 基于 Tauri + Vue 构建跨平台桌面应用（Windows / macOS / Linux）
- 提供图形界面完成 AAB → APK 的完整转换流程
- 内嵌默认 debug 证书，用户无需任何配置即可快速转换
- 支持自定义 keystore 进行正式签名
- 支持按设备配置（ABI、屏幕密度、SDK 版本等）导出定制 APK
- 提供 AAB/APK 文件的信息查看功能
- 启动时自动检测 Java 环境，缺失时引导安装
- 支持用户配置 bundletool.jar 路径，缺失时引导下载
- 偏好设置持久化到本地 JSON 配置文件
- 打包为平台原生可执行文件，用户下载即用

## Capabilities

### New Capabilities

- `aab-convert`: AAB 文件转换为 APK 的核心流程，包括文件选择、证书配置、输出目录选择、进度反馈、结果展示
- `env-check`: 启动环境检测，验证 Java 和 bundletool 是否可用，缺失时提供引导
- `keystore-manage`: 证书管理，包括选择已有证书、使用默认证书、证书信息查看
- `device-config`: 设备配置管理，支持预设模板和自定义 JSON 配置导出定制 APK
- `file-info`: AAB/APK 文件信息查看，展示包名、版本号、SDK 版本、权限等元数据
- `app-settings`: 应用偏好设置，bundletool 路径配置、默认输出目录、持久化读写

### Modified Capabilities

<!-- No existing capabilities to modify -->

## Impact

- 新项目，无现有代码影响
- 依赖外部工具：JDK（用户通过 PATH 提供）、bundletool.jar（用户配置路径）
- 底层转换能力依赖 bundletool build-apks 命令
- 打包后分发：Windows (.msi/.exe)、macOS (.dmg)、Linux (.deb/.AppImage)
