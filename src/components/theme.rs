use crate::{STATE, common::prelude::*};
use dioxus::prelude::*;
use dioxus_i18n::t;

/// Theme picker: "System" (Nord Light or Nord Dark, by the browser's preference) followed by
/// every built-in theme. The choice is stored in `CalcState` and applied immediately.
#[component]
pub fn ThemeSwitcher() -> Element {
    let current = STATE.read().theme.clone();
    apply(&current);
    let selected_value = if current == SYSTEM {
        SYSTEM.to_string()
    } else {
        resolve(&current, prefers_dark()).id
    };
    let system_label = t!("theme_system");
    rsx! {
        select {
            class: SELECT_CLASS,
            title: t!("theme"),
            aria_label: t!("theme"),
            onchange: move |evt| {
                let id = evt.value();
                if !is_known(&id) {
                    return;
                }
                let mut new_state = STATE.read().clone();
                new_state.theme = id.clone();
                *STATE.write() = new_state;
                apply(&id);
            },
            option {
                value: SYSTEM,
                selected: selected_value == SYSTEM,
                "{system_label}"
            }
            {
                builtin_themes()
                    .into_iter()
                    .map(|theme| {
                        let selected = theme.id == selected_value;
                        rsx! {
                            option {
                                value: "{theme.id}",
                                selected,
                                "{theme.name}"
                            }
                        }
                    })
            }
        }
    }
}
