use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// 错误恢复策略
#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryStrategy {
    /// 立即重试
    ImmediateRetry,
    /// 延迟重试
    DelayedRetry(Duration),
    /// 指数退避重试
    ExponentialBackoff { initial_delay: Duration, max_delay: Duration, multiplier: f64 },
    /// 跳过当前操作
    Skip,
    /// 使用默认值
    UseDefault,
    /// 尝试备用方法
    UseFallback,
    /// 完全失败，不重试
    Fail,
}

/// 错误类型分类
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    /// 网络相关错误
    Network,
    /// I/O操作错误
    IO,
    /// 解析错误
    Parsing,
    /// OCR识别错误
    OCR,
    /// 图像处理错误
    ImageProcessing,
    /// 资源不足错误
    ResourceExhaustion,
    /// 配置错误
    Configuration,
    /// 临时错误
    Temporary,
    /// 永久错误
    Permanent,
    /// 未知错误
    Unknown,
}

/// 错误恢复配置
#[derive(Debug, Clone)]
pub struct RecoveryConfig {
    /// 最大重试次数
    pub max_retries: usize,
    /// 重试策略映射
    pub strategy_map: HashMap<ErrorCategory, RecoveryStrategy>,
    /// 是否启用自适应调整
    pub adaptive_adjustment: bool,
    /// 错误阈值配置
    pub error_thresholds: ErrorThresholds,
}

/// 错误阈值配置
#[derive(Debug, Clone)]
pub struct ErrorThresholds {
    /// 连续失败阈值
    pub consecutive_failure_threshold: usize,
    /// 总体错误率阈值
    pub overall_error_rate_threshold: f64,
    /// 时间窗口内的错误数量阈值
    pub error_count_per_window: usize,
    /// 时间窗口大小
    pub time_window: Duration,
}

impl Default for RecoveryConfig {
    fn default() -> Self {
        let mut strategy_map = HashMap::new();
        strategy_map.insert(
            ErrorCategory::Network,
            RecoveryStrategy::ExponentialBackoff {
                initial_delay: Duration::from_millis(100),
                max_delay: Duration::from_secs(10),
                multiplier: 2.0,
            },
        );
        strategy_map
            .insert(ErrorCategory::IO, RecoveryStrategy::DelayedRetry(Duration::from_millis(500)));
        strategy_map.insert(ErrorCategory::Parsing, RecoveryStrategy::UseDefault);
        strategy_map.insert(ErrorCategory::OCR, RecoveryStrategy::ImmediateRetry);
        strategy_map.insert(ErrorCategory::ImageProcessing, RecoveryStrategy::UseFallback);
        strategy_map.insert(
            ErrorCategory::ResourceExhaustion,
            RecoveryStrategy::DelayedRetry(Duration::from_secs(1)),
        );
        strategy_map.insert(ErrorCategory::Configuration, RecoveryStrategy::Fail);
        strategy_map.insert(ErrorCategory::Temporary, RecoveryStrategy::ImmediateRetry);
        strategy_map.insert(ErrorCategory::Permanent, RecoveryStrategy::Fail);
        strategy_map.insert(
            ErrorCategory::Unknown,
            RecoveryStrategy::DelayedRetry(Duration::from_millis(200)),
        );

        Self {
            max_retries: 3,
            strategy_map,
            adaptive_adjustment: true,
            error_thresholds: ErrorThresholds {
                consecutive_failure_threshold: 5,
                overall_error_rate_threshold: 0.3, // 30%
                error_count_per_window: 10,
                time_window: Duration::from_secs(60),
            },
        }
    }
}

/// 可恢复的错误trait
pub trait RecoverableError: std::error::Error {
    /// 获取错误类别
    fn error_category(&self) -> ErrorCategory;

    /// 是否可以重试
    fn is_retryable(&self) -> bool {
        matches!(
            self.error_category(),
            ErrorCategory::Network
                | ErrorCategory::IO
                | ErrorCategory::OCR
                | ErrorCategory::ImageProcessing
                | ErrorCategory::ResourceExhaustion
                | ErrorCategory::Temporary
                | ErrorCategory::Unknown
        )
    }

