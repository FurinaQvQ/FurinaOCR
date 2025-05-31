use std::collections::HashMap;
use std::time::Duration;

use rand::distributions::{Alphanumeric, Distribution, Uniform};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// 模糊测试数据生成器
///
/// 用于生成各种随机测试数据，包括字符串、数值、图像等
pub struct FuzzDataGenerator {
    rng: StdRng,
    config: FuzzConfig,
}

/// 模糊测试配置
#[derive(Debug, Clone)]
pub struct FuzzConfig {
    /// 测试迭代次数
    pub iterations: usize,
    /// 随机种子
    pub seed: u64,
    /// 字符串长度范围
    pub string_length_range: (usize, usize),
    /// 数值范围
    pub numeric_range: (f64, f64),
    /// 是否包含Unicode字符
    pub include_unicode: bool,
    /// 是否包含特殊字符
    pub include_special_chars: bool,
}

impl Default for FuzzConfig {
    fn default() -> Self {
        Self {
            iterations: 1000,
            seed: 42,
            string_length_range: (1, 100),
            numeric_range: (-1000.0, 1000.0),
            include_unicode: true,
            include_special_chars: true,
        }
    }
}

impl FuzzDataGenerator {
    /// 创建新的模糊测试生成器
    pub fn new(config: FuzzConfig) -> Self {
        let rng = StdRng::seed_from_u64(config.seed);
        Self { rng, config }
    }

    /// 创建使用默认配置的生成器
    pub fn with_seed(seed: u64) -> Self {
        let config = FuzzConfig { seed, ..FuzzConfig::default() };
        Self::new(config)
    }

    /// 生成随机字符串
    pub fn generate_random_string(&mut self) -> String {
        let length_dist =
            Uniform::new(self.config.string_length_range.0, self.config.string_length_range.1 + 1);
        let length = length_dist.sample(&mut self.rng);

        if self.config.include_unicode && self.rng.gen_bool(0.3) {
            self.generate_unicode_string(length)
        } else if self.config.include_special_chars && self.rng.gen_bool(0.2) {
            self.generate_special_char_string(length)
        } else {
            self.generate_alphanumeric_string(length)
        }
    }

    /// 生成包含Unicode字符的字符串
    fn generate_unicode_string(&mut self, length: usize) -> String {
        let chinese_chars = "攻击力暴击率暴击伤害生命值防御力元素精通元素充能效率治疗加成火元素伤害加成水元素伤害加成雷元素伤害加成冰元素伤害加成风元素伤害加成岩元素伤害加成草元素伤害加成物理伤害加成";
        let chars: Vec<char> = chinese_chars.chars().collect();

        (0..length).map(|_| chars[self.rng.gen_range(0..chars.len())]).collect()
    }

    /// 生成包含特殊字符的字符串
    fn generate_special_char_string(&mut self, length: usize) -> String {
        let special_chars = "!@#$%^&*()_+-=[]{}|;':\",./<>?~`";
        let alphanumeric: String =
            (0..length / 2).map(|_| self.rng.sample(Alphanumeric) as char).collect();

        let special: String = (0..length - length / 2)
            .map(|_| {
                let chars: Vec<char> = special_chars.chars().collect();
                chars[self.rng.gen_range(0..chars.len())]
            })
            .collect();

        format!("{alphanumeric}{special}")
    }

    /// 生成字母数字字符串
    fn generate_alphanumeric_string(&mut self, length: usize) -> String {
        (0..length).map(|_| self.rng.sample(Alphanumeric) as char).collect()
    }

    /// 生成随机数值
    pub fn generate_random_number(&mut self) -> f64 {
        let dist = Uniform::new(self.config.numeric_range.0, self.config.numeric_range.1);
        dist.sample(&mut self.rng)
    }

    /// 生成边界数值
    pub fn generate_boundary_number(&mut self) -> f64 {
        let boundary_values = [
            0.0,
            -0.0,
            1.0,
            -1.0,
            f64::MIN,
            f64::MAX,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NAN,
            self.config.numeric_range.0,
            self.config.numeric_range.1,
        ];

        boundary_values[self.rng.gen_range(0..boundary_values.len())]
    }

