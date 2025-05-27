# 更新日志

本文档记录 FurinaOCR 项目的所有重要更改。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
并且本项目遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [未发布]

### 新增
- 添加 EditorConfig 配置
- 添加贡献指南
- 添加更新日志
- 完善代码质量检查配置

### 变更
- 更新 Clippy 配置，移除不支持的配置项
- 统一代码风格和文档规范

### 修复
- 修复 Clippy 配置中的错误

## [0.1.0] - 2024-03-20

### 新增
- 初始版本发布
- 支持基本的 OCR 功能
- 支持 ORT 和 Tract-ONNX 两种推理后端
- 提供命令行界面
- 支持 Windows 和 Linux 平台

### 变更
- 使用 GPL-2.0-or-later 许可证
- 优化项目结构

### 修复
- 修复内存泄漏问题
- 修复并发处理问题
- 修复错误处理机制 