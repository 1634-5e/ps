use iced::{Button,button,Text};
use crate::app::message::Message;


//TODO:设计合适的按钮样式
pub fn toolbar<'a>(state: &'a mut button::State, text: &str) -> Button<'a, Message> {
    Button::new(state, Text::new(text))
        .padding(10)
        .style(super::style::Button::Toolbar)
}

pub fn navigator<'a>(state: &'a mut button::State, text: &str) -> Button<'a, Message> {
    Button::new(state, Text::new(text))
        .padding(10)
        .style(super::style::Button::Navigator)
}