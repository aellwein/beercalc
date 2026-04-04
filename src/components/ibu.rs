use crate::common::prelude::*;
use crate::components::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn IbuCalculator() -> Element {
    rsx! {
        div {
            Header { active: Route::IbuCalculator }
            div {}
        }
    }
}
