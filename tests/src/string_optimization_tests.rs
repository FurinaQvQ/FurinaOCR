use std::time::Instant;

use furina_core::testing::fuzz_testing::{FuzzConfig, FuzzTester};
use furina_core::utils::string_optimizer::{
    parse_level_optimized, parse_stat_optimized, StringOptimizer, StringPool,
};

/// Test basic functionality of string optimizer
#[test]
fn test_string_optimizer_basic() {
    let mut optimizer = StringOptimizer::new();

    // 测试正常的属性解析
    let result = optimizer.parse_attribute_value("攻击力+46.6%");
    assert!(result.is_ok());

    let (name, value, is_percentage) = result.unwrap();
    assert_eq!(name, "攻击力");
    assert!((value - 0.466).abs() < f64::EPSILON);
    assert!(is_percentage);
}

/// Test performance improvement of string optimizer
#[test]
fn test_string_optimizer_performance() {
    let mut optimizer = StringOptimizer::new();

    let test_data = vec![
        "攻击力+46.6%",
        "暴击率+12.1%",
        "暴击伤害+22.5%",
        "生命值+4780",
        "防御力+58",
        "元素精通+42",
        "元素充能效率+16.2%",
        "治疗加成+8.9%",
    ];

    let start = Instant::now();

    // 批量处理测试
    for _ in 0..1000 {
        for data in &test_data {
            let _result = optimizer.parse_attribute_value(data);
        }
    }

    let elapsed = start.elapsed();
    println!("优化器处理8000次解析耗时: {elapsed:?}");

    // 性能要求：8000次解析应该在合理时间内完成
    assert!(elapsed.as_millis() < 2000, "字符串优化器性能测试超时: {elapsed:?}");
}

/// Test global convenience functions
#[test]
fn test_global_functions() {
    // 测试属性解析
    let result = parse_stat_optimized("元素精通+165");
    assert!(result.is_ok());

    let (name, value, is_percentage) = result.unwrap();
    assert_eq!(name, "元素精通");
    assert!((value - 165.0).abs() < f64::EPSILON);
    assert!(!is_percentage);

    // 测试等级解析
    let level = parse_level_optimized("+20");
    assert!(level.is_ok());
    assert_eq!(level.unwrap(), 20);
}

/// Test string pool functionality
#[test]
fn test_string_pool() {
    let mut pool = StringPool::new();

    // 相同字符串应该返回相同引用
    let str1 = pool.intern("测试字符串".to_string());
    let str2 = pool.intern("测试字符串".to_string());

    assert!(std::ptr::eq(str1, str2));
}

/// Test zero-copy string cleanup functionality
#[test]
fn test_zero_copy_string_cleanup() {
    let optimizer = StringOptimizer::new();

    // 测试不需要清理的字符串（应该零拷贝）
    let clean_str = "攻击力加成数值";
    let result = optimizer.fast_clean_string(clean_str, &['%', ',', '+']);

    match result {
        std::borrow::Cow::Borrowed(borrowed) => {
            // 验证是零拷贝
            assert!(std::ptr::eq(borrowed.as_ptr(), clean_str.as_ptr()));
        },
        std::borrow::Cow::Owned(_) => {
            panic!("期望零拷贝，但产生了新分配");
        },
    }

    // 测试需要清理的字符串
    let dirty_str = "攻击力+46.6%";
    let result = optimizer.fast_clean_string(dirty_str, &['%', '+']);

    match result {
        std::borrow::Cow::Borrowed(_) => {
            panic!("期望新分配，但使用了零拷贝");
        },
        std::borrow::Cow::Owned(cleaned) => {
            assert_eq!(cleaned, "攻击力46.6");
        },
    }
}