    /// 生成圣遗物属性字符串（用于测试OCR识别）
    pub fn generate_artifact_stat_string(&mut self) -> String {
        let stat_names = [
            "攻击力",
            "暴击率",
            "暴击伤害",
            "生命值",
            "防御力",
            "元素精通",
            "元素充能效率",
            "治疗加成",
            "火元素伤害加成",
            "水元素伤害加成",
        ];

        let name = stat_names[self.rng.gen_range(0..stat_names.len())];
        let value = self.generate_random_number().abs();

        if self.rng.gen_bool(0.5) {
            // 百分比格式
            format!("{name}+{:.1}%", value * 100.0)
        } else {
            // 固定值格式
            format!("{name}+{value:.0}")
        }
    }

    /// 生成损坏的属性字符串（测试错误处理）
    pub fn generate_corrupted_stat_string(&mut self) -> String {
        let patterns = [
            format!("{}+", self.generate_random_string()), // 缺少数值
            format!("+{:.2}", self.generate_random_number()), // 缺少属性名
            self.generate_random_string(),                 // 完全随机
            format!(
                "{}+{}+{}",
                self.generate_random_string(),
                self.generate_random_number(),
                self.generate_random_string()
            ), // 多个+号
            format!("{}={:.2}", self.generate_random_string(), self.generate_random_number()), // 错误分隔符
        ];

        patterns[self.rng.gen_range(0..patterns.len())].clone()
    }

    /// 生成等级字符串
    pub fn generate_level_string(&mut self) -> String {
        let level = self.rng.gen_range(0..=20);

        match self.rng.gen_range(0..4) {
            0 => format!("{level}"),     // 纯数字
            1 => format!("+{level}"),    // 带+号
            2 => format!("Lv.{level}"),  // 带前缀
            3 => format!("等级{level}"), // 中文前缀
            _ => unreachable!(),
        }
    }

    /// 生成损坏的等级字符串
    pub fn generate_corrupted_level_string(&mut self) -> String {
        let patterns = [
            "++".to_string(),
            format!("+{}", self.generate_random_string()),
            format!("{}+", self.generate_random_number()),
            self.generate_random_string(),
            "".to_string(),
        ];

        patterns[self.rng.gen_range(0..patterns.len())].clone()
    }
}

/// 模糊测试执行器
pub struct FuzzTester {
    generator: FuzzDataGenerator,
    results: FuzzTestResults,
}

/// 模糊测试结果统计
#[derive(Debug, Default)]
pub struct FuzzTestResults {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub crashes: usize,
    pub timeouts: usize,
    pub unique_failures: HashMap<String, usize>,
    pub execution_times: Vec<Duration>,
}

impl FuzzTestResults {
    /// 添加测试结果
    pub fn add_result(&mut self, result: FuzzTestResult, execution_time: Duration) {
        self.total_tests += 1;
        self.execution_times.push(execution_time);

        match result {
            FuzzTestResult::Passed => self.passed_tests += 1,
            FuzzTestResult::Failed(error) => {
                self.failed_tests += 1;
                *self.unique_failures.entry(error).or_insert(0) += 1;
            },
            FuzzTestResult::Crashed => self.crashes += 1,
            FuzzTestResult::Timeout => self.timeouts += 1,
        }
    }

    /// 获取成功率
    pub fn success_rate(&self) -> f64 {
        if self.total_tests == 0 {
            0.0
        } else {
            (self.passed_tests as f64 / self.total_tests as f64) * 100.0
        }
    }

    /// 获取平均执行时间
    pub fn average_execution_time(&self) -> Duration {
        if self.execution_times.is_empty() {
            Duration::ZERO
        } else {
            let total: Duration = self.execution_times.iter().sum();
            total / self.execution_times.len() as u32
        }
    }

    /// 获取测试报告
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== 模糊测试报告 ===\n");
        report.push_str(&format!("总测试数: {}\n", self.total_tests));
        report.push_str(&format!("通过: {} ({:.1}%)\n", self.passed_tests, self.success_rate()));
        report.push_str(&format!("失败: {}\n", self.failed_tests));
        report.push_str(&format!("崩溃: {}\n", self.crashes));
        report.push_str(&format!("超时: {}\n", self.timeouts));
        report.push_str(&format!("平均执行时间: {:?}\n", self.average_execution_time()));

        if !self.unique_failures.is_empty() {
            report.push_str("\n=== 失败模式统计 ===\n");
            for (error, count) in &self.unique_failures {
                report.push_str(&format!("{error}: {count} 次\n"));
            }
        }

