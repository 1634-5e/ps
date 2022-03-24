use super::component::canvas::{Curve, ShapeKind};
use super::component::image_box::{ImageData, Navigate};
use super::error::Error;
use super::file_dialog::DialogType;

use iced_native::Event;

pub trait ComponentMessage {
    fn describe(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum Message {
    MainPageMessage(MainPageMessage),
    UserSettingsMessage(UserSettingsMessage),
    ExternEvent(Event),
}

#[derive(Debug, Clone)]
pub enum MainPageMessage {
    ToolBarMessage(ToolBarMessage),
    ImageBoxMessage(ImageBoxMessage),
    CanvasMessage(CanvasMessage),
}

#[derive(Debug, Clone)]
pub enum UserSettingsMessage {
    GoToMainPage,
    AutomaticLoad(bool),
}

#[derive(Debug, Clone)]
pub enum ImageBoxMessage {
    ImageLoaded(Result<(Vec<ImageData>, usize), Error>),
    PickImage(DialogType),
    Navigate(Navigate),
}

impl ComponentMessage for ImageBoxMessage {
    fn describe(&self) -> String {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum ToolBarMessage {
    CloseThis,
    CloseAll,
    New,
    GoToSettings,
    ShapeChanged(ShapeKind),
}

impl ComponentMessage for ToolBarMessage {
    fn describe(&self) -> String {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum CanvasMessage {
    AddCurve(Curve),
    Clear,
}

impl ComponentMessage for CanvasMessage {
    fn describe(&self) -> String {
        todo!()
    }
}
