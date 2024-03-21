#![allow(non_snake_case)]

use dioxus::prelude::*;
use eval::eval;
use std::str::FromStr;
use dioxus::html::form;
use dioxus_desktop::{Config, WindowBuilder};
use lazy_static::lazy_static;
use log::warn;
use regex::Regex;

// 计算器应用程序
pub fn Calculator(cx: Scope) -> Element {
    // 计算器组件
    let state = use_state(cx, || String::from("0"));

    render!(div {
        link { href:"https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet" }
        style { include_str!("./calculate_sytle.css") }
        header {
            h1 { "Calculator" }
        }
        main {
            rsx! {
            div {
                class: "calculator",
                div {
                    class: "display",
                    "{state.to_string()}"
                },
                div {
                    class: "buttons",
                    button {
                        onclick: move |_| on_button_click(state, "C"),
                        "C"
                    },
                    button {
                        onclick: move |_| on_button_click(state, "+"),
                        "+"
                    },
                    button {
                        onclick: move |_| on_button_click(state, "-"),
                        "-"
                    },
                    button {
                        onclick: move |_| on_button_click(state, "*"),
                        "*"
                    },
                    button {
                        onclick: move |_| on_button_click(state, "/"),
                        "/"
                    },
                    button {
                        onclick: move |_| on_button_click(state, "7"),
                        "7"
                    },
                    button {
                        onclick: move |_| on_button_click(state, "8"),
                        "8"
                    },
                    button {
                        onclick: move |_| on_button_click(state, "9"),
                        "9"
                    },
                    button {
                        onclick: move |_| on_button_click(state, "4"),
                        "4"
                    },
                    button {
                        onclick: move |_| on_button_click(state, "5"),
                        "5"
                    },
                    button {
                        onclick: move |_| on_button_click(state, "6"),
                        "6"
                    },
                    button {
                        onclick: move |_| on_button_click(state, "1"),
                        "1"
                    },
                    button {
                        onclick: move |_| on_button_click(state, "2"),
                        "2"
                    },
                    button {
                        onclick: move |_| on_button_click(state, "3"),
                        "3"
                    },
                    button {
                        onclick: move |_| on_button_click(state, "0"),
                        "0"
                    },
                    button {
                        onclick: move |_| on_button_click(state, "."),
                        "."
                    },
                    button {
                        onclick: move |_| on_button_click(state, "="),
                        "="
                    },
                }
            }
        }
    }})
}

fn on_button_click(state: &UseState<String>, button_value: &str) {
    // 更新显示值
    let display_value = state.to_string();
    let new_value = match button_value {
        "C" => "0".to_string(),
        "." => {
            if display_value.contains(".") { // TODO 目前只能实现一个数字里面是小数的情况
                return;
            } else {
                format!("{}.", display_value)
            }
        }
        "=" => calculate(display_value.clone()),
        value => {
            let mut tmp_value = state.to_string();
            tmp_value.push_str(value);
            let result = clear_zero(&tmp_value);
            result.to_string()
        }
    };

    state.set(new_value.to_string());
}

// 去除所有非法0
fn clear_zero(origin_str: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d*\.?\d*)([+|\-|*|/]?)?").unwrap();
    }
    let mut result = String::new();
    for cap in RE.captures_iter(origin_str) {
        let num = match &cap[0].parse::<f32>() {
            Ok(num) => num.to_string(),
            Err(_e) => cap[0].to_string()
        };
        result.push_str(&num);
    }
    result
}

// 计算表达式
fn calculate(input: String) -> String {
    // 使用 eval 库计算表达式
    println!("express: {:?}", input);
    match eval(&input) {
        Ok(value) => format!("{}", value),
        Err(e) => {
            println!("{:?}", e);
            "Error".to_string()
        },
    }
}

