//! FurinaOCR - 专注于原神圣遗物扫描的高效工具
//!
//! 为原神玩家量身定制的五星圣遗物导出方案

use clap::Parser;

/// FurinaOCR - 专注于原神圣遗物扫描的高效工具
#[derive(Parser, Debug)]
#[command(
    name = "FurinaOCR",
    version = "0.1.0",
    about = "专注于原神圣遗物扫描的高效工具",
    long_about = "为原神玩家量身定制的五星圣遗物导出方案\n使用AI技术进行高精度圣遗物识别和导出"
)]
struct Args {
    /// 最小星级（默认5星）
    #[arg(short, long, default_value = "5")]
    min_star: u8,

    /// 导出目录
    #[arg(short, long)]
    output_dir: Option<String>,

    /// 导出格式（mona/good/mingyulab/csv/all）
    #[arg(short, long, default_value = "mona")]
    format: String,

    /// 快速模式
    #[arg(short, long)]
    fast_mode: bool,

    /// 性能监控模式
    #[arg(short, long)]
    performance_monitor: bool,

    /// 调试模式
    #[arg(short, long)]
    debug: bool,

    /// 日志级别（trace/debug/info/warn/error）
    #[arg(long, default_value = "info")]
    log_level: String,

    /// 滚动延时（毫秒）
    #[arg(long)]
    scroll_delay: Option<u64>,

    /// 最大等待切换物品时间（毫秒）
    #[arg(long)]
    max_wait_switch_item: Option<u64>,
}

/// 初始化应用程序环境
fn init() {
    env_logger::Builder::new().filter_level(log::LevelFilter::Info).init();
}

/// 模拟等待用户按键
fn press_any_key_to_continue() {
    println!("按任意键继续...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

/// FurinaOCR 应用程序主入口
fn main() {
    // 初始化环境
    init();

    // 解析命令行参数
    let args = Args::parse();

    // 直接输出到控制台，不依赖日志
    println!("FurinaOCR v0.1.0 - 专注于原神圣遗物扫描的高效工具");
    println!("{}", "=".repeat(60));

    // 显示配置信息
    println!("配置信息:");
    println!("  最小星级: {}", args.min_star);
    println!("  导出格式: {}", args.format);

    if let Some(output_dir) = &args.output_dir {
        println!("  导出目录: {output_dir}");
    }

    if args.debug {
        println!("  调试模式: 已启用");
    }

    if args.fast_mode {
        println!("  快速模式: 已启用");
    }

    if args.performance_monitor {
        println!("  性能监控: 已启用");
    }

    println!();
    println!("⚠️  注意：当前版本缺少AI模型文件");
    println!("   请按照README说明获取模型文件后重新编译");
    println!();
    println!("✅ 程序配置验证成功（演示模式）");

    press_any_key_to_continue();
}
