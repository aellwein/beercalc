use crate::common::prelude::*;
use crate::components::prelude::*;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn Header(active: Route) -> Element {
    let link_base = "whitespace-nowrap rounded-full px-3 py-1.5 text-sm transition-colors";
    let active_class = format!("{link_base} {LT_ANCHOR_CLASSES_ACTIVE}");
    let inactive_class = format!("{link_base} text-muted hover:bg-line hover:text-ink");
    rsx! {
        header { class: "flex flex-col gap-3 border-b border-line pb-4",
            div { class: "flex flex-row flex-wrap items-center justify-between gap-3",
                span { class: "text-lg font-semibold tracking-tight text-ink", {t!("title")} }
                div { class: "flex flex-row items-center gap-2",
                    LanguageSwitcher {}
                    ThemeSwitcher {}
                }
            }
            nav { class: "flex flex-row flex-wrap gap-2",
                {
                    CALCULATORS
                        .iter()
                        .map(|(route, key)| {
                            rsx! {
                                Link {
                                    to: route.clone(),
                                    class: if *route == active { active_class.as_str() } else { inactive_class.as_str() },
                                    key: "{route}",
                                    {t!(key)}
                                }
                            }
                        })
                }
            }
        }
    }
}
