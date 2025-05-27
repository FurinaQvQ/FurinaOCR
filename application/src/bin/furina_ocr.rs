use furina_core::utils::press_any_key_to_continue;
use genshin::application::ArtifactScannerApplication;

/// 初始化应用程序环境
///
/// 配置日志系统，设置日志级别为 Info
fn init() {
    env_logger::Builder::new().filter_level(log::LevelFilter::Info).init();
}

/// FurinaOCR 应用程序主入口
///
/// 主要功能：
/// 1. 初始化应用程序环境
/// 2. 构建命令行界面
/// 3. 运行圣遗物扫描应用
/// 4. 处理运行结果
pub fn main() {
    // 初始化环境
    init();

    // 构建命令行界面
    let cmd = ArtifactScannerApplication::build_command();
    let matches = cmd.get_matches();

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
            log::error!("程序执行出错: {}", e);
            press_any_key_to_continue();
        },
    }
}
