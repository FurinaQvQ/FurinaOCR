use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use furina_core::error_recovery::*;

#[derive(Debug, Clone)]
struct TestError {
    message: String,
    category: ErrorCategory,
    is_retryable: bool,
}

impl std::fmt::Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TestError {}

impl RecoverableError for TestError {
    fn error_category(&self) -> ErrorCategory {
        self.category.clone()
    }

    fn is_retryable(&self) -> bool {
        self.is_retryable
    }
}

/// 测试立即重试策略
#[tokio::test]
async fn test_immediate_retry_strategy() {
    let manager = ErrorRecoveryManager::new_default();
    let counter = Arc::new(AtomicUsize::new(0));

    let operation = {
        let counter = Arc::clone(&counter);
        move || {
            let counter = Arc::clone(&counter);
            async move {
                let count = counter.fetch_add(1, Ordering::SeqCst);
                if count < 2 {
                    Err(TestError {
                        message: "临时错误".to_string(),
                        category: ErrorCategory::Temporary,
                        is_retryable: true,
                    })
                } else {
                    Ok("成功".to_string())
                }
            }
        }
    };

    let error = TestError {
        message: "初始错误".to_string(),
        category: ErrorCategory::Temporary,
        is_retryable: true,
    };

    let result = manager.attempt_recovery(operation, &error).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "成功");

    // 应该调用3次（初始失败 + 2次重试成功）
    assert_eq!(counter.load(Ordering::SeqCst), 3);
}

/// 测试指数退避重试策略
#[tokio::test]
async fn test_exponential_backoff_strategy() {
    let manager = ErrorRecoveryManager::new_default();
    let counter = Arc::new(AtomicUsize::new(0));
    let start_time = std::time::Instant::now();

    let operation = {
        let counter = Arc::clone(&counter);
        move || {
            let counter = Arc::clone(&counter);
            async move {
                let count = counter.fetch_add(1, Ordering::SeqCst);
                if count < 2 {
                    Err(TestError {
                        message: "网络错误".to_string(),
                        category: ErrorCategory::Network,
                        is_retryable: true,
                    })
                } else {
                    Ok("网络恢复".to_string())
                }
            }
        }
    };

    let error = TestError {
        message: "网络连接失败".to_string(),
        category: ErrorCategory::Network,
        is_retryable: true,
    };

    let result = manager.attempt_recovery(operation, &error).await;
    let elapsed = start_time.elapsed();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "网络恢复");

    // 应该有延迟（指数退避）
    assert!(elapsed >= Duration::from_millis(100));
}

/// 测试最大重试次数限制
#[tokio::test]
async fn test_max_retries_exceeded() {
    let manager = ErrorRecoveryManager::new_default();
    let counter = Arc::new(AtomicUsize::new(0));

    let operation = {
        let counter = Arc::clone(&counter);
        move || {
            let counter = Arc::clone(&counter);
            async move {
                counter.fetch_add(1, Ordering::SeqCst);
                Err(TestError {
                    message: "持续失败".to_string(),
                    category: ErrorCategory::Network,
                    is_retryable: true,
                })
            }
        }
    };

    let error = TestError {
        message: "初始错误".to_string(),
        category: ErrorCategory::Network,
        is_retryable: true,
    };

    let result: Result<String, RecoveryError<TestError>> =
        manager.attempt_recovery(operation, &error).await;
    assert!(result.is_err());

    match result.unwrap_err() {
        RecoveryError::MaxRetriesExceeded(_) => {},
        _ => panic!("期望MaxRetriesExceeded错误"),
    }

    // 应该尝试了默认的最大重试次数
    assert_eq!(counter.load(Ordering::SeqCst), 3);
}

/// 测试不可重试错误的处理
#[tokio::test]
async fn test_non_retryable_error() {
    let manager = ErrorRecoveryManager::new_default();

    let operation = || async { Ok("不应该被调用".to_string()) };

    let error = TestError {
        message: "配置错误".to_string(),
        category: ErrorCategory::Configuration,
        is_retryable: false,
    };

    let result = manager.attempt_recovery(operation, &error).await;
    assert!(result.is_err());

    match result.unwrap_err() {
        RecoveryError::RecoveryAborted(_) => {},
        _ => panic!("期望RecoveryAborted错误"),
    }
}

