use crate::common::prelude::*;
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_sdk_storage::{use_synced_storage, LocalStorage};

#[component]
pub fn LanguageSwitcher() -> Element {
    let mut state =
        use_synced_storage::<LocalStorage, AppState>(STORAGE_KEY.to_string(), AppState::default);
    let language = state.read().language;
    let lang_list = get_lang_list(language);
    let mut i18n = i18n();
    i18n.set_language(language.into());

    return rsx! {
        select {
            class: "p-2 rounded-full bg-white hover:bg-neutral-200 dark:bg-neutral-800 dark:hover:bg-neutral-700 hover:cursor-pointer grow-0 appearance-none",
            onchange: move |evt| {
                let selected_value = evt.value().clone();
                let selected_lang: Language = selected_value.as_str().into();
                let mut new_state = state.read().clone();
                new_state.language = selected_lang;
                state.set(new_state);
                i18n.set_language(selected_lang.into());
                debug!("Language switched to {:?}", selected_lang);
            },
            {lang_list.iter().map(|(indicator, short, is_selected)| rsx! {
                option { value: "{short.0}", selected: "{is_selected}", "{indicator.0} {short.0}" }
            })}
        }
    };
}