    /// 获取建议的恢复策略
    fn suggested_recovery(&self) -> RecoveryStrategy {
        match self.error_category() {
            ErrorCategory::Network => RecoveryStrategy::ExponentialBackoff {
                initial_delay: Duration::from_millis(100),
                max_delay: Duration::from_secs(5),
                multiplier: 2.0,
            },
            ErrorCategory::Temporary => RecoveryStrategy::ImmediateRetry,
            ErrorCategory::Parsing => RecoveryStrategy::UseDefault,
            ErrorCategory::Permanent | ErrorCategory::Configuration => RecoveryStrategy::Fail,
            _ => RecoveryStrategy::DelayedRetry(Duration::from_millis(100)),
        }
    }
}

/// 错误统计信息
#[derive(Debug, Default, Clone)]
pub struct ErrorStatistics {
    pub total_errors: usize,
    pub successful_recoveries: usize,
    pub failed_recoveries: usize,
    pub category_counts: HashMap<ErrorCategory, usize>,
    pub recent_errors: Vec<(Instant, ErrorCategory)>,
}

impl ErrorStatistics {
    /// 记录错误
    pub fn record_error(&mut self, category: ErrorCategory) {
        self.total_errors += 1;
        *self.category_counts.entry(category.clone()).or_insert(0) += 1;
        self.recent_errors.push((Instant::now(), category));

        // 保持最近错误列表的大小
        if self.recent_errors.len() > 1000 {
            self.recent_errors.drain(0..100);
        }
    }

    /// 记录成功恢复
    pub fn record_successful_recovery(&mut self) {
        self.successful_recoveries += 1;
    }

    /// 记录失败恢复
    pub fn record_failed_recovery(&mut self) {
        self.failed_recoveries += 1;
    }

    /// 获取错误率
    pub fn error_rate(&self) -> f64 {
        if self.total_errors == 0 {
            0.0
        } else {
            (self.failed_recoveries as f64) / (self.total_errors as f64)
        }
    }

    /// 获取恢复成功率
    pub fn recovery_success_rate(&self) -> f64 {
        let total_recovery_attempts = self.successful_recoveries + self.failed_recoveries;
        if total_recovery_attempts == 0 {
            0.0
        } else {
            (self.successful_recoveries as f64) / (total_recovery_attempts as f64)
        }
    }

    /// 清理过期的错误记录
    pub fn cleanup_old_errors(&mut self, window: Duration) {
        let cutoff = Instant::now() - window;
        self.recent_errors.retain(|(timestamp, _)| *timestamp > cutoff);
    }

    /// 获取时间窗口内的错误数量
    pub fn error_count_in_window(&self, window: Duration) -> usize {
        let cutoff = Instant::now() - window;
        self.recent_errors.iter().filter(|(timestamp, _)| *timestamp > cutoff).count()
    }
}

/// 错误恢复管理器
pub struct ErrorRecoveryManager {
    config: RecoveryConfig,
    statistics: Arc<Mutex<ErrorStatistics>>,
}

impl ErrorRecoveryManager {
    /// 创建新的错误恢复管理器
    pub fn new(config: RecoveryConfig) -> Self {
        Self { config, statistics: Arc::new(Mutex::new(ErrorStatistics::default())) }
    }

    /// 创建默认配置的管理器
    pub fn new_default() -> Self {
        Self::new(RecoveryConfig::default())
    }

