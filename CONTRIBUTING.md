# 贡献指南

感谢您对 FurinaOCR 项目的关注！我们欢迎任何形式的贡献，包括但不限于：

- 报告 bug
- 提出新功能建议
- 改进文档
- 提交代码修复
- 添加新功能

## 开发环境设置

1. 确保您已安装以下工具：
   - Rust 工具链 (rustup, rustc, cargo)
   - Git
   - 支持 EditorConfig 的编辑器

2. 克隆仓库：
   ```bash
   git clone https://github.com/yourusername/FurinaOCR.git
   cd FurinaOCR
   ```

3. 安装开发依赖：
   ```bash
   cargo install cargo-watch
   cargo install cargo-udeps
   ```

## 代码风格

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 遵循 Rust 官方风格指南
- 使用 EditorConfig 配置编辑器

## 提交规范

我们使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

- `feat`: 新功能
- `fix`: 修复 bug
- `docs`: 文档更新
- `style`: 代码风格修改
- `refactor`: 代码重构
- `perf`: 性能优化
- `test`: 测试相关
- `chore`: 构建过程或辅助工具的变动

示例：
```
feat: 添加新的 OCR 模型支持
fix: 修复内存泄漏问题
docs: 更新 API 文档
```

## 开发流程

1. 创建新分支：
   ```bash
   git checkout -b feat/your-feature-name
   ```

2. 开发新功能或修复 bug

3. 运行测试：
   ```bash
   cargo test
   cargo clippy
   ```

4. 提交更改：
   ```bash
   git add .
   git commit -m "feat: 添加新功能"
   ```

5. 推送到远程：
   ```bash
   git push origin feat/your-feature-name
   ```

6. 创建 Pull Request

## 代码审查

所有提交都需要通过代码审查。审查重点包括：

- 代码质量和可维护性
- 测试覆盖率
- 文档完整性
- 性能影响
- 向后兼容性

## 发布流程

1. 更新版本号（遵循语义化版本）
2. 更新 CHANGELOG.md
3. 创建发布标签
4. 发布到 crates.io

## 问题反馈

- 使用 GitHub Issues 报告问题
- 提供详细的复现步骤
- 包含错误信息和日志
- 说明您的环境信息

## 行为准则

- 尊重所有贡献者
- 接受建设性的批评
- 关注问题本身
- 保持专业和友善

## 许可证

贡献代码时，您同意将代码按照项目的 GPL-2.0-or-later 许可证发布。 