use std::cell::RefCell;
use std::ops::Coroutine;
use std::rc::Rc;
use std::time::SystemTime;

use anyhow::{anyhow, Result};
use clap::{ArgMatches, FromArgMatches};
use furina_core::capture::{Capturer, GenericCapturer};
use furina_core::game_info::GameInfo;
use furina_core::positioning::Pos;
use furina_core::system_control::SystemControl;
use furina_core::utils;
use furina_core::window_info::{FromWindowInfoRepository, WindowInfoRepository};
use image::RgbImage;
use log::{error, info};

use crate::scanner_controller::repository_layout::{
    GenshinRepositoryScanControllerWindowInfo, GenshinRepositoryScannerLogicConfig, ScrollResult,
};

/// 扫描状态管理结构体
///
/// 用于跟踪扫描过程中的各种状态信息
#[derive(Debug, Clone)]
struct ScanState {
    /// 已扫描的行数
    scanned_row: usize,
    /// 已扫描的物品数量  
    scanned_count: usize,
    /// 当前页面起始行
    start_row: usize,
    /// 物品总数
    item_count: usize,
    /// 总行数
    total_row: usize,
    /// 最后一行的列数
    last_row_col: usize,
}

impl ScanState {
    /// 创建新的扫描状态
    fn new(item_count: usize, col: usize) -> Self {
        let total_row = (item_count + col - 1) / col;
        let last_row_col = if item_count % col == 0 { col } else { item_count % col };

        Self { scanned_row: 0, scanned_count: 0, start_row: 0, item_count, total_row, last_row_col }
    }

    /// 检查是否完成扫描
    fn is_scan_complete(&self) -> bool {
        self.scanned_count >= self.item_count
    }

    /// 检查是否到达最大行数
    fn is_max_row_reached(&self) -> bool {
        self.scanned_row >= self.total_row
    }

    /// 计算剩余扫描参数
    fn calculate_remaining_scan_params(&self, controller_row: usize) -> (usize, usize) {
        let remain = self.item_count - self.scanned_count;
        let remain_row = (remain + controller_row - 1) / controller_row;
        let scroll_row = remain_row.min(controller_row);
        let start_row = controller_row - scroll_row;
        (scroll_row, start_row)
    }
}

pub struct GenshinRepositoryScanController {
    // to detect whether an item changes
    pool: f64,

    initial_color: image::Rgb<u8>,

    // for scrolls
    scrolled_rows: u32,
    avg_scroll_one_row: f64,

    avg_switch_time: f64,
    scanned_count: usize,

    game_info: GameInfo,

    // row and column in one page
    row: usize,
    col: usize,

    config: GenshinRepositoryScannerLogicConfig,
    window_info: GenshinRepositoryScanControllerWindowInfo,
    system_control: SystemControl,
    capturer: Rc<dyn Capturer<RgbImage>>,

    // artifact panel have different layout
    is_artifact: bool,
}

/// 计算图像行的像素池值
///
/// 该函数计算图像行中所有红色通道值的总和，用于检测界面变化。
/// 当界面发生变化时，红色通道值的总和会发生变化，从而可以检测到界面切换。
///
/// # 参数
/// * `row` - 图像行的原始字节数据，格式为RGB
///
/// # 返回值
/// 返回红色通道值的总和
fn calc_pool(row: &[u8]) -> f32 {
    let len = row.len() / 3; // RGB格式，每3个字节表示一个像素
    let mut pool: f32 = 0.0;

    for i in 0..len {
        pool += row[i * 3] as f32; // 只累加红色通道值
    }
    pool
}

/// 获取屏幕捕获器实例
///
/// 创建一个通用的屏幕捕获器，用于截图和颜色采样
fn get_capturer() -> Result<Rc<dyn Capturer<RgbImage>>> {
    Ok(Rc::new(GenericCapturer::new()?))
}

