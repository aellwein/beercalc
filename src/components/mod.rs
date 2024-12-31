//! This module contains all the components and calculators.
mod alcohol_calc;
mod app;
mod brewhouse;
mod ebc_calc;
mod footer;
mod gravity_picker;
mod header;
mod hops_adder;
mod ibu_calc;
mod ibu_preset;
mod lang_picker;
mod not_found;
mod show_units;
mod unit_picker;

pub mod prelude {
    pub use super::alcohol_calc::AlcoholCalculator;
    pub use super::app::{App, AppProps};
    pub use super::brewhouse::BrewhouseEfficiencyCalculator;
    pub use super::ebc_calc::EbcCalculator;
    pub use super::footer::Footer;
    pub use super::gravity_picker::{change_og_value, GravityPicker};
    pub use super::header::Header;
    pub use super::hops_adder::HopsAdder;
    pub use super::ibu_calc::IBUCalculator;
    pub use super::ibu_preset::IBUPreset;
    pub use super::lang_picker::LanguagePicker;
    pub use super::not_found::NotFound;
    pub use super::show_units::ShowUnits;
    pub use super::unit_picker::UnitPicker;
}
