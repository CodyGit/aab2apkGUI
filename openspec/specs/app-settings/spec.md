# app-settings Specification

## Purpose
TBD - created by archiving change aab2apk-gui. Update Purpose after archive.
## Requirements
### Requirement: bundletool 路径配置
系统 SHALL 允许用户在设置页面配置 bundletool.jar 文件路径。

#### Scenario: 配置 bundletool 路径
- **WHEN** 用户在设置页面浏览并选择 bundletool.jar 文件，点击保存
- **THEN** 系统保存路径到配置 JSON 文件，并在主界面的环境状态中更新为"已配置"

#### Scenario: 清空 bundletool 路径
- **WHEN** 用户清空 bundletool 路径配置
- **THEN** 系统将配置项设为空字符串，环境状态更新为"未配置"

### Requirement: 默认输出目录配置
系统 SHALL 允许用户在设置页面配置默认输出目录。

#### Scenario: 配置默认输出目录
- **WHEN** 用户在设置页面选择输出目录并保存
- **THEN** 系统将目录路径保存到配置 JSON 文件，主界面转换时使用该目录作为默认值

### Requirement: 偏好设置持久化
系统 SHALL 将所有用户偏好设置保存为 JSON 文件，并在下次启动时自动加载。

#### Scenario: 保存设置
- **WHEN** 用户点击"保存设置"
- **THEN** 系统将当前设置写入 `~/.aab2apk/config.json` 文件，显示"保存成功"

#### Scenario: 启动时自动加载设置
- **WHEN** 应用启动
- **THEN** 系统读取 config.json 文件加载所有偏好设置，并应用到对应界面状态

#### Scenario: 首次启动无配置文件
- **WHEN** 应用首次启动且配置文件不存在
- **THEN** 系统使用内置默认值创建配置文件

### Requirement: 记忆上次使用的证书和设备模板
系统 SHALL 在转换完成后自动记忆用户选择的证书配置和设备模板，下次启动时恢复。

#### Scenario: 记忆用户选择
- **WHEN** 用户使用了一个自定义 keystore 和非 Universal 设备模板完成了一次转换
- **THEN** 系统在配置文件中保存这些选择，下次启动时自动恢复

