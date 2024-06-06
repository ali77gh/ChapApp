#![allow(non_snake_case)]

mod eval;
mod examples;
pub mod topbar;

use chap::common::errors::ChapError;
use dioxus::prelude::*;
use topbar::TopBar;
use tracing::Level;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn App() -> Element {
    let mut input = use_signal(|| crate::examples::get_default().to_string());
    let mut output = use_signal(|| Ok::<std::string::String, ChapError>(String::new()));

    let (output_text, output_style) = match output.read().clone() {
        Ok(s) => (s.to_string(), "right split"),
        Err(e) => (e.error_message().trim().to_owned(), "right split error"),
    };

    rsx! {
        textarea {
            class: "left split",
            white_space: "pre-wrap",
            oninput: move |evt| {
                input.set(evt.value().clone());
                output.set(Ok(String::new()));
            },
            value: "{input}"
        }
        div { class: "{output_style}", white_space: "pre-wrap", "{output_text}" }
        div {
            class: "centered fab",
            onclick: move |_| {
                output.set(crate::eval::eval(input.read().clone()));
            },
            "->"
        }
        TopBar { signal: input }
    }
}
