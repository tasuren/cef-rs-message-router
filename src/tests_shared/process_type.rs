// This code is derived from here: https://github.com/tauri-apps/cef-rs/blob/c35e1d08dbd24253d314eef4bef9455e4c67e14c/examples/tests_shared/src/common/client_app.rs

use cef::*;

pub const PROCESS_TYPE: &str = "type";
pub const RENDERER_PROCESS: &str = "renderer";
#[cfg(target_os = "linux")]
pub const ZYGOTE_PROCESS: &str = "zygote";

pub enum ProcessType {
    Browser,
    Renderer,
    #[cfg(target_os = "linux")]
    Zygote,
    Other,
}

impl From<&CommandLine> for ProcessType {
    fn from(value: &CommandLine) -> Self {
        let process_type = CefString::from(PROCESS_TYPE);
        if value.has_switch(Some(&process_type)) == 0 {
            return Self::Browser;
        }

        let value = CefString::from(&value.switch_value(Some(&process_type))).to_string();
        match value.as_str() {
            RENDERER_PROCESS => Self::Renderer,
            #[cfg(target_os = "linux")]
            ZYGOTE_PROCESS => Self::Zygote,
            _ => Self::Other,
        }
    }
}
