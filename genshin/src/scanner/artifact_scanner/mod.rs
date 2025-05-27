pub use artifact_scanner::GenshinArtifactScanner;
pub use artifact_scanner_config::GenshinArtifactScannerConfig;
pub use artifact_scanner_window_info::ArtifactScannerWindowInfo;
pub use error::{get_error_suggestion, ArtifactScanError, ErrorStatistics};
pub use scan_result::GenshinArtifactScanResult;

#[allow(clippy::module_inception)]
mod artifact_scanner;
mod artifact_scanner_config;
mod artifact_scanner_window_info;
mod artifact_scanner_worker;
mod error;
mod message_items;
mod performance_optimizations;
mod scan_result;