/// 计算两个颜色之间的欧几里得距离
///
/// 使用RGB空间中的欧几里得距离公式计算颜色差异：
/// distance = sqrt((r1-r2)² + (g1-g2)² + (b1-b2)²)
///
/// 为了提高性能，返回距离的平方值（避免开方运算）
///
/// # 参数
/// * `c1` - 第一个颜色
/// * `c2` - 第二个颜色
///
/// # 返回值
/// 返回距离的平方，值越小表示颜色越相似
fn color_distance(c1: &image::Rgb<u8>, c2: &image::Rgb<u8>) -> usize {
    let x = c1.0[0] as i32 - c2.0[0] as i32;
    let y = c1.0[1] as i32 - c2.0[1] as i32;
    let z = c1.0[2] as i32 - c2.0[2] as i32;
    (x * x + y * y + z * z) as usize
}

// constructor
impl GenshinRepositoryScanController {
    pub fn new(
        window_info_repo: &WindowInfoRepository,
        config: GenshinRepositoryScannerLogicConfig,
        game_info: GameInfo,
        is_artifact: bool,
    ) -> Result<Self> {
        let window_info = GenshinRepositoryScanControllerWindowInfo::from_window_info_repository(
            game_info.window.to_rect_usize().size(),
            game_info.ui,
            game_info.platform,
            window_info_repo,
        )?;
        let row = window_info.genshin_repository_item_row;
        let col = window_info.genshin_repository_item_col;

        Ok(GenshinRepositoryScanController {
            system_control: SystemControl::new(),

            row: row as usize,
            col: col as usize,

            window_info,
            config,

            pool: 0.0,

            initial_color: image::Rgb([0, 0, 0]),

            scrolled_rows: 0,
            avg_scroll_one_row: 0.0,

            avg_switch_time: 0.0,
            // scanned_count: 0,
            game_info,
            scanned_count: 0,

            capturer: get_capturer()?,

            is_artifact,
        })
    }

    pub fn from_arg_matches(
        window_info_repo: &WindowInfoRepository,
        arg_matches: &ArgMatches,
        game_info: GameInfo,
        is_artifact: bool,
    ) -> Result<Self> {
        Self::new(
            window_info_repo,
            GenshinRepositoryScannerLogicConfig::from_arg_matches(arg_matches)?,
            game_info,
            is_artifact,
        )
    }

    /// 初始化扫描环境
    ///
    /// 设置初始位置、点击界面并采样初始颜色
    fn initialize_scan_environment(
        object: &Rc<RefCell<GenshinRepositoryScanController>>,
    ) -> Result<()> {
        // 移动到起始位置
        object.borrow_mut().move_to(0, 0);

        #[cfg(target_os = "macos")]
        utils::sleep(20);

        // 点击界面激活
        object.borrow_mut().system_control.mouse_click()?;
        utils::sleep(1000);

        // 采样初始颜色用于检测界面变化
        object.borrow_mut().sample_initial_color()?;

        Ok(())
    }

    /// 处理页面滚动
    ///
    /// 计算滚动参数并执行滚动操作
    fn handle_page_scroll(
        object: &Rc<RefCell<GenshinRepositoryScanController>>,
        state: &mut ScanState,
    ) -> Result<()> {
        let controller_row = object.borrow().row;
        let (scroll_row, new_start_row) = state.calculate_remaining_scan_params(controller_row);
        state.start_row = new_start_row;

        match object.borrow_mut().scroll_rows(scroll_row as i32) {
            ScrollResult::TimeLimitExceeded => {
                return Err(anyhow!("翻页超时，扫描终止……"));
            },
            ScrollResult::Interrupt => {
                return Err(anyhow!("用户中断扫描"));
            },
            _ => (),
        }

        utils::sleep(100);
        Ok(())
    }
}

pub enum ReturnResult {
    Interrupted,
    Finished,
}

