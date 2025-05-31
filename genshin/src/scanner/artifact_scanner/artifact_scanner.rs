use std::cell::RefCell;
use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::mpsc::{self, Sender};
use std::time::SystemTime;

use anyhow::Result;
use clap::FromArgMatches;
use furina_core::capture::{Capturer, GenericCapturer};
use furina_core::game_info::GameInfo;
use furina_core::ocr::{ImageToText, OcrModel};
use furina_core::ocr_model;
use furina_core::positioning::Pos;
use furina_core::window_info::{FromWindowInfoRepository, WindowInfoRepository};
use image::RgbImage;
use log::{error, info, warn};

use crate::scanner::artifact_scanner::artifact_scanner_config::GenshinArtifactScannerConfig;
use crate::scanner::artifact_scanner::artifact_scanner_worker::ArtifactScannerWorker;
use crate::scanner::artifact_scanner::error::{get_error_suggestion, ArtifactScanError};
use crate::scanner::artifact_scanner::message_items::SendItem;
use crate::scanner::artifact_scanner::scan_result::GenshinArtifactScanResult;
use crate::scanner::artifact_scanner::ArtifactScannerWindowInfo;
use crate::scanner_controller::repository_layout::{
    GenshinRepositoryScanController, GenshinRepositoryScannerLogicConfig,
    ReturnResult as GenshinRepositoryControllerReturnResult,
};

fn color_distance(c1: &image::Rgb<u8>, c2: &image::Rgb<u8>) -> usize {
    let x = c1.0[0] as i32 - c2.0[0] as i32;
    let y = c1.0[1] as i32 - c2.0[1] as i32;
    let z = c1.0[2] as i32 - c2.0[2] as i32;
    (x * x + y * y + z * z) as usize
}

pub struct GenshinArtifactScanner {
    scanner_config: GenshinArtifactScannerConfig,
    window_info: ArtifactScannerWindowInfo,
    game_info: GameInfo,
    image_to_text: Box<dyn ImageToText<RgbImage> + Send>,
    controller: Rc<RefCell<GenshinRepositoryScanController>>,
    capturer: Rc<dyn Capturer<RgbImage>>,
}

impl GenshinArtifactScanner {
    pub const MAX_COUNT: usize = 2100;
}

// constructor
impl GenshinArtifactScanner {
    fn get_image_to_text() -> Result<Box<dyn ImageToText<RgbImage> + Send>> {
        let model: Box<dyn ImageToText<RgbImage> + Send> = Box::new(
            ocr_model!("./models/model_training.onnx", "./models/index_2_word.json").map_err(
                |e| {
                    let error = ArtifactScanError::ModelLoadFailed {
                        model_path: "./models/model_training.onnx".to_string(),
                        error_msg: e.to_string(),
                    };
                    error!("模型加载失败: {error}");
                    error!("建议: {}", get_error_suggestion(&error));
                    anyhow::anyhow!(error)
                },
            )?,
        );
        Ok(model)
    }

    fn get_capturer() -> Result<Rc<dyn Capturer<RgbImage>>> {
        Ok(Rc::new(GenericCapturer::new().map_err(|e| {
            let error = ArtifactScanError::ImageCaptureFailed {
                region: "屏幕捕获初始化".to_string(),
                error_msg: e.to_string(),
            };
            error!("图像捕获器初始化失败: {error}");
            error!("建议: {}", get_error_suggestion(&error));
            anyhow::anyhow!(error)
        })?))
    }

    pub fn new(
        window_info_repo: &WindowInfoRepository,
        config: GenshinArtifactScannerConfig,
        controller_config: GenshinRepositoryScannerLogicConfig,
        game_info: GameInfo,
    ) -> Result<Self> {
        let window_info = ArtifactScannerWindowInfo::from_window_info_repository(
            game_info.window.to_rect_usize().size(),
            game_info.ui,
            game_info.platform,
            window_info_repo,
        )
        .map_err(|e| {
            let error = ArtifactScanError::WindowInfoFailed { error_msg: e.to_string() };
            error!("窗口信息获取失败: {error}");
            error!("建议: {}", get_error_suggestion(&error));
            anyhow::anyhow!(error)
        })?;

        Ok(Self {
            scanner_config: config,
            window_info,
            controller: Rc::new(RefCell::new(GenshinRepositoryScanController::new(
                window_info_repo,
                controller_config,
                game_info.clone(),
                true,
            )?)),
            game_info,
            image_to_text: Self::get_image_to_text()?,
            capturer: Self::get_capturer()?,
        })
    }

