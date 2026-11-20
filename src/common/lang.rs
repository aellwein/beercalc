use serde::{Deserialize, Serialize};
use unic_langid::{LanguageIdentifier, langid};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Language {
    English,
    German,
    Russian,
    Italian,
    French,
    Spanish,
    Romanian,
    Croatian,
    Serbian,
    Greek,
    Japanese,
    Chinese,
}

pub const ALL_LANGUAGES: [Language; 12] = [
    Language::English,
    Language::German,
    Language::Russian,
    Language::Italian,
    Language::French,
    Language::Spanish,
    Language::Romanian,
    Language::Croatian,
    Language::Serbian,
    Language::Greek,
    Language::Japanese,
    Language::Chinese,
];

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
            Language::Italian => LanguageShort("IT".into()),
            Language::French => LanguageShort("FR".into()),
            Language::Spanish => LanguageShort("ES".into()),
            Language::Romanian => LanguageShort("RO".into()),
            Language::Croatian => LanguageShort("HR".into()),
            Language::Serbian => LanguageShort("SR".into()),
            Language::Greek => LanguageShort("EL".into()),
            Language::Japanese => LanguageShort("JA".into()),
            Language::Chinese => LanguageShort("ZH".into()),
        }
    }
}
impl From<&str> for Language {
    fn from(short: &str) -> Self {
        match short.to_lowercase().as_str() {
            "en" => Language::English,
            "de" => Language::German,
            "ru" => Language::Russian,
            "it" => Language::Italian,
            "fr" => Language::French,
            "es" => Language::Spanish,
            "ro" => Language::Romanian,
            "hr" => Language::Croatian,
            "sr" => Language::Serbian,
            "el" => Language::Greek,
            "ja" => Language::Japanese,
            "zh" => Language::Chinese,
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
            Language::Italian => LanguageIndicator("🇮🇹".into()),
            Language::French => LanguageIndicator("🇫🇷".into()),
            Language::Spanish => LanguageIndicator("🇪🇸".into()),
            Language::Romanian => LanguageIndicator("🇷🇴".into()),
            Language::Croatian => LanguageIndicator("🇭🇷".into()),
            Language::Serbian => LanguageIndicator("🇷🇸".into()),
            Language::Greek => LanguageIndicator("🇬🇷".into()),
            Language::Japanese => LanguageIndicator("🇯🇵".into()),
            Language::Chinese => LanguageIndicator("🇨🇳".into()),
        }
    }
}

impl From<Language> for LanguageIdentifier {
    fn from(lang: Language) -> Self {
        match lang {
            Language::English => langid!("en"),
            Language::German => langid!("de"),
            Language::Russian => langid!("ru"),
            Language::Italian => langid!("it"),
            Language::French => langid!("fr"),
            Language::Spanish => langid!("es"),
            Language::Romanian => langid!("ro"),
            Language::Croatian => langid!("hr"),
            Language::Serbian => langid!("sr"),
            Language::Greek => langid!("el"),
            Language::Japanese => langid!("ja"),
            Language::Chinese => langid!("zh"),
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
            "it" => Ok(Language::Italian),
            "fr" => Ok(Language::French),
            "es" => Ok(Language::Spanish),
            "ro" => Ok(Language::Romanian),
            "hr" => Ok(Language::Croatian),
            "sr" => Ok(Language::Serbian),
            "el" => Ok(Language::Greek),
            "ja" => Ok(Language::Japanese),
            "zh" => Ok(Language::Chinese),
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
        assert_eq!(Language::from("zh"), Language::Chinese);
    }

    #[test]
    fn unknown_code_falls_back_to_english() {
        assert_eq!(Language::from("xx"), Language::English);
        assert_eq!(Language::from(""), Language::English);
    }

    #[test]
    fn string_parsing_rejects_unknown_codes() {
        let parsed: std::result::Result<Language, _> = "de".to_string().try_into();
        assert_eq!(parsed.unwrap(), Language::German);
        let rejected: std::result::Result<Language, _> = "xx".to_string().try_into();
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
