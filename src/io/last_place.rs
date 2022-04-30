use std::{
    fs::{self, File},
    path::PathBuf,
};

use crate::ui::Curve;

const FILE_NAME: &str = "last_place";

#[derive(Debug, Default)]
pub struct SavedState {
    // pub is_editing: bool,

    // //view
    // pub images: Vec<PathBuf>,
    // pub on_view: Option<usize>,
    // //edit
    // pub curves: Vec<Curve>,
}

pub async fn save(saved_state: SavedState, path: PathBuf) -> std::io::Result<()> {
    // if !path.exists() {
    //     fs::create_dir_all(&path)?;
    // }

    // let serialized = serde_json::to_string_pretty(&saved_state)?;

    // fs::write(path.join(FILE_NAME), serialized)?;

    Ok(())
}

pub async fn load(path: PathBuf) -> std::io::Result<Option<SavedState>> {
    // match serde_json::from_reader(File::open(path.join(FILE_NAME))?) {
    //     Ok(state) => Ok(Some(state)),
    //     Err(_) => Ok(None),
    // }

    Ok(None)
}
