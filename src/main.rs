use app::Ps;
use iced::{Application, Settings};

mod app;
mod common;
pub fn main() -> iced::Result {
    Ps::run(Settings::default())
}
