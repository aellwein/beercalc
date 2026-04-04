use crate::common::prelude::*;
use crate::components::lang;
use crate::components::prelude::*;
use crate::STATE;
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;
use dioxus_sdk_storage::{use_synced_storage, LocalStorage};

#[component]
pub fn IbuCalculator() -> Element {
    rsx! {
        div {
            Header { active: Route::IbuCalculator }
            div {}
        }
    }
}
