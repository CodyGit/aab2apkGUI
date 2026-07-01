## ADDED Requirements

### Requirement: 启动时检测 Java 环境
系统 SHALL 在应用启动时自动检测系统是否安装了 Java 运行环境（JDK 11+）。

#### Scenario: Java 环境已安装
- **WHEN** 系统 PATH 中存在 `java` 命令且版本 >= 11
- **THEN** 系统记录 Java 环境可用，不弹出 Java 相关提示

#### Scenario: Java 环境未安装
- **WHEN** 系统 PATH 中不存在 `java` 命令或版本 < 11
- **THEN** 系统标记 Java 环境不可用，应用启动后弹出环境检测对话框，显示"未检测到 Java 运行环境"

### Requirement: 启动时检测 bundletool
系统 SHALL 在应用启动时根据用户保存的 bundletool.jar 路径检测 bundletool 是否可用。

#### Scenario: bundletool 路径已配置且文件存在
- **WHEN** 配置文件中 bundletool_jar_path 指向的文件存在
- **THEN** 系统记录 bundletool 可用

#### Scenario: bundletool 未配置或文件不存在
- **WHEN** 配置文件中 bundletool_jar_path 为空或指向的文件不存在
- **THEN** 系统标记 bundletool 不可用，应用启动后弹出环境检测对话框，显示"未找到 bundletool"

### Requirement: 环境缺失引导
系统 SHALL 在环境缺失时向用户显示清晰的下载和安装指引。

#### Scenario: 显示 Java 安装指引
- **WHEN** Java 环境未安装
- **THEN** 环境检测对话框显示"请安装 JDK 11 或更高版本"并提供下载链接（如 Adoptium OpenJDK），包含简要的安装说明

#### Scenario: 显示 bundletool 下载指引
- **WHEN** bundletool 未找到
- **THEN** 环境检测对话框显示"请下载 bundletool"并提供 GitHub Releases 下载链接

#### Scenario: 用户暂缓处理环境问题
- **WHEN** 用户在环境检测对话框中点击"稍后处理"
- **THEN** 对话框关闭，应用进入主界面，用户仍可在设置页面重新检查和配置
