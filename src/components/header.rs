use crate::common::prelude::*;
use crate::components::prelude::*;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn Header(active: Route, language: Signal<Language>) -> Element {
    let active_classes =
        "nav-link text-indigo-600 dark:text-indigo-500 border-b-2 border-indigo-600 pb-1";
    let inactive_classes =
        "nav-link hover:underline flex-shrink-0 text-gray-500 dark:text-gray-400";
    rsx! {
        div { class: "flex flex-row flex-wrap gap-4 xs:gap-2 items-baseline",
            {
                CALCULATORS
                    .iter()
                    .map(|(route, key)| {
                        rsx! {
                            Link {
                                to: route.clone(),
                                class: if *route == active { active_classes } else { inactive_classes },
                                key: "{route}",
                                {t!(key)}
                            }
                        }
                    })
            }
            div { class: "grow shrink-0",
                div { class: "flex flex-row gap-4 items-center justify-end",
                    LanguageSwitcher { language: *language.read() }
                    ThemeSwitcher {}
                }
            }
        }
    }
}
