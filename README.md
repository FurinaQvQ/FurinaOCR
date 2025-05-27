# FurinaOCR

FurinaOCR 是一个基于深度学习的原神圣遗物识别工具，能够自动识别游戏中的圣遗物属性并导出为多种格式。

## 功能特性

- 🎯 高精度识别：使用深度学习模型，准确识别圣遗物属性
- 🔄 多格式导出：支持导出为GOOD、Mona、Mingyu Lab等多种格式
- 🚀 高性能：使用Rust开发，提供快速的处理速度
- 🛠️ 可扩展：支持自定义导出格式和识别规则
- 📊 数据统计：提供圣遗物评分和属性分析

## 系统要求

- Windows 10/11
- Rust nightly 工具链
- CUDA支持（可选，用于GPU加速）

## 快速开始

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

如果遇到编译错误，请确保：
1. 已正确安装Rust nightly工具链
2. 使用`--recursive`参数克隆仓库
3. 所有子模块都已正确克隆
4. 项目目录结构完整

## 使用说明

1. 启动原神游戏
2. 打开圣遗物背包
3. 运行FurinaOCR
4. 选择要导出的圣遗物
5. 选择导出格式
6. 导出数据

### 开发环境设置
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

## 贡献指南

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'feat: add some amazing feature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

## 许可证

本项目采用 GPL-2.0-or-later 许可证 - 详见 [LICENSE](LICENSE) 文件

## 致谢

- [原神](https://genshin.hoyoverse.com/) - 游戏本体
- [ONNX Runtime](https://github.com/microsoft/onnxruntime) - 深度学习推理引擎
- [Tesseract OCR](https://github.com/tesseract-ocr/tesseract) - OCR引擎

## 联系方式

- 项目维护者：[FurinaQvQ](https://github.com/FurinaQvQ)
- 项目仓库：[FurinaOCR](https://github.com/FurinaQvQ/FurinaOCR)

## 更新日志

### v0.1.0 (2024-03-xx)
- 初始版本发布
- 支持GOOD、Mona、Mingyu Lab格式导出
- 实现基本的圣遗物识别功能
