//save session when exiting normally and restore it next time.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
enum Error {}

//TODO:Curve这里有点麻烦，暂时放着
#[derive(Serialize, Deserialize, Debug)]
pub struct SavedState {
    pub is_editing: bool,

    //view
    pub images: Vec<PathBuf>,
    pub on_view: Option<usize>,
    //edit
    // pending: Curve,
    // curves: Vec<Curve>,
    // selected_curve: Option<usize>,
}

pub async fn save() {}

pub async fn load() -> Option<SavedState> {
    None
}
