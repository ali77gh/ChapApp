#![allow(non_snake_case)]

mod ui;
mod core;
mod data;

use ui::app::App;

fn main() {
    dioxus_web::launch(App);
}
