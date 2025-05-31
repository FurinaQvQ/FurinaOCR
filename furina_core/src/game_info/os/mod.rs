#[cfg(windows)]
mod winodws;
#[cfg(windows)]
pub use winodws::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;
