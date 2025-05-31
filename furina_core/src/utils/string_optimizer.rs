use std::borrow::Cow;
use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

/// 字符串处理性能优化工具集
///
/// 提供优化的字符串操作，减少不必要的内存分配和提高解析性能
pub struct StringOptimizer {
    regex_cache: HashMap<String, Regex>,
}

impl StringOptimizer {
    /// 创建新的字符串优化器实例
    pub fn new() -> Self {
        Self { regex_cache: HashMap::new() }
    }

    /// 获取缓存的正则表达式，避免重复编译
    pub fn get_cached_regex(&mut self, pattern: &str) -> anyhow::Result<&Regex> {
        if !self.regex_cache.contains_key(pattern) {
            let regex = Regex::new(pattern)?;
            self.regex_cache.insert(pattern.to_string(), regex);
        }
        Ok(self.regex_cache.get(pattern).unwrap())
    }

    /// 优化的属性值解析，使用零拷贝技术
    pub fn parse_attribute_value<'a>(
        &mut self,
        input: &'a str,
    ) -> anyhow::Result<(Cow<'a, str>, f64, bool)> {
        // 检查是否包含非法的连续分隔符
        if input.contains("++") {
            return Err(anyhow::anyhow!("包含非法的连续分隔符"));
        }

        // 使用迭代器避免临时Vec分配
        let mut parts = input.splitn(2, '+');

        let name = parts.next().ok_or_else(|| anyhow::anyhow!("缺少属性名称"))?;
        let value_str = parts.next().ok_or_else(|| anyhow::anyhow!("缺少属性值"))?;

        // 验证属性名不能为空
        if name.is_empty() {
            return Err(anyhow::anyhow!("属性名称不能为空"));
        }

        // 验证属性值不能为空
        if value_str.is_empty() {
            return Err(anyhow::anyhow!("属性值不能为空"));
        }

        let is_percentage = value_str.contains('%');

        // 使用缓存的正则表达式
        let regex = self.get_cached_regex(r"[%,]")?;
        let clean_value_str = regex.replace_all(value_str, "");

        let mut value = clean_value_str
            .parse::<f64>()
            .map_err(|e| anyhow::anyhow!("无法解析数值 '{}': {}", clean_value_str, e))?;

        if is_percentage {
            value /= 100.0;
        }

        Ok((Cow::Borrowed(name), value, is_percentage))
    }

    /// 快速字符串清理，移除不需要的字符
    pub fn fast_clean_string<'a>(&self, input: &'a str, chars_to_remove: &[char]) -> Cow<'a, str> {
        if !input.chars().any(|c| chars_to_remove.contains(&c)) {
            // 如果没有需要移除的字符，返回原字符串的借用
            Cow::Borrowed(input)
        } else {
            // 只有在需要时才分配新字符串
            let cleaned: String = input.chars().filter(|c| !chars_to_remove.contains(c)).collect();
            Cow::Owned(cleaned)
        }
    }

    /// 高效的等级解析
    pub fn parse_level_fast(&self, input: &str) -> anyhow::Result<i32> {
        // 快速路径：直接尝试解析整个字符串
        if let Ok(level) = input.parse::<i32>() {
            return Ok(level);
        }

        // 查找'+'符号
        if let Some(pos) = input.find('+') {
            let level_str = &input[pos + 1..];
            level_str
                .parse::<i32>()
                .map_err(|e| anyhow::anyhow!("无法解析等级 '{}': {}", level_str, e))
        } else {
            Err(anyhow::anyhow!("等级格式无效: '{}'", input))
        }
    }

    /// 批量处理字符串，提高吞吐量
    pub fn batch_process_stats(
        &mut self,
        stats: &[String],
    ) -> Vec<anyhow::Result<(String, f64, bool)>> {
        stats
            .iter()
            .map(|stat| match self.parse_attribute_value(stat) {
                Ok((name, value, is_percentage)) => Ok((name.into_owned(), value, is_percentage)),
                Err(e) => Err(e),
            })
            .collect()
    }
}

impl Default for StringOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

// 全局字符串优化器实例
lazy_static! {
    static ref GLOBAL_OPTIMIZER: std::sync::Mutex<StringOptimizer> =
        std::sync::Mutex::new(StringOptimizer::new());
}

/// 便利函数：使用全局优化器解析属性值
pub fn parse_stat_optimized(input: &str) -> anyhow::Result<(String, f64, bool)> {
    let mut optimizer = GLOBAL_OPTIMIZER.lock().unwrap();
    let (name, value, is_percentage) = optimizer.parse_attribute_value(input)?;
    Ok((name.into_owned(), value, is_percentage))
}

/// 便利函数：使用全局优化器解析等级
pub fn parse_level_optimized(input: &str) -> anyhow::Result<i32> {
    let optimizer = GLOBAL_OPTIMIZER.lock().unwrap();
    optimizer.parse_level_fast(input)
}

/// 高性能字符串池，减少重复字符串的内存占用
pub struct StringPool {
    pool: HashMap<String, &'static str>,
}

impl StringPool {
    pub fn new() -> Self {
        Self { pool: HashMap::new() }
    }

    /// 获取或插入字符串到池中
    pub fn intern(&mut self, s: String) -> &'static str {
        if let Some(&existing) = self.pool.get(&s) {
            existing
        } else {
            let leaked: &'static str = Box::leak(s.clone().into_boxed_str());
            self.pool.insert(s, leaked);
            leaked
        }
    }
}

impl Default for StringPool {
    fn default() -> Self {
        Self::new()
    }
}

