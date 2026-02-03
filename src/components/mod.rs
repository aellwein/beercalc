mod alcohol;
mod brewhouse;
mod color;
mod ebc;
mod gravity_picker;
mod header;
mod ibu;
mod lang;
mod theme;
mod unit_picker;

pub mod prelude {
    pub use crate::components::alcohol::AlcoholCalc;
    // pub use crate::components::brewhouse::BrewhouseCalc;
    // pub use crate::components::color::{ColorCalc, ColorConversionCalc};
    // pub use crate::components::ebc::EbcCalc;
    pub use crate::components::header::Header;
    pub use crate::components::unit_picker::UnitPicker;
    // pub use crate::components::ibu::IBUCalc;
    pub use crate::components::gravity_picker::GravityPicker;
    pub use crate::components::lang::LanguageSwitcher;
    pub use crate::components::theme::{set_theme, ThemeSwitcher};
}