    /// 尝试恢复错误
    pub async fn attempt_recovery<T, E, F, Fut>(
        &self,
        operation: F,
        error: &E,
    ) -> Result<T, RecoveryError<E>>
    where
        E: RecoverableError + Clone,
        F: Fn() -> Fut + Clone,
        Fut: std::future::Future<Output = Result<T, E>>,
    {
        let category = error.error_category();

        // 记录错误
        {
            let mut stats = self.statistics.lock().unwrap();
            stats.record_error(category.clone());
        }

        // 检查是否应该尝试恢复
        if !self.should_attempt_recovery(&category) {
            return Err(RecoveryError::RecoveryAborted(error.clone()));
        }

        let strategy = self.get_recovery_strategy(&category);

        match strategy {
            RecoveryStrategy::Fail => Err(RecoveryError::RecoveryAborted(error.clone())),
            RecoveryStrategy::Skip => Err(RecoveryError::OperationSkipped),
            RecoveryStrategy::UseDefault => Err(RecoveryError::UseDefaultRequested),
            RecoveryStrategy::UseFallback => Err(RecoveryError::UseFallbackRequested),
            RecoveryStrategy::ImmediateRetry => self.retry_with_strategy(operation, 0).await,
            RecoveryStrategy::DelayedRetry(delay) => {
                tokio::time::sleep(delay).await;
                self.retry_with_strategy(operation, 0).await
            },
            RecoveryStrategy::ExponentialBackoff { initial_delay, max_delay, multiplier } => {
                self.retry_with_exponential_backoff(operation, initial_delay, max_delay, multiplier)
                    .await
            },
        }
    }

    /// 使用指定策略重试
    async fn retry_with_strategy<T, E, F, Fut>(
        &self,
        operation: F,
        delay_ms: u64,
    ) -> Result<T, RecoveryError<E>>
    where
        E: RecoverableError + Clone,
        F: Fn() -> Fut + Clone,
        Fut: std::future::Future<Output = Result<T, E>>,
    {
        if delay_ms > 0 {
            tokio::time::sleep(Duration::from_millis(delay_ms)).await;
        }

        for attempt in 0..self.config.max_retries {
            match operation().await {
                Ok(result) => {
                    // 记录成功恢复
                    {
                        let mut stats = self.statistics.lock().unwrap();
                        stats.record_successful_recovery();
                    }
                    return Ok(result);
                },
                Err(e) => {
                    if attempt == self.config.max_retries - 1 {
                        // 最后一次尝试失败
                        {
                            let mut stats = self.statistics.lock().unwrap();
                            stats.record_failed_recovery();
                        }
                        return Err(RecoveryError::MaxRetriesExceeded(e));
                    }

                    // 记录重试前的错误
                    {
                        let mut stats = self.statistics.lock().unwrap();
                        stats.record_error(e.error_category());
                    }

                    // 简单延迟后重试
                    if attempt < self.config.max_retries - 1 {
                        tokio::time::sleep(Duration::from_millis(100 * (attempt + 1) as u64)).await;
                    }
                },
            }
        }

        unreachable!()
    }

    /// 使用指数退避重试
    async fn retry_with_exponential_backoff<T, E, F, Fut>(
        &self,
        operation: F,
        initial_delay: Duration,
        max_delay: Duration,
        multiplier: f64,
    ) -> Result<T, RecoveryError<E>>
    where
        E: RecoverableError + Clone,
        F: Fn() -> Fut + Clone,
        Fut: std::future::Future<Output = Result<T, E>>,
    {
        let mut current_delay = initial_delay;

        for attempt in 0..self.config.max_retries {
            if attempt > 0 {
                tokio::time::sleep(current_delay).await;
                current_delay =
                    Duration::from_millis(((current_delay.as_millis() as f64) * multiplier) as u64)
                        .min(max_delay);
            }

            match operation().await {
                Ok(result) => {
                    {
                        let mut stats = self.statistics.lock().unwrap();
                        stats.record_successful_recovery();
                    }
                    return Ok(result);
                },
                Err(e) => {
                    if attempt == self.config.max_retries - 1 {
                        {
                            let mut stats = self.statistics.lock().unwrap();
                            stats.record_failed_recovery();
                        }
                        return Err(RecoveryError::MaxRetriesExceeded(e));
                    }

                    {
                        let mut stats = self.statistics.lock().unwrap();
                        stats.record_error(e.error_category());
                    }
                },
            }
        }

        unreachable!()
    }

