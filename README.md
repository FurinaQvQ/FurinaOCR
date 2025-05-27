# <p align="center">FurinaOCR</p>

<p align="center">
  <a href="https://www.gnu.org/licenses/old-licenses/gpl-2.0-standalone.html">
    <img src="https://img.shields.io/badge/License-GPL%202.0--or--later-blue.svg?style=for-the-badge" alt="GPL-2.0-or-later License"/>
  </a>
  <img src="https://img.shields.io/github/actions/workflow/status/FurinaQvQ/FurinaOCR/ci.yml?label=CI&logo=github&style=for-the-badge" alt="CI Status"/>
  <img src="https://img.shields.io/github/stars/FurinaQvQ/FurinaOCR?style=for-the-badge" alt="Stars"/>
  <img src="https://img.shields.io/badge/Rust-nightly-orange?style=for-the-badge&logo=rust" alt="Rust Nightly"/>
  <img src="https://img.shields.io/badge/Platform-Windows-blue?style=for-the-badge&logo=windows" alt="Platform"/>
</p>

<div align="center">
  <h3>🎮 原神圣遗物识别神器 | 🚀 极速 · 高精度 · 多格式导出</h3>
  <p>让你的圣遗物管理更轻松，数据更专业！</p>
</div>

---

## ✨ 项目简介

FurinaOCR 是一个基于深度学习的原神圣遗物识别工具，能够自动识别游戏中的圣遗物属性并导出为多种格式。告别手动记录，让数据管理更智能！

## 🌟 功能特性

| 特性 | 描述 |
|------|------|
| 🎯 高精度识别 | 使用深度学习模型，准确识别圣遗物属性 |
| 🔄 多格式导出 | 支持导出为GOOD、Mona、Mingyu Lab等多种格式 |
| 🚀 高性能 | 使用Rust开发，提供快速的处理速度 |
| 🛠️ 可扩展 | 支持自定义导出格式和识别规则 |
| 📊 数据统计 | 提供圣遗物评分和属性分析 |

## 🎯 使用场景

- 📱 快速导出圣遗物数据
- 📊 批量分析圣遗物属性
- 🔄 与其他工具无缝对接
- 📈 数据可视化分析

## 💻 系统要求

- 🪟 Windows 10/11
- 🦀 Rust nightly 工具链
- 🎮 CUDA支持（可选，用于GPU加速）

## 🚀 快速开始

1. 安装Rust nightly工具链：
```powershell
rustup default nightly
rustup component add rustfmt clippy
```

2. 克隆仓库（确保使用--recursive参数）：
```powershell
git clone --recursive https://github.com/FurinaQvQ/FurinaOCR.git
cd FurinaOCR
```

3. 安装项目依赖：
```powershell
cargo build --release
```

4. 运行程序：
```powershell
.\target\release\FurinaOCR.exe
```

> ⚠️ 如果遇到编译错误，请确保：
> 1. 已正确安装Rust nightly工具链
> 2. 使用`--recursive`参数克隆仓库
> 3. 所有子模块都已正确克隆
> 4. 项目目录结构完整

## 📖 使用说明

1. 🎮 启动原神游戏
2. 🎒 打开圣遗物背包
3. 🚀 运行FurinaOCR
4. 📋 选择要导出的圣遗物
5. 📤 选择导出格式
6. 💾 导出数据

### 🎯 命令行使用

#### 基本语法
```powershell
furina_ocr [选项] [命令] [参数]
```

#### 📤 导出格式指令

##### GOOD格式导出
```powershell
# 导出为GOOD格式
furina_ocr export --format good --output artifacts.json

# 指定输入图片目录
furina_ocr export --format good --input ./screenshots --output artifacts_good.json

# 批量处理
furina_ocr export --format good --batch --input ./images --output ./exports/good_format.json
```

##### Mona格式导出
```powershell
# 导出为Mona格式
furina_ocr export --format mona --output artifacts_mona.json

# 包含详细统计信息
furina_ocr export --format mona --stats --output mona_with_stats.json

# 指定语言（中文/英文）
furina_ocr export --format mona --language zh-CN --output mona_cn.json
```

##### Mingyu Lab格式导出
```powershell
# 导出为Mingyu Lab格式
furina_ocr export --format mingyu --output artifacts_mingyu.json

# 包含评分信息
furina_ocr export --format mingyu --include-score --output mingyu_scored.json

# 过滤特定品质
furina_ocr export --format mingyu --rarity 5 --output five_star_artifacts.json
```

##### CSV格式导出
```powershell
# 导出为CSV格式
furina_ocr export --format csv --output artifacts.csv

# 包含所有属性列
furina_ocr export --format csv --full-columns --output detailed_artifacts.csv
```

#### 🔧 所有命令功能

