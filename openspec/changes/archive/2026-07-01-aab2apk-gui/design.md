## Context

项目从零开始，工作区为空。基于 Tauri v2 + Vue 3 构建跨平台桌面应用，底层调用 bundletool（Google 官方 AAB 工具）完成 AAB 到 APK 的转换。目标用户是 Android 开发者和需要安装 AAB 格式应用的普通用户。

外部依赖：
- **JDK 11+**：运行 bundletool 所必需，通过系统 PATH 检测
- **bundletool.jar**：Google 官方工具，用户可配置路径，约 30MB

## Goals / Non-Goals

**Goals:**
- 提供直观的 GUI 完成 AAB → APK 转换
- 启动时自动检测环境，缺失时清晰引导
- 内置默认证书，零配置即可转换
- 支持自定义 keystore 签名
- 支持设备配置模板导出定制 APK
- 跨平台分发（Windows / macOS / Linux）

**Non-Goals:**
- 不替代 bundletool 的转换逻辑（只做封装）
- 不提供 ADB 设备连接功能（首版）
- 不提供 APK 反编译或修改功能
- 不做在线转换服务
- 不内置 JDK 分发（体积原因）

## Decisions

### 1. 技术栈：Tauri v2 + Vue 3 + TypeScript

```
┌─────────────────────────────────────────────────────────────┐
│                    Tauri v2 Shell                           │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  Frontend (Vue 3 + TypeScript + Vite)                  │ │
│  │                                                        │ │
│  │  src/                                                  │ │
│  │  ├── App.vue                     # 根组件               │ │
│  │  ├── views/                      # 页面视图             │ │
│  │  │   ├── HomeView.vue            # 主转换页             │ │
│  │  │   └── SettingsView.vue        # 设置页               │ │
│  │  ├── components/                 # 通用组件             │ │
│  │  │   ├── FileDropZone.vue        # 拖拽/选择文件区域     │ │
│  │  │   ├── KeystoreSelector.vue    # 证书选择/信息展示     │ │
│  │  │   ├── DeviceConfigPanel.vue   # 设备配置选择          │ │
│  │  │   ├── ConvertProgress.vue     # 转换进度             │ │
│  │  │   ├── EnvCheckDialog.vue      # 环境检测弹窗          │ │
│  │  │   ├── FileInfoPanel.vue       # 文件信息展示          │ │
│  │  │   └── SettingsForm.vue        # 设置表单             │ │
│  │  ├── stores/                     # Pinia 状态管理        │ │
│  │  │   ├── convert.ts              # 转换流程状态          │ │
│  │  │   ├── settings.ts             # 设置状态             │ │
│  │  │   └── env.ts                  # 环境状态             │ │
│  │  ├── types/                      # TypeScript 类型      │ │
│  │  └── assets/                     # 静态资源             │ │
│  └────────────────────────────────────────────────────────┘ │
│                            │ IPC (invoke)                    │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  Backend (Rust)                                        │ │
│  │                                                        │ │
│  │  src-tauri/src/                                        │ │
│  │  ├── main.rs                    # 入口                  │ │
│  │  ├── commands/                  # Tauri 命令            │ │
│  │  │   ├── env_check.rs           # Java/bundletool 检测  │ │
│  │  │   ├── convert.rs             # 转换命令              │ │
│  │  │   ├── keystore.rs            # 证书操作              │ │
│  │  │   ├── file_info.rs           # 文件信息读取           │ │
│  │  │   └── settings.rs            # 配置读写              │ │
│  │  ├── bundletool/                # bundletool 封装       │ │
│  │  │   ├── mod.rs                 # 命令构建与执行         │ │
│  │  │   └── parser.rs              # 输出解析              │ │
│  │  ├── config/                    # 配置管理              │ │
│  │  │   ├── mod.rs                 # 配置结构体            │ │
│  │  │   └── defaults.rs            # 默认值                │ │
│  │  └── resources/                 # 嵌入资源              │ │
│  │      └── debug.keystore         # 默认 debug 证书        │ │
│  └────────────────────────────────────────────────────────┘┘
└─────────────────────────────────────────────────────────────┘
```

**选择理由**：
- Tauri：相比 Electron 包体积小 10 倍以上，Rust 性能好，对 bundletool 的命令行调用更自然
- Vue 3：中文社区活跃，上手快，Composition API 适合中型应用
- TypeScript：类型安全，IPC 接口清晰
- Pinia：Vue 3 官方状态管理，轻量

### 2. 前端路由

单页应用，两个视图：

| 路由 | 视图 | 说明 |
|------|------|------|
| `/` | HomeView | 主转换页面（文件选择、证书、设备配置、进度） |
| `/settings` | SettingsView | 偏好设置（bundletool 路径、默认输出目录） |

环境检测弹窗在 App.vue 根级别，启动时自动弹出。

### 3. 核心 IPC 接口设计

前端通过 Tauri `invoke` 调用 Rust 命令：

