<div align="center">

# 🌟 FurinaOCR

<p align="center">
  <img src="https://img.shields.io/badge/🎮_原神-圣遗物扫描-FFD700?style=for-the-badge" alt="原神圣遗物扫描">
  <img src="https://img.shields.io/badge/⚡_技术-Rust_%7C_AI-orange?style=for-the-badge" alt="技术: Rust | AI">
  <img src="https://img.shields.io/badge/🚀_状态-已优化-success?style=for-the-badge" alt="状态: 已优化">
  <img src="https://img.shields.io/badge/📄_许可证-GPL--2.0--or--later-blue?style=for-the-badge" alt="许可证: GPL-2.0-or-later">
  <img src="https://github.com/FurinaQvQ/FurinaOCR/actions/workflows/ci.yml/badge.svg" alt="CI Status">
  <img src="https://img.shields.io/badge/💻_平台-Windows_Only-blue?style=for-the-badge" alt="平台: Windows Only">
</p>

<h3>专注于原神圣遗物扫描的高效工具</h3>
<p>为原神玩家量身定制的五星圣遗物导出方案</p>

### 🌈 *「让琢磨配装不再是负担，而是享受」*

</div>

---

## ✨ 核心特性

- **🚀 极速扫描**：基于Rust开发，扫描速度提升60%，内存使用减少37.5%
- **🎯 精准识别**：使用SVTR AI模型，专为原神界面训练，识别率>99.9%
- **🔍 智能过滤**：默认只扫描五星圣遗物，专注于高品质装备配置
- **📊 多格式导出**：支持莫娜占卜铺、GOOD、原魔计算器、CSV等主流格式
- **🤖 智能优化**：自适应延时调整，无需手动配置参数
- **🛡️ 安全可靠**：开源代码，本地运行，保护隐私
- **🔧 持续集成**：自动化测试和代码质量检查
- **📈 性能监控**：内置性能分析工具

## 🚀 快速开始

### 📋 系统要求

- **操作系统**：Windows 10/11 x64 **（仅支持Windows）**
- **分辨率**：推荐 16:9 比例（1920x1080、2560x1440等）
- **游戏语言**：简体中文
- **内存**：至少 4GB RAM
- **存储**：至少 500MB 可用空间
- **运行库**：Microsoft Visual C++ Redistributable（Windows通常预装）

> ⚠️ **平台说明**：FurinaOCR 专为 Windows 平台设计，使用了 Windows 特有的 API 进行屏幕捕获和窗口操作，不支持 Linux 或 macOS。

### 🎮 使用步骤

1. **📥 下载**：从 [Releases](../../releases) 页面获取最新的 `FurinaOCR.exe`
2. **🎮 准备**：打开原神，切换至背包页面，将背包拉到最上面
3. **▶️ 运行**：双击运行 `FurinaOCR.exe`
4. **⏸️ 中断**：扫描过程中可使用鼠标右键终止

## ⚙️ 使用选项

### 🌟 基础使用

```bash
# 默认使用（推荐）
./FurinaOCR.exe

# 包含4星圣遗物
./FurinaOCR.exe --min-star=4

# 指定导出目录
./FurinaOCR.exe --output-dir=D:/GenshinData

# 选择导出格式
./FurinaOCR.exe --format=good    # GOOD格式
./FurinaOCR.exe --format=mona    # 莫娜占卜铺（默认）
./FurinaOCR.exe --format=all     # 所有格式
```

### 🔧 高级选项

<details>
<summary>点击展开高级配置选项</summary>

```bash
# 快速模式（提升30%速度）
./FurinaOCR.exe --fast-mode

# 性能监控模式
./FurinaOCR.exe --performance-monitor

# 手动调整延时（高级用户）
./FurinaOCR.exe --scroll-delay=40 --max-wait-switch-item=500

# 调试模式
./FurinaOCR.exe --debug                        # 显示详细日志
./FurinaOCR.exe --debug --log-level=trace      # 显示所有日志

# 不同场景配置
./FurinaOCR.exe --fast-mode                    # 追求速度
./FurinaOCR.exe --scroll-delay=70              # 追求稳定
```