        report
    }
}

/// 单个模糊测试结果
#[derive(Debug, Clone)]
pub enum FuzzTestResult {
    Passed,
    Failed(String),
    Crashed,
    Timeout,
}

impl FuzzTester {
    /// 创建新的模糊测试器
    pub fn new(config: FuzzConfig) -> Self {
        Self { generator: FuzzDataGenerator::new(config), results: FuzzTestResults::default() }
    }

    /// 执行字符串解析模糊测试
    pub fn fuzz_string_parsing<F>(&mut self, test_fn: F)
    where
        F: Fn(&str) -> Result<(), String> + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    {
        for _ in 0..self.generator.config.iterations {
            let test_string = self.generator.generate_random_string();
            let start_time = std::time::Instant::now();

            let result = std::panic::catch_unwind(|| test_fn(&test_string));

            let execution_time = start_time.elapsed();

            let test_result = match result {
                Ok(Ok(())) => FuzzTestResult::Passed,
                Ok(Err(error)) => FuzzTestResult::Failed(error),
                Err(_) => FuzzTestResult::Crashed,
            };

            self.results.add_result(test_result, execution_time);
        }
    }

    /// 执行属性解析模糊测试
    pub fn fuzz_attribute_parsing<F>(&mut self, test_fn: F)
    where
        F: Fn(&str) -> Result<(String, f64, bool), String>
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
    {
        for _ in 0..self.generator.config.iterations {
            let test_string = if self.generator.rng.gen_bool(0.7) {
                self.generator.generate_artifact_stat_string()
            } else {
                self.generator.generate_corrupted_stat_string()
            };

            let start_time = std::time::Instant::now();

            let result = std::panic::catch_unwind(|| test_fn(&test_string));

            let execution_time = start_time.elapsed();

            let test_result = match result {
                Ok(Ok(_)) => FuzzTestResult::Passed,
                Ok(Err(error)) => FuzzTestResult::Failed(error),
                Err(_) => FuzzTestResult::Crashed,
            };

            self.results.add_result(test_result, execution_time);
        }
    }

    /// 执行等级解析模糊测试
    pub fn fuzz_level_parsing<F>(&mut self, test_fn: F)
    where
        F: Fn(&str) -> Result<i32, String> + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    {
        for _ in 0..self.generator.config.iterations {
            let test_string = if self.generator.rng.gen_bool(0.8) {
                self.generator.generate_level_string()
            } else {
                self.generator.generate_corrupted_level_string()
            };

            let start_time = std::time::Instant::now();

            let result = std::panic::catch_unwind(|| test_fn(&test_string));

            let execution_time = start_time.elapsed();

            let test_result = match result {
                Ok(Ok(_)) => FuzzTestResult::Passed,
                Ok(Err(error)) => FuzzTestResult::Failed(error),
                Err(_) => FuzzTestResult::Crashed,
            };

            self.results.add_result(test_result, execution_time);
        }
    }

    /// 获取测试结果
    pub fn get_results(&self) -> &FuzzTestResults {
        &self.results
    }

    /// 重置测试结果
    pub fn reset(&mut self) {
        self.results = FuzzTestResults::default();
    }
}