impl GenshinRepositoryScanController {
    pub fn get_generator(
        object: Rc<RefCell<GenshinRepositoryScanController>>,
        item_count: usize,
    ) -> impl Coroutine<Yield = (), Return = Result<ReturnResult>> {
        let generator = #[coroutine]
        move || {
            // 初始化扫描状态
            let col = object.borrow().col;
            let mut state = ScanState::new(item_count, col);

            info!(
                "扫描任务: {} 个物品，共 {} 行，尾行 {} 个",
                state.item_count, state.total_row, state.last_row_col
            );

            // 初始化扫描环境
            Self::initialize_scan_environment(&object)?;

            // 主扫描循环
            'outer: while !state.is_scan_complete() {
                let controller_row = object.borrow().row.min(state.total_row);

                '_row: for row in state.start_row..controller_row {
                    // 确定当前行的物品数量
                    let row_item_count = if state.scanned_row == state.total_row - 1 {
                        state.last_row_col
                    } else {
                        object.borrow().col
                    };

                    '_col: for col in 0..row_item_count {
                        // 检查扫描完成条件
                        if state.scanned_count >= state.item_count {
                            break 'outer;
                        }

                        // 检查用户中断
                        if utils::is_rmb_down() {
                            return Ok(ReturnResult::Interrupted);
                        }

                        // 准备扫描：移动和点击
                        object.borrow_mut().move_to(row, col);
                        object.borrow_mut().system_control.mouse_click()?;

                        #[cfg(target_os = "macos")]
                        utils::sleep(20);

                        // 等待界面切换
                        object.borrow_mut().wait_until_switched()?;

                        // yield 让出控制权，允许外部处理
                        yield;

                        // 更新扫描计数
                        state.scanned_count += 1;
                        object.borrow_mut().scanned_count = state.scanned_count;
                    }

                    state.scanned_row += 1;

                    // 检查是否到达最大行数
                    if state.is_max_row_reached() {
                        info!("到达最大行数，准备退出");
                        break 'outer;
                    }
                }

