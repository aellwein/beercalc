//! Common features for the calculator.
mod calculus;
mod color;
mod consts;
mod gravity;
mod lang;
mod route;
mod state;
mod theme;
mod utils;

pub mod prelude {
    //! Re-exports of the most commonly used items from the common module.
    pub use super::calculus::*;
    pub use super::color::*;
    pub use super::consts::*;
    pub use super::gravity::*;
    pub use super::lang::*;
    pub use super::route::*;
    pub use super::state::*;
    pub use super::theme::*;
    pub use super::utils::*;
}
