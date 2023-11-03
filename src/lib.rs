use std::rc::Rc;
use dioxus::prelude::*;
use dioxus_signals::{Signal, use_signal};

pub fn use_mounted<T>(cx: Scope<T>) -> UseMounted {
    let signal = use_signal(cx, || None);
    UseMounted { signal }
}

#[derive(Clone, Copy, PartialEq)]
pub struct UseMounted {
    pub signal: Signal<Option<Rc<MountedData>>>
}

impl UseMounted {
    pub fn onmounted(self, event: Event<MountedData>) {
        self.mount(event.data)
    }

    pub fn mount(self, data: Rc<MountedData>) {
        self.signal.set(Some(data));
    }
}
