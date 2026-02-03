mod alcohol;
mod brewhouse;
mod color;
mod ebc;
mod header;
mod ibu;
mod lang;
mod theme;

pub mod prelude {
    pub use crate::components::alcohol::AlcoholCalc;
    pub use crate::components::brewhouse::BrewhouseCalc;
    pub use crate::components::color::{ColorCalc, ColorConversionCalc};
    pub use crate::components::ebc::EbcCalc;
    pub use crate::components::header::Header;
    pub use crate::components::ibu::IBUCalc;
    pub use crate::components::theme::{set_theme, ThemeSwitcher};
}
