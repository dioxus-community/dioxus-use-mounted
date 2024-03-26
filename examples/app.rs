use dioxus::prelude::*;
use dioxus_use_mounted::use_mounted;
use dioxus_web::Config;

fn app() -> Element {
    let mounted = use_mounted();
    rsx!(div {
        onmounted: move |event| mounted.onmounted(event)
    })
}

fn main() {
    dioxus_web::launch::launch_cfg(app, Config::new())
}
