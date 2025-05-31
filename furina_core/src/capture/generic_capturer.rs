#[cfg(target_os = "windows")]
use crate::capture::WindowsCapturer;
#[cfg(target_os = "windows")]
pub type GenericCapturer = WindowsCapturer;

// #[cfg(target_os = "macos")]
// pub type GenericCapturer =