    /// 检查是否应该尝试恢复
    fn should_attempt_recovery(&self, category: &ErrorCategory) -> bool {
        let stats = self.statistics.lock().unwrap();

        // 检查连续失败阈值
        let recent_failures = stats
            .recent_errors
            .iter()
            .rev()
            .take(self.config.error_thresholds.consecutive_failure_threshold)
            .filter(|(_, cat)| cat == category)
            .count();

        if recent_failures >= self.config.error_thresholds.consecutive_failure_threshold {
            return false;
        }

        // 检查时间窗口内错误数量
        if stats.error_count_in_window(self.config.error_thresholds.time_window)
            >= self.config.error_thresholds.error_count_per_window
        {
            return false;
        }

        // 检查总体错误率
        if stats.error_rate() > self.config.error_thresholds.overall_error_rate_threshold {
            return false;
        }

        true
    }

    /// 获取恢复策略
    fn get_recovery_strategy(&self, category: &ErrorCategory) -> RecoveryStrategy {
        self.config
            .strategy_map
            .get(category)
            .cloned()
            .unwrap_or(RecoveryStrategy::DelayedRetry(Duration::from_millis(200)))
    }

    /// 获取统计信息
    pub fn get_statistics(&self) -> ErrorStatistics {
        self.statistics.lock().unwrap().clone()
    }

    /// 清理过期统计数据
    pub fn cleanup_statistics(&self) {
        let mut stats = self.statistics.lock().unwrap();
        stats.cleanup_old_errors(self.config.error_thresholds.time_window * 2);
    }
}

/// 恢复错误类型
#[derive(Debug)]
pub enum RecoveryError<E> {
    /// 达到最大重试次数
    MaxRetriesExceeded(E),
    /// 恢复被中止
    RecoveryAborted(E),
    /// 操作被跳过
    OperationSkipped,
    /// 需要使用默认值
    UseDefaultRequested,
    /// 需要使用备用方法
    UseFallbackRequested,
}

impl<E: fmt::Display> fmt::Display for RecoveryError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecoveryError::MaxRetriesExceeded(e) => write!(f, "达到最大重试次数: {e}"),
            RecoveryError::RecoveryAborted(e) => write!(f, "恢复被中止: {e}"),
            RecoveryError::OperationSkipped => write!(f, "操作被跳过"),
            RecoveryError::UseDefaultRequested => write!(f, "需要使用默认值"),
            RecoveryError::UseFallbackRequested => write!(f, "需要使用备用方法"),
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for RecoveryError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RecoveryError::MaxRetriesExceeded(e) | RecoveryError::RecoveryAborted(e) => Some(e),
            _ => None,
        }
    }
}

