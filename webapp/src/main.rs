#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{prelude::*};

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {

    let output = use_state(cx, || "".to_string());
    let input = use_state(cx, || "".to_string());

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
                output.set(input.get().clone());
            },
            "->"
        }
    })
}

