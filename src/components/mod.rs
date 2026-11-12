//! This module contains all the components and calculators.
mod alcohol;
mod brewhouse;
mod carbonation;
mod color;
mod color_conversion;
mod footer;
mod form;
mod gravity_picker;
mod header;
mod ibu;
mod lang;
mod show_units;
mod theme;
mod unit_picker;
mod water;

pub mod prelude {
    pub use super::alcohol::AlcoholCalculator;
    pub use super::brewhouse::BrewhouseEfficiencyCalculator;
    pub use super::carbonation::CarbonationCalculator;
    pub use super::color::{BeerColorCalculator, EbcColorDisplay};
    pub use super::color_conversion::ColorConversionCalculator;
    pub use super::footer::Footer;
    pub use super::form::number_field;
    pub use super::gravity_picker::GravityPicker;
    pub use super::header::Header;
    pub use super::ibu::IbuCalculator;
    pub use super::lang::LanguageSwitcher;
    pub use super::show_units::ShowUnits;
    pub use super::theme::ThemeSwitcher;
    pub use super::unit_picker::GravityUnitPicker;
    pub use super::water::WaterCalculator;
}
