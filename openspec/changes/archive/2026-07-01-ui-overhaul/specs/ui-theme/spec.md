## ADDED Requirements

### Requirement: 明亮灰白配色方案
系统 SHALL 使用明亮灰白配色方案，替换原有的暗色主题。

#### Scenario: 页面背景和卡片
- **WHEN** 用户打开应用
- **THEN** 页面背景为浅灰色（#f5f5f5），卡片为白色（#ffffff），边框为浅灰（#e5e7eb）

#### Scenario: 文字颜色
- **WHEN** 页面展示任何文字
- **THEN** 主要文字为深灰色（#1f2937），次要文字为灰色（#6b7280）

### Requirement: 按钮和链接使用蓝色系
系统 SHALL 将主按钮和链接的强调色从红色改为蓝色系（#2563eb）。

#### Scenario: 主按钮样式
- **WHEN** 页面渲染主操作按钮（如"开始转换"）
- **THEN** 按钮背景色为蓝色（#2563eb），hover 时稍微变深

#### Scenario: 链接样式
- **WHEN** 页面渲染可点击链接（如"查看证书目录"）
- **THEN** 链接文字颜色为蓝色（#2563eb），hover 时带下划线

### Requirement: 成功和错误状态色
系统 SHALL 使用绿色（#16a34a）表示成功，红色（#dc2626）表示错误。

#### Scenario: 状态指示
- **WHEN** 显示成功或错误状态
- **THEN** 成功使用绿色，错误使用红色
