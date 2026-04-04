use crate::common::prelude::*;
use crate::components::prelude::*;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn Header(active: Route) -> Element {
    let active_classes = format!(
        "nav-link {} {} pb-1",
        LT_ANCHOR_CLASSES_ACTIVE, DT_ANCHOR_CLASSES_ACTIVE
    );
    let inactive_classes = format!(
        "nav-link {} {} flex-shrink-0",
        LT_ANCHOR_CLASSES, DT_ANCHOR_CLASSES
    );
    rsx! {
        div { class: "flex flex-row flex-wrap gap-4 xs:gap-2 items-baseline",
            {
                CALCULATORS
                    .iter()
                    .map(|(route, key)| {
                        rsx! {
                            Link {
                                to: route.clone(),
                                class: if *route == active { active_classes.as_str() } else { inactive_classes.as_str() },
                                key: "{route}",
                                {t!(key)}
                            }
                        }
                    })
            }
            div { class: "grow shrink-0",
                div { class: "flex flex-row gap-4 items-center justify-end",
                    LanguageSwitcher {}
                    ThemeSwitcher {}
                }
            }
        }
    }
}
