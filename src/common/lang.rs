use serde::{Deserialize, Serialize};
use unic_langid::{LanguageIdentifier, langid};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Language {
    English,
    German,
    Russian,
}

pub const ALL_LANGUAGES: [Language; 3] = [Language::English, Language::German, Language::Russian];

#[derive(Clone, PartialEq)]
pub struct LanguageIndicator(pub String);

#[derive(Clone, PartialEq)]
pub struct LanguageShort(pub String);

impl From<&Language> for LanguageShort {
    fn from(lang: &Language) -> Self {
        match *lang {
            Language::English => LanguageShort("EN".into()),
            Language::German => LanguageShort("DE".into()),
            Language::Russian => LanguageShort("RU".into()),
        }
    }
}
impl From<&str> for Language {
    fn from(short: &str) -> Self {
        match short.to_lowercase().as_str() {
            "en" => Language::English,
            "de" => Language::German,
            "ru" => Language::Russian,
            _ => Language::English,
        }
    }
}

impl From<&Language> for LanguageIndicator {
    fn from(lang: &Language) -> Self {
        match *lang {
            Language::English => LanguageIndicator("🇬🇧".into()),
            Language::German => LanguageIndicator("🇩🇪".into()),
            Language::Russian => LanguageIndicator("🇷🇺".into()),
        }
    }
}

impl From<Language> for LanguageIdentifier {
    fn from(lang: Language) -> Self {
        match lang {
            Language::English => langid!("en"),
            Language::German => langid!("de"),
            Language::Russian => langid!("ru"),
        }
    }
}

impl TryInto<Language> for String {
    type Error = Box<dyn std::error::Error>;

    fn try_into(self) -> std::result::Result<Language, Self::Error> {
        match self.to_lowercase().as_str() {
            "en" => Ok(Language::English),
            "de" => Ok(Language::German),
            "ru" => Ok(Language::Russian),
            _ => Err("Unknown language".into()),
        }
    }
}

pub fn get_lang_list(selected_lang: Language) -> Vec<(LanguageIndicator, LanguageShort, bool)> {
    ALL_LANGUAGES
        .iter()
        .map(|lang| {
            let indicator: LanguageIndicator = lang.into();
            let short: LanguageShort = lang.into();
            (indicator, short, *lang == selected_lang)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_codes_map_to_languages() {
        assert_eq!(Language::from("en"), Language::English);
        assert_eq!(Language::from("DE"), Language::German);
        assert_eq!(Language::from("ru"), Language::Russian);
    }

    #[test]
    fn unknown_code_falls_back_to_english() {
        assert_eq!(Language::from("fr"), Language::English);
        assert_eq!(Language::from(""), Language::English);
    }

    #[test]
    fn string_parsing_rejects_unknown_codes() {
        let parsed: std::result::Result<Language, _> = "de".to_string().try_into();
        assert_eq!(parsed.unwrap(), Language::German);
        let rejected: std::result::Result<Language, _> = "fr".to_string().try_into();
        assert!(rejected.is_err());
    }

    #[test]
    fn short_name_is_upper_case() {
        let short: LanguageShort = (&Language::German).into();
        assert_eq!(short.0, "DE");
    }

    #[test]
    fn language_list_marks_only_the_selected_one() {
        let list = get_lang_list(Language::Russian);
        assert_eq!(list.len(), ALL_LANGUAGES.len());
        let selected: Vec<_> = list.iter().filter(|(_, _, s)| *s).collect();
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].1.0, "RU");
    }
}
