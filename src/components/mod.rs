//! This module contains all the components and calculators.
pub mod alcohol_calc;
pub mod app;
pub mod brewhouse;
pub mod ebc_calc;
pub mod footer;
pub mod gravity_picker;
pub mod header;
pub mod ibu_calc;
pub mod lang_picker;
pub mod not_found;
pub mod show_units;
pub mod unit_picker;

pub mod prelude {
    pub use super::alcohol_calc::AlcoholCalculator;
    pub use super::app::{App, AppProps};
    pub use super::brewhouse::BrewhouseEfficiencyCalculator;
    pub use super::ebc_calc::EbcCalculator;
    pub use super::footer::Footer;
    pub use super::gravity_picker::GravityPicker;
    pub use super::header::Header;
    pub use super::ibu_calc::IBUCalculator;
    pub use super::lang_picker::LanguagePicker;
    pub use super::not_found::NotFound;
    pub use super::show_units::ShowUnits;
    pub use super::unit_picker::UnitPicker;
}
