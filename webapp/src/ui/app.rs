use dioxus::prelude::*;

use crate::core::eval::eval;
use crate::core::eval_result::EvalResult;
use crate::ui::topbar::TopBar;

pub fn App(cx: Scope) -> Element {

    let input = use_state(&cx, || crate::data::example::get_default().to_string());
    let output = use_state(&cx, || EvalResult::Ok(String::new()));

    let (output_text, output_style) = match output.get() {
        Ok(s) => (s.to_string(), "right split"),
        Err(e) => (e.error_message().trim().to_owned(), "right split error"),
    };
    
    cx.render(rsx! {
        textarea {
            class: "left split",
            white_space: "pre-wrap", // make \n works
            oninput: move |evt| {
                input.set(evt.value.clone());
                output.set(EvalResult::Ok(String::new()));
            },
            value: "{input}"
        },
        div {
            class: "{output_style}",
            white_space: "pre-wrap", // make \n works
            "{output_text}"
        },
        div {
            class: "centered fab",
            onclick: move |_| {
                output.set(eval(input.get().clone()));
            },
            "->"
        },
        TopBar(cx, move |x|{
            input.set(crate::data::example::get_by_name(&x).to_string());
            output.set(EvalResult::Ok(String::new()));
        })
    })
}