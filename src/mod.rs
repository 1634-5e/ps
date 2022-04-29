mod io {
    pub mod dialogs;
    pub mod last_place;

    pub use dialogs::{open, pick, save, PathBuf};
    pub use last_place::*;
}

mod ui {
    pub mod edit;
    mod icons;
    pub mod shape;
    pub mod style;
    pub mod toolbar;
    pub mod utils;
    pub mod viewer;
    pub mod welcome;

    pub use edit::*;
    pub use shape::*;
    pub use style::*;
    pub use toolbar::*;
    pub use viewer::*;
    pub use welcome::welcome;
}