/// 测试错误统计功能
#[test]
fn test_error_statistics() {
    let mut stats = ErrorStatistics::default();

    // 记录各种错误
    stats.record_error(ErrorCategory::Network);
    stats.record_error(ErrorCategory::OCR);
    stats.record_error(ErrorCategory::Parsing);

    // 记录恢复结果
    stats.record_successful_recovery();
    stats.record_successful_recovery();
    stats.record_failed_recovery();

    assert_eq!(stats.total_errors, 3);
    assert_eq!(stats.successful_recoveries, 2);
    assert_eq!(stats.failed_recoveries, 1);

    // 测试错误率计算
    assert!((stats.error_rate() - 0.333).abs() < 0.01); // 1 failed / 3 total = 0.33

    // 测试恢复成功率
    assert!((stats.recovery_success_rate() - 0.667).abs() < 0.01); // 2 success / 3 attempts = 0.67

    // 测试分类统计
    assert_eq!(stats.category_counts[&ErrorCategory::Network], 1);
    assert_eq!(stats.category_counts[&ErrorCategory::OCR], 1);
    assert_eq!(stats.category_counts[&ErrorCategory::Parsing], 1);
}

/// 测试自定义恢复配置
#[tokio::test]
async fn test_custom_recovery_config() {
    let config = RecoveryConfig {
        max_retries: 5,
        strategy_map: {
            let mut map = RecoveryConfig::default().strategy_map;
            map.insert(
                ErrorCategory::OCR,
                RecoveryStrategy::DelayedRetry(Duration::from_millis(50)),
            );
            map
        },
        ..RecoveryConfig::default()
    };

    let manager = ErrorRecoveryManager::new(config);
    let counter = Arc::new(AtomicUsize::new(0));

    let operation = {
        let counter = Arc::clone(&counter);
        move || {
            let counter = Arc::clone(&counter);
            async move {
                let count = counter.fetch_add(1, Ordering::SeqCst);
                if count < 4 {
                    Err(TestError {
                        message: "OCR识别失败".to_string(),
                        category: ErrorCategory::OCR,
                        is_retryable: true,
                    })
                } else {
                    Ok("OCR成功".to_string())
                }
            }
        }
    };

    let error = TestError {
        message: "OCR错误".to_string(),
        category: ErrorCategory::OCR,
        is_retryable: true,
    };

    let start_time = std::time::Instant::now();
    let result = manager.attempt_recovery(operation, &error).await;
    let elapsed = start_time.elapsed();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "OCR成功");

    // 应该有延迟（每次重试前延迟50ms）
    assert!(elapsed >= Duration::from_millis(200)); // 至少4次延迟
                                                    // 应该调用5次（初始 + 4次重试）
    assert_eq!(counter.load(Ordering::SeqCst), 5);
}

/// 测试跳过操作策略
#[tokio::test]
async fn test_skip_operation_strategy() {
    let mut config = RecoveryConfig::default();
    config.strategy_map.insert(ErrorCategory::Parsing, RecoveryStrategy::Skip);

    let manager = ErrorRecoveryManager::new(config);

    let operation = || async {
        panic!("操作不应该被执行");
    };

    let error = TestError {
        message: "解析错误".to_string(),
        category: ErrorCategory::Parsing,
        is_retryable: true,
    };

    let result: Result<String, RecoveryError<TestError>> =
        manager.attempt_recovery(operation, &error).await;
    assert!(result.is_err());

    match result.unwrap_err() {
        RecoveryError::OperationSkipped => {},
        _ => panic!("期望OperationSkipped错误"),
    }
}

/// 测试使用默认值策略
#[tokio::test]
async fn test_use_default_strategy() {
    let mut config = RecoveryConfig::default();
    config.strategy_map.insert(ErrorCategory::Parsing, RecoveryStrategy::UseDefault);

    let manager = ErrorRecoveryManager::new(config);

    let operation = || async {
        Err(TestError {
            message: "解析失败".to_string(),
            category: ErrorCategory::Parsing,
            is_retryable: true,
        })
    };

    let error = TestError {
        message: "解析错误".to_string(),
        category: ErrorCategory::Parsing,
        is_retryable: true,
    };

    let result: Result<String, RecoveryError<TestError>> =
        manager.attempt_recovery(operation, &error).await;
    assert!(result.is_err());

    match result.unwrap_err() {
        RecoveryError::UseDefaultRequested => {},
        _ => panic!("期望UseDefaultRequested错误"),
    }
}

