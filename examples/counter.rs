#![allow(non_snake_case)]

use dioxus::html::button;
use dioxus::prelude::*;
use dioxus_signals::*;
use dioxus_desktop::{Config, WindowBuilder};

pub fn App(cx: Scope) -> Element {
    let mut count = use_signal(cx, ||0);

    render!{
        rsx! {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}

fn main() {
    dioxus_desktop::launch_cfg(
        App,
        Config::default().with_window(WindowBuilder::new().with_resizable(false).with_inner_size(
            dioxus_desktop::wry::application::dpi::LogicalSize::new(400.0, 200.0),
        )),
    );
}