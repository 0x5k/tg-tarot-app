use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Translations {
    pub language: String,
    pub buttons: ButtonTranslations,
    pub spreads: SpreadTranslations,
    pub empty: EmptyTranslations,
    pub feedback: FeedbackTranslations,
    pub orientation: OrientationTranslations,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CardTranslation {
    #[serde(default)]
    pub name: Option<String>,
    pub upright: String,
    pub reversed: String,
    pub keywords: Vec<String>,
}

pub type CardTranslations = HashMap<String, CardTranslation>;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ButtonTranslations {
    pub draw: String,
    pub copy: String,
    pub language: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SpreadTranslations {
    pub single: SpreadOption,
    pub three: SpreadOption,
    pub five: SpreadOption,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SpreadOption {
    pub label: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct EmptyTranslations {
    pub title: String,
    pub subtitle: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FeedbackTranslations {
    pub cleared: String,
    pub copied: String,
    pub draw_first: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OrientationTranslations {
    pub upright: String,
    pub reversed: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Ukrainian,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Ukrainian => "ua",
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code {
            "ua" | "uk" => Language::Ukrainian,
            _ => Language::English,
        }
    }

    pub fn load_translations(&self) -> Translations {
        let json = match self {
            Language::English => include_str!("../translations/en.json"),
            Language::Ukrainian => include_str!("../translations/ua.json"),
        };

        serde_json::from_str(json).unwrap_or_else(|_| {
            // Fallback to English if parsing fails
            serde_json::from_str(include_str!("../translations/en.json"))
                .expect("English translations must be valid")
        })
    }

    pub fn load_card_translations(&self) -> CardTranslations {
        let json = match self {
            Language::English => include_str!("../translations/cards_en.json"),
            Language::Ukrainian => include_str!("../translations/cards_ua.json"),
        };

        serde_json::from_str(json).unwrap_or_else(|_| {
            // Fallback to English if parsing fails
            serde_json::from_str(include_str!("../translations/cards_en.json"))
                .expect("English card translations must be valid")
        })
    }

    pub fn toggle(&self) -> Self {
        match self {
            Language::English => Language::Ukrainian,
            Language::Ukrainian => Language::English,
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

// Helper function to format copied message
pub fn format_copied_message(t: &Translations, count: usize) -> String {
    let plural = if count == 1 { "" } else { "s" };
    t.feedback.copied
        .replace("{count}", &count.to_string())
        .replace("{plural}", plural)
}
