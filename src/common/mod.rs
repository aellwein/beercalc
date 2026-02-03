mod lang;
mod state;
mod theme;
mod util;

pub mod prelude {
    pub use crate::common::lang::{get_lang_list, Language};
    pub use crate::common::state::AppState;
    pub use crate::common::theme::Theme;
    pub use crate::common::util::{
        get_preferred_language, get_preferred_theme, remove_classes, set_classes,
    };
}
