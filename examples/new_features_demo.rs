use furina_core::utils::string_optimizer::*;
use furina_core::testing::fuzz_testing::*;
use furina_core::error_recovery::*;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    println!("🚀 FurinaOCR 新功能演�?);
    println!("=".repeat(50));
    
    // 1. 字符串处理性能优化演示
    println!("\n📈 1. 字符串处理性能优化");
    println!("-".repeat(30));
    
    let mut optimizer = StringOptimizer::new();
    
    // 演示优化的属性解�?
    let test_stats = vec![
        "攻击�?46.6%",
        "暴击�?12.1%",
        "生命�?4780",
        "元素精�?165",
    ];
    
    println!("解析圣遗物属�?");
    for stat in &test_stats {
        match optimizer.parse_attribute_value(stat) {
            Ok((name, value, is_percentage)) => {
                println!("  �?{}: {} ({:.3}) {}",
                    stat, name, value, if is_percentage { "百分�? } else { "固定�? });
            },
            Err(e) => println!("  �?{}: 解析失败 - {}", stat, e),
        }
    }
    
    // 演示批量处理
    let batch_stats: Vec<String> = test_stats.iter().map(|s| s.to_string()).collect();
    let results = optimizer.batch_process_stats(&batch_stats);
    println!("\n批量处理结果: 成功 {}/{}",
        results.iter().filter(|r| r.is_ok()).count(),
        results.len()
    );
    
    // 2. 模糊测试框架演示
    println!("\n🧪 2. 模糊测试框架");
    println!("-".repeat(30));
    
    let config = FuzzConfig {
        iterations: 100,
        seed: 42,
        ..Default::default()
    };
    
    let mut tester = FuzzTester::new(config);
    
    // 演示属性解析的模糊测试
    tester.fuzz_attribute_parsing(|input| {
        match parse_stat_optimized(input) {
            Ok((name, value, _)) => {
                if name.is_empty() || !value.is_finite() {
                    Err("解析结果无效".to_string())
                } else {
                    Ok((name, value, true))
                }
            },
            Err(e) => Err(e.to_string()),
        }
    });
    
    let results = tester.get_results();
    println!("模糊测试结果:");
    println!("  总测试数: {}", results.total_tests);
    println!("  成功: {}", results.passed_tests);
    println!("  失败: {}", results.failed_tests);
    println!("  崩溃: {}", results.crashes);
    println!("  成功�? {:.1}%", results.success_rate());
    
    // 3. 错误恢复机制演示
    println!("\n🔄 3. 错误恢复机制");
    println!("-".repeat(30));
    
    // 创建一个简单的异步运行时来演示
    let rt = tokio::runtime::Runtime::new()?;
    
    rt.block_on(async {
        let manager = ErrorRecoveryManager::new_default();
        
        #[derive(Debug, Clone)]
        struct DemoError {
            message: String,
            category: ErrorCategory,
        }
        
        impl std::fmt::Display for DemoError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.message)
            }
        }
        
        impl std::error::Error for DemoError {}
        
        impl RecoverableError for DemoError {
            fn error_category(&self) -> ErrorCategory {
                self.category.clone()
            }
        }
        
        // 模拟一个可能失败的操作
        let mut attempt_count = 0;
        let operation = || {
            attempt_count += 1;
            async move {
                if attempt_count < 3 {
                    Err(DemoError {
                        message: format!("模拟临时错误 (尝试 {})", attempt_count),
                        category: ErrorCategory::Temporary,
                    })
                } else {
                    Ok(format!("操作成功 (第{}次尝�?", attempt_count))
                }
            }
        };
        
        let error = DemoError {
            message: "初始错误".to_string(),
            category: ErrorCategory::Temporary,
        };
        
        println!("尝试错误恢复...");
        match manager.attempt_recovery(operation, &error).await {
            Ok(result) => println!("  �?恢复成功: {}", result),
            Err(e) => println!("  �?恢复失败: {}", e),
        }
        
        // 显示统计信息
        let stats = manager.get_statistics();
        println!("错误恢复统计:");
        println!("  总错误数: {}", stats.total_errors);
        println!("  成功恢复: {}", stats.successful_recoveries);
        println!("  失败恢复: {}", stats.failed_recoveries);
        println!("  恢复成功�? {:.1}%", stats.recovery_success_rate());
    });
    
    println!("\n🎯 所有新功能演示完成�?);
    println!("FurinaOCR 现在具备�?");
    println!("  �?高性能字符串处�?);
    println!("  🧪 全面的模糊测�?);
    println!("  🛡�?智能错误恢复");
    
    Ok(())
} 
