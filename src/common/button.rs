use crate::app::message::ComponentMessage;
use iced::{button, Button, Length, Text};

use super::style;

pub fn toolbar<'a, T>(state: &'a mut button::State, text: &str) -> Button<'a, T>
where
    T: ComponentMessage + Clone,
{
    Button::new(state, Text::new(text))
        .padding(10)
        .style(style::Button::Toolbar)
}

//TODO: 想办法让按钮变成一个竖长条，目前想到的是用很多个按钮，但是明显不合适。。
pub fn navigator<'a, T>(state: &'a mut button::State, text: &str) -> Button<'a, T>
where
    T: ComponentMessage + Clone,
{
    Button::new(state, Text::new(text))
        .padding(10)
        .style(style::Button::Navigator)
}

pub fn entry<'a, T>(state: &'a mut button::State, text: &str) -> Button<'a, T>
where
    T: ComponentMessage + Clone,
{
    Button::new(state, Text::new(text))
        .padding(10)
        .style(style::Button::PickImage)
}
