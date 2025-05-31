use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use furina_core::utils::string_optimizer::{StringOptimizer, parse_stat_optimized};
use std::time::Duration;

fn bench_string_optimizer_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("字符串解析性能");
    
    // 设置测试数据
    let test_cases = vec![
        "攻击力+46.6%",
        "暴击率+12.1%", 
        "暴击伤害+22.5%",
        "生命值+4780",
        "防御力+58",
        "元素精通+165",
        "元素充能效率+16.2%",
        "治疗加成+8.9%",
    ];
    
    // 基准测试：单次解析
    group.bench_function("单次解析", |b| {
        let mut optimizer = StringOptimizer::new();
        b.iter(|| {
            for case in &test_cases {
                black_box(optimizer.parse_attribute_value(case));
            }
        });
    });
    
    // 基准测试：批量解析
    group.bench_function("批量解析", |b| {
        let mut optimizer = StringOptimizer::new();
        let test_data: Vec<String> = test_cases.iter().map(|s| s.to_string()).collect();
        
        b.iter(|| {
            black_box(optimizer.batch_process_stats(&test_data));
        });
    });
    
    // 基准测试：全局函数
    group.bench_function("全局函数解析", |b| {
        b.iter(|| {
            for case in &test_cases {
                black_box(parse_stat_optimized(case));
            }
        });
    });
    
    group.finish();
}

fn bench_regex_caching(c: &mut Criterion) {
    let mut group = c.benchmark_group("正则表达式缓存性能");
    
    let pattern = r"[%,]";
    let test_string = "攻击力+46.6%暴击率+12.1%";
    
    // 测试正则表达式缓存效果
    group.bench_function("缓存命中", |b| {
        let mut optimizer = StringOptimizer::new();
        // 预热缓存
        let _ = optimizer.get_cached_regex(pattern);
        
        b.iter(|| {
            black_box(optimizer.get_cached_regex(pattern));
        });
    });
    
    group.bench_function("缓存未命中", |b| {
        b.iter(|| {
            let mut optimizer = StringOptimizer::new();
            black_box(optimizer.get_cached_regex(pattern));
        });
    });
    
    group.finish();
}

fn bench_throughput_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("吞吐量扩展性");
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(10));
    
    let base_case = "攻击力+46.6%";
    
    for size in [10, 100, 1000, 10000].iter() {
        let test_data: Vec<String> = (0..*size).map(|_| base_case.to_string()).collect();
        
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            BenchmarkId::new("批量处理", size),
            size,
            |b, _| {
                let mut optimizer = StringOptimizer::new();
                b.iter(|| {
                    black_box(optimizer.batch_process_stats(&test_data));
                });
            },
        );
    }
    
    group.finish();
}

fn bench_memory_efficiency(c: &mut Criterion) {
    let mut group = c.benchmark_group("内存效率");
    
    let test_cases = vec![
        "clean_string_without_special_chars",  // 无需清理
        "dirty%string,with%special,chars",     // 需要清理
    ];
    
    group.bench_function("零拷贝字符串清理", |b| {
        let optimizer = StringOptimizer::new();
        
        b.iter(|| {
            for case in &test_cases {
                black_box(optimizer.fast_clean_string(case, &['%', ',']));
            }
        });
    });
    
    group.finish();
}

fn bench_concurrent_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("并发性能");
    
    let test_data = "攻击力+46.6%";
    
    group.bench_function("并发解析", |b| {
        use std::sync::Arc;
        use std::thread;
        
        b.iter(|| {
            let handles: Vec<_> = (0..4).map(|_| {
                let data = test_data.to_string();
                thread::spawn(move || {
                    black_box(parse_stat_optimized(&data))
                })
            }).collect();
            
            for handle in handles {
                black_box(handle.join());
            }
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_string_optimizer_parsing,
    bench_regex_caching,
    bench_throughput_scaling,
    bench_memory_efficiency,
    bench_concurrent_performance
);

criterion_main!(benches); 