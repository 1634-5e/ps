use iced::{Button,button,Text};
use crate::app::message::Message;


//现在的样式直接拿来用的，不太合适
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