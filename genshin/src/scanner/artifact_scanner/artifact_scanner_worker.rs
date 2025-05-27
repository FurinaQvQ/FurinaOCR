use std::collections::HashSet;
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;
use std::time::Instant;

use anyhow::Result;
use furina_core::positioning::{Pos, Rect};
use image::{Rgb, RgbImage};
use log::{error, info, warn};

use crate::scanner::artifact_scanner::artifact_scanner_window_info::ArtifactScannerWindowInfo;
use crate::scanner::artifact_scanner::error::{
    get_error_suggestion, ArtifactScanError, ErrorStatistics,
};
use crate::scanner::artifact_scanner::message_items::SendItem;
use crate::scanner::artifact_scanner::performance_optimizations::{
    AdaptiveDelayManager, OptimizedImageProcessor, OptimizedOCRRecognizer, PerformanceMonitor,
};
use crate::scanner::artifact_scanner::scan_result::GenshinArtifactScanResult;
use crate::scanner::artifact_scanner::GenshinArtifactScannerConfig;

fn parse_level(s: &str) -> Result<i32> {
    let pos = s.find('+');

    if pos.is_none() {
        let level = s
            .parse::<i32>()
            .map_err(|e| anyhow::anyhow!("等级解析失败: 无法解析数字 '{}', 错误: {}", s, e))?;
        return anyhow::Ok(level);
    }

    let level_str = &s[pos.unwrap()..];
    let level = level_str.parse::<i32>().map_err(|e| {
        anyhow::anyhow!("等级解析失败: 无法解析带+号的等级 '{}', 错误: {}", level_str, e)
    })?;
    anyhow::Ok(level)
}

/// 优化版本的扫描工作器，使用优化的OCR识别和性能监控
pub struct ArtifactScannerWorker {
    ocr_recognizer: OptimizedOCRRecognizer,
    window_info: ArtifactScannerWindowInfo,
    config: GenshinArtifactScannerConfig,
    error_stats: ErrorStatistics,
    performance_monitor: PerformanceMonitor,
    adaptive_delay: AdaptiveDelayManager,
}

impl ArtifactScannerWorker {
    pub fn new(
        window_info: ArtifactScannerWindowInfo,
        config: GenshinArtifactScannerConfig,
    ) -> Result<Self> {
        Ok(ArtifactScannerWorker {
            ocr_recognizer: OptimizedOCRRecognizer::new()?,
            window_info,
            config,
            error_stats: ErrorStatistics::new(),
            performance_monitor: PerformanceMonitor::new(),
            adaptive_delay: AdaptiveDelayManager::new(10), // 基础延时10ms
        })
    }

    /// 优化版本的OCR推理，使用性能监控
    fn model_inference_optimized(
        &mut self,
        rect: Rect<f64>,
        captured_img: &RgbImage,
        field_name: &str,
    ) -> Result<String> {
        let start_time = Instant::now();

        let relative_rect = rect.translate(Pos {
            x: -self.window_info.panel_rect.left,
            y: -self.window_info.panel_rect.top,
        });

        // 使用优化的图像裁剪
        let cropped_img = OptimizedImageProcessor::crop_optimized(captured_img, &relative_rect);

        let inference_result = self
            .ocr_recognizer
            .recognize(&cropped_img)
            .map_err(|e| anyhow::anyhow!("OCR识别失败 - 字段: {}, 错误: {}", field_name, e))?;

        let ocr_time = start_time.elapsed();
        self.performance_monitor.record_ocr_time(ocr_time);

        Ok(inference_result)
    }

    /// 批量OCR识别，提高效率
    fn batch_model_inference(
        &mut self,
        rects_and_names: Vec<(Rect<f64>, &str)>,
        captured_img: &RgbImage,
    ) -> Vec<Result<String>> {
        let start_time = Instant::now();

        let cropped_images: Vec<RgbImage> = rects_and_names
            .iter()
            .map(|(rect, _)| {
                let relative_rect = rect.translate(Pos {
                    x: -self.window_info.panel_rect.left,
                    y: -self.window_info.panel_rect.top,
                });
                OptimizedImageProcessor::crop_optimized(captured_img, &relative_rect)
            })
            .collect();

        let results = self.ocr_recognizer.batch_recognize(&cropped_images);

        let batch_ocr_time = start_time.elapsed();
        self.performance_monitor.record_ocr_time(batch_ocr_time);

        results
    }

