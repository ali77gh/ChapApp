use dioxus::prelude::*;

use crate::core::eval::eval;
use crate::ui::topbar::TopBar;

pub fn App(cx: Scope) -> Element {

    let output = use_state(cx, || "".to_string());
    let input = use_state(cx, || crate::data::example::get_default().to_string());

    cx.render(rsx! {
        textarea {
            class: "left split",
            oninput: move |evt| input.set(evt.value.clone()),
            "{input}"
        },
        div {
            class: "right split",
            white_space: "pre-wrap", // make \n works
            "{output}"
        },
        div {
            class: "centered fab",
            onclick: move |_| {
                output.set(eval( input.get().clone()));
            },
            "->"
        },
        TopBar(cx)
    })
}