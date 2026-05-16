use crate::common::prelude::*;
use crate::STATE;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn Footer() -> Element {
    let commit_link = format!(
        "https://github.com/aellwein/beercalc/commit/{}",
        STATE.read().commit_hash
    );
    rsx! {
        footer { class: "my-6 text-center dark:text-gray-400",
            a {
                class: "{LT_ANCHOR_CLASSES} {DT_ANCHOR_CLASSES}",
                href: "https://github.com/aellwein/beercalc",
                {format!(" {}", t!("project_on_github"))}
            }
            {" | "}
            span {
                " "
                a {
                    class: "{LT_ANCHOR_CLASSES} {DT_ANCHOR_CLASSES}",
                    href: commit_link,
                    {STATE.read().commit_hash.to_string()}
                }
            }
            {" | "}
            {"󰿃 "}
            {t!("licensed_under")}
            {" "}
            a {
                class: "{LT_ANCHOR_CLASSES} {DT_ANCHOR_CLASSES}",
                href: "https://github.com/aellwein/beercalc/blob/main/LICENSE",
                {t!("mit_license")}
            }
            {format!(" | 🛠️ {}", t!("made_with_love"))}
        }
    }
}