    pub fn from_arg_matches(
        window_info_repo: &WindowInfoRepository,
        arg_matches: &clap::ArgMatches,
        game_info: GameInfo,
    ) -> Result<Self> {
        let window_info = ArtifactScannerWindowInfo::from_window_info_repository(
            game_info.window.to_rect_usize().size(),
            game_info.ui,
            game_info.platform,
            window_info_repo,
        )
        .map_err(|e| {
            let error = ArtifactScanError::WindowInfoFailed { error_msg: e.to_string() };
            error!("窗口信息获取失败: {error}");
            error!("建议: {}", get_error_suggestion(&error));
            anyhow::anyhow!(error)
        })?;

        Ok(GenshinArtifactScanner {
            scanner_config: GenshinArtifactScannerConfig::from_arg_matches(arg_matches)?,
            window_info,
            controller: Rc::new(RefCell::new(GenshinRepositoryScanController::from_arg_matches(
                window_info_repo,
                arg_matches,
                game_info.clone(),
                true,
            )?)),
            game_info,
            image_to_text: Self::get_image_to_text()?,
            capturer: Self::get_capturer()?,
        })
    }
}

impl GenshinArtifactScanner {
    pub fn capture_panel(&self) -> Result<RgbImage> {
        self.capturer
            .capture_relative_to(
                self.window_info.panel_rect.to_rect_i32(),
                self.game_info.window.origin(),
            )
            .map_err(|e| {
                let error = ArtifactScanError::ImageCaptureFailed {
                    region: "圣遗物面板".to_string(),
                    error_msg: e.to_string(),
                };
                warn!("图像捕获失败: {error}");
                warn!("建议: {}", get_error_suggestion(&error));
                anyhow::anyhow!(error)
            })
    }

    pub fn get_star(&self) -> Result<usize> {
        let pos: Pos<i32> = Pos {
            x: self.game_info.window.left + self.window_info.star_pos.x as i32,
            y: self.game_info.window.top + self.window_info.star_pos.y as i32,
        };
        let color = self.capturer.capture_color(pos).map_err(|e| {
            let error = ArtifactScanError::ImageCaptureFailed {
                region: "星级颜色采样".to_string(),
                error_msg: e.to_string(),
            };
            warn!("星级颜色采样失败: {error}");
            warn!("建议: {}", get_error_suggestion(&error));
            anyhow::anyhow!(error)
        })?;

        let match_colors = [
            image::Rgb([113, 119, 139]), // 1星
            image::Rgb([42, 143, 114]),  // 2星
            image::Rgb([81, 127, 203]),  // 3星
            image::Rgb([161, 86, 224]),  // 4星
            image::Rgb([188, 105, 50]),  // 5星
        ];

        let mut min_dis: usize = 0xdeadbeef;
        let mut ret: usize = 1;
        for (i, match_color) in match_colors.iter().enumerate() {
            let dis2 = color_distance(match_color, &color);
            if dis2 < min_dis {
                min_dis = dis2;
                ret = i + 1;
            }
        }

        // 检查识别置信度
        if min_dis > 10000 {
            // 颜色差距过大，可能识别错误
            let error = ArtifactScanError::StarRecognitionFailed {
                detected_color: format!("RGB({}, {}, {})", color.0[0], color.0[1], color.0[2]),
                confidence: 1.0 - (min_dis as f64 / 50000.0).min(1.0),
            };
            warn!("星级识别置信度较低: {error}");
            warn!("建议: {}", get_error_suggestion(&error));
        }

        anyhow::Ok(ret)
    }

    pub fn get_item_count(&self) -> Result<i32> {
        let count = self.scanner_config.number;
        let item_name = "圣遗物";

        let max_count = Self::MAX_COUNT as i32;
        if count > 0 {
            return Ok(max_count.min(count));
        }

        let im = self
            .capturer
            .capture_relative_to(
                self.window_info.item_count_rect.to_rect_i32(),
                self.game_info.window.origin(),
            )
            .map_err(|e| {
                let error = ArtifactScanError::ImageCaptureFailed {
                    region: "物品数量区域".to_string(),
                    error_msg: e.to_string(),
                };
                warn!("物品数量区域捕获失败: {error}");
                warn!("建议: {}", get_error_suggestion(&error));
                anyhow::anyhow!(error)
            })?;

        let s = self.image_to_text.image_to_text(&im, false).map_err(|e| {
            let error = ArtifactScanError::OcrRecognitionFailed {
                field: "物品数量".to_string(),
                raw_text: "".to_string(),
                error_msg: e.to_string(),
            };
            warn!("物品数量识别失败: {error}");
            warn!("建议: {}", get_error_suggestion(&error));
            anyhow::anyhow!(error)
        })?;

        info!("物品信息: {s}");

        if s.starts_with(item_name) {
            let chars = s.chars().collect::<Vec<char>>();
            if chars.len() > 9 {
                // 确保有足够的字符
                let count_str = chars[4..chars.len() - 5].iter().collect::<String>();
                Ok(match count_str.parse::<usize>() {
                    Ok(v) => (v as i32).min(max_count),
                    Err(e) => {
                        warn!("物品数量解析失败: '{count_str}', 错误: {e}, 使用默认最大值");
                        max_count
                    },
                })
            } else {
                warn!("物品信息格式异常: '{s}', 使用默认最大值");
                Ok(max_count)
            }
        } else {
            warn!("未识别到圣遗物信息: '{s}', 使用默认最大值");
            Ok(max_count)
        }
    }

