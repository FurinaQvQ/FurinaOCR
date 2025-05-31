use std::io::{self, Write};

use furina_core::utils::press_any_key_to_continue;
use genshin::application::ArtifactScannerApplication;
use genshin::export::artifact::GenshinArtifactExportFormat;

/// 显示程序启动Logo和作者信息
fn show_logo() {
    println!("\n{}", "═".repeat(72));
    println!(
        "
    ███████╗██╗   ██╗██████╗ ██╗███╗   ██╗ █████╗  ██████╗  ██████╗██████╗ 
    ██╔════╝██║   ██║██╔══██╗██║████╗  ██║██╔══██╗██╔═══██╗██╔════╝██╔══██╗
    █████╗  ██║   ██║██████╔╝██║██╔██╗ ██║███████║██║   ██║██║     ██████╔╝
    ██╔══╝  ██║   ██║██╔══██╗██║██║╚██╗██║██╔══██║██║   ██║██║     ██╔══██╗
    ██║     ╚██████╔╝██║  ██║██║██║ ╚████║██║  ██║╚██████╔╝╚██████╗██║  ██║
    ╚═╝      ╚═════╝ ╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝ ╚═════╝  ╚═════╝╚═╝  ╚═╝
 
    ╔════════════════════════════════════════════════════════════════════╗
    ║                          原神圣遗物扫描工具                        ║
    ║                   Genshin Impact Artifact Scanner                  ║
    ╚════════════════════════════════════════════════════════════════════╝
    "
    );
    println!("{}", "═".repeat(72));

    println!("📌 项目信息:");
    println!("   🔖 版本: v{}", env!("CARGO_PKG_VERSION"));
    println!("   📄 许可: {}", env!("CARGO_PKG_LICENSE"));

    println!("\n💡 使用说明:");
    println!("    1. 确保原神游戏窗口处于可见状态");
    println!("    2. 打开背包中的圣遗物页面");
    println!("    3. 支持分辨率: 2560×1440, 1920×1080, 1600×900");

    println!("{}", "═".repeat(72));
}

/// 初始化应用程序环境
///
/// 配置日志系统，使用带颜色的英文格式，去掉时间戳和模块路径
fn init() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .format(|buf, record| {
            use std::io::Write;

            // 自定义日志格式：使用带颜色的英文状态标识
            let level_str = match record.level() {
                log::Level::Error => "\x1b[31m[ERROR]\x1b[0m >>>", // 红色
                log::Level::Warn => "\x1b[33m[WARN] \x1b[0m >>>",  // 黄色
                log::Level::Info => "\x1b[32m[INFO] \x1b[0m >>>",  // 绿色
                log::Level::Debug => "\x1b[34m[DEBUG]\x1b[0m >>>", // 蓝色
                log::Level::Trace => "\x1b[36m[TRACE]\x1b[0m >>>", // 青色
            };

            writeln!(buf, "{} {}", level_str, record.args())
        })
        .init();
}

/// 获取用户输入
fn get_user_input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// 交互式配置选择界面
fn interactive_config_selection() -> clap::ArgMatches {
    println!("\n🔧 配置选择界面");
    println!("{}", "=".repeat(50));

    // 询问用户是否要自定义配置
    println!("请选择配置方式：");
    println!("  1. 使用默认配置");
    println!("  2. 自定义配置");

    let choice = get_user_input("请输入选择 (1-2): ");

    match choice.as_str() {
        "1" => {
            println!("✅ 使用默认配置");
            ArtifactScannerApplication::build_command().get_matches_from(vec!["furinaocr"])
        },
        "2" => {
            println!("🛠️  开始自定义配置...\n");
            interactive_custom_config()
        },
        _ => {
            println!("❌ 无效选择，使用默认配置");
            ArtifactScannerApplication::build_command().get_matches_from(vec!["furinaocr"])
        },
    }
}

