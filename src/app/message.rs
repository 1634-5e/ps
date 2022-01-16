use super::file_dialog::DialogType;
use super::image_box::{ImageBox, Navigate};
use super::settings::SettingsType;

//这地方我看教程利用map单独弄了一个StepMessage，感觉还掌握不来
#[derive(Debug, Clone)]
pub enum Message {
    ImageLoaded(ImageBox),
    PickImage(DialogType),
    ChangePage,
    SettingsChanged(SettingsType),
    Navigate(Navigate),
}
