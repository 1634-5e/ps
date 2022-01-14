use iced::Application;
use iced::Settings;

mod common;
mod app;
pub fn main() -> iced::Result {
    app::app::Ps::run(Settings::default())
}