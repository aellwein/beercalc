use crate::common::prelude::*;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn ShowUnits(gravity: Gravity) -> Element {
    match gravity {
        Gravity::Plato(v) => rsx! {
            span {
                {
                    format!(
                        "{:.1} {} / {:.1} {} / {:.1} {} / {:.3} {}",
                        v,
                        t!("plato"),
                        gravity.to_brix().value(),
                        t!("brix"),
                        gravity.to_oechsle().value(),
                        t!("oechsle"),
                        gravity.to_sg().value(),
                        t!("sg"),
                    )
                }
            }
        },
        Gravity::Brix(v) => rsx! {
            span {
                {
                    format!(
                        "{:.1} {} / {:.1} {} / {:.1} {} / {:.3} {}",
                        v,
                        t!("brix"),
                        gravity.to_plato().value(),
                        t!("plato"),
                        gravity.to_oechsle().value(),
                        t!("oechsle"),
                        gravity.to_sg().value(),
                        t!("sg"),
                    )
                }
            }
        },
        _ => rsx! {
            div {}
        },
    }
}
