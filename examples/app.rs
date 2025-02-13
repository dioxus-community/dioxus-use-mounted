use dioxus::launch;
use dioxus::prelude::*;
use dioxus_use_mounted::use_mounted;

fn app() -> Element {
    let mounted = use_mounted();
    rsx!(div {
        onmounted: move |event| mounted.onmounted(event)
    })
}

fn main() {
    launch(app)
}
