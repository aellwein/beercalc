//! Colour themes. A theme is a set of colour values for the semantic tokens below. The app
//! writes them as CSS variables (`--c-<token>`), so components only use `bg-card`, `text-ink`, ...
//!
//! Adding a theme: write a `ThemeInfo` (for example with [`theme`] or [`popular`]) and add it to
//! [`builtin_themes`]. Nothing else changes; the tests check that the theme is complete.
//! The themes and their colours follow the t-rex web app, see `popular.rs` for the source.

use std::collections::BTreeMap;

use crate::common::prelude::*;
use web_sys::wasm_bindgen::JsCast;

mod popular;

pub const LT_TEXT_CLASSES: &str = "bg-page text-ink";
pub const DT_TEXT_CLASSES: &str = "";
pub const LT_ANCHOR_CLASSES: &str = "text-accent hover:underline";
pub const DT_ANCHOR_CLASSES: &str = "";
pub const LT_ANCHOR_CLASSES_ACTIVE: &str = "bg-accent text-accent-ink";

/// Surface of a calculator section: rounded card with a border and a soft shadow.
pub const CARD_CLASS: &str = "rounded-2xl border border-line bg-card p-5 shadow-sm";
/// Number and text inputs.
pub const INPUT_CLASS: &str = "rounded-md border border-line bg-field px-2 py-1 text-ink outline-none focus:border-accent focus:ring-2 focus:ring-accent/30";
/// Dropdowns.
pub const SELECT_CLASS: &str = "rounded-md border border-line bg-field px-2 py-1 text-ink outline-none appearance-none focus:border-accent focus:ring-2 focus:ring-accent/30";
/// Main action, e.g. "add hops".
pub const BUTTON_PRIMARY_CLASS: &str = "rounded-full bg-accent px-4 py-2 text-sm font-medium text-accent-ink shadow-sm hover:opacity-90";
/// Destructive small action, e.g. removing a hop addition.
pub const BUTTON_DANGER_CLASS: &str =
    "rounded-full bg-danger px-3 py-1 text-sm font-medium text-white shadow-sm hover:opacity-90";
/// Page heading.
pub const TITLE_CLASS: &str = "text-3xl font-semibold tracking-tight text-ink";
/// Section heading inside a card.
pub const SECTION_TITLE_CLASS: &str = "text-lg font-semibold text-ink";

/// Stored theme value that follows the browser's colour scheme (Nord Light or Nord Dark).
pub const SYSTEM: &str = "system";

/// The semantic colour tokens every theme sets, in the order [`ThemeInfo::colors`] is built from.
pub const TOKENS: [&str; 9] = [
    "page",
    "card",
    "field",
    "line",
    "ink",
    "muted",
    "accent",
    "accent-ink",
    "danger",
];

/// A colour theme: id, name, whether it is dark, and a `#rrggbb` value per token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeInfo {
    /// Id stored in the app state, for example `nord-dark`.
    pub id: String,
    /// Name shown in the theme picker.
    pub name: String,
    /// Dark themes also set the `dark` class, so `dark:` variants work.
    pub dark: bool,
    /// Colour per token, see [`TOKENS`].
    pub colors: BTreeMap<String, String>,
}

impl ThemeInfo {
    /// Checks that every token has a `#rrggbb` colour and that there are no unknown tokens.
    pub fn validate(&self) -> Result<(), String> {
        if self.id.trim().is_empty() || self.name.trim().is_empty() {
            return Err("a theme needs an id and a name".into());
        }
        for token in TOKENS {
            match self.colors.get(token) {
                None => return Err(format!("theme '{}' has no colour for '{token}'", self.id)),
                Some(value) if !is_hex_colour(value) => {
                    return Err(format!("'{value}' is not a #rrggbb colour"));
                }
                Some(_) => {}
            }
        }
        if let Some(unknown) = self
            .colors
            .keys()
            .find(|key| !TOKENS.contains(&key.as_str()))
        {
            return Err(format!("unknown colour token '{unknown}'"));
        }
        Ok(())
    }
}

/// Whether a value is a `#rrggbb` colour. Hex digits may be upper or lower case.
fn is_hex_colour(value: &str) -> bool {
    value.len() == 7 && value.starts_with('#') && value[1..].chars().all(|c| c.is_ascii_hexdigit())
}

/// Builds a theme from its colours, given in the order of [`TOKENS`].
fn theme(id: &str, name: &str, dark: bool, colors: [&str; 9]) -> ThemeInfo {
    ThemeInfo {
        id: id.into(),
        name: name.into(),
        dark,
        colors: TOKENS
            .iter()
            .zip(colors)
            .map(|(token, value)| (token.to_string(), value.to_string()))
            .collect(),
    }
}

