## ADDED Requirements

### Requirement: 设置改为弹窗
系统 SHALL 将设置页替换为点击 header 齿轮按钮触发的弹窗。

#### Scenario: 打开设置弹窗
- **WHEN** 用户点击 header 中的齿轮按钮
- **THEN** 弹出设置弹窗，包含 bundletool.jar 路径配置和"重新检测环境"按钮

#### Scenario: 保存设置
- **WHEN** 用户在设置弹窗中点击"保存"
- **THEN** 设置保存到本地配置，环境状态自动更新，弹窗关闭

#### Scenario: 取消设置
- **WHEN** 用户在设置弹窗中点击"取消"
- **THEN** 弹窗关闭，未保存的修改丢弃

### Requirement: 空状态引导
系统 SHALL 在用户未选择文件时展示更醒目的操作引导。

#### Scenario: 文件选择区空状态
- **WHEN** 用户尚未选择 AAB 文件
- **THEN** 文件拖拽/选择区域显示大图标和文字"拖拽 .aab 文件到此处，或点击选择"，占据卡片中央

#### Scenario: 环境未就绪引导
- **WHEN** Java 或 bundletool 环境未就绪
- **THEN** 转换按钮下方显示引导信息，说明需要解决环境问题，附带跳转设置链接

### Requirement: 成功状态展示
系统 SHALL 在转换成功后自动展开日志并突出显示成功结果。

#### Scenario: 转换成功自动展开日志
- **WHEN** 转换成功完成
- **THEN** 日志区域自动展开，顶部显示绿色成功提示条："✓ 转换完成！"

#### Scenario: APK 文件链接
- **WHEN** 转换成功后在日志中展示 APK 文件
- **THEN** APK 文件名以蓝色链接显示，点击可打开文件所在目录

### Requirement: 键盘快捷键
系统 SHALL 支持以下键盘快捷键：Enter 开始转换，Escape 关闭弹窗。

#### Scenario: Enter 开始转换
- **WHEN** 用户已选择 AAB 文件且环境就绪，按下 Enter 键
- **THEN** 触发开始转换（等同于点击"开始转换"按钮）

#### Scenario: Enter 不触发（条件不满足）
- **WHEN** 用户未选择 AAB 文件或环境未就绪，按下 Enter 键
- **THEN** 不触发任何操作

#### Scenario: Escape 关闭弹窗
- **WHEN** 任一弹窗（设置、环境检测、添加证书、添加配置）打开时，用户按下 Escape 键
- **THEN** 当前打开的弹窗关闭
