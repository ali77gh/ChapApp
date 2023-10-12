use dioxus::prelude::*;

pub fn TopBar(cx: Scope) -> Element {

    cx.render(rsx! {
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