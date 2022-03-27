use std::path::PathBuf;

use super::component::canvas::{Curve, ShapeKind};
use super::component::image_box::Navigate;
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

#[derive(Debug, Clone)]
pub enum MainPageMessage {
    ImageBoxMessage(ImageBoxMessage),
    CanvasMessage(CanvasMessage),
    GoToSettings,
}

impl MessageType for MainPageMessage {
    fn describe(&self) -> String {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum UserSettingsMessage {
    GoToMainPage,
    AutomaticLoad(bool),
}

#[derive(Debug, Clone)]
pub enum ImageBoxMessage {
    ImageLoaded(Result<(Vec<PathBuf>, usize), Error>),
    PickImage(DialogType),
    Navigate(Navigate),
    CloseThis,
    CloseAll,
    New,
}

impl MessageType for ImageBoxMessage {
    fn describe(&self) -> String {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum CanvasMessage {
    CurvesMessage(CurvesMessage),
    SelectShapeKind(ShapeKind),
    Clear,
    Save,
    Back,
}

impl MessageType for CanvasMessage {
    fn describe(&self) -> String {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum CurvesMessage {
    AddCurve(Curve),
    SelectCurve(Option<usize>),
}
