use anyhow::Result;
use clap::{command, ArgMatches, Args};
use furina_core::export::{AssetEmitter, ExportAssets};
use furina_core::game_info::{GameInfo, GameInfoBuilder};
use furina_core::load_window_info_repo;
use furina_core::window_info::{WindowInfoRepository, WindowInfoTemplatePerSize};
use log::{error, info, warn};

use crate::artifact::GenshinArtifact;
use crate::export::artifact::{ExportArtifactConfig, GenshinArtifactExporter};
use crate::scanner::{
    get_error_suggestion, ArtifactScanError, GenshinArtifactScanner, GenshinArtifactScannerConfig,
};
use crate::scanner_controller::repository_layout::GenshinRepositoryScannerLogicConfig;

pub struct ArtifactScannerApplication {
    arg_matches: ArgMatches,
}

impl ArtifactScannerApplication {
    pub fn new(matches: ArgMatches) -> Self {
        ArtifactScannerApplication { arg_matches: matches }
    }

    pub fn build_command() -> clap::Command {
        let mut cmd = command!();
        cmd = <ExportArtifactConfig as Args>::augment_args_for_update(cmd);
        cmd = <GenshinArtifactScannerConfig as Args>::augment_args_for_update(cmd);
        cmd = <GenshinRepositoryScannerLogicConfig as Args>::augment_args_for_update(cmd);
        cmd
    }

    fn get_window_info_repository() -> WindowInfoRepository {
        load_window_info_repo!(
            "../../window_info/windows1600x900.json",
            "../../window_info/windows1280x960.json",
            "../../window_info/windows1440x900.json",
            "../../window_info/windows2100x900.json",
            "../../window_info/windows3440x1440.json",
        )
    }

    // fn init() {
    //     env_logger::Builder::new()
    //         .filter_level(log::LevelFilter::Info)
    //         .init();
    // }

    fn get_game_info() -> Result<GameInfo> {
        let game_info = GameInfoBuilder::new()
            .add_local_window_name("原神")
            .add_local_window_name("Genshin Impact")
            .add_cloud_window_name("云·原神")
            .build();
        game_info
    }
}

