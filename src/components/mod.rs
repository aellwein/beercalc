//! This module contains all the components and calculators.
mod alcohol;
mod footer;
mod gravity_picker;
mod header;
mod ibu;
mod lang;
mod show_units;
mod theme;
mod unit_picker;

pub mod prelude {
    pub use super::alcohol::AlcoholCalculator;
    pub use super::footer::Footer;
    pub use super::gravity_picker::GravityPicker;
    pub use super::header::Header;
    pub use super::ibu::IbuCalculator;
    pub use super::lang::LanguageSwitcher;
    pub use super::show_units::ShowUnits;
    pub use super::theme::ThemeSwitcher;
    pub use super::unit_picker::GravityUnitPicker;
}
