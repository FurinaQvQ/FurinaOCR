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

/// Test immediate retry strategy
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
                        message: "temporary error".to_string(),
                        category: ErrorCategory::Temporary,
                        is_retryable: true,
                    })
                } else {
                    Ok("success".to_string())
                }
            }
        }
    };

    let error = TestError {
        message: "initial error".to_string(),
        category: ErrorCategory::Temporary,
        is_retryable: true,
    };

    let result = manager.attempt_recovery(operation, &error).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");

    // should call 3 times (initial failure + 2 times retry success)
    assert_eq!(counter.load(Ordering::SeqCst), 3);
}

/// Test exponential backoff retry strategy
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
                        message: "network error".to_string(),
                        category: ErrorCategory::Network,
                        is_retryable: true,
                    })
                } else {
                    Ok("network recovery".to_string())
                }
            }
        }
    };

    let error = TestError {
        message: "network connection failure".to_string(),
        category: ErrorCategory::Network,
        is_retryable: true,
    };

    let result = manager.attempt_recovery(operation, &error).await;
    let elapsed = start_time.elapsed();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "network recovery");

    // should have delay (exponential backoff)
    assert!(elapsed >= Duration::from_millis(100));
}

/// Test maximum retry limit
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
                    message: "persistent failure".to_string(),
                    category: ErrorCategory::Network,
                    is_retryable: true,
                })
            }
        }
    };

    let error = TestError {
        message: "initial error".to_string(),
        category: ErrorCategory::Network,
        is_retryable: true,
    };

    let result: Result<String, RecoveryError<TestError>> =
        manager.attempt_recovery(operation, &error).await;
    assert!(result.is_err());

    match result.unwrap_err() {
        RecoveryError::MaxRetriesExceeded(_) => {},
        _ => panic!("expected MaxRetriesExceeded error"),
    }

    // should attempt default maximum retry count
    assert_eq!(counter.load(Ordering::SeqCst), 3);
}

/// Test non-retryable error handling
#[tokio::test]
async fn test_non_retryable_error() {
    let manager = ErrorRecoveryManager::new_default();

    let operation = || async { Ok("should not be called".to_string()) };

    let error = TestError {
        message: "configuration error".to_string(),
        category: ErrorCategory::Configuration,
        is_retryable: false,
    };

    let result = manager.attempt_recovery(operation, &error).await;
    assert!(result.is_err());

    match result.unwrap_err() {
        RecoveryError::RecoveryAborted(_) => {},
        _ => panic!("expected RecoveryAborted error"),
    }
}

/// Test error statistics functionality
#[test]
fn test_error_statistics() {
    let mut stats = ErrorStatistics::default();

    // record various errors
    stats.record_error(ErrorCategory::Network);
    stats.record_error(ErrorCategory::OCR);
    stats.record_error(ErrorCategory::Parsing);

    // record recovery results
    stats.record_successful_recovery();
    stats.record_successful_recovery();
    stats.record_failed_recovery();

    assert_eq!(stats.total_errors, 3);
    assert_eq!(stats.successful_recoveries, 2);
    assert_eq!(stats.failed_recoveries, 1);

    // test error rate calculation
    assert!((stats.error_rate() - 0.333).abs() < 0.01); // 1 failed / 3 total = 0.33

    // test recovery success rate
    assert!((stats.recovery_success_rate() - 0.667).abs() < 0.01); // 2 success / 3 attempts = 0.67

    // test category statistics
    assert_eq!(stats.category_counts[&ErrorCategory::Network], 1);
    assert_eq!(stats.category_counts[&ErrorCategory::OCR], 1);
    assert_eq!(stats.category_counts[&ErrorCategory::Parsing], 1);
}

/// Test custom recovery configuration
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
                        message: "OCR recognition failure".to_string(),
                        category: ErrorCategory::OCR,
                        is_retryable: true,
                    })
                } else {
                    Ok("OCR success".to_string())
                }
            }
        }
    };

    let error = TestError {
        message: "OCR error".to_string(),
        category: ErrorCategory::OCR,
        is_retryable: true,
    };

    let start_time = std::time::Instant::now();
    let result = manager.attempt_recovery(operation, &error).await;
    let elapsed = start_time.elapsed();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "OCR success");

    // should have delay (50ms delay before each retry)
    assert!(elapsed >= Duration::from_millis(200)); // at least 4 times delay
                                                    // should call 5 times (initial + 4 retries)
    assert_eq!(counter.load(Ordering::SeqCst), 5);
}

/// Test skip operation strategy
#[tokio::test]
async fn test_skip_operation_strategy() {
    let mut config = RecoveryConfig::default();
    config.strategy_map.insert(ErrorCategory::Parsing, RecoveryStrategy::Skip);

    let manager = ErrorRecoveryManager::new(config);

    let operation = || async {
        panic!("operation should not be executed");
    };

    let error = TestError {
        message: "parsing error".to_string(),
        category: ErrorCategory::Parsing,
        is_retryable: true,
    };

    let result: Result<String, RecoveryError<TestError>> =
        manager.attempt_recovery(operation, &error).await;
    assert!(result.is_err());

    match result.unwrap_err() {
        RecoveryError::OperationSkipped => {},
        _ => panic!("expected OperationSkipped error"),
    }
}

