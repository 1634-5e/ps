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

pub async fn open(paths: Vec<PathBuf>, automatic_load: bool) -> (Vec<PathBuf>, Option<usize>) {
    //要处理两个情况，
    //1：用户使用按钮打开文件或者文件夹，目前还只能打开单个文件/文件夹
    //2：用户使用拖拽方式打开，这时可能有多个路径需要处理

    let mut images = vec![];
    let mut current = None;
    for path in paths {
        if path.is_dir() || automatic_load {
            let parent;
            if path.is_dir() {
                parent = path.as_path();
            } else {
                parent = match path.parent() {
                    Some(pt) => pt,
                    None => {
                        return (vec![], None);
                    }
                };
            }

            for entry in parent.read_dir().unwrap() {
                match entry {
                    Ok(d) => {
                        let p = d.path();
                        match p.extension() {
                            Some(e) if e.eq("png") || e.eq("svg") || e.eq("jpg") => {
                                if p == path {
                                    current = Some(images.len());
                                }
                                images.push(p);
                            }
                            _ => {}
                        }
                    }
                    Err(_) => {}
                }
            }
        } else {
            images.push(path);
        }
    }
    (images, current)
}
