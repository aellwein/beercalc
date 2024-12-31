use super::utils::{get_base_href, window};
use crate::Language;
use anyhow::Result;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

#[derive(PartialEq, Clone)]
pub struct Translator {
    translations: HashMap<String, HashMap<String, String>>,
}

impl Translator {
    pub async fn new() -> Result<Self> {
        let url = get_base_href() + "i18n/i18n.yaml";
        let translations = parse_yaml(load_table_from_yaml(url.as_str()).await)
            .await
            .unwrap();
        Ok(Translator { translations })
    }
    /// Translate a key into the current language.
    pub fn t(&self, key: &str, lang: &Language) -> String {
        let l: &'static str = From::<&Language>::from(lang);
        let translations = self.translations.get(key);
        if translations.is_none() {
            return format!("_(\"{}\")", key);
        }
        match translations.unwrap().get(l) {
            Some(v) => v.clone(),
            None => format!("_(\"{}\")", key),
        }
    }

    /// Get preferred language from the browser, falling back to English.
    pub fn get_preferred_language() -> Language {
        let navigator = window().navigator();
        let languages: Vec<Language> = navigator
            .languages()
            .iter()
            .map(|j| {
                let s_lang =
                    j.as_string().unwrap().split("-").collect::<Vec<&str>>()[0].to_string();
                let r = s_lang.try_into();
                match r {
                    Ok(lang) => lang,
                    Err(_) => Language::English,
                }
            })
            .collect();
        //console::log_1(&format!("Languages: {:?}", languages).into());
        if !languages.is_empty() {
            languages[0].clone()
        } else {
            Language::English
        }
    }
}

async fn load_table_from_yaml(url: &str) -> String {
    let resp: Response = JsFuture::from(window().fetch_with_str(url))
        .await
        .unwrap()
        .dyn_into()
        .unwrap();
    JsFuture::from(resp.text().unwrap())
        .await
        .unwrap()
        .as_string()
        .unwrap()
}

async fn parse_yaml(s: String) -> Result<HashMap<String, HashMap<String, String>>> {
    let s1 = s.clone();
    let s1s = s1.as_str();
    let result = serde_yaml::from_str(s1s)?;
    Ok(result)
}
