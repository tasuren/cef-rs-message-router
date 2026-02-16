// This module is derived from here: https://github.com/tauri-apps/cef-rs/tree/c35e1d08dbd24253d314eef4bef9455e4c67e14c/examples/tests_shared/src/browser/resource_util

#[cfg(target_os = "windows")]
pub mod win;
#[cfg(target_os = "windows")]
pub use win::*;

#[cfg(not(target_os = "windows"))]
pub mod posix;
#[cfg(not(target_os = "windows"))]
pub use posix::*;
