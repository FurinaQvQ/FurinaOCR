use furina_core::positioning::{Pos, Scalable};

/// 集成测试：测试性能关键路径
#[test]
fn test_performance_critical_paths() {
    use std::time::Instant;

    // 测试大量位置计算的性能（减少数量以适应调试模式）
    let start = Instant::now();
    let positions: Vec<Pos<i32>> = (0..1000).map(|i| Pos::new(i, i * 2)).collect();
    let creation_time = start.elapsed();

    let start = Instant::now();
    let scaled_positions: Vec<Pos<i32>> = positions.iter().map(|pos| pos.scale(1.5)).collect();
    let scaling_time = start.elapsed();

    // 验证结果正确性
    assert_eq!(positions.len(), 1000);
    assert_eq!(scaled_positions.len(), 1000);
    assert_eq!(scaled_positions[0], Pos::new(0, 0));
    assert_eq!(scaled_positions[1], Pos::new(1, 3)); // 1*1.5=1, 2*1.5=3

    // 性能断言（调整阈值以适应调试模式下的性能）
    assert!(creation_time.as_millis() < 1000, "位置创建耗时过长: {creation_time:?}");
    assert!(scaling_time.as_millis() < 1000, "位置缩放耗时过长: {scaling_time:?}");
}
