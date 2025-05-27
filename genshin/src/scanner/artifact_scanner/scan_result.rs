use std::hash::{Hash, Hasher};

use super::error::ArtifactScanError;

#[derive(Debug, Clone)]
pub struct GenshinArtifactScanResult {
    pub name: String,
    pub main_stat_name: String,
    pub main_stat_value: String,
    pub sub_stat: [String; 4],
    pub equip: String,
    pub level: i32,
    pub star: i32,
    pub lock: bool,
    /// 扫描过程中遇到的错误（如果有）
    pub scan_errors: Vec<String>,
    /// 识别置信度评分 (0.0-1.0)
    pub confidence_score: f64,
}

// 手动实现Hash，只对核心字段进行哈希，忽略错误信息和置信度
impl Hash for GenshinArtifactScanResult {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.main_stat_name.hash(state);
        self.main_stat_value.hash(state);
        self.sub_stat.hash(state);
        self.equip.hash(state);
        self.level.hash(state);
        self.star.hash(state);
        self.lock.hash(state);
        // 不对 scan_errors 和 confidence_score 进行哈希
    }
}

// 手动实现PartialEq，只比较核心字段
impl PartialEq for GenshinArtifactScanResult {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.main_stat_name == other.main_stat_name
            && self.main_stat_value == other.main_stat_value
            && self.sub_stat == other.sub_stat
            && self.equip == other.equip
            && self.level == other.level
            && self.star == other.star
            && self.lock == other.lock
        // 不比较 scan_errors 和 confidence_score
    }
}

// 实现Eq trait
impl Eq for GenshinArtifactScanResult {}

impl GenshinArtifactScanResult {
    /// 创建一个新的扫描结果
    pub fn new(
        name: String,
        main_stat_name: String,
        main_stat_value: String,
        sub_stat: [String; 4],
        equip: String,
        level: i32,
        star: i32,
        lock: bool,
    ) -> Self {
        Self {
            name,
            main_stat_name,
            main_stat_value,
            sub_stat,
            equip,
            level,
            star,
            lock,
            scan_errors: Vec::new(),
            confidence_score: 1.0,
        }
    }

    /// 添加扫描错误
    pub fn add_error(&mut self, error: &ArtifactScanError) {
        self.scan_errors.push(error.to_string());
        // 根据错误类型降低置信度
        match error {
            ArtifactScanError::OcrRecognitionFailed { .. } => self.confidence_score *= 0.8,
            ArtifactScanError::ArtifactParsingFailed { .. } => self.confidence_score *= 0.7,
            ArtifactScanError::StarRecognitionFailed { .. } => self.confidence_score *= 0.9,
            ArtifactScanError::LevelParsingFailed { .. } => self.confidence_score *= 0.9,
            _ => self.confidence_score *= 0.95,
        }
        self.confidence_score = self.confidence_score.max(0.0);
    }

    /// 检查是否有错误
    pub fn has_errors(&self) -> bool {
        !self.scan_errors.is_empty()
    }

    /// 获取错误数量
    pub fn error_count(&self) -> usize {
        self.scan_errors.len()
    }

    /// 检查置信度是否足够高
    pub fn is_reliable(&self, threshold: f64) -> bool {
        self.confidence_score >= threshold
    }
}
