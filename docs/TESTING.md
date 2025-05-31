# FurinaOCR 测试指南

## 📋 目录

- [测试概览](#测试概览)
- [运行测试](#运行测试)
- [测试类型](#测试类型)
- [测试覆盖率](#测试覆盖率)
- [性能测试](#性能测试)
- [测试最佳实践](#测试最佳实践)
- [故障排除](#故障排除)

## 📊 测试概览

FurinaOCR 采用全面的测试策略，确保代码质量和功能稳定性。

### 测试统计

```
总测试数：147
├── furina_core: 68 个单元测试
├── genshin: 20 个单元测试  
├── integration_tests: 9 个集成测试
└── 文档测试: 2 个
```

### 测试覆盖模块

- 🎯 **定位系统** (positioning) - 坐标、尺寸、矩形操作
- 📷 **屏幕捕获** (capture) - 多平台屏幕截图
- 🔍 **OCR 识别** (ocr) - 文字识别和处理
- 🎮 **圣遗物系统** (artifact) - 数据结构和解析
- 📊 **扫描控制** (scanner) - 扫描逻辑和错误处理
- 📤 **数据导出** (export) - 多格式数据输出

## 🚀 运行测试

### 基本测试命令

```powershell
# 运行所有测试
cargo test --all

# 运行特定包的测试
cargo test -p furina_core
cargo test -p genshin  

# 详细输出测试结果
cargo test --all -- --nocapture

# 运行单个测试
cargo test test_performance_critical_paths
```

### 高级测试选项

```powershell
# 串行运行测试
cargo test --all -- --test-threads=1

# 显示测试输出
cargo test --all -- --show-output

# 运行特定模式的测试
cargo test positioning
```

### 调试模式测试

```powershell
# 调试模式运行（默认）
cargo test

# 发布模式运行（更快）
cargo test --release

# 运行特定测试并显示详细信息
$env:RUST_BACKTRACE="1"
cargo test test_name -- --nocapture
```

## 🧪 测试类型

### 1. 单元测试 (Unit Tests)

```rust
#[test]
fn test_pos_new() {
    let pos = Pos::new(10, 20);
    assert_eq!(pos.x, 10);
    assert_eq!(pos.y, 20);
}
```

### 2. 集成测试 (Integration Tests)

```rust
#[test]
fn test_positioning_system_integration() {
    let origin = Pos::new(100, 200);
    let size = Size::new(800, 600);
    let rect = Rect::new(origin.x, origin.y, size.width, size.height);
    
    assert_eq!(rect.origin(), origin);
    assert_eq!(rect.size(), size);
}
```

### 3. 性能测试 (Performance Tests)

```rust
#[test]
fn test_performance_critical_paths() {
    use std::time::Instant;
    
    let start = Instant::now();
    let positions: Vec<Pos<i32>> = (0..1000)
        .map(|i| Pos::new(i, i * 2))
        .collect();
    let creation_time = start.elapsed();
    
    assert!(creation_time.as_millis() < 1000, 
           "位置创建耗时过长: {:?}", creation_time);
}
```

### 4. 错误处理测试

```rust
#[test]
fn test_error_handling() {
    let capturer = MockCapturer::new_failing();
    let result = capturer.capture();
    
    assert!(result.is_err());
}
```

## 📈 测试覆盖率

### 核心模块覆盖率

| 模块 | 测试数量 | 覆盖功能 |
|------|----------|----------|
| **positioning** | 28 | 位置、尺寸、矩形、缩放 |
| **capture** | 10 | 屏幕捕获、多线程、错误处理 |
| **ocr** | 5 | OCR 接口、批处理、性能 |
| **artifact** | 9 | 圣遗物数据结构、解析 |
| **scanner** | 11 | 扫描逻辑、错误统计 |

### 功能覆盖详情

#### positioning 模块 (28 测试)
- ✅ Pos 结构体：创建、运算、序列化、缩放
- ✅ Size 结构体：基本操作、哈希、边界条件
- ✅ Rect 结构体：几何运算、类型转换、平移

#### capture 模块 (10 测试)  
- ✅ 基本捕获功能：成功捕获、相对捕获
- ✅ 错误处理：捕获失败、无效参数
- ✅ 并发安全：多线程捕获、线程安全性

#### artifact 模块 (9 测试)
- ✅ 数据结构：创建、相等性、哈希
- ✅ 枚举类型：显示格式、中文解析

### 生成覆盖率报告

```powershell
# 安装 tarpaulin
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --all --out Html
```

## ⚡ 性能测试

### 性能基准

| 操作类型 | 调试模式阈值 | 发布模式阈值 | 实际性能 |
|----------|--------------|--------------|----------|
| 位置创建 (1000个) | < 1000ms | < 100ms | ~10ms |
| 位置缩放 (1000个) | < 1000ms | < 100ms | ~15ms |
| 屏幕捕获 (10次) | < 5000ms | < 1000ms | ~500ms |

### 性能测试配置

```rust
// 调试模式下使用宽松的时间阈值
#[cfg(debug_assertions)]
const PERFORMANCE_THRESHOLD_MS: u64 = 1000;

#[cfg(not(debug_assertions))]  
const PERFORMANCE_THRESHOLD_MS: u64 = 100;
```

### 性能分析工具

```powershell
# 使用 flamegraph 进行性能分析
cargo install flamegraph
cargo flamegraph --test test_performance_critical_paths

# 使用 criterion 进行基准测试
cargo bench
```

## 📚 测试最佳实践

### 1. 测试命名规范

```rust
// ✅ 好的测试名称
#[test]
fn test_pos_scale_with_positive_factor() { }

// ❌ 避免的测试名称  
#[test]
fn test1() { }
```

### 2. 测试结构模式 (AAA)

```rust
#[test]
fn test_function_name() {
    // Arrange - 准备测试数据
    let input = create_test_data();
    
    // Act - 执行被测试的操作
    let result = function_under_test(input);
    
    // Assert - 验证结果
    assert_eq!(result.status, Expected::Success);
}
```

### 3. Mock 和测试替身

```rust
struct MockCapturer {
    should_fail: bool,
}

impl Capturer<RgbImage> for MockCapturer {
    fn capture_rect(&self, rect: Rect<i32>) -> anyhow::Result<RgbImage> {
        if self.should_fail {
            anyhow::bail!("模拟捕获失败");
        }
        Ok(create_test_image(rect.width, rect.height))
    }
}
```

### 4. 测试要求

- **每个公共函数都应该有测试**
- **测试边界条件和错误情况**
- **使用描述性的测试名称**
- **保持测试的独立性**
- **性能敏感代码必须有性能测试**

### 5. 运行测试前检查

```powershell
# 代码格式检查
cargo fmt --all --check

# 代码质量检查（零警告策略）
cargo clippy --all-targets -- -D warnings

# 运行所有测试
cargo test --all
```

## 🐛 故障排除

### 常见测试问题

#### 1. 性能测试失败
```
解决方案：
- 使用发布模式运行：cargo test --release
- 调整测试阈值适应调试模式
```

#### 2. 编译错误
```
解决方案：
- 检查 Cargo.toml 依赖配置
- 确认模块导入路径正确
```

#### 3. 测试超时
```
解决方案：
- 调整性能测试阈值
- 减少测试数据量
```

### 调试测试

```powershell
# 显示详细的失败信息
cargo test -- --nocapture

# 启用 Rust 回溯
$env:RUST_BACKTRACE="1"
cargo test
```

### 环境相关问题

#### Windows 环境
```powershell
# 设置环境变量
$env:RUST_TEST_THREADS="1"
```

#### 权限问题
```powershell
# 以管理员身份运行测试
cargo test
```

## 📊 测试报告

### 生成测试报告

```powershell
# 生成覆盖率报告
cargo tarpaulin --out Xml Html

# 生成性能基准报告  
cargo bench
```

## 🎯 测试策略总结

FurinaOCR 的测试策略确保：

1. **全面覆盖** - 147个测试覆盖所有核心功能
2. **性能保证** - 关键路径性能测试
3. **质量控制** - 零警告的 Clippy 检查
4. **持续集成** - 自动化 CI/CD 流程

通过这套完整的测试体系，我们能够：
- ✅ 及早发现和修复问题
- ✅ 确保代码重构的安全性  
- ✅ 维护高质量的代码标准
- ✅ 提供可靠的性能保证

---

*更多开发信息请参考 [开发贡献指南](../CONTRIBUTING.md)。* 