/// 测试错误阈值机制
#[tokio::test]
async fn test_error_threshold_mechanism() {
    let mut config = RecoveryConfig::default();
    config.error_thresholds.consecutive_failure_threshold = 2;

    let manager = ErrorRecoveryManager::new(config);

    // 先产生一些失败，达到阈值
    for _ in 0..3 {
        let operation = || async {
            Err(TestError {
                message: "网络错误".to_string(),
                category: ErrorCategory::Network,
                is_retryable: true,
            })
        };

        let error = TestError {
            message: "网络连接失败".to_string(),
            category: ErrorCategory::Network,
            is_retryable: true,
        };

        let _result: Result<String, RecoveryError<TestError>> =
            manager.attempt_recovery(operation, &error).await;
    }

    // 现在应该拒绝进一步的恢复尝试
    let operation = || async {
        panic!("操作不应该被执行");
    };

    let error = TestError {
        message: "又一个网络错误".to_string(),
        category: ErrorCategory::Network,
        is_retryable: true,
    };

    let result: Result<String, RecoveryError<TestError>> =
        manager.attempt_recovery(operation, &error).await;
    assert!(result.is_err());

    match result.unwrap_err() {
        RecoveryError::RecoveryAborted(_) => {},
        _ => panic!("期望RecoveryAborted错误"),
    }
}

/// 测试统计信息清理功能
#[test]
fn test_statistics_cleanup() {
    let manager = ErrorRecoveryManager::new_default();

    // 通过执行操作来添加统计数据
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let operation = || async {
            Err(TestError {
                message: "测试错误".to_string(),
                category: ErrorCategory::Network,
                is_retryable: true,
            })
        };

        let error = TestError {
            message: "测试错误".to_string(),
            category: ErrorCategory::Network,
            is_retryable: true,
        };

        let _: Result<String, RecoveryError<TestError>> =
            manager.attempt_recovery(operation, &error).await;
    });

    // 执行清理
    manager.cleanup_statistics();

    // 验证统计数据存在
    let stats = manager.get_statistics();
    assert!(stats.total_errors > 0);
}

/// 测试时间窗口内错误计数
#[test]
fn test_error_count_in_time_window() {
    let mut stats = ErrorStatistics::default();

    // 添加不同时间的错误
    stats
        .recent_errors
        .push((std::time::Instant::now() - Duration::from_secs(120), ErrorCategory::Network));
    stats
        .recent_errors
        .push((std::time::Instant::now() - Duration::from_secs(30), ErrorCategory::OCR));
    stats.recent_errors.push((std::time::Instant::now(), ErrorCategory::Parsing));

    // 检查1分钟内的错误数量
    let count = stats.error_count_in_window(Duration::from_secs(60));
    assert_eq!(count, 2); // 应该只包含最近的2个错误
}

/// 压力测试：大量并发错误恢复
#[tokio::test]
async fn test_concurrent_error_recovery() {
    let mut handles = vec![];

    for i in 0..10 {
        let handle = tokio::spawn(async move {
            // 每个任务使用独立的管理器实例，避免竞争条件
            let manager = ErrorRecoveryManager::new_default();
            let counter = Arc::new(AtomicUsize::new(0));

            let operation = {
                let counter = Arc::clone(&counter);
                move || {
                    let counter = Arc::clone(&counter);
                    async move {
                        let count = counter.fetch_add(1, Ordering::SeqCst);
                        if count < 1 {
                            Err(TestError {
                                message: format!("错误-{i}"),
                                category: ErrorCategory::Temporary,
                                is_retryable: true,
                            })
                        } else {
                            Ok(format!("成功-{i}"))
                        }
                    }
                }
            };

            let error = TestError {
                message: format!("初始错误-{i}"),
                category: ErrorCategory::Temporary,
                is_retryable: true,
            };

            manager.attempt_recovery(operation, &error).await
        });

        handles.push(handle);
    }

    // 等待所有任务完成
    let mut success_count = 0;
    for handle in handles {
        if let Ok(Ok(_)) = handle.await {
            success_count += 1;
        }
    }

    // 所有恢复都应该成功
    assert_eq!(success_count, 10);
}
