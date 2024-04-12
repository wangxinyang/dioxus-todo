#![allow(non_snake_case)]

use std::fmt::Display;

use dioxus::prelude::*;
use log::LevelFilter;
use rand::Rng;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

#[derive(Debug, Props, Clone, PartialEq)]
struct Todo {
    id: usize,
    content: String,
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

#[component]
fn App() -> Element {
    let todos = use_signal(Vec::new);
    rsx! {
        div { class: "h-full bg-red-200 max-w-[960px] mx-auto p-4",
            div { class: "w-full flex flex-col justify-center items-center",
                p { class: "text-3xl font-bold text-white tracking-wide mb-2", "TODO APP" }
                TodoInput { todos }
                TodoList { todos }
            }
        }
    }
}

#[component]
fn TodoInput(todos: Signal<Vec<Todo>>) -> Element {
    let mut todo_val = use_signal(|| "".to_string());

    rsx!(
        div { class: "w-full flex justify-center gap-x-2 mb-4",
            input {
                class: "w-full max-w-md p-4 rounded-md drop-shadow-md",
                placeholder: "ADD TODO ITEM",
                value: "{todo_val}",
                oninput: move |ev| {
                    todo_val.set(ev.value());
                },
                onkeypress: move |ev| {
                    if ev.key() == Key::Enter && !todo_val.read().is_empty() {
                        todos
                            .push(Todo {
                                id: get_id(),
                                content: todo_val.to_string(),
                            });
                        todo_val.set("".to_string());
                    }
                }
            }
            button {
                class: "p-2 w-[120px] bg-red-400 rounded-md drop-shadow-md font-semibold text-white text-xl
    scale-100 hover:scale-110 transition",
                onclick: move |_| {
                    todos
                        .push(Todo {
                            id: get_id(),
                            content: todo_val.to_string(),
                        });
                    todo_val.set("".to_string());
                },
                "ADD"
            }
        }
    )
}

#[component]
fn TodoList(todos: Signal<Vec<Todo>>) -> Element {
    let my_todos = move || {
        todos
            .iter()
            .map(|item| (item.id, item.clone()))
            .collect::<Vec<_>>()
    };

    rsx!(
        div { class: "w-full",
            div { class: "flex flex-col  max-w-[600px] mx-auto bg-white rounded-md drop-shadow-md",
                for item in my_todos().into_iter() {
                    div { class: "flex justify-center items-center border-b-2 border-gray-200",
                        div { class: "w-full flex justify-between items-center p-6",
                            p { class: "text-2xl font-semibold tracking-wide", "{item.1}" }
                            button {
                                class: "p-2 w-[120px] bg-red-400 rounded-md drop-shadow-md
    font-semibold text-white text-xl",
                                onclick: move |_| { todos.retain(|todo| todo.clone().id != item.1.id) },
                                "delete"
                            }
                        }
                    }
                }
            }
        }
    )
}

fn get_id() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen()
}