    pub fn scan(&mut self) -> Result<Vec<GenshinArtifactScanResult>> {
        info!("开始扫描，使用鼠标右键中断扫描");

        let now = SystemTime::now();
        let (tx, rx) = mpsc::channel::<Option<SendItem>>();

        let count = self.get_item_count().unwrap_or_else(|e| {
            error!("获取物品数量失败: {e}, 使用默认值");
            Self::MAX_COUNT as i32
        });

        let window_size = (self.game_info.window.width as u32, self.game_info.window.height as u32);
        let worker = ArtifactScannerWorker::new(
            self.window_info.clone(),
            self.scanner_config.clone(),
            window_size,
        )?;

        let join_handle = worker.run(rx);

        self.send(&tx, count);

        match tx.send(None) {
            Ok(_) => info!("扫描结束，等待识别线程结束，请勿关闭程序"),
            Err(_) => info!("扫描结束，识别已完成"),
        }

        match join_handle.join() {
            Ok(v) => {
                info!("识别耗时: {:?}", now.elapsed()?);

                // filter min level
                let min_level = self.scanner_config.min_level;
                let filtered_results: Vec<GenshinArtifactScanResult> =
                    v.iter().filter(|a| a.level >= min_level).cloned().collect();

                // 统计有错误的物品
                let error_count = filtered_results.iter().filter(|r| r.has_errors()).count();
                let low_confidence_count =
                    filtered_results.iter().filter(|r| !r.is_reliable(0.8)).count();

                if error_count > 0 {
                    warn!("扫描完成，但有 {error_count} 个圣遗物存在识别错误");
                }
                if low_confidence_count > 0 {
                    warn!("扫描完成，但有 {low_confidence_count} 个圣遗物置信度较低（<80%）");
                }

                info!("最终结果: 成功识别 {} 个圣遗物", filtered_results.len());

                Ok(filtered_results)
            },
            Err(_) => {
                let error = ArtifactScanError::ScanInterrupted {
                    reason: "识别线程异常退出".to_string(),
                    scanned_count: 0,
                };
                error!("扫描失败: {error}");
                error!("建议: {}", get_error_suggestion(&error));
                Err(anyhow::anyhow!(error))
            },
        }
    }

    fn is_page_first_artifact(&self, cur_index: i32) -> bool {
        let col = self.window_info.col;
        let row = self.window_info.row;

        let page_size = col * row;
        cur_index % page_size == 0
    }

    /// Get the starting row in the page where `cur_index` is in
    /// max count: total count
    /// cur_index: current item index (starting from 0)
    fn get_start_row(&self, max_count: i32, cur_index: i32) -> i32 {
        let col = self.window_info.col;
        let row = self.window_info.row;

        let page_size = col * row;
        if max_count - cur_index >= page_size {
            0
        } else {
            let remain = max_count - cur_index;
            let remain_row = (remain + col - 1) / col;
            let scroll_row = remain_row.min(row);
            row - scroll_row
        }
    }

    fn send(&mut self, tx: &Sender<Option<SendItem>>, count: i32) {
        let mut generator =
            GenshinRepositoryScanController::get_generator(self.controller.clone(), count as usize);
        let mut artifact_index: i32 = 0;

        loop {
            let pinned_generator = Pin::new(&mut generator);
            match pinned_generator.resume(()) {
                CoroutineState::Yielded(_) => {
                    let image = self.capture_panel().unwrap();
                    let star = self.get_star().unwrap();

                    let list_image = if self.is_page_first_artifact(artifact_index) {
                        let origin = self.game_info.window;
                        let margin = self.window_info.scan_margin_pos;
                        let gap = self.window_info.item_gap_size;
                        let size = self.window_info.item_size;

                        let left = (origin.left as f64 + margin.x) as i32;
                        let top = (origin.top as f64
                            + margin.y
                            + (gap.height + size.height)
                                * self.get_start_row(count, artifact_index) as f64)
                            as i32;
                        let width = (origin.width as f64 - margin.x) as i32;
                        let height = (origin.height as f64
                            - margin.y
                            - (gap.height + size.height)
                                * self.get_start_row(count, artifact_index) as f64)
                            as i32;

                        let game_image = self
                            .capturer
                            .capture_rect(furina_core::positioning::Rect {
                                left,
                                top,
                                width,
                                height,
                            })
                            .unwrap();
                        Some(game_image)
                    } else {
                        None
                    };

                    artifact_index += 1;

                    if (star as i32) < self.scanner_config.min_star {
                        info!(
                            "找到满足最低星级要求 {} 的物品，准备退出……",
                            self.scanner_config.min_star
                        );
                        break;
                    }

                    if tx.send(Some(SendItem { panel_image: image, star, list_image })).is_err() {
                        break;
                    }
                },
                CoroutineState::Complete(result) => {
                    match result {
                        Err(e) => error!("扫描发生错误：{e}"),
                        Ok(value) => match value {
                            GenshinRepositoryControllerReturnResult::Interrupted => {
                                info!("用户中断")
                            },
                            GenshinRepositoryControllerReturnResult::Finished => (),
                        },
                    }

                    break;
                },
            }
        }
    }
}
