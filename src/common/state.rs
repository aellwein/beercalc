use crate::common::prelude::*;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub struct AppState {
    pub theme: Signal<Theme>,
    pub lang: Signal<Language>,
}

impl AppState {
    pub fn new(theme: Signal<Theme>, lang: Signal<Language>) -> Self {
        Self { theme, lang }
    }
}