/// 模糊测试宏
#[macro_export]
macro_rules! fuzz_test {
    ($name:ident, $iterations:expr, $test_fn:expr) => {
        #[test]
        fn $name() {
            let config = $crate::testing::fuzz_testing::FuzzConfig {
                iterations: $iterations,
                ..Default::default()
            };

            let mut tester = $crate::testing::fuzz_testing::FuzzTester::new(config);

            tester.fuzz_string_parsing(|input| $test_fn(input).map_err(|e| e.to_string()));

            let results = tester.get_results();

            println!("{}", results.generate_report());

            // 确保没有崩溃
            assert_eq!(results.crashes, 0, "模糊测试检测到崩溃");

            // 确保大部分测试能正常处理（即使失败也要优雅处理）
            let handled_rate = ((results.passed_tests + results.failed_tests) as f64
                / results.total_tests as f64)
                * 100.0;
            assert!(handled_rate >= 95.0, "测试处理率过低: {:.1}%", handled_rate);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzz_data_generator() {
        let mut generator = FuzzDataGenerator::with_seed(42);

        // 测试字符串生成
        let string1 = generator.generate_random_string();
        let string2 = generator.generate_random_string();

        assert!(!string1.is_empty());
        assert!(!string2.is_empty());

        // 同一种子应该产生不同的字符串（因为状态在变化）
        // 但序列应该是可重现的

        // 测试数值生成
        let number = generator.generate_random_number();
        assert!(number >= generator.config.numeric_range.0);
        assert!(number <= generator.config.numeric_range.1);
    }

    #[test]
    fn test_artifact_stat_generation() {
        let mut generator = FuzzDataGenerator::with_seed(123);

        for _ in 0..100 {
            let stat = generator.generate_artifact_stat_string();
            assert!(stat.contains('+'));

            // 应该包含有效的属性名称之一
            let valid_names = [
                "攻击力",
                "暴击率",
                "暴击伤害",
                "生命值",
                "防御力",
                "元素精通",
                "元素充能效率",
                "治疗加成",
                "火元素伤害加成",
                "水元素伤害加成",
            ];

            let has_valid_name = valid_names.iter().any(|name| stat.contains(name));
            assert!(has_valid_name, "生成的属性字符串不包含有效名称: {stat}");
        }
    }

    #[test]
    fn test_corrupted_data_generation() {
        let mut generator = FuzzDataGenerator::with_seed(456);

        for _ in 0..50 {
            let corrupted = generator.generate_corrupted_stat_string();
            // 损坏的数据应该不为空
            assert!(!corrupted.is_empty());
        }
    }

    #[test]
    fn test_fuzz_test_results() {
        let mut results = FuzzTestResults::default();

        results.add_result(FuzzTestResult::Passed, Duration::from_millis(10));
        results.add_result(
            FuzzTestResult::Failed("test error".to_string()),
            Duration::from_millis(20),
        );
        results.add_result(
            FuzzTestResult::Failed("test error".to_string()),
            Duration::from_millis(15),
        );

        assert_eq!(results.total_tests, 3);
        assert_eq!(results.passed_tests, 1);
        assert_eq!(results.failed_tests, 2);
        assert_eq!(results.unique_failures.len(), 1);
        assert_eq!(results.unique_failures["test error"], 2);

        assert!((results.success_rate() - 33.33).abs() < 0.1);
    }

    #[test]
    fn test_boundary_number_generation() {
        let mut generator = FuzzDataGenerator::with_seed(789);

        let mut has_infinity = false;
        let mut has_zero = false;
        let mut has_nan = false;

        for _ in 0..1000 {
            let num = generator.generate_boundary_number();
            if num.is_infinite() {
                has_infinity = true;
            }
            if num == 0.0 {
                has_zero = true;
            }
            if num.is_nan() {
                has_nan = true;
            }
        }

        // 在足够多的迭代中，应该能生成这些边界值
        assert!(has_infinity || has_zero || has_nan, "应该生成一些边界值");
    }

    #[test]
    fn test_level_string_generation() {
        let mut generator = FuzzDataGenerator::with_seed(101112);

        for _ in 0..100 {
            let level_str = generator.generate_level_string();
            assert!(!level_str.is_empty());

            // 应该包含数字
            let has_digit = level_str.chars().any(|c| c.is_ascii_digit());
            assert!(has_digit, "等级字符串应该包含数字: {level_str}");
        }
    }

    #[test]
    fn test_unicode_string_generation() {
        let config = FuzzConfig {
            include_unicode: true,
            string_length_range: (5, 10),
            ..FuzzConfig::default()
        };

        let mut generator = FuzzDataGenerator::new(config);

        // 强制生成Unicode字符串
        let unicode_str = generator.generate_unicode_string(10);
        assert_eq!(unicode_str.chars().count(), 10);

        // 应该包含中文字符
        let has_chinese = unicode_str.chars().any(|c| {
            c as u32 >= 0x4E00 && c as u32 <= 0x9FFF // 中文字符范围
        });
        assert!(has_chinese, "应该包含中文字符: {unicode_str}");
    }

    // 示例：使用模糊测试宏
    fuzz_test!(fuzz_simple_parsing, 100, |input: &str| -> anyhow::Result<()> {
        // 测试简单的字符串解析是否会崩溃
        let _result = input.parse::<f64>();
        Ok(())
    });
}
