# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 通用行为准则

**用中文沟通。** 代码注释和 commit message 用中文。

## 构建命令

```bash
# 开发模式（仅前端热重载，需另外启动 Rust 后端）
npm run dev

# 完整构建（TypeScript 检查 + Vite 打包 + Rust 编译 + 打包）
npx tauri build

# 仅 Rust 编译检查（比完整构建快）
cd src-tauri && cargo check

# 开发运行（完整构建 + 启动窗口，适合验收）
npx tauri dev
```

本机 Rust 工具链为 `x86_64-pc-windows-gnu`，需 MinGW-w64 bin 目录在 PATH 中：
```bash
export PATH="$HOME/.cargo/bin:/c/mingw64/mingw64/bin:$PATH"
```

构建产物在 `src-tauri/target/release/bundle/`（.msi + .exe）。

## 外部依赖（运行时）

应用本身不捆绑 JDK 和 bundletool.jar。用户在启动时进行环境检测，缺失时弹窗引导下载：
- **JDK 11+**：需安装并加入 PATH（推荐 [Adoptium](https://adoptium.net/download/)）
- **bundletool.jar**：在设置页面配置路径（[GitHub Releases](https://github.com/google/bundletool/releases)）

## 架构

```
┌── Frontend (Vue 3 + TS + Vite) ──────────────────────────┐
│  src/views/           路由: / (HomeView) + /settings      │
│  src/components/      UI 组件                              │
│  src/stores/          Pinia: convert / env / settings      │
│  src/router/          Vue Router                           │
│  src/types/           TS 类型定义                          │
├─── IPC (invoke) ──────────────────────────────────────────┤
│  invoke("command_name", { ...args }) ←→ Rust handler      │
├── Backend (Rust) ─────────────────────────────────────────┤
│  src-tauri/src/                                            │
│    lib.rs             注册所有 Tauri commands + setup 钩子 │
│    main.rs            Windows 入口                         │
│    config/mod.rs      AppSettings 结构体 + JSON 持久化      │
│    bundletool/mod.rs  构建 bundletool 命令 + .apks 解压    │
│    bundletool/parser.rs  manifest 输出解析                  │
│    commands/                                               │
│      convert.rs       核心: convert_aab (异步, Tauri Event 推送进度) │
│      env_check.rs     check_java / check_bundletool / env_check │
│      keystore.rs      list/add/list_info/validate/parse keystore │
│      device_config.rs list/add device config               │
│      settings.rs      load/save settings, reveal_in_explorer │
│      file_info.rs     read_aab_info / read_apk_info        │
```

### 核心数据流：AAB → APK 转换

1. 前端 `invoke("convert_aab", { aabPath, outputDir, keystoreConfig, deviceSpec })`
2. Rust 构建 `java -jar bundletool.jar build-apks ...` 命令
3. 通过 `app.emit("convert-progress", ...)` 实时推送 stdout/stderr 到前端
4. bundletool 生成 `.apks`（zip），解压提取 `.apk`
5. 按 `<原文件名>-<MMDDHHMM>.apk` 格式重命名放到输出目录
6. 返回 `ConvertResult { apkPath, outputDir, apkFiles }`

### 运行时数据目录

```
<app_data_dir>/          # 由 Tauri path API 解析
  config.json            # 用户设置（bundletool 路径、语言偏好等）
  keystores/             # 证书管理
    <name>.keystore      # 证书文件
    <name>.keystore.json # 对应配置: { keystorePath, storePassword, keyAlias, keyPassword }
  device-configs/        # 设备配置
    <name>.json          # bundletool device-spec JSON
```

启动时自动检查并生成默认文件（debug.keystore、4 个设备预设配置、config.json）。

### 设备配置 `{"mode":"universal"}` 的特殊处理

`convert_aab` 会将 `{"mode":"universal"}` 识别为通用模式，直接传 `--mode=universal` 给 bundletool，不走 `--device-spec` 路径，不生成临时 JSON 文件。

## Tauri v2 注意事项

- **shell 插件配置**: `tauri.conf.json` 中 `plugins.shell` 只支持 `{ "open": true }`，不支持 `scope` 字段。外部命令执行 (`java`, `keytool`) 通过 Rust `std::process::Command` 直接调用，不经过 Tauri shell 插件。
- **拖拽事件**: 使用 Tauri v2 原生事件 `tauri://drag-drop` / `tauri://drag-enter` / `tauri://drag-leave`（通过 `@tauri-apps/api/event` 的 `listen()`），浏览器原生 DnD 无法获取文件系统路径。
- **emit 要求**: `app.emit()` 的 payload 必须实现 `Serialize + Clone`。
- **Manager trait**: 使用 `app.path()` 等 Manager 方法需要 `use tauri::Manager;`。