/// 字符串处理性能统计
#[derive(Debug, Default)]
pub struct StringProcessingStats {
    pub total_operations: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub fast_path_usage: usize,
    pub slow_path_usage: usize,
}

impl StringProcessingStats {
    pub fn cache_hit_rate(&self) -> f64 {
        if self.total_operations == 0 {
            0.0
        } else {
            (self.cache_hits as f64 / self.total_operations as f64) * 100.0
        }
    }

    pub fn fast_path_rate(&self) -> f64 {
        let path_operations = self.fast_path_usage + self.slow_path_usage;
        if path_operations == 0 {
            0.0
        } else {
            (self.fast_path_usage as f64 / path_operations as f64) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn test_string_optimizer_basic() {
        let mut optimizer = StringOptimizer::new();

        let result = optimizer.parse_attribute_value("攻击力+46.6%");
        assert!(result.is_ok());

        let (name, value, is_percentage) = result.unwrap();
        assert_eq!(name, "攻击力");
        assert!((value - 0.466).abs() < f64::EPSILON);
        assert!(is_percentage);
    }

    #[test]
    fn test_string_optimizer_performance() {
        let mut optimizer = StringOptimizer::new();

        let test_stats = vec![
            "攻击力+46.6%".to_string(),
            "暴击率+12.1%".to_string(),
            "暴击伤害+22.5%".to_string(),
            "生命值+4780".to_string(),
            "防御力+58".to_string(),
        ];

        let start = Instant::now();

        // 重复处理多次来测试性能
        for _ in 0..1000 {
            let _results = optimizer.batch_process_stats(&test_stats);
        }

        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 1000, "字符串处理性能测试超时: {elapsed:?}");
    }

    #[test]
    fn test_regex_caching() {
        let mut optimizer = StringOptimizer::new();

        // 第一次获取正则表达式
        let pattern = r"[%,]";
        optimizer.get_cached_regex(pattern).unwrap();

        // 验证缓存工作正常（通过检查缓存大小）
        assert_eq!(optimizer.regex_cache.len(), 1);

        // 第二次获取相同的正则表达式（应该来自缓存）
        let regex2 = optimizer.get_cached_regex(pattern).unwrap();
        assert!(regex2.is_match("%"));
    }

    #[test]
    fn test_fast_clean_string() {
        let optimizer = StringOptimizer::new();

        // 测试无需清理的情况（零拷贝）
        let clean_input = "clean_string";
        let result = optimizer.fast_clean_string(clean_input, &['%', ',']);
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "clean_string");

        // 测试需要清理的情况
        let dirty_input = "dirty%string,with%chars";
        let result = optimizer.fast_clean_string(dirty_input, &['%', ',']);
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "dirtystringwithchars");
    }

    #[test]
    fn test_level_parsing_fast() {
        let optimizer = StringOptimizer::new();

        // 测试直接数字
        assert_eq!(optimizer.parse_level_fast("20").unwrap(), 20);

        // 测试带+号的等级
        assert_eq!(optimizer.parse_level_fast("+15").unwrap(), 15);

        // 测试错误情况
        assert!(optimizer.parse_level_fast("invalid").is_err());
    }

    #[test]
    fn test_batch_processing() {
        let mut optimizer = StringOptimizer::new();

        let stats =
            vec!["攻击力+46.6%".to_string(), "无效格式".to_string(), "暴击率+12.1%".to_string()];

        let results = optimizer.batch_process_stats(&stats);

        assert_eq!(results.len(), 3);
        assert!(results[0].is_ok());
        assert!(results[1].is_err());
        assert!(results[2].is_ok());

        let (name, value, is_percentage) = results[0].as_ref().unwrap();
        assert_eq!(name, "攻击力");
        assert!((value - 0.466).abs() < f64::EPSILON);
        assert!(*is_percentage);
    }

    #[test]
    fn test_global_functions() {
        // 测试全局便利函数
        let result = parse_stat_optimized("元素精通+42");
        assert!(result.is_ok());

        let (name, value, is_percentage) = result.unwrap();
        assert_eq!(name, "元素精通");
        assert!((value - 42.0).abs() < f64::EPSILON);
        assert!(!is_percentage);

        // 测试等级解析
        let level = parse_level_optimized("+16");
        assert!(level.is_ok());
        assert_eq!(level.unwrap(), 16);
    }

    #[test]
    fn test_string_pool() {
        let mut pool = StringPool::new();

        let str1 = pool.intern("test_string".to_string());
        let str2 = pool.intern("test_string".to_string());

        // 应该返回相同的引用
        assert!(std::ptr::eq(str1, str2));
    }

    #[test]
    fn test_performance_stats() {
        let stats = StringProcessingStats {
            total_operations: 100,
            cache_hits: 80,
            cache_misses: 20,
            fast_path_usage: 70,
            slow_path_usage: 30,
        };

        assert!((stats.cache_hit_rate() - 80.0).abs() < f64::EPSILON);
        assert!((stats.fast_path_rate() - 70.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_cow_string_efficiency() {
        let optimizer = StringOptimizer::new();

        // 测试Cow<str>的效率
        let original = "test_string_without_special_chars";
        let result = optimizer.fast_clean_string(original, &['%', ',', '!']);

        // 应该返回借用，因为没有需要移除的字符
        if let Cow::Borrowed(borrowed) = result {
            assert!(std::ptr::eq(borrowed.as_ptr(), original.as_ptr()));
        } else {
            panic!("期望返回借用的字符串");
        }
    }
}
