use crate::common::prelude::*;
use crate::components::prelude::*;
use enum_iterator::all;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub active: Route,
}

#[function_component]
pub fn Header(props: &HeaderProps) -> Html {
    let t = use_context::<Translator>().unwrap();
    let (state, _) = use_store::<CalcState>();
    let lang = &state.language;
    let active_classes = "nav-link text-indigo-600 dark:text-indigo-500";
    let inactive_classes =
        "nav-link hover:underline flex-shrink-0 text-gray-500 dark:text-gray-400";
    let items = all::<Calculator>()
        .map(|c| {
            match c {
                Calculator::Alcohol => html! {
                    <Link<Route> to={Route::AlcoholCalculator}><a class={if props.active == Route::AlcoholCalculator {active_classes} else {inactive_classes}}>{t.t("alcohol calculator", lang)}</a></Link<Route>>
                },
                Calculator::Ibu => html! {
                    <Link<Route> to={Route::IbuCalculator}><a class={if props.active == Route::IbuCalculator {active_classes} else {inactive_classes}}>{t.t("ibu calculator", lang)}</a></Link<Route>>
                },
                Calculator::EbcColor =>html! {
                    <Link<Route> to={Route::BeerColorCalculator}><a class={if props.active == Route::BeerColorCalculator {active_classes} else {inactive_classes}}>{t.t("color calculator", lang)}</a></Link<Route>>
                },
                Calculator::BrewhouseEfficiency => html! {
                    <Link<Route> to={Route::BrewhouseEfficiencyCalculator}><a class={if props.active == Route::BrewhouseEfficiencyCalculator {active_classes} else {inactive_classes}}>{t.t("brewhouse calculator", lang)}</a></Link<Route>>
                },
            }
        })
        .collect::<Vec<_>>();
    html! {
            <div class="flex flex-row flex-wrap gap-4 xs:gap-2 items-baseline">
                {items}
                <div class="flex-grow flex-shrink-0">
                    <div class="flex flex-row gap-4 items-center justify-end">
                        <LanguagePicker selected={state.language.clone()} />
                        <ThemeSwitcher theme={state.theme.clone()}/>
                    </div>
                </div>
            </div>
    }
}