/// 便利宏：为错误类型实现RecoverableError
#[macro_export]
macro_rules! impl_recoverable_error {
    ($error_type:ty, $category:expr) => {
        impl $crate::error_recovery::RecoverableError for $error_type {
            fn error_category(&self) -> $crate::error_recovery::ErrorCategory {
                $category
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    #[derive(Debug, Clone)]
    struct TestError {
        message: String,
        category: ErrorCategory,
    }

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl std::error::Error for TestError {}

    impl RecoverableError for TestError {
        fn error_category(&self) -> ErrorCategory {
            self.category.clone()
        }
    }

    #[tokio::test]
    async fn test_immediate_retry_success() {
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
                        })
                    } else {
                        Ok("success".to_string())
                    }
                }
            }
        };

        let error =
            TestError { message: "初始错误".to_string(), category: ErrorCategory::Temporary };

        let result = manager.attempt_recovery(operation, &error).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }

    #[tokio::test]
    async fn test_max_retries_exceeded() {
        let manager = ErrorRecoveryManager::new_default();

        let operation = || async {
            Err(TestError { message: "持续错误".to_string(), category: ErrorCategory::Network })
        };

        let error =
            TestError { message: "初始错误".to_string(), category: ErrorCategory::Network };

        let result: Result<String, RecoveryError<TestError>> =
            manager.attempt_recovery(operation, &error).await;
        assert!(result.is_err());

        match result.unwrap_err() {
            RecoveryError::MaxRetriesExceeded(_) => {},
            _ => panic!("期望MaxRetriesExceeded错误"),
        }
    }

    #[tokio::test]
    async fn test_exponential_backoff() {
        let manager = ErrorRecoveryManager::new_default();
        let counter = Arc::new(AtomicUsize::new(0));
        let start_time = Instant::now();

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
                        })
                    } else {
                        Ok("success".to_string())
                    }
                }
            }
        };

        let error =
            TestError { message: "网络错误".to_string(), category: ErrorCategory::Network };

        let result = manager.attempt_recovery(operation, &error).await;
        let elapsed = start_time.elapsed();

        assert!(result.is_ok());
        // 验证确实有延迟（指数退避）
        assert!(elapsed >= Duration::from_millis(100));
    }

    #[test]
    fn test_error_statistics() {
        let mut stats = ErrorStatistics::default();

        stats.record_error(ErrorCategory::Network);
        stats.record_error(ErrorCategory::OCR);
        stats.record_successful_recovery();
        stats.record_failed_recovery();

        assert_eq!(stats.total_errors, 2);
        assert_eq!(stats.successful_recoveries, 1);
        assert_eq!(stats.failed_recoveries, 1);
        assert_eq!(stats.category_counts[&ErrorCategory::Network], 1);
        assert_eq!(stats.category_counts[&ErrorCategory::OCR], 1);
    }

    #[test]
    fn test_recovery_config_default() {
        let config = RecoveryConfig::default();

        assert_eq!(config.max_retries, 3);
        assert!(config.strategy_map.contains_key(&ErrorCategory::Network));
        assert!(config.adaptive_adjustment);
    }

    #[test]
    fn test_error_rate_calculation() {
        let mut stats = ErrorStatistics::default();

        // 测试零除情况
        assert_eq!(stats.error_rate(), 0.0);
        assert_eq!(stats.recovery_success_rate(), 0.0);

        // 添加一些数据
        stats.record_error(ErrorCategory::Network);
        stats.record_successful_recovery();
        stats.record_error(ErrorCategory::OCR);
        stats.record_failed_recovery();

        assert_eq!(stats.error_rate(), 0.5); // 1 failed / 2 total = 0.5
        assert_eq!(stats.recovery_success_rate(), 0.5); // 1 success / 2 attempts = 0.5
    }

    #[tokio::test]
    async fn test_recovery_strategy_skip() {
        let mut config = RecoveryConfig::default();
        config.strategy_map.insert(ErrorCategory::Configuration, RecoveryStrategy::Skip);

        let manager = ErrorRecoveryManager::new(config);

        let operation = || async { Ok("should not be called".to_string()) };

        let error = TestError {
            message: "配置错误".to_string(),
            category: ErrorCategory::Configuration,
        };

        let result = manager.attempt_recovery(operation, &error).await;
        assert!(result.is_err());

        match result.unwrap_err() {
            RecoveryError::OperationSkipped => {},
            _ => panic!("期望OperationSkipped错误"),
        }
    }

    #[test]
    fn test_cleanup_old_errors() {
        let mut stats = ErrorStatistics::default();

        // 添加一些旧错误
        stats
            .recent_errors
            .push((Instant::now() - Duration::from_secs(120), ErrorCategory::Network));
        stats.recent_errors.push((Instant::now() - Duration::from_secs(30), ErrorCategory::OCR));
        stats.recent_errors.push((Instant::now(), ErrorCategory::Parsing));

        assert_eq!(stats.recent_errors.len(), 3);

        // 清理1分钟前的错误
        stats.cleanup_old_errors(Duration::from_secs(60));

        assert_eq!(stats.recent_errors.len(), 2);
    }

    #[test]
    fn test_error_count_in_window() {
        let mut stats = ErrorStatistics::default();

        // 添加不同时间的错误
        stats
            .recent_errors
            .push((Instant::now() - Duration::from_secs(120), ErrorCategory::Network));
        stats.recent_errors.push((Instant::now() - Duration::from_secs(30), ErrorCategory::OCR));
        stats.recent_errors.push((Instant::now(), ErrorCategory::Parsing));

        // 计算1分钟内的错误数量
        let count = stats.error_count_in_window(Duration::from_secs(60));
        assert_eq!(count, 2); // 应该只有最近2个
    }
}
