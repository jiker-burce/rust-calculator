use dioxus_desktop::{Config, WindowBuilder};
mod calculate_lib;
use calculate_lib::*;
// 计算器应用程序
fn main() {
    dioxus_desktop::launch_cfg(
        calculate_lib::Calculator,
        Config::default().with_window(WindowBuilder::new().with_resizable(false).with_inner_size(
            dioxus_desktop::wry::application::dpi::LogicalSize::new(240.0, 450.0),
        )),
    );
}