#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div { class: "w-full h-full flex flex-col justify-center items-center",
            TodoInput {}
            TodoList {}
        }
    }
}

#[component]
fn TodoInput() -> Element {
    rsx!(
        div { class: "", "TODO INPUT" }
    )
}

#[component]
fn TodoList() -> Element {
    rsx!(
        div { class: "", "TODO LIST11" }
    )
}
