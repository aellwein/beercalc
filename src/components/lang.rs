use crate::{common::prelude::*, STATE};
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;

#[component]
pub fn LanguageSwitcher() -> Element {
    let language = STATE.read().language;
    let mut i18n = i18n();
    i18n.set_language(language.into());

    return rsx! {
        select {
            class: "p-2 rounded-full bg-white hover:bg-neutral-200 dark:bg-neutral-800 dark:hover:bg-neutral-700 hover:cursor-pointer border border-zinc-500 grow-0 appearance-none",
            onchange: move |evt| {
                let selected_value = evt.value().clone();
                let selected_lang: Language = selected_value.as_str().into();
                let mut new_state = STATE.read().clone();
                new_state.language = selected_lang;
                *STATE.write() = new_state;
                i18n.set_language(selected_lang.into());
                debug!("Language switched to {:?}", selected_lang);
            },
            {
                get_lang_list(STATE.read().language)
                    .iter()
                    .map(|(indicator, short, is_selected)| rsx! {
                        option { value: "{short.0}", selected: "{is_selected}", "{indicator.0} {short.0}" }
                    })
            }
        }
    };
}
