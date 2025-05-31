# FurinaOCR - 原神圣遗物扫描工具

<div align="center">

```
███████╗██╗   ██╗██████╗ ██╗███╗   ██╗ █████╗  ██████╗  ██████╗██████╗ 
██╔════╝██║   ██║██╔══██╗██║████╗  ██║██╔══██╗██╔═══██╗██╔════╝██╔══██╗
█████╗  ██║   ██║██████╔╝██║██╔██╗ ██║███████║██║   ██║██║     ██████╔╝
██╔══╝  ██║   ██║██╔══██╗██║██║╚██╗██║██╔══██║██║   ██║██║     ██╔══██╗
██║     ╚██████╔╝██║  ██║██║██║ ╚████║██║  ██║╚██████╔╝╚██████╗██║  ██║
╚═╝      ╚═════╝ ╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝ ╚═════╝  ╚═════╝╚═╝  ╚═╝
```

<div align="center" style="margin: 15px 0;">
  <!-- 基础信息徽章 -->
  <img src="https://img.shields.io/badge/license-GPL--2.0--or--later-blue.svg" alt="License" style="margin: 2px;"/>
  <img src="https://img.shields.io/badge/rust-1.70+-orange.svg" alt="Rust Version" style="margin: 2px;"/>
  <img src="https://img.shields.io/badge/platform-Windows-lightgrey.svg" alt="Platform" style="margin: 2px;"/>
  <img src="https://img.shields.io/badge/version-0.56.2-green.svg" alt="Version" style="margin: 2px;"/>
  <br style="margin: 4px 0;"/>
  <!-- CI/CD 流程徽章 -->
  <img src="https://img.shields.io/github/actions/workflow/status/FurinaQvQ/FurinaOCR/enhanced-ci.yml?branch=main&label=🚀%20Enhanced%20CI&logo=github-actions&logoColor=white" alt="Enhanced CI Status" style="margin: 2px;"/>
  <img src="https://img.shields.io/github/actions/workflow/status/FurinaQvQ/FurinaOCR/test.yml?branch=main&label=🧪%20Tests&logo=github-actions&logoColor=white" alt="Tests Status" style="margin: 2px;"/>
  <img src="https://img.shields.io/github/actions/workflow/status/FurinaQvQ/FurinaOCR/ci.yml?branch=main&label=🔧%20Build&logo=github-actions&logoColor=white" alt="Build Status" style="margin: 2px;"/>
  <img src="https://img.shields.io/badge/🛡️%20Security-Audit-Included-brightgreen?style=flat-square&logo=security&logoColor=white" alt="Security Audit" style="margin: 2px;"/>
  <img src="https://img.shields.io/badge/📊%20Tests-148-Passed-success?style=flat-square&logo=checkmarx&logoColor=white" alt="Test Results" style="margin: 2px;"/>
  <img src="https://img.shields.io/badge/📋%20Quality%20Gate-✅%20Passed-success?style=flat-square&logo=sonarcloud&logoColor=white" alt="Quality Gate" style="margin: 2px;"/>
  <br style="margin: 4px 0;"/>
  <!-- 技术栈徽章 -->
  <img src="https://img.shields.io/badge/🦀%20Rust-高性能核心-000000?style=flat-square&logo=rust&logoColor=white" alt="Rust" style="margin: 2px;"/>
  <img src="https://img.shields.io/badge/🤖%20ONNX-AI推理引擎-005CED?style=flat-square&logo=onnx&logoColor=white" alt="ONNX" style="margin: 2px;"/>
  <img src="https://img.shields.io/badge/👁️%20OpenCV-计算机视觉-5C3EE8?style=flat-square&logo=opencv&logoColor=white" alt="OpenCV" style="margin: 2px;"/>
  <img src="https://img.shields.io/badge/⚡%20Tokio-异步运行时-000000?style=flat-square&logo=rust&logoColor=white" alt="Tokio" style="margin: 2px;"/>
  <img src="https://img.shields.io/badge/🪟%20WinAPI-系统集成-0078D4?style=flat-square&logo=windows&logoColor=white" alt="Windows API" style="margin: 2px;"/>
  <img src="https://img.shields.io/badge/🛡️%20内存安全-零成本抽象-orange?style=flat-square&logo=rust&logoColor=white" alt="Memory Safe" style="margin: 2px;"/>
  <img src="https://img.shields.io/badge/🌟%20基于-YAS项目-purple?style=flat-square&logo=github&logoColor=white" alt="Based on YAS" style="margin: 2px;"/>
  <img src="https://img.shields.io/badge/💝%20用心制作-❤️%20FurinaQvQ-red?style=flat-square&logo=heart&logoColor=white" alt="Made with Love" style="margin: 2px;"/>
