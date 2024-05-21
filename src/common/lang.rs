use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Sequence, Serialize, Deserialize)]
pub enum Language {
    English,
    German,
    Russian,
}

impl From<&Language> for &'static str {
    fn from(value: &Language) -> Self {
        match value {
            Language::English => "en",
            Language::German => "de",
            Language::Russian => "ru",
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

impl From<&Language> for String {
    fn from(value: &Language) -> Self {
        match value {
            Language::English => "en".into(),
            Language::German => "de".into(),
            Language::Russian => "ru".into(),
        }
    }
}

impl Language {
    pub fn as_caption(&self) -> String {
        match self {
            Language::English => "🇬🇧 EN".into(),
            Language::German => "🇩🇪 DE".into(),
            Language::Russian => "🇷🇺 РУ".into(),
        }
    }
}
