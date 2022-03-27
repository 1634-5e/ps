use chrono::{Datelike, Local, Timelike};
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
    let now = Local::now();

    let (is_pm, hour) = now.hour12();
    let (_, year) = now.year_ce();
    let filename = format!(
        "{}-{:02}-{:02} {:?} {:02}-{:02}-{:02} {}",
        year,
        now.month(),
        now.day(),
        now.weekday(),
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    );

    FileDialog::new()
        .set_location("D://Desktop")
        .set_filename(filename.as_str())
        .add_filter("SVG Image", &["svg"])
        .show_save_single_file()
        .unwrap()
}