                // 处理页面滚动（如果还有物品需要扫描）
                if !state.is_scan_complete() {
                    if let Err(e) = Self::handle_page_scroll(&object, &mut state) {
                        return match e.downcast_ref::<String>() {
                            Some(msg) if msg == "用户中断扫描" => {
                                Ok(ReturnResult::Interrupted)
                            },
                            _ => Err(e),
                        };
                    }
                }
            }

            Ok(ReturnResult::Finished)
        };

        generator
    }

    #[inline(always)]
    pub fn get_flag_color(&self) -> Result<image::Rgb<u8>> {
        let mut pos_f64 = Pos {
            x: self.window_info.flag_pos.x + self.game_info.window.left as f64,
            y: self.window_info.flag_pos.y + self.game_info.window.top as f64,
        };
        if self.is_artifact {
            pos_f64.x += self.window_info.artifact_panel_offset.width;
            pos_f64.y += self.window_info.artifact_panel_offset.height;
        }
        let pos_i32 = Pos { x: pos_f64.x as i32, y: pos_f64.y as i32 };
        self.capturer.capture_color(pos_i32)
    }

    #[inline(always)]
    pub fn sample_initial_color(&mut self) -> Result<()> {
        self.initial_color = self.get_flag_color()?;
        anyhow::Ok(())
    }

    pub fn align_row(&mut self) {
        for _ in 0..10 {
            let color = match self.get_flag_color() {
                Ok(color) => color,
                Err(_) => return,
            };

            if color_distance(&self.initial_color, &color) > 10 {
                self.mouse_scroll(1, false);
                utils::sleep(self.config.scroll_delay.try_into().unwrap());
            } else {
                break;
            }
        }
    }

    pub fn move_to(&mut self, row: usize, col: usize) {
        let (row, col) = (row as u32, col as u32);
        let origin = self.game_info.window.to_rect_f64().origin();

        let gap = self.window_info.item_gap_size;
        let mut margin = self.window_info.scan_margin_pos;
        let size = self.window_info.item_size;
        if self.is_artifact {
            margin = margin + self.window_info.artifact_panel_offset;
        }

        let left = origin.x + margin.x + (gap.width + size.width) * (col as f64) + size.width / 2.0;
        let top =
            origin.y + margin.y + (gap.height + size.height) * (row as f64) + size.height / 4.0;

        self.system_control.mouse_move_to(left as i32, top as i32).unwrap();

        #[cfg(target_os = "macos")]
        utils::sleep(20);
    }

    pub fn scroll_one_row(&mut self) -> ScrollResult {
        let mut state = 0;
        let mut count = 0;
        let max_scroll = 25;

        while count < max_scroll {
            if utils::is_rmb_down() {
                return ScrollResult::Interrupt;
            }

            let _ = self.system_control.mouse_scroll(1, false);

            utils::sleep(self.config.scroll_delay.try_into().unwrap());
            count += 1;

            let color = match self.get_flag_color() {
                Ok(color) => color,
                Err(_) => return ScrollResult::Failed,
            };

            if state == 0 && color_distance(&self.initial_color, &color) > 10 {
                state = 1;
            } else if state == 1 && color_distance(&self.initial_color, &color) <= 10 {
                self.update_avg_row(count);
                return ScrollResult::Success;
            }
        }

        ScrollResult::TimeLimitExceeded
    }

    pub fn scroll_rows(&mut self, count: i32) -> ScrollResult {
        if cfg!(not(target_os = "macos")) && self.scrolled_rows >= 5 {
            let length = self.estimate_scroll_length(count);

            for _ in 0..length {
                if self.system_control.mouse_scroll(1, false).is_err() {
                    return ScrollResult::Failed;
                }
            }

            utils::sleep(self.config.scroll_delay.try_into().unwrap());

            self.align_row();
            return ScrollResult::Skip;
        }

        for _ in 0..count {
            match self.scroll_one_row() {
                ScrollResult::Success | ScrollResult::Skip => continue,
                ScrollResult::Interrupt => return ScrollResult::Interrupt,
                v => {
                    error!("滚动失败: {v:?}");
                    return v;
                },
            }
        }

        ScrollResult::Success
    }

    pub fn wait_until_switched(&mut self) -> Result<()> {
        if self.game_info.is_cloud {
            let wait_time = self.config.get_optimized_cloud_wait();
            utils::sleep(wait_time as u32);
            return anyhow::Ok(());
        }

        let now = SystemTime::now();
        let max_wait = self.config.get_optimized_switch_wait() as u128;

        let mut consecutive_time = 0;
        let mut diff_flag = false;
        while now.elapsed().unwrap().as_millis() < max_wait {
            let im = self.capturer.capture_relative_to(
                self.window_info.pool_rect.to_rect_i32(),
                self.game_info.window.origin(),
            )?;

            let pool = calc_pool(im.as_raw()) as f64;

            if (pool - self.pool).abs() > 0.000001 {
                self.pool = pool;
                diff_flag = true;
                consecutive_time = 0;
            } else if diff_flag {
                consecutive_time += 1;
                if consecutive_time == 1 {
                    self.avg_switch_time = (self.avg_switch_time * self.scanned_count as f64
                        + now.elapsed().unwrap().as_millis() as f64)
                        / (self.scanned_count as f64 + 1.0);
                    self.scanned_count += 1;
                    return anyhow::Ok(());
                }
            }

            // 减少等待检查的频率，降低CPU使用率
            if self.config.fast_mode {
                utils::sleep(5); // 快速模式下更频繁地检查
            } else {
                utils::sleep(10); // 正常模式下适度检查
            }
        }

        Err(anyhow!("Wait until switched failed"))
    }

    #[inline(always)]
    pub fn mouse_scroll(&mut self, length: i32, try_find: bool) {
        #[cfg(windows)]
        self.system_control.mouse_scroll(length, try_find).unwrap();

        #[cfg(target_os = "linux")]
        self.system_control.mouse_scroll(length, try_find).unwrap();

        #[cfg(target_os = "macos")]
        {
            match self.game_info.ui {
                crate::common::UI::Desktop => {
                    self.system_control.mouse_scroll(length);
                    utils::sleep(20);
                },
                crate::common::UI::Mobile => {
                    if try_find {
                        self.system_control.mac_scroll_fast(length);
                    } else {
                        self.system_control.mac_scroll_slow(length);
                    }
                },
            }
        }
    }

    #[inline(always)]
    fn update_avg_row(&mut self, count: i32) {
        let current = self.avg_scroll_one_row * self.scrolled_rows as f64 + count as f64;
        self.scrolled_rows += 1;
        self.avg_scroll_one_row = current / self.scrolled_rows as f64;
    }

    #[inline(always)]
    fn estimate_scroll_length(&self, count: i32) -> i32 {
        ((self.avg_scroll_one_row * count as f64 - 2.0).round() as i32).max(0)
    }
}
