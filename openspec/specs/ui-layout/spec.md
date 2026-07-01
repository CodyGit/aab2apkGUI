# ui-layout Specification

## Purpose
TBD - created by archiving change ui-overhaul. Update Purpose after archive.
## Requirements
### Requirement: 紧凑布局
系统 SHALL 将页面间距和 padding 压缩，使所有交互元素在一屏内展示，不出现纵向滚动条。

#### Scenario: 所有内容一屏可见
- **WHEN** 用户在 700×500 的窗口中打开主页
- **THEN** 所有 UI 元素（文件选择区、证书配置、设备配置、输出目录、转换按钮）在一屏内完整显示，无滚动条

#### Scenario: Section 间距压缩
- **WHEN** 页面渲染各功能区块
- **THEN** 区块之间的间距从 20px 减少到 12px，区块内 padding 从 16px 减少到 12px

### Requirement: 窗口尺寸调整
系统 SHALL 将默认窗口大小调整至约 700×500，最小 600×420。

#### Scenario: 启动应用
- **WHEN** 用户首次启动应用
- **THEN** 窗口大小为 700×500 像素，居中显示

#### Scenario: 窗口最小尺寸
- **WHEN** 用户尝试缩小窗口
- **THEN** 窗口宽度不小于 600px，高度不小于 420px

### Requirement: Header 简化
系统 SHALL 将顶部导航栏简化为应用名称 + 环境状态指示灯 + 设置按钮。

#### Scenario: Header 内容
- **WHEN** 用户查看顶部区域
- **THEN** 左侧显示应用图标和名称，右侧显示 Java 和 bundletool 状态圆点（●/○）和齿轮设置按钮，导航链接已移除

#### Scenario: Header 高度
- **WHEN** 页面渲染 header
- **THEN** header 高度为 36px（原 48px）

### Requirement: 输出目录降级
系统 SHALL 将输出目录从独立 section 改为转换按钮上方的单行文本。

#### Scenario: 输出目录显示
- **WHEN** 用户选择了 AAB 文件
- **THEN** 转换按钮上方显示"输出至: /path/to/dir"单行文本，右侧有"浏览..."按钮可更改目录

