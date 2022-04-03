use chrono::{Datelike, Local, Timelike};
pub use native_dialog::FileDialog;
use std::collections::hash_set::HashSet;
pub use std::path::PathBuf;

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum DialogType {
//     File,
//     Dir,
// }

// impl DialogType {
//     pub const ALL: [DialogType; 2] = [DialogType::File, DialogType::Dir];
// }

// impl std::fmt::Display for DialogType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 DialogType::File => "File",
//                 DialogType::Dir => "Directory",
//             }
//         )
//     }
// }

pub fn pick() -> Option<Vec<PathBuf>> {
    FileDialog::new()
        .set_location("D://Desktop")
        .add_filter("PNG Image", &["png"])
        .add_filter("JPEG Image", &["jpg", "jpeg"])
        .add_filter("SVG Image", &["svg"])
        .show_open_multiple_file()
        .ok()
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

pub async fn open(mut paths: Vec<PathBuf>, automatic_load: bool) -> (Vec<PathBuf>, Option<usize>) {
    //要处理两个情况，
    //1：用户使用按钮打开文件或者文件夹，目前还只能打开单个文件/文件夹
    //2：用户使用拖拽方式打开，这时可能有多个路径需要处理

    let mut parents = HashSet::new();

    if automatic_load {
        paths.retain(|path| {
            path.is_dir()
                || match path.parent() {
                    Some(parent) => {
                        parents.insert(parent.to_path_buf());
                        false
                    }
                    None => true,
                }
        })
    }

    let mut images = vec![];
    for path in paths.into_iter().chain(parents.into_iter()) {
        if path.is_dir() {
            for entry in path.read_dir().unwrap() {
                match entry {
                    Ok(d) => {
                        let p = d.path();
                        match p.extension() {
                            Some(e) if e.eq("png") || e.eq("svg") || e.eq("jpg") => {
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

    //FIXME:暂时去除了自动选current
    (images, Some(0))
}
