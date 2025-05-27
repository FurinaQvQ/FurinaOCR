use std::fmt;

/// 圣遗物扫描错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum ArtifactScanError {
    /// OCR识别失败
    OcrRecognitionFailed { field: String, raw_text: String, error_msg: String },
    /// 图像捕获失败
    ImageCaptureFailed { region: String, error_msg: String },
    /// 圣遗物数据解析失败
    ArtifactParsingFailed { field: String, value: String, expected_format: String },
    /// 连续重复物品检测
    ConsecutiveDuplicateItems { count: usize, threshold: usize },
    /// 星级识别错误
    StarRecognitionFailed { detected_color: String, confidence: f64 },
    /// 等级解析错误
    LevelParsingFailed { raw_text: String, error_msg: String },
    /// 模型加载失败
    ModelLoadFailed { model_path: String, error_msg: String },
    /// 窗口信息获取失败
    WindowInfoFailed { error_msg: String },
    /// 扫描中断
    ScanInterrupted { reason: String, scanned_count: usize },
    /// 未知错误
    Unknown { error_msg: String },
}

impl fmt::Display for ArtifactScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArtifactScanError::OcrRecognitionFailed { field, raw_text, error_msg } => {
                write!(
                    f,
                    "OCR识别失败 - 字段: {}, 原始文本: '{}', 错误: {}",
                    field, raw_text, error_msg
                )
            },
            ArtifactScanError::ImageCaptureFailed { region, error_msg } => {
                write!(f, "图像捕获失败 - 区域: {}, 错误: {}", region, error_msg)
            },
            ArtifactScanError::ArtifactParsingFailed { field, value, expected_format } => {
                write!(
                    f,
                    "圣遗物数据解析失败 - 字段: {}, 值: '{}', 期望格式: {}",
                    field, value, expected_format
                )
            },
            ArtifactScanError::ConsecutiveDuplicateItems { count, threshold } => {
                write!(
                    f,
                    "检测到连续重复物品 - 数量: {}, 阈值: {} (可能为翻页错误或非背包顶部开始扫描)",
                    count, threshold
                )
            },
            ArtifactScanError::StarRecognitionFailed { detected_color, confidence } => {
                write!(
                    f,
                    "星级识别失败 - 检测到颜色: {}, 置信度: {:.2}",
                    detected_color, confidence
                )
            },
            ArtifactScanError::LevelParsingFailed { raw_text, error_msg } => {
                write!(f, "等级解析失败 - 原始文本: '{}', 错误: {}", raw_text, error_msg)
            },
            ArtifactScanError::ModelLoadFailed { model_path, error_msg } => {
                write!(f, "模型加载失败 - 路径: {}, 错误: {}", model_path, error_msg)
            },
            ArtifactScanError::WindowInfoFailed { error_msg } => {
                write!(f, "窗口信息获取失败 - 错误: {}", error_msg)
            },
            ArtifactScanError::ScanInterrupted { reason, scanned_count } => {
                write!(f, "扫描中断 - 原因: {}, 已扫描数量: {}", reason, scanned_count)
            },
            ArtifactScanError::Unknown { error_msg } => {
                write!(f, "未知错误 - {}", error_msg)
            },
        }
    }
}

impl std::error::Error for ArtifactScanError {}

/// 错误统计信息
#[derive(Debug, Clone, Default)]
pub struct ErrorStatistics {
    pub ocr_errors: usize,
    pub image_capture_errors: usize,
    pub parsing_errors: usize,
    pub star_recognition_errors: usize,
    pub level_parsing_errors: usize,
    pub duplicate_items: usize,
    pub total_errors: usize,
    pub successful_scans: usize,
}

impl ErrorStatistics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_error(&mut self, error: &ArtifactScanError) {
        self.total_errors += 1;
        match error {
            ArtifactScanError::OcrRecognitionFailed { .. } => self.ocr_errors += 1,
            ArtifactScanError::ImageCaptureFailed { .. } => self.image_capture_errors += 1,
            ArtifactScanError::ArtifactParsingFailed { .. } => self.parsing_errors += 1,
            ArtifactScanError::StarRecognitionFailed { .. } => self.star_recognition_errors += 1,
            ArtifactScanError::LevelParsingFailed { .. } => self.level_parsing_errors += 1,
            ArtifactScanError::ConsecutiveDuplicateItems { .. } => self.duplicate_items += 1,
            _ => {},
        }
    }

    pub fn record_success(&mut self) {
        self.successful_scans += 1;
    }

    pub fn get_success_rate(&self) -> f64 {
        let total = self.successful_scans + self.total_errors;
        if total == 0 {
            0.0
        } else {
            self.successful_scans as f64 / total as f64 * 100.0
        }
    }

    pub fn get_error_summary(&self) -> String {
        format!(
            "错误统计报告:\n\
            - 成功扫描: {} 个\n\
            - 总错误数: {} 个\n\
            - 成功率: {:.1}%\n\
            - OCR识别错误: {} 个\n\
            - 图像捕获错误: {} 个\n\
            - 数据解析错误: {} 个\n\
            - 星级识别错误: {} 个\n\
            - 等级解析错误: {} 个\n\
            - 重复物品检测: {} 个",
            self.successful_scans,
            self.total_errors,
            self.get_success_rate(),
            self.ocr_errors,
            self.image_capture_errors,
            self.parsing_errors,
            self.star_recognition_errors,
            self.level_parsing_errors,
            self.duplicate_items
        )
    }
}

/// 错误处理建议
pub fn get_error_suggestion(error: &ArtifactScanError) -> String {
    match error {
        ArtifactScanError::OcrRecognitionFailed { field, .. } => {
            format!("建议: 检查游戏界面是否清晰，确保{}区域没有被遮挡", field)
        },
        ArtifactScanError::ImageCaptureFailed { region, .. } => {
            format!("建议: 检查游戏窗口是否正常显示，{}区域是否可见", region)
        },
        ArtifactScanError::ArtifactParsingFailed { field, .. } => {
            format!("建议: 检查{}的显示格式是否正常，可能需要切换游戏语言为简体中文", field)
        },
        ArtifactScanError::ConsecutiveDuplicateItems { .. } => {
            "建议: 请确保从背包顶部开始扫描，避免在扫描过程中手动翻页".to_string()
        },
        ArtifactScanError::StarRecognitionFailed { .. } => {
            "建议: 检查游戏亮度设置，确保星级显示清晰可见".to_string()
        },
        ArtifactScanError::LevelParsingFailed { .. } => {
            "建议: 检查圣遗物等级显示是否正常，确保使用简体中文界面".to_string()
        },
        ArtifactScanError::ModelLoadFailed { .. } => {
            "建议: 检查模型文件是否存在且完整，可能需要重新下载程序".to_string()
        },
        ArtifactScanError::WindowInfoFailed { .. } => {
            "建议: 检查游戏分辨率是否受支持，推荐使用16:9比例的分辨率".to_string()
        },
        ArtifactScanError::ScanInterrupted { .. } => {
            "建议: 扫描被中断，可以重新开始扫描".to_string()
        },
        ArtifactScanError::Unknown { .. } => {
            "建议: 遇到未知错误，请检查系统环境或联系开发者".to_string()
        },
    }
}
