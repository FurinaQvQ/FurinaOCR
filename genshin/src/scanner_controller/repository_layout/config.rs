use clap::arg;

#[derive(Clone, clap::Args)]
pub struct GenshinRepositoryScannerLogicConfig {
    /// Max rows to scan
    #[arg(id = "max-row", long = "max-row", help = "最大扫描行数", default_value_t = -1)]
    pub max_row: i32,

    // todo move to another scanner
    /// Will the scanner capture only?
    // pub capture_only: bool,

    /// The time to wait for scrolling. Consider increasing this value if the scrolling is not correct
    #[arg(
        id = "scroll-delay",
        long = "scroll-delay",
        help = "翻页时滚轮停顿时间（ms）（翻页不正确可以考虑加大该选项）",
        default_value_t = 50
    )]
    pub scroll_delay: i32,

    /// Dump the captured image
    // pub dump_mode: bool,

    /// The maximum time to wait for switching to the next item
    #[arg(
        id = "max-wait-switch-item",
        long = "max-wait-switch-item",
        help = "切换物品最大等待时间（ms）",
        default_value_t = 600
    )]
    pub max_wait_switch_item: i32,

    /// The time to wait for switching to the next item in cloud game
    #[arg(
        id = "cloud-wait-switch-item",
        long = "cloud-wait-switch-item",
        help = "云游戏切换物品等待时间（ms）",
        default_value_t = 200
    )]
    pub cloud_wait_switch_item: i32,

    /// Enable fast mode with reduced delays
    #[arg(id = "fast-mode", long = "fast-mode", help = "启用快速模式，减少等待时间")]
    pub fast_mode: bool,

    /// Enable adaptive timing that adjusts delays based on performance
    #[arg(id = "adaptive-timing", long = "adaptive-timing", help = "启用自适应时间调整")]
    pub adaptive_timing: bool,

    /// Enable performance monitoring
    #[arg(id = "performance-monitor", long = "performance-monitor", help = "启用性能监控")]
    pub performance_monitor: bool,
}

impl Default for GenshinRepositoryScannerLogicConfig {
    fn default() -> Self {
        GenshinRepositoryScannerLogicConfig {
            max_row: -1,
            // capture_only: false,
            scroll_delay: 50,
            // number: -1,
            // dump_mode: false,
            max_wait_switch_item: 600,
            cloud_wait_switch_item: 200,
            fast_mode: false,
            adaptive_timing: true,
            performance_monitor: false,
        }
    }
}

impl GenshinRepositoryScannerLogicConfig {
    /// 获取优化后的滚动延时
    pub fn get_optimized_scroll_delay(&self) -> i32 {
        if self.fast_mode {
            (self.scroll_delay as f64 * 0.7) as i32 // 快速模式减少30%延时
        } else {
            self.scroll_delay
        }
    }

    /// 获取优化后的切换等待时间
    pub fn get_optimized_switch_wait(&self) -> i32 {
        if self.fast_mode {
            (self.max_wait_switch_item as f64 * 0.8) as i32 // 快速模式减少20%等待
        } else {
            self.max_wait_switch_item
        }
    }

    /// 获取优化后的云游戏等待时间
    pub fn get_optimized_cloud_wait(&self) -> i32 {
        if self.fast_mode {
            (self.cloud_wait_switch_item as f64 * 0.8) as i32
        } else {
            self.cloud_wait_switch_item
        }
    }
}
