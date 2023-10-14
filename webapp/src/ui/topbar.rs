use dioxus::prelude::*;

pub fn TopBar<'a>(cx: Scope<'a>, cb: impl Fn(String) + 'a) -> Element {

    cx.render(rsx! {
        div{
            class: "topbar",
            div {
                margin_left:"10px",
                display: "flex",
                div{
                    "Chap online compiler "
                },
                div{
                    class: "desktop_only",
                    "(Powerd by WASM/Dioxus)"
                }
            },
            ExampleLoader(cx, cb),
            img{
                class: "desktop_only",
                src: "https://badgen.net/github/stars/ali77gh/Chap",
            },
            img{
                class: "desktop_only",
                src: "https://badgen.net/github/last-commit/ali77gh/chap",
            },
            img{
                class: "desktop_only",
                src: "https://img.shields.io/github/release/ali77gh/chap.svg",
            },
            img{
                class: "desktop_only",
                src: "https://img.shields.io/github/license/ali77gh/chap.svg",
            },
            a {
                margin_right: "10px",
                color: "white",
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
                let selected = evt.value.clone();
                cb(selected);
            },
            option{ "hello_world" },
            option{ "count_down" },
            option{ "christmas_tree" },
            option{ "is_prime" },
        }
    })
}
