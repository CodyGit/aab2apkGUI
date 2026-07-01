# file-info Specification

## Purpose
TBD - created by archiving change aab2apk-gui. Update Purpose after archive.
## Requirements
### Requirement: 查看 AAB 文件信息
系统 SHALL 支持加载 AAB 文件后展示其基本信息。

#### Scenario: 加载 AAB 后自动显示信息
- **WHEN** 用户选择了 AAB 文件
- **THEN** 系统通过 bundletool dump manifest 读取并显示以下信息：包名、版本名称、版本号、最低 SDK 版本、目标 SDK 版本

#### Scenario: 读取 AAB 失败
- **WHEN** AAB 文件损坏或无法读取
- **THEN** 系统显示"无法读取 AAB 文件信息，文件可能已损坏"

### Requirement: 查看 APK 文件信息
系统 SHALL 支持加载 APK 文件后展示其基本信息。

#### Scenario: 查看生成的 APK 信息
- **WHEN** 用户点击"查看 APK 信息"并选择了 APK 文件
- **THEN** 系统通过 bundletool dump manifest 读取并显示：包名、版本名称、版本号、最低 SDK 版本、目标 SDK 版本、权限列表

#### Scenario: 读取 APK 失败
- **WHEN** APK 文件损坏或无法读取
- **THEN** 系统显示"无法读取 APK 文件信息，文件可能已损坏"