    /// 优化版本的物品扫描，使用批量处理
    fn scan_item_image_optimized(
        &mut self,
        item: SendItem,
        lock: bool,
    ) -> Result<GenshinArtifactScanResult> {
        let image = &item.panel_image;
        let mut result_errors = Vec::new();

        // 准备批量OCR识别的区域
        let ocr_regions = vec![
            (self.window_info.title_rect, "圣遗物名称"),
            (self.window_info.main_stat_name_rect, "主属性名称"),
            (self.window_info.main_stat_value_rect, "主属性数值"),
            (self.window_info.level_rect, "等级"),
            (self.window_info.item_equip_rect, "装备状态"),
        ];

        // 批量进行主要字段的OCR识别
        let ocr_results = self.batch_model_inference(ocr_regions, image);

        // 处理主要字段结果
        let str_title = match &ocr_results[0] {
            Ok(text) => text.clone(),
            Err(e) => {
                let error = ArtifactScanError::OcrRecognitionFailed {
                    field: "圣遗物名称".to_string(),
                    raw_text: "".to_string(),
                    error_msg: e.to_string(),
                };
                result_errors.push(error);
                "未识别".to_string()
            },
        };

        let str_main_stat_name = match &ocr_results[1] {
            Ok(text) => text.clone(),
            Err(e) => {
                let error = ArtifactScanError::OcrRecognitionFailed {
                    field: "主属性名称".to_string(),
                    raw_text: "".to_string(),
                    error_msg: e.to_string(),
                };
                result_errors.push(error);
                "未识别".to_string()
            },
        };

        let str_main_stat_value = match &ocr_results[2] {
            Ok(text) => text.clone(),
            Err(e) => {
                let error = ArtifactScanError::OcrRecognitionFailed {
                    field: "主属性数值".to_string(),
                    raw_text: "".to_string(),
                    error_msg: e.to_string(),
                };
                result_errors.push(error);
                "0".to_string()
            },
        };

        let str_level = match &ocr_results[3] {
            Ok(text) => text.clone(),
            Err(e) => {
                let error = ArtifactScanError::OcrRecognitionFailed {
                    field: "等级".to_string(),
                    raw_text: "".to_string(),
                    error_msg: e.to_string(),
                };
                result_errors.push(error);
                "0".to_string()
            },
        };

        let str_equip = match &ocr_results[4] {
            Ok(text) => text.clone(),
            Err(e) => {
                warn!("装备状态识别失败，使用默认值: {e}");
                String::new()
            },
        };

        // 副属性仍使用单独识别（通常文本较短，批量处理收益不大）
        let str_sub_stat0 = self
            .model_inference_optimized(self.window_info.sub_stat_1, image, "副属性1")
            .unwrap_or_default();
        let str_sub_stat1 = self
            .model_inference_optimized(self.window_info.sub_stat_2, image, "副属性2")
            .unwrap_or_default();
        let str_sub_stat2 = self
            .model_inference_optimized(self.window_info.sub_stat_3, image, "副属性3")
            .unwrap_or_default();
        let str_sub_stat3 = self
            .model_inference_optimized(self.window_info.sub_stat_4, image, "副属性4")
            .unwrap_or_default();

        // 解析等级
        let level = match parse_level(&str_level) {
            Ok(l) => l,
            Err(e) => {
                let error = ArtifactScanError::LevelParsingFailed {
                    raw_text: str_level.clone(),
                    error_msg: e.to_string(),
                };
                result_errors.push(error);
                0
            },
        };

        // 创建扫描结果
        let mut result = GenshinArtifactScanResult::new(
            str_title,
            str_main_stat_name,
            str_main_stat_value,
            [str_sub_stat0, str_sub_stat1, str_sub_stat2, str_sub_stat3],
            str_equip,
            level,
            item.star as i32,
            lock,
        );

        // 添加所有错误到结果中
        for error in &result_errors {
            result.add_error(error);
        }

        // 更新自适应延时统计
        if result_errors.is_empty() {
            self.adaptive_delay.record_success();
        } else {
            self.adaptive_delay.record_failure();
        }

        anyhow::Ok(result)
    }

    /// 优化版本的锁定状态检测，使用批量颜色距离计算
    fn get_page_locks_optimized(&self, list_image: &RgbImage) -> Vec<bool> {
        let mut result = Vec::new();
        let mut colors_to_check = Vec::new();

        let row = self.window_info.row;
        let col = self.window_info.col;
        let gap = self.window_info.item_gap_size;
        let size = self.window_info.item_size;
        let lock_pos = self.window_info.lock_pos;

        // 收集所有需要检查的颜色位置
        for r in 0..row {
            if ((gap.height + size.height) * (r as f64)) as u32 > list_image.height() {
                break;
            }
            for c in 0..col {
                let pos_x = (gap.width + size.width) * (c as f64) + lock_pos.x;
                let pos_y = (gap.height + size.height) * (r as f64) + lock_pos.y;

                if (pos_x as u32) < list_image.width() && (pos_y as u32) < list_image.height() {
                    let color = *list_image.get_pixel(pos_x as u32, pos_y as u32);
                    colors_to_check.push(color);
                } else {
                    result.push(false);
                }
            }
        }

        // 批量计算颜色距离
        let target_color = Rgb([255, 138, 117]);
        let distances =
            OptimizedImageProcessor::batch_color_distance(&colors_to_check, &target_color);

        // 根据距离判断锁定状态
        for distance in distances {
            result.push(distance < 900); // 30*30 = 900
        }

        result
    }