```
env_check
  → { java_ok: bool, java_version: string, bundletool_ok: bool, bundletool_path: string }

convert_aab(aab_path, output_dir, keystore_config, device_spec?)
  → 通过 Event 发送进度，最终返回 { apk_path, logs[] }

read_aab_info(aab_path)
  → { package_name, version_name, version_code, min_sdk, target_sdk, permissions[], ... }

read_apk_info(apk_path)
  → { package_name, version_name, version_code, min_sdk, target_sdk, permissions[], ... }

list_keystore_info(keystore_path, store_password)
  → { aliases: [{ name, expires }] }

validate_keystore(keystore_path, store_password, key_alias, key_password)
  → { valid: bool, error?: string }

load_settings() / save_settings(settings)
  → AppSettings JSON

get_default_output_dir()
  → string (系统默认下载目录)
```

### 4. 数据流：AAB → APK 转换

```
用户点击"开始转换"
  │
  ▼
Frontend invoke("convert_aab", { aab_path, output_dir, keystore_config, device_spec })
  │
  ▼
Rust 构建 bundletool 命令:
  java -jar <bundletool_jar> build-apks
    --bundle=<aab_path>
    --output=<temp_dir>/output.apks
    --ks=<keystore_path>
    --ks-pass=pass:<store_password>
    --ks-key-alias=<alias>
    --key-pass=pass:<key_password>
    [--device-spec=<spec_json>]   # 如果有设备配置
    [--mode=universal]             # 如果没有设备配置

  │ 通过 Command::new("java") 执行
  │ stdout/stderr 逐行解析，通过 Tauri Event 推送进度
  ▼
生成 output.apks (zip 文件)
  │
  ▼
解压 output.apks:
  - universal 模式 → 提取 standalones/ 或 universal.apk
  - device-spec 模式 → 提取 splits/ 目录下的所有 .apk
  │
  ▼
复制 .apk 到用户指定的输出目录
  │
  ▼
返回结果 { success: true, apk_path, output_dir }
  → Frontend: "转换完成！打开所在目录？"
```

### 5. 环境检测流程

```
App 启动
  │
  ▼
检查 java --version
  ├── 成功 → 记录 java_version
  └── 失败 → 标记 java_ok = false
  
读取 settings.json 获取 bundletool_jar_path
  ├── 已配置 & 文件存在 → 标记 bundletool_ok = true
  └── 未配置或文件不存在 → 标记 bundletool_ok = false
  
任一检查失败？
  ├── 是 → 弹出 EnvCheckDialog
  │        • Java 缺失: 显示下载指引 (OpenJDK 链接)
  │        • bundletool 缺失: 显示下载指引 (GitHub Releases)
  │        用户可点击"稍后处理"关闭弹窗
  └── 否 → 进入主界面
```

### 6. 配置持久化格式

`~/.aab2apk/config.json`（通过 Tauri path API 获取 app data dir）：

```json
{
  "bundletool_jar_path": "/home/user/tools/bundletool.jar",
  "default_output_dir": "",
  "last_keystore_config": {
    "keystore_path": "",
    "store_password": "",
    "key_alias": "",
    "key_password": ""
  },
  "last_device_template": "universal",
  "language": "zh-CN",
  "check_updates": true
}
```

### 7. Keystore 配置格式

每个 keystore 文件配一个 `.json` 配置文件（用户可选是否使用此格式）：

```json
{
  "keystorePath": "release.keystore",
  "storePassword": "mypassword",
  "keyAlias": "myalias",
  "keyPassword": "mykeypass"
}
```

默认证书 `debug.keystore` 及其配置 `debug.keystore.json` 编译时嵌入 Tauri resources，运行时释放到 app data dir。

### 8. 设备配置模板

预设模板内置在 Rust 代码中，用户可自定义 JSON：

```json
// 通用 (Universal)
{ "mode": "universal" }

// Pixel 8 (arm64-v8a, xxhdpi, SDK 34)
{
  "supportedAbis": ["arm64-v8a"],
  "screenDensity": 480,
  "sdkVersion": 34,
  "supportedLocales": ["zh-CN", "en-US"]
}

// 主流中端机
{
  "supportedAbis": ["arm64-v8a", "armeabi-v7a"],
  "screenDensity": 320,
  "sdkVersion": 28
}
```

### 9. 打包分发

使用 Tauri bundler，CI 矩阵构建三平台产物：

| 平台 | 格式 |
|------|------|
| Windows | `.msi` (安装版) + `.exe` (便携版) |
| macOS | `.dmg` |
| Linux | `.deb` + `.AppImage` |

## Risks / Trade-offs

- **JDK 依赖** → 用户可能需要额外安装 JDK，增加使用门槛。缓解：检测时提供清晰的下载指引和链接
- **bundletool 版本兼容性** → 不同版本的 bundletool 参数可能变化。缓解：设定最低版本要求，检测时验证版本
- **首版操作步骤较多** → 首次使用需配置 JDK + bundletool。缓解：默认证书减少一个配置项，环境检测弹窗提供一键跳转下载页
- **平台打包差异** → macOS 签名和公证需要 Apple Developer 账号。缓解：首版提供未签名的 dmg，文档说明如何打开
- **大文件转换性能** → bundletool 执行可能耗时数分钟。缓解：异步执行 + 进度推送，UI 不阻塞
