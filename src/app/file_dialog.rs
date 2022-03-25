pub use native_dialog::FileDialog;
pub use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum DialogType {
    File,
    Dir,
}

pub fn pick(dialog_type: DialogType) -> Option<PathBuf> {
    match dialog_type {
        DialogType::File => FileDialog::new()
            .set_location("D://Desktop")
            .add_filter("PNG Image", &["png"])
            .add_filter("JPEG Image", &["jpg", "jpeg"])
            .add_filter("SVG Image", &["svg"])
            .show_open_single_file()
            .unwrap(),
        DialogType::Dir => FileDialog::new().show_open_single_dir().unwrap(),
    }
}

pub fn save() -> Option<PathBuf> {
    FileDialog::new()
        .set_location("D://Desktop")
        .add_filter("SVG Image", &["svg"])
        .show_save_single_file()
        .unwrap()
}