##### 1. 识别命令 (recognize)
```powershell
# 识别单张图片
furina_ocr recognize --input artifact.png

# 识别多张图片
furina_ocr recognize --input ./screenshots --batch

# 指定识别模型
furina_ocr recognize --model ./models/custom_model.onnx --input artifact.png

# 调整识别精度
furina_ocr recognize --confidence 0.85 --input artifact.png
```

##### 2. 导出命令 (export)
```powershell
# 基本导出
furina_ocr export --format [good|mona|mingyu|csv] --output filename

# 高级导出选项
furina_ocr export --format good \
  --input ./screenshots \
  --output artifacts.json \
  --filter-rarity 4,5 \
  --include-metadata \
  --pretty-print
```

##### 3. 批量处理命令 (batch)
```powershell
# 批量处理目录
furina_ocr batch --input ./images --output ./results

# 并行处理
furina_ocr batch --input ./images --output ./results --threads 4

# 递归处理子目录
furina_ocr batch --input ./images --output ./results --recursive
```

##### 4. 配置命令 (config)
```powershell
# 查看当前配置
furina_ocr config show

# 设置默认导出格式
furina_ocr config set default-format good

# 设置模型路径
furina_ocr config set model-path ./models/model.onnx

# 重置配置
furina_ocr config reset
```

##### 5. 验证命令 (validate)
```powershell
# 验证识别结果
furina_ocr validate --input result.json

# 验证模型文件
furina_ocr validate --model ./models/model.onnx

# 验证配置文件
furina_ocr validate --config ./config.toml
```

##### 6. 信息命令 (info)
```powershell
# 显示版本信息
furina_ocr info --version

# 显示系统信息
furina_ocr info --system

# 显示支持的格式
furina_ocr info --formats

# 显示模型信息
furina_ocr info --model
```

#### 🎛️ 通用选项

| 选项 | 简写 | 描述 | 示例 |
|------|------|------|------|
| `--help` | `-h` | 显示帮助信息 | `furina_ocr -h` |
| `--version` | `-V` | 显示版本号 | `furina_ocr -V` |
| `--verbose` | `-v` | 详细输出 | `furina_ocr -v export` |
| `--quiet` | `-q` | 静默模式 | `furina_ocr -q export` |
| `--config` | `-c` | 指定配置文件 | `furina_ocr -c config.toml` |
| `--log-level` | | 设置日志级别 | `--log-level debug` |

#### 📁 配置文件示例

创建 `config.toml` 文件：
```toml
[ocr]
model_path = "./models/model_training.onnx"
confidence_threshold = 0.8
language = "zh-CN"

[export]
default_format = "good"
include_metadata = true
pretty_print = true

[batch]
max_threads = 4
recursive = false
skip_errors = true

[logging]
level = "info"
file = "./logs/furina_ocr.log"
```

#### 🔄 使用示例

##### 完整工作流程
```powershell
# 1. 配置环境
furina_ocr config set model-path "./models/model_training.onnx"

# 2. 批量识别截图
furina_ocr batch --input "./screenshots" --output "./results" --threads 4

# 3. 导出为GOOD格式
furina_ocr export --format good --input "./results" --output "my_artifacts.json" --pretty-print

# 4. 验证结果
furina_ocr validate --input "my_artifacts.json"
```

##### 快速导出
```powershell
# 一键导出（自动识别 + 导出）
furina_ocr quick-export --input "./screenshots" --format good --output "artifacts.json"

# 多格式同时导出
furina_ocr multi-export --input "./screenshots" --formats good,mona,csv --output-dir "./exports"
```

## 🛠️ 开发环境设置

1. 安装开发依赖：
```powershell
cargo install cargo-watch
cargo install cargo-expand
```

2. 运行测试：
```powershell
cargo test
```

3. 代码格式化：
```powershell
cargo fmt
```

4. 代码检查：
```powershell
cargo clippy
```

## 🤝 贡献指南

1. 🍴 Fork 项目
2. 🌿 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 💾 提交更改 (`git commit -m 'feat: add some amazing feature'`)
4. 📤 推送到分支 (`git push origin feature/AmazingFeature`)
5. 📬 创建 Pull Request

## 📄 许可证

本项目采用 GPL-2.0-or-later 许可证 - 详见 [LICENSE](LICENSE) 文件

## 🙏 致谢

- 🎮 [原神](https://genshin.hoyoverse.com/) - 游戏本体
- 🧠 [ONNX Runtime](https://github.com/microsoft/onnxruntime) - 深度学习推理引擎
- 👁️ [Tesseract OCR](https://github.com/tesseract-ocr/tesseract) - OCR引擎

## 📞 联系方式

- 👤 项目维护者：[FurinaQvQ](https://github.com/FurinaQvQ)
- 📦 项目仓库：[FurinaOCR](https://github.com/FurinaQvQ/FurinaOCR)