/// Maps the names stored before theme ids existed (`Light`, `Dark`, `light`, `dark`) onto ids.
fn alias(id: &str) -> &str {
    match id {
        "light" | "Light" => "nord-light",
        "dark" | "Dark" => "nord-dark",
        other => other,
    }
}

/// Whether a stored value is allowed: [`SYSTEM`] or the id of a built-in theme.
pub fn is_known(id: &str) -> bool {
    id == SYSTEM || builtin_themes().iter().any(|theme| theme.id == id)
}

/// The theme to show for a stored value. [`SYSTEM`] picks Nord Dark or Nord Light by the
/// browser's preference. Unknown values fall back to Nord Light.
pub fn resolve(id: &str, prefers_dark: bool) -> ThemeInfo {
    let themes = builtin_themes();
    let wanted = match alias(id) {
        SYSTEM if prefers_dark => "nord-dark",
        SYSTEM => "nord-light",
        other => other,
    };
    themes
        .iter()
        .find(|theme| theme.id == wanted)
        .cloned()
        .unwrap_or_else(|| themes[0].clone())
}

/// Every built-in theme, in the order the picker lists them.
pub fn builtin_themes() -> Vec<ThemeInfo> {
    let mut themes = vec![nord_light(), nord_dark()];
    themes.extend(CATPPUCCIN.iter().map(catppuccin));
    themes.extend(popular::themes());
    themes
}

/// Nord, light flavour (<https://www.nordtheme.com/docs/colors-and-palettes/>).
fn nord_light() -> ThemeInfo {
    theme(
        "nord-light",
        "Nord Light",
        false,
        [
            "#e5e9f0", // page
            "#d8dee9", // card
            "#eceff4", // field
            "#aeb7c7", // line
            "#2e3440", // ink
            "#6b7487", // muted
            "#5e81ac", // accent
            "#eceff4", // accent-ink
            "#a84e57", // danger
        ],
    )
}

/// Nord, dark flavour.
fn nord_dark() -> ThemeInfo {
    theme(
        "nord-dark",
        "Nord Dark",
        true,
        [
            "#2e3440", // page
            "#3b4252", // card
            "#2e3440", // field
            "#434c5e", // line
            "#d8dee9", // ink
            "#7b8497", // muted
            "#88c0d0", // accent
            "#2e3440", // accent-ink
            "#bf616a", // danger
        ],
    )
}

/// The colours of one Catppuccin flavour, from the official palette
/// (<https://github.com/catppuccin/rust>, palette.json, version 2.8.0).
struct Flavor {
    id: &'static str,
    name: &'static str,
    dark: bool,
    base: &'static str,
    mantle: &'static str,
    surface0: &'static str,
    surface1: &'static str,
    overlay2: &'static str,
    subtext0: &'static str,
    text: &'static str,
    blue: &'static str,
    red: &'static str,
}

/// The four Catppuccin flavours: Latte, Frappé, Macchiato and Mocha.
const CATPPUCCIN: [Flavor; 4] = [
    Flavor {
        id: "catppuccin-latte",
        name: "Catppuccin Latte",
        dark: false,
        base: "#eff1f5",
        mantle: "#e6e9ef",
        surface0: "#ccd0da",
        surface1: "#bcc0cc",
        overlay2: "#7c7f93",
        subtext0: "#6c6f85",
        text: "#4c4f69",
        blue: "#1e66f5",
        red: "#d20f39",
    },
    Flavor {
        id: "catppuccin-frappe",
        name: "Catppuccin Frappé",
        dark: true,
        base: "#303446",
        mantle: "#292c3c",
        surface0: "#414559",
        surface1: "#51576d",
        overlay2: "#949cbb",
        subtext0: "#a5adce",
        text: "#c6d0f5",
        blue: "#8caaee",
        red: "#e78284",
    },
    Flavor {
        id: "catppuccin-macchiato",
        name: "Catppuccin Macchiato",
        dark: true,
        base: "#24273a",
        mantle: "#1e2030",
        surface0: "#363a4f",
        surface1: "#494d64",
        overlay2: "#939ab7",
        subtext0: "#a5adcb",
        text: "#cad3f5",
        blue: "#8aadf4",
        red: "#ed8796",
    },
    Flavor {
        id: "catppuccin-mocha",
        name: "Catppuccin Mocha",
        dark: true,
        base: "#1e1e2e",
        mantle: "#181825",
        surface0: "#313244",
        surface1: "#45475a",
        overlay2: "#9399b2",
        subtext0: "#a6adc8",
        text: "#cdd6f4",
        blue: "#89b4fa",
        red: "#f38ba8",
    },
];

