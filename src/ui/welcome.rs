use iced::{Element, Text};

use crate::Message;

pub fn welcome<'a>() -> Element<'a, Message> {
    Text::new("Program Loading...").into()
}
