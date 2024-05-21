use crate::common::prelude::*;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ShowUnitsProps {
    pub gravity: Gravity,
}

#[function_component]
pub fn ShowUnits(props: &ShowUnitsProps) -> Html {
    let t = use_context::<Translator>().unwrap();
    let lang = use_store::<CalcState>().0.language.clone();
    let g = props.gravity.clone();
    match props.gravity {
        Gravity::Plato(v) => html! {
            <span>{format!("{:.1} {} / {:.1} {} / {:.1} {} / {:.3} {}",
                v, t.t("plato", &lang),
                g.to_brix().value(), t.t("brix", &lang),
                g.to_oechsle().value(), t.t("oechsle", &lang),
                g.to_sg().value(), t.t("sg", &lang),
            )}</span>
        },
        Gravity::Brix(v) => html! {
            <span>{format!("{:.1} {} / {:.1} {} / {:.1} {} / {:.3} {}",
                v, t.t("brix", &lang),
                g.to_plato().value(), t.t("plato", &lang),
                g.to_oechsle().value(), t.t("oechsle", &lang),
                g.to_sg().value(), t.t("sg", &lang),
            )}</span>
        },
        Gravity::SG(v) => html! {
            <span>{format!("{:.3} {} / {:.1} {} / {:.1} {} / {:.1} {}",
            v, t.t("sg", &lang),
            g.to_brix().value(), t.t("brix", &lang),
            g.to_plato().value(), t.t("plato", &lang),
            g.to_oechsle().value(), t.t("oechsle", &lang),
            )}</span>
        },
        Gravity::Oechsle(v) => html! {
            <span>{format!("{:.1} {} / {:.1} {} / {:.1} {} / {:.3} {}",
                v, t.t("oechsle", &lang),
                g.to_brix().value(), t.t("brix", &lang),
                g.to_plato().value(), t.t("plato", &lang),
                g.to_sg().value(), t.t("sg", &lang),
            )}</span>
        },
    }
}
