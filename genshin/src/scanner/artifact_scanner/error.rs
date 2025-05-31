use std::fmt;

use furina_core::error_recovery::{ErrorCategory, RecoverableError};

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

impl RecoverableError for ArtifactScanError {
    fn error_category(&self) -> ErrorCategory {
        match self {
            ArtifactScanError::OcrRecognitionFailed { .. } => ErrorCategory::OCR,
            ArtifactScanError::ImageCaptureFailed { .. } => ErrorCategory::ImageProcessing,
            ArtifactScanError::ArtifactParsingFailed { .. } => ErrorCategory::Parsing,
            ArtifactScanError::ConsecutiveDuplicateItems { .. } => ErrorCategory::Temporary,
            ArtifactScanError::StarRecognitionFailed { .. } => ErrorCategory::OCR,
            ArtifactScanError::LevelParsingFailed { .. } => ErrorCategory::Parsing,
            ArtifactScanError::ModelLoadFailed { .. } => ErrorCategory::Configuration,
            ArtifactScanError::WindowInfoFailed { .. } => ErrorCategory::Configuration,
            ArtifactScanError::ScanInterrupted { .. } => ErrorCategory::Temporary,
            ArtifactScanError::Unknown { .. } => ErrorCategory::Unknown,
        }
    }
}

impl fmt::Display for ArtifactScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArtifactScanError::OcrRecognitionFailed { field, raw_text, error_msg } => {
                write!(f, "OCR识别失败 - 字段: {field}, 原始文本: '{raw_text}', 错误: {error_msg}")
            },
            ArtifactScanError::ImageCaptureFailed { region, error_msg } => {
                write!(f, "图像捕获失败 - 区域: {region}, 错误: {error_msg}")
            },
            ArtifactScanError::ArtifactParsingFailed { field, value, expected_format } => {
                write!(
                    f,
                    "圣遗物数据解析失败 - 字段: {field}, 值: '{value}', 期望格式: {expected_format}"
                )
            },
            ArtifactScanError::ConsecutiveDuplicateItems { count, threshold } => {
                write!(
                    f,
                    "检测到连续重复物品 - 数量: {count}, 阈值: {threshold} (可能为翻页错误或非背包顶部开始扫描)"
                )
            },
            ArtifactScanError::StarRecognitionFailed { detected_color, confidence } => {
                write!(f, "星级识别失败 - 检测到颜色: {detected_color}, 置信度: {confidence:.2}")
            },
            ArtifactScanError::LevelParsingFailed { raw_text, error_msg } => {
                write!(f, "等级解析失败 - 原始文本: '{raw_text}', 错误: {error_msg}")
            },
            ArtifactScanError::ModelLoadFailed { model_path, error_msg } => {
                write!(f, "模型加载失败 - 路径: {model_path}, 错误: {error_msg}")
            },
            ArtifactScanError::WindowInfoFailed { error_msg } => {
                write!(f, "窗口信息获取失败 - 错误: {error_msg}")
            },
            ArtifactScanError::ScanInterrupted { reason, scanned_count } => {
                write!(f, "扫描中断 - 原因: {reason}, 已扫描: {scanned_count} 个")
            },
            ArtifactScanError::Unknown { error_msg } => {
                write!(f, "未知错误: {error_msg}")
            },
        }
    }
}

impl std::error::Error for ArtifactScanError {}

/// 错误统计信息
#[derive(Debug, Clone, Default)]
pub struct ErrorStatistics {
    pub successful_scans: usize,
    pub total_errors: usize,
    pub ocr_errors: usize,
    pub image_capture_errors: usize,
    pub parsing_errors: usize,
    pub star_recognition_errors: usize,
    pub level_parsing_errors: usize,
    pub duplicate_items: usize,
    pub model_load_errors: usize,
    pub window_info_errors: usize,
    pub interruption_errors: usize,
    pub unknown_errors: usize,
}

