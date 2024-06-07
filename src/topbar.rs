use dioxus::prelude::*;

// Remember: Owned props must implement `PartialEq`!
#[derive(PartialEq, Props, Clone)]
pub struct TopBarProps {
    pub signal: Signal<String>,
}

pub fn TopBar(props: TopBarProps) -> Element {
    rsx! {
        div { class: "topbar",
            div { margin_left: "10px", display: "flex",
                div { "Chap online compiler " }
                div { class: "desktop_only", "(Powerd by WASM/Dioxus)" }
            }
            ExampleLoader { signal: props.signal }
            img {
                class: "desktop_only",
                src: "https://badgen.net/github/stars/ali77gh/Chap"
            }
            img {
                class: "desktop_only",
                src: "https://badgen.net/github/last-commit/ali77gh/chap"
            }
            img {
                class: "desktop_only",
                src: "https://img.shields.io/github/release/ali77gh/chap.svg"
            }
            img {
                class: "desktop_only",
                src: "https://img.shields.io/github/license/ali77gh/chap.svg"
            }
            a {
                margin_right: "10px",
                color: "white",
                href: "https://github.com/ali77gh/Chap",
                target: "_blank",
                "Github"
            }
        }
    }
}

pub fn ExampleLoader(mut props: TopBarProps) -> Element {
    rsx! {
        select {
            onchange: move |evt| {
                let selected = evt.value().clone();
                let source = crate::examples::get_by_name(&selected);
                props.signal.set(source.to_string());
            },
            option { "hello_world" }
            option { "count_down" }
            option { "christmas_tree" }
            option { "say_my_name" }
            option { "is_prime" }
            option { "better_tree" }
        }
    }
}
