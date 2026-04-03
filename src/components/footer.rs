use crate::common::prelude::*;
use crate::components::lang;
use crate::components::prelude::*;
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;
use dioxus_sdk_storage::{use_synced_storage, LocalStorage};

#[component]
pub fn Footer(hash: String) -> Element {
    rsx! {
        div { class: "grow flex flex-row flex-wrap mx-auto gap-1",
            span {
                {
                    format!(
                        " {} | 📝 Docs |  {} | {} MIT | 🛠️ {}",
                        t!("project_on_github"),
                        hash,
                        t!("licensed_under"),
                        t!("made_with_love"),
                    )
                }
            }
        }
    }
}
