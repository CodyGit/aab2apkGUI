# aab2apkGUI

将 Android App Bundle (`.aab`) 转换为可安装的 APK 文件的跨平台桌面工具。

## 功能

- **拖拽或选择 .aab 文件**，自动读取基本信息并设置输出目录
- **证书本地管理**：下拉选择签名证书，支持添加自定义证书
- **设备配置管理**：预设多款设备模板，支持自定义 JSON 配置
- **转换进度实时显示**，日志可折叠展开
- **输出目录可选**，默认与 AAB 文件同目录
- **启动环境检测**：自动检测 Java 和 bundletool，缺失时引导下载

## 前置要求

| 依赖 | 说明 |
|------|------|
| [JDK 11+](https://adoptium.net/download/) | 安装后请确保 `java` 已加入系统 PATH |
| [bundletool.jar](https://github.com/google/bundletool/releases) | 在应用的设置弹窗中配置路径 |

## 安装

从 [Releases](../../releases) 下载对应平台的安装包：

| 平台 | 安装包 |
|------|--------|
| Windows | `.msi` 或 `.exe` |
| macOS | `.dmg` |
| Linux | `.deb` 或 `.AppImage` |

## 构建

```bash
# 安装前端依赖
npm install

# 安装 Rust 工具链（如未安装）
# https://rustup.rs/

# 开发运行
npx tauri dev

# 生产构建
npx tauri build
```

Windows 下使用 GNU 工具链需 MinGW-w64 在 PATH 中。

## 技术栈

- [Tauri v2](https://v2.tauri.app/) — 跨平台桌面框架
- [Vue 3](https://vuejs.org/) + TypeScript — 前端 UI
- [Rust](https://www.rust-lang.org/) — 后端命令逻辑
- [bundletool](https://github.com/google/bundletool) — Google 官方 AAB 转换工具

## 许可

MIT
