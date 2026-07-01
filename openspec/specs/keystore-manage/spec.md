# keystore-manage Specification

## Purpose
TBD - created by archiving change aab2apk-gui. Update Purpose after archive.
## Requirements
### Requirement: 使用默认证书
系统 SHALL 内置一份 debug.keystore 及其对应的 JSON 配置文件，用户无需任何操作即可使用该证书进行签名。

#### Scenario: 默认证书预选中
- **WHEN** 用户打开应用
- **THEN** 证书选择区域显示"默认证书 (debug)"为已选中状态

#### Scenario: 默认证书信息展示
- **WHEN** 默认证书处于选中状态
- **THEN** 界面显示证书的别名（alias）和有效期等基本信息

### Requirement: 选择自定义 keystore
系统 SHALL 允许用户选择自定义的 keystore 文件或 keystore JSON 配置文件。

#### Scenario: 浏览并选择 keystore 文件
- **WHEN** 用户点击"选择证书"并选择了一个 `.keystore` 或 `.jks` 文件
- **THEN** 系统显示文件路径，用户需手动输入 store password、key alias 和 key password

#### Scenario: 加载 keystore JSON 配置
- **WHEN** 用户点击"选择证书"并选择了一个 keystore JSON 配置文件
- **THEN** 系统解析 JSON，自动填充 keystore 路径、密码和别名到对应字段

### Requirement: 查看 keystore 信息
系统 SHALL 允许用户在选择了 keystore 后查看其包含的别名列表和证书有效期。

#### Scenario: 查看证书别名和有效期
- **WHEN** 用户提供了 keystore 和 store password，点击"查看证书信息"
- **THEN** 系统调用 keytool -list 读取 keystore，显示所有别名及对应的证书到期日期

#### Scenario: 密码错误时查看失败
- **WHEN** 用户提供了错误的 store password 并点击"查看证书信息"
- **THEN** 系统显示"证书密码错误，请重试"

### Requirement: 验证 keystore 有效性
系统 SHALL 在转换开始前验证所选 keystore 的密码和别名是否有效。

#### Scenario: 证书验证通过
- **WHEN** 用户填写的 store password、key alias 和 key password 均正确
- **THEN** 系统确认证书可用，允许开始转换

#### Scenario: 证书验证失败
- **WHEN** 用户填写的密码或别名有误
- **THEN** 系统提示具体的错误原因（密码错误 / 别名不存在），阻止转换

