use unic_langid::{langid, LanguageIdentifier};

#[derive(Clone, Copy, PartialEq, Debug)]
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
