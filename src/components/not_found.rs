use crate::common::prelude::*;
use yew::prelude::*;
use yew_router::prelude::Link;
use yewdux::prelude::*;

#[function_component]
pub fn NotFound() -> Html {
    let t = use_context::<Translator>().unwrap();
    let (state, _) = use_store::<CalcState>();
    let lang = &state.language;
    html! {
        <div class="flex flex-col items-center justify-center text-center min-h-screen">
            <div class="text-lg">{t.t("page_not_found", lang)}</div>
            <Link<Route> to={Route::Home}><a class="underline" href="#">{t.t("go_back", lang) }</a></Link<Route>>
        </div>
    }
}