//! Common features for the calculator.
mod calculus;
mod gravity;
mod lang;
mod route;
mod state;
mod theme;
mod translator;
mod utils;

pub mod prelude {
    pub use super::calculus::*;
    pub use super::gravity::Gravity;
    pub use super::lang::Language;
    pub use super::route::{Calculator, Route};
    pub use super::state::{CalcState, Hops, HopsForm, Ibu};
    pub use super::theme::{Theme, ThemeSwitcher};
    pub use super::translator::Translator;
    pub use super::utils::{
        format_gravity, get_preferred_theme, set_body_classes, set_page_title, set_theme_classes,
    };
}
