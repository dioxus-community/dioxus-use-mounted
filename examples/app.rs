use dioxus::prelude::*;
use dioxus_use_mounted::use_mounted;

fn app(cx: Scope) -> Element {
    let mounted = use_mounted(cx);
    render!( div { onmounted: move |event| mounted.onmounted(event) } )
}

fn main() {
    dioxus_web::launch(app)
}
