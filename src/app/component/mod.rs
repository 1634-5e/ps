use std::cell::RefCell;
use std::rc::Rc;

use iced::{button, Button, Command, Element};

use crate::common::style;

use super::{message::MessageType, Flags, UserSettings};

pub mod canvas;
pub mod image_box;

//构成页面的组件，会有上方的按钮区，左下的显示区，和右下的区域
pub trait Component: Sized {
    type Message;

    fn new(flags: &mut Flags) -> (Self, Command<Self::Message>);

    //返回(main_content, toolbar)
    fn view(
        &mut self,
        settings: Rc<RefCell<UserSettings>>,
    ) -> (Element<Self::Message>, Element<Self::Message>);
    fn update(
        &mut self,
        message: Self::Message,
        settings: Rc<RefCell<UserSettings>>,
    ) -> Command<Self::Message>;
}

#[derive(Default, Clone, Debug)]
pub struct ControllableButton {
    state: button::State,
    disabled: bool,
}

impl ControllableButton {
    pub fn view<'a, T: MessageType + Clone, E: Into<Element<'a, T>>>(
        &'a mut self,
        content: E,
        style: style::Button,
        message: T,
    ) -> Button<'a, T> {
        let button = Button::new(&mut self.state, content).style(style);
        if self.disabled {
            button
        } else {
            button.on_press(message)
        }
    }

    pub fn enable(&mut self) {
        self.disabled = false;
    }

    pub fn disable(&mut self) {
        self.disabled = true;
    }
}
