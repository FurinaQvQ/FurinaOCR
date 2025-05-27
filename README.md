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

2. 克隆仓库：
```powershell
git clone https://github.com/FurinaQvQ/FurinaOCR.git
cd FurinaOCR
```

3. 编译项目：
```powershell
cargo build --release
```

4. 运行程序：
```powershell
.\target\release\furina_ocr.exe
```

## 使用说明

1. 启动原神游戏
2. 打开圣遗物背包
3. 运行FurinaOCR
4. 选择要导出的圣遗物
5. 选择导出格式
6. 导出数据

## 导出格式

### GOOD格式
```json
{
  "format": "GOOD",
  "version": 2,
  "source": "FurinaOCR",
  "artifacts": [
    {
      "setKey": "EmblemOfSeveredFate",
      "slotKey": "plume",
      "level": 20,
      "rarity": 5,
      "mainStatKey": "atk",
      "substats": [
        {"key": "critRate_", "value": 3.5},
        {"key": "critDMG_", "value": 7.0},
        {"key": "atk_", "value": 4.7},
        {"key": "def_", "value": 5.8}
      ]
    }
  ]
}
```

### Mona格式
```json
{
  "version": "2.0",
  "source": "FurinaOCR",
  "artifacts": [
    {
      "setName": "绝缘之旗印",
      "position": "死之羽",
      "level": 20,
      "star": 5,
      "mainTag": {
        "name": "攻击力",
        "value": 311
      },
      "normalTags": [
        {"name": "暴击率", "value": "3.5%"},
        {"name": "暴击伤害", "value": "7.0%"},
        {"name": "攻击力百分比", "value": "4.7%"},
        {"name": "防御力百分比", "value": "5.8%"}
      ]
    }
  ]
}
```

### Mingyu Lab格式
```json
{
  "version": "1.0",
  "source": "FurinaOCR",
  "artifacts": [
    {
      "set": "绝缘之旗印",
      "slot": "羽",
      "level": 20,
      "rarity": 5,
      "main": {
        "stat": "攻击力",
        "value": 311
      },
      "sub": [
        {"stat": "暴击率", "value": "3.5%"},
        {"stat": "暴击伤害", "value": "7.0%"},
        {"stat": "攻击力百分比", "value": "4.7%"},
        {"stat": "防御力百分比", "value": "5.8%"}
      ]
    }
  ]
}
```

## 开发说明

### 项目结构
```
FurinaOCR/
├── genshin/              # 原神相关功能模块
│   ├── src/
│   │   ├── artifact/    # 圣遗物数据结构
│   │   ├── export/      # 导出功能
│   │   └── ocr/         # OCR识别功能
├── src/                  # 主程序
└── tests/               # 测试文件
```

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

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

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