    pub fn run(
        mut self,
        rx: Receiver<Option<SendItem>>,
    ) -> JoinHandle<Vec<GenshinArtifactScanResult>> {
        std::thread::spawn(move || {
            let mut results = Vec::new();
            let mut hash: HashSet<GenshinArtifactScanResult> = HashSet::new();
            let mut consecutive_dup_count = 0;

            let is_verbose = self.config.verbose;
            let min_level = self.config.min_level;
            let info = self.window_info.clone();

            let mut locks = Vec::new();
            let mut artifact_index: i32 = 0;

            for item in rx.into_iter() {
                let item = match item {
                    Some(v) => v,
                    None => break,
                };

                // 使用优化版本的锁定状态检测
                if let Some(v) = item.list_image.as_ref() { locks = [locks, self.get_page_locks_optimized(v)].concat() };

                artifact_index += 1;
                let result = match self.scan_item_image_optimized(
                    item,
                    locks.get(artifact_index as usize - 1).copied().unwrap_or(false),
                ) {
                    Ok(v) => {
                        self.error_stats.record_success();
                        v
                    },
                    Err(e) => {
                        let scan_error = ArtifactScanError::Unknown { error_msg: e.to_string() };
                        self.error_stats.record_error(&scan_error);
                        error!("识别错误: {e}");
                        error!("建议: {}", get_error_suggestion(&scan_error));
                        continue;
                    },
                };

                // 记录结果中的错误
                for error_msg in &result.scan_errors {
                    warn!("扫描警告: {error_msg}");
                }

                if is_verbose {
                    info!("{result:?}");
                    if result.has_errors() {
                        warn!(
                            "该圣遗物识别存在 {} 个错误，置信度: {:.2}",
                            result.error_count(),
                            result.confidence_score
                        );
                    }
                }

                if result.level < min_level {
                    info!(
                        "找到满足最低等级要求 {} 的物品({})，准备退出……",
                        min_level, result.level
                    );
                    break;
                }

                if hash.contains(&result) {
                    consecutive_dup_count += 1;
                    let dup_error = ArtifactScanError::ConsecutiveDuplicateItems {
                        count: consecutive_dup_count,
                        threshold: info.col as usize,
                    };
                    self.error_stats.record_error(&dup_error);
                    warn!("识别到重复物品: {result:#?}");
                    warn!("{}", get_error_suggestion(&dup_error));
                } else {
                    consecutive_dup_count = 0;
                    hash.insert(result.clone());
                    results.push(result);
                }

                if consecutive_dup_count >= info.col as usize && !self.config.ignore_dup {
                    error!("识别到连续多个重复物品，可能为翻页错误，或者为非背包顶部开始扫描");
                    error!("建议: 请确保从背包顶部开始扫描，避免在扫描过程中手动翻页");
                    break;
                }

                // 应用自适应延时
                let current_delay = self.adaptive_delay.get_current_delay();
                if current_delay > 0 {
                    std::thread::sleep(std::time::Duration::from_millis(current_delay as u64));
                }
            }

            // 输出性能统计
            info!("{}", self.performance_monitor.get_performance_summary());
            info!("自适应延时最终值: {}ms", self.adaptive_delay.get_current_delay());

            info!("识别结束，非重复物品数量: {}", hash.len());

            // 输出错误统计报告
            if self.error_stats.total_errors > 0 || results.iter().any(|r| r.has_errors()) {
                let items_with_errors = results.iter().filter(|r| r.has_errors()).count();
                warn!("扫描过程中发现问题，详细统计如下:");
                for line in self.error_stats.get_error_summary().lines() {
                    warn!("{line}");
                }
                if items_with_errors > 0 {
                    warn!("- 存在错误的物品: {items_with_errors} 个");
                }

                if self.error_stats.get_success_rate() < 80.0 {
                    error!(
                        "识别成功率较低 ({:.1}%)，建议检查游戏设置和环境",
                        self.error_stats.get_success_rate()
                    );
                    error!("常见解决方案:");
                    error!("1. 确保游戏语言设置为简体中文");
                    error!("2. 检查游戏分辨率是否为16:9比例");
                    error!("3. 确保游戏界面清晰，无遮挡");
                    error!("4. 检查游戏亮度设置");
                }
            } else {
                info!("扫描完成，未发现错误！");
            }

            results
        })
    }
}
