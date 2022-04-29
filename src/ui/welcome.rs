use iced::{Container, Element, Length, Text};

use crate::Message;

pub fn welcome<'a>() -> Element<'a, Message> {
    Container::new(Text::new("Program Loading...").size(30))
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