/// Light flavours use the mantle as the page and the base as the card; dark flavours use the
/// base as the page and the surface as the card.
fn catppuccin(flavor: &Flavor) -> ThemeInfo {
    let f = flavor;
    let colors = if f.dark {
        [
            f.base, f.surface0, f.base, f.surface1, f.text, f.overlay2, f.blue, f.base, f.red,
        ]
    } else {
        [
            f.mantle, f.base, f.base, f.surface1, f.text, f.subtext0, f.blue, f.base, f.red,
        ]
    };
    theme(f.id, f.name, f.dark, colors)
}

/// Prefers the browser's colour scheme. `false` when the query is not supported.
pub fn prefers_dark() -> bool {
    window()
        .match_media("(prefers-color-scheme: dark)")
        .ok()
        .flatten()
        .map(|query| query.matches())
        .unwrap_or(false)
}

/// Resolves a stored theme value and applies it to the document.
pub fn apply(id: &str) {
    apply_theme(&resolve(id, prefers_dark()));
}

/// Writes the theme to the document: the `data-theme` attribute, the `dark` class, the
/// `color-scheme`, and one `--c-<token>` variable per colour.
pub fn apply_theme(theme: &ThemeInfo) {
    if let Err(error) = theme.validate() {
        web_sys::console::error_1(&error.into());
        return;
    }
    let root: web_sys::HtmlElement = document()
        .document_element()
        .expect("unable to get document element")
        .dyn_into()
        .expect("document element is not an HTML element");
    root.set_attribute("data-theme", &theme.id)
        .expect("unable to set data-theme");
    if theme.dark {
        set_classes("dark");
    } else {
        remove_classes("dark");
    }
    let style = root.style();
    style
        .set_property("color-scheme", if theme.dark { "dark" } else { "light" })
        .expect("unable to set color-scheme");
    for token in TOKENS {
        if let Some(value) = theme.colors.get(token) {
            style
                .set_property(&format!("--c-{token}"), value)
                .expect("unable to set theme colour");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_builtin_theme_is_complete_and_valid() {
        for theme in builtin_themes() {
            assert_eq!(theme.validate(), Ok(()), "{}", theme.id);
        }
    }

    #[test]
    fn builtin_ids_and_names_are_unique_and_known() {
        let themes = builtin_themes();
        let mut ids: Vec<&str> = themes.iter().map(|t| t.id.as_str()).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), themes.len());
        let mut names: Vec<&str> = themes.iter().map(|t| t.name.as_str()).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), themes.len(), "two themes share a name");
        for theme in &themes {
            assert!(is_known(&theme.id));
        }
        assert!(is_known(SYSTEM));
        assert!(!is_known("neon"));
    }

    #[test]
    fn the_t_rex_theme_list_is_complete() {
        // Nord (2), Catppuccin (4) and the popular themes (15).
        assert_eq!(builtin_themes().len(), 21);
        assert_eq!(builtin_themes()[0].id, "nord-light");
        assert_eq!(builtin_themes()[1].id, "nord-dark");
    }

    #[test]
    fn system_follows_the_browser_between_nord_light_and_dark() {
        assert_eq!(resolve(SYSTEM, true).id, "nord-dark");
        assert_eq!(resolve(SYSTEM, false).id, "nord-light");
    }

    #[test]
    fn legacy_states_and_unknown_values_resolve() {
        assert_eq!(resolve("Light", true).id, "nord-light");
        assert_eq!(resolve("Dark", false).id, "nord-dark");
        assert_eq!(resolve("catppuccin-mocha", false).id, "catppuccin-mocha");
        assert_eq!(resolve("neon", false).id, "nord-light");
    }

    #[test]
    fn dark_flags_match_the_flavours() {
        assert!(!resolve("catppuccin-latte", false).dark);
        assert!(resolve("catppuccin-frappe", false).dark);
        assert!(resolve("catppuccin-macchiato", false).dark);
        assert!(resolve("catppuccin-mocha", false).dark);
    }

    #[test]
    fn invalid_colours_and_missing_tokens_are_rejected() {
        let mut theme = nord_light();
        theme.colors.insert("accent".into(), "blue".into());
        assert!(theme.validate().is_err());

        let mut theme = nord_light();
        theme.colors.remove("danger");
        assert!(theme.validate().is_err());

        let mut theme = nord_light();
        theme.colors.insert("neon".into(), "#00ff00".into());
        assert!(theme.validate().is_err());
    }

    #[test]
    fn stylesheet_declares_a_fallback_for_every_token() {
        let css = include_str!("../../tailwind.css");
        for token in TOKENS {
            assert!(
                css.contains(&format!("--c-{token}:")),
                "tailwind.css has no fallback for {token}"
            );
        }
    }
}