/// 交互式自定义配置
fn interactive_custom_config() -> clap::ArgMatches {
    let mut args = vec!["furinaocr".to_string()];

    // 最小星级设置
    println!("🌟 最小星级设置 (默认: 5星)");
    let min_star = get_user_input("请输入最小星级 (4-5): ");
    if !min_star.is_empty() && min_star.parse::<i32>().is_ok() {
        let star_val = min_star.parse::<i32>().unwrap();
        if (4..=5).contains(&star_val) {
            args.push("--min-star".to_string());
            args.push(min_star);
        }
    }

    // 最小等级设置
    println!("\n⬆️  最小等级设置 (默认: 0级)");
    let min_level = get_user_input("请输入最小等级 (0-20): ");
    if !min_level.is_empty() && min_level.parse::<i32>().is_ok() {
        let level_val = min_level.parse::<i32>().unwrap();
        if (0..=20).contains(&level_val) {
            args.push("--min-level".to_string());
            args.push(min_level);
        }
    }

    // 导出格式选择
    println!("\n📤 导出格式选择 (默认: mona)");
    println!("  1. mona - 莫娜占卜铺");
    println!("  2. mingyu-lab - 原魔计算器");
    println!("  3. good - GOOD通用格式");
    println!("  4. csv - CSV表格");
    println!("  5. all - 所有格式");
    let format_choice = get_user_input("请选择导出格式 (1-5): ");
    let format = match format_choice.as_str() {
        "1" => "mona",
        "2" => "mingyu-lab",
        "3" => "good",
        "4" => "csv",
        "5" => "all",
        _ => "mona",
    };
    if format != "mona" {
        args.push("--format".to_string());
        args.push(format.to_string());
    }

    println!("\n✅ 配置完成！");

    ArtifactScannerApplication::build_command().get_matches_from(args)
}

/// 显示当前配置选项
fn show_config_options(matches: &clap::ArgMatches) {
    println!("\n⚙️  当前配置选项:");
    println!("{}", "-".repeat(50));

    // 扫描配置
    let min_star = matches.get_one::<i32>("min-star").unwrap_or(&5);
    let min_level = matches.get_one::<i32>("min-level").unwrap_or(&0);

    println!("🔍 扫描设置:");
    println!("   最小星级: {min_star}星");
    println!("   最小等级: {min_level}级");

    // 导出配置
    let default_format = GenshinArtifactExportFormat::Mona;
    let format =
        matches.get_one::<GenshinArtifactExportFormat>("format").unwrap_or(&default_format);

    println!("\n📤 导出设置:");
    let format_desc = match format {
        GenshinArtifactExportFormat::Mona => "莫娜占卜铺",
        GenshinArtifactExportFormat::MingyuLab => "原魔计算器",
        GenshinArtifactExportFormat::Good => "GOOD通用格式",
        GenshinArtifactExportFormat::CSV => "CSV表格",
        GenshinArtifactExportFormat::All => "所有格式",
    };
    let format_name = match format {
        GenshinArtifactExportFormat::Mona => "mona",
        GenshinArtifactExportFormat::MingyuLab => "mingyu-lab",
        GenshinArtifactExportFormat::Good => "good",
        GenshinArtifactExportFormat::CSV => "csv",
        GenshinArtifactExportFormat::All => "all",
    };
    println!("   导出格式: {format_name} ({format_desc})");

    // 滚动配置
    let scroll_delay = matches.get_one::<i32>("scroll-delay").unwrap_or(&50);
    let max_wait = matches.get_one::<i32>("max-wait-switch-item").unwrap_or(&600);
    let cloud_wait = matches.get_one::<i32>("cloud-wait-switch-item").unwrap_or(&200);

    println!("\n⚡ 性能设置:");
    println!("   滚动延时: {scroll_delay}ms");
    println!("   切换等待: {max_wait}ms");
    println!("   云游戏等待: {cloud_wait}ms");

    println!("{}", "-".repeat(50));

    // 确认开始扫描
    println!("\n💡 准备开始扫描！");
    let start = get_user_input("按回车键开始扫描，或输入 'q' 退出: ");
    if start.to_lowercase() == "q" || start.to_lowercase() == "quit" {
        println!("👋 已退出程序");
        std::process::exit(0);
    }
}

/// FurinaOCR 应用程序主入口
///
/// 主要功能：
/// 1. 显示程序Logo和作者信息
/// 2. 初始化应用程序环境
/// 3. 交互式配置选择
/// 4. 显示配置选项并确认
/// 5. 运行圣遗物扫描应用
/// 6. 处理运行结果
fn main() {
    // 显示程序Logo
    show_logo();

    // 初始化环境
    init();

    // 检查是否有命令行参数
    let args: Vec<String> = std::env::args().collect();
    let matches = if args.len() > 1 {
        // 如果有命令行参数，直接解析
        let cmd = ArtifactScannerApplication::build_command();
        cmd.get_matches()
    } else {
        // 如果没有命令行参数，启动交互式界面
        interactive_config_selection()
    };

    // 显示当前配置选项并确认
    show_config_options(&matches);

    // 创建并运行应用程序
    let application = ArtifactScannerApplication::new(matches);
    let res = application.run();

    // 处理运行结果
    match res {
        Ok(_) => {
            log::info!("程序执行成功");
            press_any_key_to_continue();
        },
        Err(e) => {
            log::error!("程序执行出错: {e}");
            press_any_key_to_continue();
        },
    }
}