impl ErrorStatistics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_error(&mut self, error: &ArtifactScanError) {
        self.total_errors += 1;
        match error {
            ArtifactScanError::OcrRecognitionFailed { .. } => self.ocr_errors += 1,
            ArtifactScanError::ImageCaptureFailed { .. } => self.image_capture_errors += 1,
            ArtifactScanError::ArtifactParsingFailed { .. } => self.parsing_errors += 1,
            ArtifactScanError::StarRecognitionFailed { .. } => self.star_recognition_errors += 1,
            ArtifactScanError::LevelParsingFailed { .. } => self.level_parsing_errors += 1,
            ArtifactScanError::ConsecutiveDuplicateItems { .. } => self.duplicate_items += 1,
            ArtifactScanError::ModelLoadFailed { .. } => self.model_load_errors += 1,
            ArtifactScanError::WindowInfoFailed { .. } => self.window_info_errors += 1,
            ArtifactScanError::ScanInterrupted { .. } => self.interruption_errors += 1,
            ArtifactScanError::Unknown { .. } => self.unknown_errors += 1,
        }
    }

    pub fn add_success(&mut self) {
        self.successful_scans += 1;
    }

    pub fn get_success_rate(&self) -> f64 {
        let total_attempts = self.successful_scans + self.total_errors;
        if total_attempts == 0 {
            return 0.0;
        }
        (self.successful_scans as f64 / total_attempts as f64) * 100.0
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
            format!("建议: 检查游戏界面是否清晰，确保{field}区域没有被遮挡")
        },
        ArtifactScanError::ImageCaptureFailed { region, .. } => {
            format!("建议: 检查游戏窗口是否正常显示，{region}区域是否可见")
        },
        ArtifactScanError::ArtifactParsingFailed { field, .. } => {
            format!("建议: 检查{field}的显示格式是否正常，可能需要切换游戏语言为简体中文")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_formatting() {
        let ocr_error = ArtifactScanError::OcrRecognitionFailed {
            field: "圣遗物名称".to_string(),
            raw_text: "不清楚的文字".to_string(),
            error_msg: "模型推理失败".to_string(),
        };
        let display = format!("{ocr_error}");
        assert!(display.contains("OCR识别失败"));
        assert!(display.contains("圣遗物名称"));
        assert!(display.contains("不清楚的文字"));
        assert!(display.contains("模型推理失败"));

        let capture_error = ArtifactScanError::ImageCaptureFailed {
            region: "主属性区域".to_string(),
            error_msg: "屏幕捕获异常".to_string(),
        };
        let display = format!("{capture_error}");
        assert!(display.contains("图像捕获失败"));
        assert!(display.contains("主属性区域"));
        assert!(display.contains("屏幕捕获异常"));

        let parsing_error = ArtifactScanError::ArtifactParsingFailed {
            field: "攻击力".to_string(),
            value: "46.6".to_string(),
            expected_format: "攻击力+46.6%".to_string(),
        };
        let display = format!("{parsing_error}");
        assert!(display.contains("圣遗物数据解析失败"));
        assert!(display.contains("攻击力"));
        assert!(display.contains("46.6"));
        assert!(display.contains("攻击力+46.6%"));
    }

    #[test]
    fn test_error_equality() {
        let error1 = ArtifactScanError::OcrRecognitionFailed {
            field: "测试字段".to_string(),
            raw_text: "测试文本".to_string(),
            error_msg: "测试错误".to_string(),
        };

        let error2 = ArtifactScanError::OcrRecognitionFailed {
            field: "测试字段".to_string(),
            raw_text: "测试文本".to_string(),
            error_msg: "测试错误".to_string(),
        };

        let error3 = ArtifactScanError::ImageCaptureFailed {
            region: "测试区域".to_string(),
            error_msg: "测试错误".to_string(),
        };

        assert_eq!(error1, error2);
        assert_ne!(error1, error3);
    }

    #[test]
    fn test_error_statistics_basic() {
        let mut stats = ErrorStatistics::new();

        // 初始状态
        assert_eq!(stats.successful_scans, 0);
        assert_eq!(stats.total_errors, 0);
        assert_eq!(stats.get_success_rate(), 0.0);

        // 添加成功记录
        stats.add_success();
        stats.add_success();
        assert_eq!(stats.successful_scans, 2);
        assert_eq!(stats.get_success_rate(), 100.0);

        // 添加错误记录
        let ocr_error = ArtifactScanError::OcrRecognitionFailed {
            field: "test".to_string(),
            raw_text: "test".to_string(),
            error_msg: "test".to_string(),
        };
        stats.add_error(&ocr_error);

        assert_eq!(stats.total_errors, 1);
        assert_eq!(stats.ocr_errors, 1);
        assert!((stats.get_success_rate() - 66.7).abs() < 0.1); // 2/(2+1) * 100 ≈ 66.7%
    }

    #[test]
    fn test_error_statistics_comprehensive() {
        let mut stats = ErrorStatistics::new();

        // 测试各种类型的错误统计
        let errors = vec![
            ArtifactScanError::OcrRecognitionFailed {
                field: "名称".to_string(),
                raw_text: "".to_string(),
                error_msg: "".to_string(),
            },
            ArtifactScanError::ImageCaptureFailed {
                region: "区域".to_string(),
                error_msg: "".to_string(),
            },
            ArtifactScanError::ArtifactParsingFailed {
                field: "属性".to_string(),
                value: "".to_string(),
                expected_format: "".to_string(),
            },
            ArtifactScanError::StarRecognitionFailed {
                detected_color: "RGB(255,0,0)".to_string(),
                confidence: 0.5,
            },
            ArtifactScanError::LevelParsingFailed {
                raw_text: "+20".to_string(),
                error_msg: "".to_string(),
            },
            ArtifactScanError::ConsecutiveDuplicateItems { count: 5, threshold: 3 },
            ArtifactScanError::ModelLoadFailed {
                model_path: "./model.onnx".to_string(),
                error_msg: "文件不存在".to_string(),
            },
            ArtifactScanError::WindowInfoFailed { error_msg: "分辨率不支持".to_string() },
            ArtifactScanError::ScanInterrupted {
                reason: "用户中断".to_string(),
                scanned_count: 10,
            },
            ArtifactScanError::Unknown { error_msg: "未知异常".to_string() },
        ];

        for error in &errors {
            stats.add_error(error);
        }

        // 验证各类错误的计数
        assert_eq!(stats.total_errors, 10);
        assert_eq!(stats.ocr_errors, 1);
        assert_eq!(stats.image_capture_errors, 1);
        assert_eq!(stats.parsing_errors, 1);
        assert_eq!(stats.star_recognition_errors, 1);
        assert_eq!(stats.level_parsing_errors, 1);
        assert_eq!(stats.duplicate_items, 1);
        assert_eq!(stats.model_load_errors, 1);
        assert_eq!(stats.window_info_errors, 1);
        assert_eq!(stats.interruption_errors, 1);
        assert_eq!(stats.unknown_errors, 1);
    }

    #[test]
    fn test_error_statistics_success_rate() {
        let mut stats = ErrorStatistics::new();

        // 添加5次成功，5次失败
        for _ in 0..5 {
            stats.add_success();
        }

        for _ in 0..5 {
            let error = ArtifactScanError::Unknown { error_msg: "test".to_string() };
            stats.add_error(&error);
        }

        assert_eq!(stats.get_success_rate(), 50.0); // 5/(5+5) * 100 = 50%
    }

    #[test]
    fn test_error_suggestions() {
        let test_cases = vec![
            (
                ArtifactScanError::OcrRecognitionFailed {
                    field: "圣遗物名称".to_string(),
                    raw_text: "".to_string(),
                    error_msg: "".to_string(),
                },
                "圣遗物名称",
            ),
            (
                ArtifactScanError::ImageCaptureFailed {
                    region: "副属性区域".to_string(),
                    error_msg: "".to_string(),
                },
                "副属性区域",
            ),
            (
                ArtifactScanError::ArtifactParsingFailed {
                    field: "攻击力".to_string(),
                    value: "".to_string(),
                    expected_format: "".to_string(),
                },
                "攻击力",
            ),
            (ArtifactScanError::ConsecutiveDuplicateItems { count: 5, threshold: 3 }, "背包顶部"),
            (
                ArtifactScanError::StarRecognitionFailed {
                    detected_color: "".to_string(),
                    confidence: 0.0,
                },
                "亮度设置",
            ),
            (
                ArtifactScanError::LevelParsingFailed {
                    raw_text: "".to_string(),
                    error_msg: "".to_string(),
                },
                "简体中文",
            ),
            (
                ArtifactScanError::ModelLoadFailed {
                    model_path: "".to_string(),
                    error_msg: "".to_string(),
                },
                "模型文件",
            ),
            (ArtifactScanError::WindowInfoFailed { error_msg: "".to_string() }, "分辨率"),
            (
                ArtifactScanError::ScanInterrupted { reason: "".to_string(), scanned_count: 0 },
                "重新开始",
            ),
            (ArtifactScanError::Unknown { error_msg: "".to_string() }, "未知错误"),
        ];

        for (error, expected_keyword) in test_cases {
            let suggestion = get_error_suggestion(&error);
            assert!(
                suggestion.contains(expected_keyword),
                "建议中应包含关键词 '{expected_keyword}', 实际建议: {suggestion}"
            );
        }
    }

    #[test]
    fn test_error_summary_report() {
        let mut stats = ErrorStatistics::new();

        // 添加一些测试数据
        for _ in 0..20 {
            stats.add_success();
        }

        for _ in 0..3 {
            stats.add_error(&ArtifactScanError::OcrRecognitionFailed {
                field: "test".to_string(),
                raw_text: "test".to_string(),
                error_msg: "test".to_string(),
            });
        }

        for _ in 0..2 {
            stats.add_error(&ArtifactScanError::ImageCaptureFailed {
                region: "test".to_string(),
                error_msg: "test".to_string(),
            });
        }

        let summary = stats.get_error_summary();

        // 验证摘要包含期望的信息
        assert!(summary.contains("成功扫描: 20 个"));
        assert!(summary.contains("总错误数: 5 个"));
        assert!(summary.contains("成功率: 80.0%")); // 20/(20+5) * 100
        assert!(summary.contains("OCR识别错误: 3 个"));
        assert!(summary.contains("图像捕获错误: 2 个"));
    }

    #[test]
    fn test_error_chaining() {
        // 测试错误链和调试输出
        let base_error = ArtifactScanError::ModelLoadFailed {
            model_path: "./models/ocr.onnx".to_string(),
            error_msg: "文件不存在".to_string(),
        };

        let error_msg = format!("{base_error:?}");
        assert!(error_msg.contains("ModelLoadFailed"));
        assert!(error_msg.contains("./models/ocr.onnx"));
        assert!(error_msg.contains("文件不存在"));
    }

    #[test]
    fn test_edge_cases() {
        let mut stats = ErrorStatistics::new();

        // 测试空统计的成功率
        assert_eq!(stats.get_success_rate(), 0.0);

        // 测试只有成功的情况
        stats.add_success();
        assert_eq!(stats.get_success_rate(), 100.0);

        // 测试只有错误的情况
        let mut error_only_stats = ErrorStatistics::new();
        error_only_stats.add_error(&ArtifactScanError::Unknown { error_msg: "test".to_string() });
        assert_eq!(error_only_stats.get_success_rate(), 0.0);
    }

    #[test]
    fn test_error_confidence_simulation() {
        // 模拟不同置信度的星级识别错误
        let high_confidence_error = ArtifactScanError::StarRecognitionFailed {
            detected_color: "RGB(255, 215, 0)".to_string(),
            confidence: 0.95,
        };

        let low_confidence_error = ArtifactScanError::StarRecognitionFailed {
            detected_color: "RGB(128, 128, 128)".to_string(),
            confidence: 0.25,
        };

        let high_display = format!("{high_confidence_error}");
        let low_display = format!("{low_confidence_error}");

        assert!(high_display.contains("0.95"));
        assert!(low_display.contains("0.25"));

        // 高置信度错误可能仍需要建议
        let suggestion = get_error_suggestion(&high_confidence_error);
        assert!(suggestion.contains("亮度设置"));
    }
}
