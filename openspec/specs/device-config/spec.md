# device-config Specification

## Purpose
TBD - created by archiving change aab2apk-gui. Update Purpose after archive.
## Requirements
### Requirement: 提供预设设备模板
系统 SHALL 提供至少 3 个预设设备配置模板供用户选择，默认选中"通用 (Universal)"。

#### Scenario: 查看并选择预设模板
- **WHEN** 用户点击设备配置下拉菜单
- **THEN** 系统显示预设模板列表（Universal、Pixel 系列、主流中端机、纯 64 位），用户可选择一项

#### Scenario: Universal 为默认选项
- **WHEN** 用户打开应用
- **THEN** 设备配置默认选中"通用 (Universal)"模式，生成单个包含所有资源的 APK

### Requirement: 自定义设备配置
系统 SHALL 允许用户输入自定义的 bundletool device-spec JSON 配置。

#### Scenario: 输入自定义 JSON 配置
- **WHEN** 用户选择"自定义"模板并输入合法的 device-spec JSON
- **THEN** 系统使用该 JSON 进行转换

#### Scenario: 输入非法 JSON 格式
- **WHEN** 用户输入的 JSON 格式不正确
- **THEN** 系统显示"JSON 格式错误"，阻止转换

### Requirement: 根据设备配置导出 APK
系统 SHALL 根据用户选择的设备模板或自定义配置，使用 bundletool 的 --device-spec 参数生成对应设备的 APK。

#### Scenario: 按设备模板生成 APK
- **WHEN** 用户选择非 Universal 的设备模板并开始转换
- **THEN** 系统生成包含对应 ABI、屏幕密度、SDK 版本的 APK（可能包含多个 split APK），并告知用户生成的 APK 列表

#### Scenario: Universal 模式生成 APK
- **WHEN** 用户选择 Universal 模板并开始转换
- **THEN** 系统生成单个 universal.apk 文件

