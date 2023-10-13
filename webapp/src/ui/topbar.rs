use dioxus::prelude::*;

pub fn TopBar<'a>(cx: Scope<'a>, cb: impl Fn(String) + 'a) -> Element {

    cx.render(rsx! {
        div{
            class: "topbar",
            div {
                margin_left:"10px",
                "Chap online compiler (Powerd by WASM/Dioxus)"
            },
            ExampleLoader(cx, move |x|{
                js_runner(&cx, format!("console.log('{}')",x).as_str());
                cb(x);
            }),
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

pub fn ExampleLoader<'a>(cx: Scope<'a>, cb: impl Fn(String) + 'a) -> Element {

    cx.render(rsx!{
        select{
            onchange: move |evt|{
                cb(evt.value.clone());
            },
            option{ "hello_world" },
            option{ "count_down" },
            option{ "christmas_tree" },
            option{ "is_prime" },
        }
    })
}

fn js_runner(cx: &Scope, js: &str){

    let eval = use_eval(cx);
    eval(js).unwrap();
}