</div>

</div>

## 📖 项目简介

FurinaOCR 是一个基于 Rust 开发的现代化原神圣遗物智能扫描工具，基于 [yas](https://github.com/wormtql/yas) 项目重构优化。提供高效准确的 OCR 识别和多格式数据导出功能。

## ✨ 核心功能

- **🔍 智能扫描**: 基于 ONNX 模型的高精度 OCR 识别
- **📊 多格式导出**: 支持莫娜占卜铺、原魔计算器、GOOD、CSV 格式
- **⚡ 性能优化**: 快速模式和自适应时序调整
- **🛠️ 智能筛选**: 按星级、等级、装备状态筛选

## 🚀 快速开始

### 系统要求

- **操作系统**: Windows 10/11 (64位)
- **游戏版本**: 原神 PC 版 (简体中文)
- **分辨率**: 2560×1440、1920×1080 或 1600×900
- **权限**: 管理员权限

### 安装使用

1. **下载程序**
   - 前往 [Releases](https://github.com/FurinaQvQ/FurinaOCR/releases) 页面
   - 下载最新版本的 `FurinaOCR-windows-x64.zip`
   - 解压到任意目录

2. **准备游戏**
   - 启动原神游戏，设置为简体中文
   - 调整到支持的分辨率
   - 打开背包中的圣遗物页面

3. **开始扫描**
   - 以管理员身份运行 `FurinaOCR.exe`
   - 确保游戏窗口可见且未被遮挡
   - 按回车键开始自动扫描
   - 扫描完成后查看导出文件

## 📋 常用参数

```powershell
FurinaOCR.exe [选项]
```

### 主要选项
- `--min-star <数字>`: 最小星级筛选 (4-5，默认: 5)
- `--min-level <数字>`: 最小等级筛选 (0-20，默认: 0)
- `--format <格式>`: 导出格式 (mona/mingyu-lab/good/csv/all)
- `--fast-mode`: 启用快速扫描模式

## 🐛 常见问题

**Q: 扫描识别错误较多？**
- 确保游戏语言为简体中文
- 检查分辨率是否在支持列表中
- 确保游戏窗口完全可见

**Q: 程序无法检测游戏窗口？**
- 以管理员身份运行程序
- 检查游戏为窗口化或无边框模式
- 重启游戏和扫描工具

**Q: 扫描速度太慢？**
- 启用 `--fast-mode` 快速模式
- 关闭不必要的后台程序

## 🔧 开发贡献

如果您想参与开发或报告问题，请查看：

- 📋 **[贡献指南](CONTRIBUTING.md)** - 完整的开发环境设置和规范
- 🧪 **[测试文档](docs/TESTING.md)** - 测试执行和覆盖要求
- 📋 **[Issues](https://github.com/FurinaQvQ/FurinaOCR/issues)** - 报告 Bug 或请求功能

## 🔒 安全说明

- 程序仅在本地运行，不会上传任何数据
- 截图仅用于 OCR 识别，不会保存或传输
- 所有数据处理均在本地完成

## 📄 许可证

本项目采用 [GPL-2.0-or-later](LICENSE) 许可证。

基于 [yas](https://github.com/wormtql/yas) 项目开发，遵循开源精神。

## 🤝 社区支持

- 📋 [Issues](https://github.com/FurinaQvQ/FurinaOCR/issues): 报告 Bug 或请求功能
- 💬 [Discussions](https://github.com/FurinaQvQ/FurinaOCR/discussions): 社区讨论和交流
- 📧 Email: Furina@FurinaQvQ.top

---

<div align="center">

**如果这个项目对你有帮助，请给它一个 ⭐ Star！**

Made with ❤️ by [FurinaQvQ](https://github.com/FurinaQvQ)

</div> 