use dioxus::prelude::*;

pub fn js_runner(cx: &Scope, js: &str){
    let eval = use_eval(cx);
    eval(js).unwrap();
}

pub fn set_path(cx: &Scope, path: &str){
    js_runner(cx, format!("document.location.href = '{}';",path).as_str());
}