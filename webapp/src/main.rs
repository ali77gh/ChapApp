#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

use chap::eval::eval;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {

    let output = use_state(cx, || "".to_string());
    let input = use_state(cx, || DEFAULT_SOURCE.to_string());

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
                output.set(eval_and_collect_stdout( input.get().clone()));
            },
            "->"
        },
        // top bar
        div{
            class: "topbar",
            div {
                margin_left:"10px",
                "Chap online compiler (Powerd by WASM/Dioxus)"
            },
            img{
                src: "https://badgen.net/github/stars/ali77gh/Chap",
            },
            img{
                src: "https://badgen.net/github/last-commit/ali77gh/chap",
            },
            img{
                src: "https://img.shields.io/github/release/ali77gh/chap.svg",
            },
            img{
                src: "https://img.shields.io/github/license/ali77gh/chap.svg",
            },
            a {
                margin_right: "10px",
                href: "https://github.com/ali77gh/Chap",
                target: "_blank",
                "Github"
            },
        }
    })
}

static mut TEMP: String = String::new();
fn eval_and_collect_stdout(source: String) -> String{
    unsafe{TEMP.clear()}
    eval(source, |x|{
        unsafe{
            TEMP.push_str(x.clone());
            TEMP.push_str("\n")
        }
    }, ||{ return "".to_string(); }, ||{}, |e|{});
    unsafe{ TEMP.clone() } 
}


const DEFAULT_SOURCE: &str = r#"0 -> $counter
@loop
    $counter -> increase

    $counter, 2 -> multiply -> $stars_size
    10, $counter -> minus -> $space_size

    "*", $stars_size -> repeat -> $stars
    " ", $space_size -> repeat -> $spaces

    $spaces, $stars -> cat
@loop, $counter, 10 -> jump if not equal
"#;