impl ArtifactScannerApplication {
    pub fn run(&self) -> Result<()> {
        let arg_matches = &self.arg_matches;
        let window_info_repository = Self::get_window_info_repository();
        let game_info = Self::get_game_info().map_err(|e| {
            let error = ArtifactScanError::WindowInfoFailed {
                error_msg: format!("游戏窗口检测失败: {}", e),
            };
            error!("游戏窗口检测失败: {}", error);
            error!("建议: {}", get_error_suggestion(&error));
            anyhow::anyhow!(error)
        })?;

        info!("window: {:?}", game_info.window);
        info!("ui: {:?}", game_info.ui);
        info!("cloud: {}", game_info.is_cloud);
        info!("resolution family: {:?}", game_info.resolution_family);

        #[cfg(target_os = "windows")]
        {
            // assure admin
            if !furina_core::utils::is_admin() {
                let error = ArtifactScanError::Unknown {
                    error_msg: "需要管理员权限运行程序".to_string(),
                };
                error!("权限检查失败: {}", error);
                error!("建议: 请右键点击程序，选择\"以管理员身份运行\"");
                return Err(anyhow::anyhow!(error));
            }
        }

        let mut scanner = GenshinArtifactScanner::from_arg_matches(
            &window_info_repository,
            arg_matches,
            game_info.clone(),
        )
        .map_err(|e| {
            error!("扫描器初始化失败: {}", e);
            if e.to_string().contains("模型加载失败") {
                error!("可能的解决方案:");
                error!("1. 检查程序目录下是否存在 models 文件夹");
                error!("2. 检查 models 文件夹中是否包含必要的模型文件");
                error!("3. 重新下载完整的程序包");
            }
            e
        })?;

        info!("开始扫描圣遗物...");
        let scan_start_time = std::time::Instant::now();

        let result = scanner.scan().map_err(|e| {
            error!("扫描过程发生错误: {}", e);
            if e.to_string().contains("图像捕获失败") {
                error!("图像捕获相关问题的解决方案:");
                error!("1. 确保原神游戏窗口完全可见且未被遮挡");
                error!("2. 检查游戏分辨率设置是否为支持的16:9比例");
                error!("3. 尝试切换到窗口模式或全屏模式");
                error!("4. 检查是否有其他程序占用屏幕捕获功能");
            } else if e.to_string().contains("OCR识别失败") {
                error!("文字识别相关问题的解决方案:");
                error!("1. 确保游戏语言设置为简体中文");
                error!("2. 检查游戏界面亮度和对比度设置");
                error!("3. 确保圣遗物详情界面完全显示");
                error!("4. 尝试调整游戏窗口大小");
            }
            e
        })?;

        let scan_duration = scan_start_time.elapsed();
        info!("扫描完成，耗时: {:?}", scan_duration);

        // 详细的扫描结果分析
        let total_scanned = result.len();
        let error_items = result.iter().filter(|r| r.has_errors()).count();
        let low_confidence_items = result.iter().filter(|r| !r.is_reliable(0.8)).count();
        let high_quality_items = result.iter().filter(|r| r.star >= 4).count();

        info!("扫描结果统计:");
        info!("- 总计扫描: {} 个圣遗物", total_scanned);
        info!("- 高品质物品(4星及以上): {} 个", high_quality_items);

        if error_items > 0 {
            warn!("- 存在识别错误: {} 个", error_items);
            warn!("  这些物品的数据可能不准确，建议手动检查");
        }

        if low_confidence_items > 0 {
            warn!("- 置信度较低: {} 个", low_confidence_items);
            warn!("  这些物品的识别可能存在问题");
        }

        // 显示有问题的物品详情
        if error_items > 0 && total_scanned <= 50 {
            // 只在物品数量不多时显示详情
            warn!("存在错误的物品详情:");
            for (i, item) in result.iter().enumerate() {
                if item.has_errors() {
                    warn!(
                        "  第{}个: {} ({}星, 等级{}, 置信度:{:.2})",
                        i + 1,
                        item.name,
                        item.star,
                        item.level,
                        item.confidence_score
                    );
                    for error in &item.scan_errors {
                        warn!("    错误: {}", error);
                    }
                }
            }
        }

        // 转换为导出格式，并记录转换失败的物品
        let mut artifacts = Vec::new();
        let mut conversion_failed_items = Vec::new();

        for (index, scan_result) in result.iter().enumerate() {
            match GenshinArtifact::try_from(scan_result) {
                Ok(artifact) => artifacts.push(artifact),
                Err(_) => {
                    // 详细诊断转换失败的原因
                    let mut failure_reasons = Vec::new();

                    // 检查套装识别
                    if crate::artifact::ArtifactSetName::from_zh_cn(&scan_result.name).is_none() {
                        failure_reasons.push(format!("套装名称无法识别: '{}'", scan_result.name));
                    }

                    // 检查部位识别
                    if crate::artifact::ArtifactSlot::from_zh_cn(&scan_result.name).is_none() {
                        failure_reasons.push(format!("部位无法识别: '{}'", scan_result.name));
                    }

                    // 检查主属性解析
                    let main_stat_raw =
                        format!("{}+{}", scan_result.main_stat_name, scan_result.main_stat_value);
                    if crate::artifact::ArtifactStat::from_zh_cn_raw(&main_stat_raw).is_none() {
                        failure_reasons.push(format!("主属性解析失败: '{}'", main_stat_raw));
                    }

                    // 检查是否为明显的OCR识别错误
                    if scan_result.name.len() <= 3
                        || scan_result.name.chars().any(|c| !c.is_alphabetic())
                    {
                        failure_reasons
                            .push("疑似OCR识别错误：圣遗物名称过短或包含异常字符".to_string());
                    }

                    conversion_failed_items.push((index + 1, scan_result, failure_reasons));
                },
            }
        }

        let conversion_errors = conversion_failed_items.len();
        if conversion_errors > 0 {
            warn!("数据转换过程中丢失了 {} 个物品", conversion_errors);
            warn!("这通常是由于识别错误导致的数据格式问题");

            // 显示转换失败的物品详情（限制显示数量避免日志过长）
            if conversion_errors <= 10 {
                warn!("转换失败的物品详情:");
                for (index, item, reasons) in &conversion_failed_items {
                    warn!(
                        "  第{}个: {} ({}星, 等级{}, 置信度:{:.2})",
                        index, item.name, item.star, item.level, item.confidence_score
                    );
                    if item.has_errors() {
                        warn!("    该物品存在 {} 个识别错误", item.error_count());
                    }
                    warn!("    转换失败原因:");
                    for reason in reasons {
                        warn!("      - {}", reason);
                    }

                    // 为OCR识别错误提供特殊建议
                    if reasons.iter().any(|r| r.contains("疑似OCR识别错误")) {
                        warn!("    💡 OCR识别错误解决建议:");
                        warn!("      1. 确保游戏界面清晰，圣遗物名称完全可见");
                        warn!("      2. 检查游戏语言设置是否为简体中文");
                        warn!("      3. 调整游戏窗口大小或分辨率");
                        warn!("      4. 确保圣遗物详情界面没有被其他窗口遮挡");
                        warn!("      5. 如果问题持续，可以尝试重新扫描该物品");
                    }

                    warn!("    原始数据:");
                    warn!("      - 主属性: {} = {}", item.main_stat_name, item.main_stat_value);
                    warn!("      - 副属性: {:?}", item.sub_stat);
                    warn!("      - 装备状态: {}", item.equip);
                }
            } else {
                warn!("转换失败的物品过多({})，建议检查扫描质量", conversion_errors);
            }
        }

        // 导出结果
        let exporter = GenshinArtifactExporter::new(arg_matches, &artifacts).map_err(|e| {
            error!("导出器初始化失败: {}", e);
            error!("可能的解决方案:");
            error!("1. 检查导出目录是否存在且有写入权限");
            error!("2. 检查磁盘空间是否充足");
            e
        })?;

        let mut export_assets = ExportAssets::new();
        exporter.emit(&mut export_assets);

        let stats = export_assets.save();
        info!("导出结果：");
        let table = format!("{}", stats);
        // print multiline
        for line in table.lines() {
            info!("{}", line);
        }

        // 最终总结
        info!("=== 扫描完成总结 ===");
        info!("✅ 成功识别 {} 件圣遗物", total_scanned);
        info!("✅ 成功导出 {} 件圣遗物", artifacts.len());
        info!("⏱️  总耗时: {:?}", scan_duration);

        // 综合判断是否有任何问题
        let has_any_issues = error_items > 0 || low_confidence_items > 0 || conversion_errors > 0;

        if !has_any_issues {
            info!("🎉 扫描过程完美，未发现任何错误！");
        } else {
            if error_items > 0 {
                warn!("⚠️  {} 个物品存在识别错误", error_items);
            }
            if low_confidence_items > 0 {
                warn!("⚠️  {} 个物品置信度较低", low_confidence_items);
            }
            if conversion_errors > 0 {
                warn!("⚠️  {} 个物品在数据转换时丢失", conversion_errors);
            }
            warn!("💡 建议检查游戏设置和环境，以提高识别准确率");
        }

        Ok(())
    }
}