/// Test use default strategy
#[tokio::test]
async fn test_use_default_strategy() {
    let mut config = RecoveryConfig::default();
    config.strategy_map.insert(ErrorCategory::Parsing, RecoveryStrategy::UseDefault);

    let manager = ErrorRecoveryManager::new(config);

    let operation = || async {
        Err(TestError {
            message: "parsing failure".to_string(),
            category: ErrorCategory::Parsing,
            is_retryable: true,
        })
    };

    let error = TestError {
        message: "parsing error".to_string(),
        category: ErrorCategory::Parsing,
        is_retryable: true,
    };

    let result: Result<String, RecoveryError<TestError>> =
        manager.attempt_recovery(operation, &error).await;
    assert!(result.is_err());

    match result.unwrap_err() {
        RecoveryError::UseDefaultRequested => {},
        _ => panic!("expected UseDefaultRequested error"),
    }
}

/// Test error threshold mechanism
#[tokio::test]
async fn test_error_threshold_mechanism() {
    let mut config = RecoveryConfig::default();
    config.error_thresholds.consecutive_failure_threshold = 2;

    let manager = ErrorRecoveryManager::new(config);

    // first produce some failures, reach threshold
    for _ in 0..3 {
        let operation = || async {
            Err(TestError {
                message: "network error".to_string(),
                category: ErrorCategory::Network,
                is_retryable: true,
            })
        };

        let error = TestError {
            message: "network connection failure".to_string(),
            category: ErrorCategory::Network,
            is_retryable: true,
        };

        let _result: Result<String, RecoveryError<TestError>> =
            manager.attempt_recovery(operation, &error).await;
    }

    // now should refuse further recovery attempts
    let operation = || async {
        panic!("operation should not be executed");
    };

    let error = TestError {
        message: "another network error".to_string(),
        category: ErrorCategory::Network,
        is_retryable: true,
    };

    let result: Result<String, RecoveryError<TestError>> =
        manager.attempt_recovery(operation, &error).await;
    assert!(result.is_err());

    match result.unwrap_err() {
        RecoveryError::RecoveryAborted(_) => {},
        _ => panic!("expected RecoveryAborted error"),
    }
}

/// Test statistics cleanup functionality
#[test]
fn test_statistics_cleanup() {
    let manager = ErrorRecoveryManager::new_default();

    // add statistics data by executing operations
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let operation = || async {
            Err(TestError {
                message: "test error".to_string(),
                category: ErrorCategory::Network,
                is_retryable: true,
            })
        };

        let error = TestError {
            message: "test error".to_string(),
            category: ErrorCategory::Network,
            is_retryable: true,
        };

        let _: Result<String, RecoveryError<TestError>> =
            manager.attempt_recovery(operation, &error).await;
    });

    // execute cleanup
    manager.cleanup_statistics();

    // verify statistics data exists
    let stats = manager.get_statistics();
    assert!(stats.total_errors > 0);
}

/// Test error count in time window
#[test]
fn test_error_count_in_time_window() {
    let mut stats = ErrorStatistics::default();

    // add errors at different times
    stats
        .recent_errors
        .push((std::time::Instant::now() - Duration::from_secs(120), ErrorCategory::Network));
    stats
        .recent_errors
        .push((std::time::Instant::now() - Duration::from_secs(30), ErrorCategory::OCR));
    stats.recent_errors.push((std::time::Instant::now(), ErrorCategory::Parsing));

    // check error count in 1 minute
    let count = stats.error_count_in_window(Duration::from_secs(60));
    assert_eq!(count, 2); // should only include recent 2 errors
}

/// Stress test: large concurrent error recovery
#[tokio::test]
async fn test_concurrent_error_recovery() {
    let mut handles = vec![];

    for i in 0..10 {
        let handle = tokio::spawn(async move {
            // each task uses independent manager instance, avoid race conditions
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
                                message: format!("error-{i}"),
                                category: ErrorCategory::Temporary,
                                is_retryable: true,
                            })
                        } else {
                            Ok(format!("success-{i}"))
                        }
                    }
                }
            };

            let error = TestError {
                message: format!("initial error-{i}"),
                category: ErrorCategory::Temporary,
                is_retryable: true,
            };

            manager.attempt_recovery(operation, &error).await
        });

        handles.push(handle);
    }

    // wait for all tasks to complete
    let mut success_count = 0;
    for handle in handles {
        if let Ok(Ok(_)) = handle.await {
            success_count += 1;
        }
    }

    // all recoveries should succeed
    assert_eq!(success_count, 10);
}
