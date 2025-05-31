# FurinaOCR 开发贡献指南

感谢您对 FurinaOCR 项目的关注！我们欢迎所有形式的贡献，无论是代码、文档、问题报告还是功能建议。

## 📋 目录

- [行为准则](#行为准则)
- [如何贡献](#如何贡献)
- [开发环境设置](#开发环境设置)
- [项目架构](#项目架构)
- [代码规范](#代码规范)
- [提交规范](#提交规范)
- [CI/CD工作流](#ci-cd工作流)
- [测试要求](#测试要求)
- [代码审查](#代码审查)

## 🤝 行为准则

### 我们的承诺

为了营造一个开放和友好的环境，我们作为贡献者和维护者承诺，无论年龄、体型、残疾、种族、性别认同和表达、经验水平、国籍、个人形象、种族、宗教或性取向如何，参与我们项目和社区的每个人都能享受无骚扰的体验。

### 我们的标准

有助于创造积极环境的行为包括：
- 使用友好和包容的语言
- 尊重不同的观点和经验
- 优雅地接受建设性批评
- 关注对社区最有利的事情
- 对其他社区成员表示同情

不可接受的行为包括：
- 使用性化的语言或图像，以及不受欢迎的性关注或性骚扰
- 恶意评论、人身攻击或政治攻击
- 公开或私下骚扰
- 未经明确许可发布他人的私人信息
- 在专业环境中可能被认为不合适的其他行为

### 执行

如果您遇到不当行为，请联系项目团队：Furina@FurinaQvQ.top

## 🚀 如何贡献

### 贡献类型

我们欢迎以下类型的贡献：

1. **🐛 问题报告** - 报告 bug 或问题
2. **💡 功能请求** - 建议新功能或改进
3. **🔧 代码贡献** - 修复 bug 或实现新功能
4. **📝 文档改进** - 改进文档质量
5. **🌐 翻译** - 帮助翻译项目
6. **🧪 测试** - 编写或改进测试
7. **🎨 设计** - UI/UX 设计改进

### 贡献流程

1. **🍴 Fork 仓库**
   ```powershell
   # 在 GitHub 上 fork 仓库，然后克隆到本地
   git clone https://github.com/YOUR_USERNAME/FurinaOCR.git
   cd FurinaOCR
   ```

2. **🌿 创建分支**
   ```powershell
   git checkout -b feature/your-feature-name
   # 或者修复 bug
   git checkout -b bugfix/issue-number
   ```

3. **💻 进行开发**
   - 遵循代码规范
   - 编写测试
   - 更新文档

4. **✅ 本地验证**
   ```powershell
   # 运行质量检查
   .\scripts\quality-check.ps1 -Quick
   ```

5. **💾 提交更改**
   ```powershell
   git add .
   git commit -m "feat: 添加你的功能描述"
   ```

6. **📤 推送分支并创建 PR**

## 🛠️ 开发环境设置

### 系统要求

- **操作系统**: Windows 10/11 (64位) - 项目专为Windows平台设计
- **Rust**: nightly 版本 (项目使用了一些 nightly 特性)
- **Git**: 版本控制
- **Visual Studio Build Tools**: Windows开发必需的C++构建工具

### 安装步骤

1. **安装 Visual Studio Build Tools**
   ```powershell
   # 下载并安装 Visual Studio Build Tools
   # https://visualstudio.microsoft.com/visual-cpp-build-tools/
   # 确保安装 "C++ build tools" 工作负载
   ```

2. **安装 Rust nightly 工具链**
   ```powershell
   # 方式一：访问 https://rustup.rs/ 下载安装
   # 方式二：PowerShell 命令安装
   Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "rustup-init.exe"
   ./rustup-init.exe
   
   # 安装完成后
   rustup install nightly
   rustup default nightly
   rustup component add rustfmt clippy
   ```

3. **克隆项目**
   ```powershell
   git clone --recursive https://github.com/FurinaQvQ/FurinaOCR.git
   cd FurinaOCR
   ```

4. **构建项目**
   ```powershell
   cargo build
   cargo build --release
   ```

5. **安装开发工具**
   ```powershell
   # 安装Git预提交钩子
   .\scripts\install-hooks.ps1
   
   # 安装可选工具
   cargo install cargo-watch cargo-audit cargo-outdated
   ```

### 推荐开发工具

- **IDE**: VS Code + rust-analyzer 插件
- **Git GUI**: GitHub Desktop 或 SourceTree
- **终端**: Windows Terminal
- **调试器**: VS Code 内置调试器 + CodeLLDB 扩展

## 🏗️ 项目架构

### 模块设计原则

1. **单一职责原则**: 每个模块只负责一个特定功能
2. **依赖倒置**: 高层模块不依赖低层模块，都依赖抽象
3. **接口隔离**: 使用 trait 定义清晰的接口
4. **开闭原则**: 对扩展开放，对修改封闭

### 核心模块架构

```
furina_core/
├── capture/          # 屏幕截图抽象层
├── ocr/             # OCR 识别引擎
├── positioning/     # 位置计算算法
├── system_control/  # 系统控制接口
├── window_info/     # 窗口信息管理
├── export/          # 数据导出框架
├── game_info/       # 游戏信息检测
├── common/          # 通用数据结构
└── utils/           # 工具函数集合
```

### 依赖关系

```
application → genshin → furina_core
                ↓
              derive
```

## 📝 代码规范

### Rust 代码风格

1. **命名规范**
   ```rust
   // 结构体和枚举：PascalCase
   struct ArtifactScanner;
   enum ScanResult;
   
   // 函数和变量：snake_case
   fn scan_artifact() -> Result<()>;
   let scan_result = scanner.scan();
   
   // 常量：SCREAMING_SNAKE_CASE
   const MAX_RETRY_COUNT: usize = 3;
   
   // 模块：snake_case
   mod artifact_scanner;
   ```

2. **代码格式**
   - 使用 rustfmt 默认配置
   - 行长度限制：100 字符
   - 缩进：4 个空格
   - 尾随逗号：保留

3. **错误处理**
   ```rust
   use anyhow::{Result, Context};
   
   fn scan_item() -> Result<Item> {
       let data = capture_screen()
           .context("Failed to capture screen")?;
       
       let result = ocr_recognize(&data)
           .context("OCR recognition failed")?;
       
       Ok(result)
   }
   ```

### 文档注释规范

```rust
/// 扫描单个圣遗物
///
/// 对指定位置的圣遗物进行 OCR 识别，返回结构化数据。
///
/// # 参数
///
/// * `position` - 圣遗物在屏幕上的位置
/// * `config` - 扫描配置参数
///
/// # 返回值
///
/// 成功时返回 `Ok(ArtifactData)`，失败时返回错误信息。
///
/// # 错误
///
/// 当以下情况发生时会返回错误：
/// - 屏幕截图失败
/// - OCR 识别失败
/// - 数据解析错误
pub fn scan_artifact(
    position: Position,
    config: &ScanConfig,
) -> Result<ArtifactData> {
    // 实现
}
```

## 📝 提交规范

我们使用 [约定式提交](https://www.conventionalcommits.org/zh-hans/v1.0.0/) 规范。

### 提交格式

```
<类型>(<作用域>): <描述>

[可选的正文]

[可选的脚注]
```

### 类型说明

- `feat`: 新功能
- `fix`: 修复 bug
- `docs`: 文档更新
- `style`: 代码格式调整（不影响功能）
- `refactor`: 重构代码（不是新功能也不是修复）
- `perf`: 性能优化
- `test`: 添加或修改测试
- `chore`: 构建工具或辅助工具的变动
- `ci`: CI/CD 相关更改

### 作用域示例

- `scanner`: 扫描器相关
- `ocr`: OCR 识别相关
- `export`: 导出功能相关
- `config`: 配置相关
- `docs`: 文档相关

### 提交示例

```powershell
# 新功能
git commit -m "feat(scanner): 添加自适应时序调整功能"

# 修复 bug
git commit -m "fix(ocr): 修复特殊字符识别错误"

# 文档更新
git commit -m "docs: 更新安装指南"
```

## 🔄 CI/CD工作流

### 质量检查命令

```powershell
# 快速检查（推荐在提交前使用）
.\scripts\quality-check.ps1 -Quick

# 完整检查（推荐在推送前使用）
.\scripts\quality-check.ps1 -Full

# 自动修复格式问题
.\scripts\quality-check.ps1 -Fix
```

### 自动化检查

- **代码格式**: 自动检查和修复代码格式
- **静态分析**: Clippy零警告策略
- **安全审计**: 依赖漏洞扫描
- **测试覆盖**: 147个测试用例，100%通过率
- **性能基准**: 防止性能回归

### 开发工作流

1. **日常开发**: 在功能分支进行开发
2. **本地验证**: 运行 `.\scripts\quality-check.ps1 -Quick`
3. **提交代码**: Git钩子自动进行质量检查
4. **推送代码**: CI/CD自动运行完整检查

### 质量标准

| 检查项 | 要求 | 状态 |
|--------|------|------|
| 编译错误 | 0个 | ✅ |
| Clippy警告 | 0个 | ✅ |
| 测试通过率 | 100% | ✅ |
| 代码覆盖率 | ≥85% | ✅ |
| 安全漏洞 | 0个 | ✅ |

## 🧪 测试要求

详细的测试指南请参考 **[测试文档](docs/TESTING.md)**。

### 测试运行

```powershell
# 运行所有测试
cargo test --all

# 运行特定测试
cargo test test_name -- --nocapture

# 性能基准测试
cargo bench --bench string_optimization_bench
```

### 测试覆盖范围

- 68 个 furina_core 单元测试
- 20 个 genshin 单元测试
- 9 个集成测试
- 2 个文档测试

### 测试编写规范

- 为新功能编写单元测试
- 关键路径需要集成测试
- 性能敏感代码需要基准测试
- 公共API需要文档测试

## 🔍 代码审查

### 审查清单

- [ ] 代码遵循项目规范
- [ ] 包含适当的测试
- [ ] 文档已更新
- [ ] 性能影响可接受
- [ ] 安全考虑充分

### 审查流程

1. 创建 Pull Request
2. 自动运行 CI 检查
3. 代码审查员审查代码
4. 解决审查意见
5. 合并到主分支

## 🐛 故障排除

### 代码格式问题
```powershell
cargo fmt --all
```

### Clippy警告
```powershell
cargo clippy --all-targets -- -D warnings
cargo clippy --fix --all-targets --allow-dirty
```

### 测试失败
```powershell
cargo test --all --verbose
```

## 📞 获取帮助

- 📋 [Issues](https://github.com/FurinaQvQ/FurinaOCR/issues): 报告 Bug 或请求功能
- 💬 [Discussions](https://github.com/FurinaQvQ/FurinaOCR/discussions): 社区讨论和交流
- 📧 Email: Furina@FurinaQvQ.top

---

感谢您的贡献！🎉 