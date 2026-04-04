use crate::common::prelude::*;
use crate::components::lang;
use crate::components::prelude::*;
use crate::STATE;
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;
use dioxus_sdk_storage::{use_synced_storage, LocalStorage};

#[component]
pub fn Footer() -> Element {
    let commit_link = format!(
        "https://github.com/aellwein/beercalc/commit/{}",
        STATE.read().commit_hash
    );
    rsx! {
        div { class: "grow flex flex-row flex-wrap mx-auto gap-4 max-md:gap-2 mt-1",
            div {
                span { " " }
                a {
                    class: "hover:underline text-indigo-500",
                    href: "https://github.com/aellwein/beercalc",
                    {t!("project_on_github")}
                }
            }
            div {

                span { "📝 " }
                a { class: "hover:underline text-indigo-500", href: "#/docs", "Docs" }
            }
            div {
                span { " " }
                a {
                    class: "hover:underline text-indigo-500",
                    href: commit_link,
                    {format!("{}", STATE.read().commit_hash)}
                }
            }
            div {
                span { "󰿃 " }
                a {
                    class: "hover:underline text-indigo-500",
                    href: "https://github.com/aellwein/beercalc/blob/main/LICENSE",
                    {format!("{} {}", t!("licensed_under"), t!("mit_license"))}
                }
            }
            div {
                span { "🛠️ " }
                a {
                    class: "hover:underline text-indigo-500",
                    href: "https://github.com/DioxusLabs/dioxus",
                    {t!("made_with_love")}
                }
            }
        }
    }
}
