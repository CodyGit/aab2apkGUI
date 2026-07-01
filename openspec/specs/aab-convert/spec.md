# aab-convert Specification

## Purpose
TBD - created by archiving change aab2apk-gui. Update Purpose after archive.
## Requirements
### Requirement: 用户可选择并加载 AAB 文件
系统 SHALL 支持以下两种方式加载 AAB 文件：
1. 拖拽 `.aab` 文件到应用指定区域
2. 点击文件选择区域，调用系统文件选择器筛选 `.aab` 文件

#### Scenario: 拖拽 AAB 文件成功
- **WHEN** 用户将 `.aab` 文件拖入文件选择区域
- **THEN** 系统接受文件，显示文件路径和文件名，并自动读取文件基本信息

#### Scenario: 拖拽非 AAB 文件
- **WHEN** 用户将非 `.aab` 文件拖入文件选择区域
- **THEN** 系统拒绝该文件，显示错误提示"请选择 .aab 格式的文件"

#### Scenario: 点击选择 AAB 文件
- **WHEN** 用户点击文件选择区域
- **THEN** 系统打开系统文件选择器，过滤显示 `.aab` 文件，选中后加载文件

### Requirement: 用户可配置签名证书
系统 SHALL 允许用户选择 keystore 文件用于 APK 签名，支持：
1. 使用内置默认 debug 证书（预选中状态）
2. 手动选择 `.keystore` 文件或选择配套的 keystore JSON 配置文件

#### Scenario: 使用默认证书
- **WHEN** 用户不进行证书选择操作
- **THEN** 系统使用内置 debug.keystore，显示"默认证书 (debug)"及其密码与别名信息

#### Scenario: 选择 keystore 文件
- **WHEN** 用户通过浏览按钮选择 `.keystore` 文件
- **THEN** 系统加载该文件，显示文件路径，并提示用户填写 store password、key alias 和 key password

#### Scenario: 选择 keystore JSON 配置文件
- **WHEN** 用户通过浏览按钮选择 keystore JSON 配置文件
- **THEN** 系统解析 JSON 并自动填充 keystore 路径、密码和别名信息

### Requirement: 用户可选择输出目录
系统 SHALL 允许用户选择 APK 输出目录。若用户未选择，默认使用系统下载目录。

#### Scenario: 用户选择输出目录
- **WHEN** 用户点击选择输出目录按钮并选择了一个目录
- **THEN** 系统记录该目录路径并显示在界面上

#### Scenario: 用户未选择输出目录
- **WHEN** 用户没有进行输出目录选择，直接开始转换
- **THEN** 系统使用系统默认下载目录作为输出目录

### Requirement: 执行 AAB 到 APK 的转换
系统 SHALL 通过调用 bundletool 完成 AAB 到 APK 的转换流程。

#### Scenario: Universal 模式转换成功
- **WHEN** 用户已选择 AAB 文件、证书且 Java 环境和 bundletool 均可用，设备模板选择"通用(Universal)"，点击"开始转换"
- **THEN** 系统调用 bundletool build-apks 以 universal 模式生成 .apks 文件，解压后提取 universal.apk 放置到输出目录，显示转换成功并提示可打开所在目录

#### Scenario: 转换过程中显示进度
- **WHEN** 转换正在进行
- **THEN** 系统实时显示 bundletool 的执行日志（stdout/stderr），界面上显示进度指示器和最近日志行

#### Scenario: 转换失败时的错误处理
- **WHEN** bundletool 执行返回非零退出码
- **THEN** 系统捕获错误输出，向用户显示具体的错误信息（如证书密码错误、AAB 文件损坏等）

#### Scenario: Java 环境不可用时的阻断
- **WHEN** 用户在 Java 环境不可用的情况下点击"开始转换"
- **THEN** 系统阻止转换，显示"请先配置 Java 运行环境"的提示，并引导跳转到环境检测

#### Scenario: bundletool 不可用时的阻断
- **WHEN** 用户在 bundletool 路径未配置或文件不存在的情况下点击"开始转换"
- **THEN** 系统阻止转换，显示"请先配置 bundletool 路径"的提示，并引导跳转到设置页面

### Requirement: 转换完成后打开输出目录
系统 SHALL 在转换成功后提供"打开所在目录"功能。

#### Scenario: 点击打开所在目录
- **WHEN** 转换成功完成，用户点击"打开所在目录"按钮
- **THEN** 系统调用操作系统的文件管理器打开 APK 文件所在目录，并选中该 APK 文件

