//! Common features for the calculator.
mod calculus;
mod color;
mod gravity;
mod lang;
mod route;
mod state;
mod theme;
mod translator;
mod utils;

pub mod prelude {
    //! Re-exports of the most commonly used items from the common module.
    pub use super::calculus::*;
    pub use super::color::ColorUnit;
    pub use super::gravity::Gravity;
    pub use super::lang::Language;
    pub use super::route::{Calculator, Route};
    pub use super::state::{
        Brewhouse, CalcState, ColorConversion, GrainMassFromBrewhouse, Hops, HopsForm, Ibu, Malt,
        MassUnit, VolumeMeasuredAt,
    };
    pub use super::theme::{Theme, ThemeSwitcher};
    pub use super::translator::Translator;
    pub use super::utils::{
        format_gravity, get_base_href, get_preferred_theme, set_body_classes, set_page_title,
        set_theme_classes,
    };
}