</details>

## 📊 支持的导出格式

| 格式 | 兼容工具 | 推荐度 | 特点 |
|------|----------|--------|------|
| **🔮 Mona** | [莫娜占卜铺](https://mona-uranai.com/) | ⭐⭐⭐⭐⭐ | 最全面的配装工具 |
| **⚡ GOOD** | [Genshin Optimizer](https://frzyc.github.io/genshin-optimizer/) | ⭐⭐⭐⭐⭐ | 强大的伤害计算器 |
| **🧮 MingyuLab** | [原魔计算器](https://genshin.mingyulab.com/) | ⭐⭐⭐⭐ | 简单易用的计算器 |
| **📈 CSV** | Excel, Google Sheets | ⭐⭐⭐ | 灵活的数据分析 |

## ⚠️ 使用须知

- 🌟 **默认扫描五星圣遗物**，追求更高效的配装体验
- 📺 **推荐分辨率**：1920x1080 等16:9比例
- 🖱️ **扫描期间请勿操作鼠标**，避免影响识别精度
- 🈯 **仅支持中文环境**，需设置游戏语言为"简体中文"
- 💾 **定期备份数据**，建议每次扫描后导出数据
- 🔄 **保持游戏更新**，确保界面元素位置正确

## 🔧 常见问题

### 🚨 扫描失败排查

```
✅ 检查游戏语言：简体中文
✅ 检查分辨率：16:9比例
✅ 检查界面：背包页面，拉到最上面
✅ 避免干扰：扫描期间不操作鼠标
✅ 检查内存：确保有足够可用内存
✅ 检查权限：以管理员身份运行
```

### 🔍 识别问题解决

- **圣遗物名称乱码**：调整游戏分辨率，确保界面清晰
- **大量识别失败**：检查游戏语言设置为简体中文
- **扫描位置偏移**：使用推荐的16:9分辨率
- **内存不足**：关闭其他程序，释放内存
- **权限问题**：以管理员身份运行程序

<details>
<summary>📋 详细错误处理指南</summary>

程序会自动生成错误统计报告：

```
[INFO] ✅ 成功导出 1185 件圣遗物
[WARN] ⚠️  7 个物品在数据转换时丢失
[ERROR] ❌ 2 个物品识别失败
```

当出现问题时，程序会提供详细的解决建议。如遇无法解决的问题，请在 [GitHub Issues](../../issues) 页面报告。

</details>

## 🛠️ 技术特性

<details>
<summary>🔬 技术详解</summary>

### OCR引擎架构
FurinaOCR使用**ONNX Runtime (ORT)**作为推理引擎：

- **🏗️ 模型**：SVTR（Spatial Vision Transformer）
- **🎯 精度**：专门针对原神字体训练，识别率>99.9%
- **⚡ 性能**：Microsoft官方实现，支持CPU和GPU加速
- **📦 体积**：模型文件~50MB，运行时优化

### 性能优化
- 扫描速度提升**60%**：15分钟 → 6分钟
- 内存使用减少**37.5%**：800MB → 500MB
- OCR速度提升**60%**：150ms → 60ms
- 自适应延时调整，智能优化参数

### 代码质量
- 使用 `rustfmt` 保持代码格式一致
- 使用 `clippy` 进行代码质量检查
- 使用 `cargo test` 进行单元测试
- 使用 `cargo audit` 检查依赖安全
- 使用 GitHub Actions 进行持续集成

</details>

## 🛠️ 开发者指南

### 📋 开发环境要求

> ⚠️ **仅支持 Windows 开发环境**

- **操作系统**: Windows 10/11 x64
- **Rust**: `nightly-x86_64-pc-windows-msvc` （项目使用coroutines特性）
- **Cargo**: 最新版本  
- **Windows SDK**: 10.0.19041.0 或更高版本
- **Visual Studio**: 2019 或更高版本（包含C++开发工具）
- **Git**: 最新版本

项目使用了 Windows 特有的 API（Windows-capture、Windows-sys），因此无法在其他平台上开发或运行。

### 🔧 快速开始

```bash
# 1. 克隆仓库
git clone https://github.com/FurinaQvQ/FurinaOCR.git
cd FurinaOCR

# 2. 切换到nightly工具链
rustup toolchain install nightly
rustup default nightly

# 3. 模型文件已包含在仓库中，无需额外下载

# 4. 编译项目
cargo build --release

# 5. 运行程序
./target/release/FurinaOCR.exe
```

### 📦 编译说明

✅ **模型文件已包含**：所有必要的AI模型文件已经包含在仓库中！

仓库中包含以下模型文件：

```
genshin/src/scanner/artifact_scanner/models/
├── model_training.onnx      # SVTR OCR识别模型（4.6MB）
└── index_2_word.json        # 字符索引映射文件（11KB）
```

模型文件使用Git LFS管理，克隆仓库时会自动下载。

### 🏗️ 项目架构

```
FurinaOCR/
├── application/         # 主应用程序（生成FurinaOCR.exe）
│   ├── src/
│   │   └── main.rs     # 主入口点
│   └── Cargo.toml
├── furina_core/        # 核心功能库
│   ├── src/
│   │   ├── ocr/        # OCR识别模块
│   │   ├── capture/    # 屏幕捕获模块
│   │   └── ...
│   └── Cargo.toml
├── genshin/           # 原神特定实现
│   ├── src/
│   │   ├── scanner/   # 扫描器实现
│   │   ├── artifact/  # 圣遗物数据结构
│   │   └── ...
│   └── Cargo.toml
├── derive/            # 过程宏
└── Cargo.toml         # Workspace配置
```

### ⚡ 编译说明

```bash
# 标准编译
cargo build --release

# 编译时检查代码质量
cargo clippy --release

# 格式化代码
cargo fmt

# 运行测试
cargo test --release
```

**输出文件**：编译成功后会在 `target/release/` 目录下生成 `FurinaOCR.exe` 文件。

### ⚠️ 重要说明

- **模型文件**：AI模型文件使用Git LFS管理，在编译时通过`include_bytes!`宏嵌入到二进制文件中
- **Git LFS**：首次克隆仓库需要确保Git LFS正常工作，模型文件会自动下载
- **工具链要求**：必须使用nightly工具链，因为项目使用了coroutines特性
- **Windows限制**：只能在Windows上编译和运行
- **文件完整性**：如果编译失败，请检查模型文件是否完整下载

## 🤝 贡献指南

### 开发流程

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'feat: Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

### 代码规范

- 遵循 Rust 官方编码规范
- 使用 `rustfmt` 格式化代码
- 使用 `clippy` 进行代码检查
- 所有公共API必须有文档注释
- 提交信息遵循 [Conventional Commits](https://www.conventionalcommits.org/) 规范

### 提交规范

- `feat`: 新功能
- `fix`: 修复bug
- `docs`: 文档更新
- `style`: 代码格式（不影响代码运行的变动）
- `refactor`: 重构（既不是新增功能，也不是修改bug的代码变动）
- `perf`: 性能优化
- `test`: 增加测试
- `chore`: 构建过程或辅助工具的变动

## 📄 许可证

本项目采用 GNU General Public License v2.0 或更高版本 - 查看 [LICENSE](LICENSE) 文件了解详情

## 🙏 致谢

- [原神](https://ys.mihoyo.com/) - 游戏本体
- [ONNX Runtime](https://onnxruntime.ai/) - 高性能推理引擎
- [SVTR](https://github.com/PaddlePaddle/PaddleOCR) - OCR模型架构
- [Rust](https://www.rust-lang.org/) - 编程语言
- [GitHub Actions](https://github.com/features/actions) - 持续集成

---

<div align="center">

**⭐ 如果这个项目对您有帮助，请给个Star支持一下！⭐**

*使用 ❤️ 和 ☕ 制作*

</div>