/// Test batch processing functionality
#[test]
fn test_batch_processing() {
    let mut optimizer = StringOptimizer::new();

    let stats = vec![
        "攻击力+46.6%".to_string(),
        "暴击率+12.1%".to_string(),
        "暴击伤害+22.5%".to_string(),
        "无效格式".to_string(),
        "生命值+4780".to_string(),
    ];

    let results = optimizer.batch_process_stats(&stats);

    assert_eq!(results.len(), 5);

    // 前3个和最后1个应该成功
    assert!(results[0].is_ok());
    assert!(results[1].is_ok());
    assert!(results[2].is_ok());
    assert!(results[3].is_err()); // 无效格式
    assert!(results[4].is_ok());

    // 验证解析结果
    if let Ok((name, value, is_percentage)) = &results[0] {
        assert_eq!(name, "攻击力");
        assert!((value - 0.466).abs() < f64::EPSILON);
        assert!(*is_percentage);
    }
}

/// Test error handling robustness
#[test]
fn test_error_handling_robustness() {
    let mut optimizer = StringOptimizer::new();

    let invalid_inputs = vec![
        "",                   // 空字符串
        "+",                  // 只有分隔符
        "攻击力",             // 缺少数值
        "+46.6%",             // 缺少属性名
        "攻击力+abc",         // 无效数值
        "攻击力+46.6%+extra", // 多余部分
        "攻击力++46.6%",      // 多个分隔符
    ];

    for input in invalid_inputs {
        let result = optimizer.parse_attribute_value(input);
        // 所有无效输入都应该返回错误，而不是崩溃
        assert!(result.is_err(), "输入 '{input}' 应该返回错误");
    }
}

/// Test string optimizer fuzzing
#[test]
fn test_string_optimizer_fuzz() {
    let config = FuzzConfig {
        iterations: 500,
        seed: 42,
        string_length_range: (5, 50),
        include_unicode: true,
        include_special_chars: true,
        ..Default::default()
    };

    let mut tester = FuzzTester::new(config);

    // 测试属性解析的鲁棒性
    tester.fuzz_attribute_parsing(|input| match parse_stat_optimized(input) {
        Ok((name, value, _)) => {
            if name.is_empty() || !value.is_finite() {
                Err("解析结果无效".to_string())
            } else {
                Ok((name, value, true))
            }
        },
        Err(e) => Err(e.to_string()),
    });

    let results = tester.get_results();
    println!("模糊测试结果:\n{}", results.generate_report());

    // 确保没有崩溃
    assert_eq!(results.crashes, 0, "字符串优化器模糊测试检测到崩溃");

    // 确保大部分输入都能正常处理（即使失败也要优雅）
    let handled_rate =
        ((results.passed_tests + results.failed_tests) as f64 / results.total_tests as f64) * 100.0;
    assert!(handled_rate >= 95.0, "字符串优化器处理率过低: {handled_rate:.1}%");
}

/// Performance comparison test: before vs after optimization
#[test]
fn test_performance_comparison() {
    use regex::Regex;

    let test_data =
        vec!["攻击力+46.6%", "暴击率+12.1%", "暴击伤害+22.5%", "生命值+4780", "防御力+58"];

    // 测试传统方法性能
    let re = Regex::new("[%,]").unwrap();
    let start = Instant::now();
    for _ in 0..1000 {
        for data in &test_data {
            // 模拟传统解析方法
            let temp: Vec<&str> = data.split('+').collect();
            if temp.len() == 2 {
                let _cleaned = re.replace_all(temp[1], "");
            }
        }
    }
    let traditional_time = start.elapsed();

    // 测试优化方法性能
    let mut optimizer = StringOptimizer::new();
    let start = Instant::now();
    for _ in 0..1000 {
        for data in &test_data {
            let _result = optimizer.parse_attribute_value(data);
        }
    }
    let optimized_time = start.elapsed();

    println!("传统方法耗时: {traditional_time:?}");
    println!("优化方法耗时: {optimized_time:?}");

    // 优化方法应该不会显著慢于传统方法（考虑到额外的功能）
    // 在某些情况下甚至可能更快（由于正则表达式缓存）
    let ratio = optimized_time.as_nanos() as f64 / traditional_time.as_nanos() as f64;
    assert!(ratio < 3.0, "优化方法比传统方法慢太多: {ratio:.2}倍");
}
