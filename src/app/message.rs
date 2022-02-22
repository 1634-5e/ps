use super::component::image_box::{ImageData, Navigate};
use super::error::Error;
use super::file_dialog::DialogType;

use iced_native::Event;

pub trait MessageType {
    fn describe(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum Message {
    MainPageMessage(MainPageMessage),
    UserSettingsMessage(UserSettingsMessage),
    ExternEvent(Event),
}

impl MessageType for Message {
    fn describe(&self) -> String {
        match self {
            Message::MainPageMessage(mm) => mm.describe(),
            Message::UserSettingsMessage(um) => um.describe(),
            Message::ExternEvent(ee) => "an extern event occured.".to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MainPageMessage {
    ImageBoxMessage(ImageBoxMessage),
    GoToSettings, // ToolBarMessage(ToolBarMessage),
}

impl MessageType for MainPageMessage {
    fn describe(&self) -> String {
        match self {
            MainPageMessage::GoToSettings => "display settings".to_owned(),
            MainPageMessage::ImageBoxMessage(im) => im.describe(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum UserSettingsMessage {
    GoToMainPage,
    AutomaticLoad(bool),
}

impl MessageType for UserSettingsMessage {
    fn describe(&self) -> String {
        match self {
            UserSettingsMessage::GoToMainPage => "Go to MainPage".to_owned(),
            UserSettingsMessage::AutomaticLoad(al) => match al {
                true => "Automatically load all images under the smae dir".to_owned(),
                false => "only load the image you picked".to_owned(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum ImageBoxMessage {
    ImageLoaded(Result<(Vec<ImageData>, usize), Error>),
    PickImage(DialogType),
    Navigate(Navigate),
    CloseImage { whole: bool },
}

impl MessageType for ImageBoxMessage {
    fn describe(&self) -> String {
        match self {
            ImageBoxMessage::CloseImage { whole } => match whole {
                true => "close this image".to_owned(),
                false => "close all".to_owned(),
            },
            ImageBoxMessage::ImageLoaded(Ok((images, current))) => format!(
                "{} umages are loaded.This is the {}th",
                images.len(),
                current
            ),
            ImageBoxMessage::ImageLoaded(Err(_)) => {
                "Failed to load images whether the folder is empty or it's root dir.".to_owned()
            }
            ImageBoxMessage::Navigate(n) => match n {
                Navigate::Next => "switch to the next".to_owned(),
                Navigate::Previous => "switch to the previous".to_owned(),
            },
            ImageBoxMessage::PickImage(dt) => match dt {
                DialogType::Dir => "Open a directory".to_owned(),
                DialogType::File => "Open an image".to_owned(),
            },
        }
    }
}

// #[derive(Debug, Clone)]
// pub enum ToolBarMessage {

// }
