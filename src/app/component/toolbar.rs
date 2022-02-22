use std::{cell::RefCell, rc::Rc};

use super::Component;

use crate::{
    app::{
        message::{ImageBoxMessage, MainPageMessage},
        Flags, UserSettings,
    },
    common::{
        button::{entry, toolbar},
        custom_element::row_with_blanks,
    },
};
use iced::{button, Alignment, Column, Command, Element, Length, Row};

//这里面按钮绑定的事件比较宽泛，所以内联的message是主页的
//TODO:像close这种按钮需要有禁用的情况，目前貌似不自带，得自己手动实现。。
#[derive(Debug, Clone, Default)]
pub struct ToolBar {
    close_this: button::State,
    close_all: button::State,
    pub settings: button::State,
}

impl Component for ToolBar {
    type Message = MainPageMessage;

    fn new(flags: &mut Flags) -> (ToolBar, Command<MainPageMessage>) {
        (
            ToolBar {
                close_this: button::State::new(),
                close_all: button::State::new(),
                settings: button::State::new(),
            },
            Command::none(),
        )
    }

    fn view(&mut self, settings: Rc<RefCell<UserSettings>>) -> Element<MainPageMessage> {
        let settings_button = row_with_blanks(
            Row::new().align_items(Alignment::Center).push(
                entry(&mut self.settings, "settings").on_press(MainPageMessage::GoToSettings),
            ),
            1,
            0,
        )
        .width(Length::FillPortion(2));

        let function_buttons = Row::new().height(Length::FillPortion(1)).push(
            Column::new()
                .push(toolbar(&mut self.close_this, "close this").on_press(
                    MainPageMessage::ImageBoxMessage(ImageBoxMessage::CloseImage { whole: false }),
                ))
                .push(toolbar(&mut self.close_all, "close all").on_press(
                    MainPageMessage::ImageBoxMessage(ImageBoxMessage::CloseImage { whole: true }),
                )),
        );

        function_buttons.push(settings_button).into()
    }

    fn update(
        &mut self,
        message: Self::Message,
        settings: Rc<RefCell<UserSettings>>,
    ) -> Command<MainPageMessage> {
        Command::none()
    }
}
