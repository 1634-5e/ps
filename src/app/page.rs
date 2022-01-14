use iced::{button, Alignment, Column, Element, Length, Row, Text};

use super::app::Message;
use super::image_box::ImageBox;
use super::super::common::button as common_button;

//程序的每一个页面，预计只包含主页和设置页面，写成这样方便加入新的页面
pub enum Page {
    MainPage {
        image_box: ImageBox,
        toolbar: ToolBar,
    },
    UserSettings {
        back: button::State,
    },
}

impl<'a> Page {
    pub fn view(&mut self) -> Element<Message> {
        match self {
            Page::MainPage { image_box, toolbar } => Self::main_page(image_box, toolbar),
            Page::UserSettings { back } => Self::settings(back),
        }
    }

    //TODO: 用pane_grid重新布局
    fn main_page(image_box: &'a mut ImageBox, toolbar: &'a mut ToolBar) -> Element<'a, Message> {
        let settings = Row::new()
            .align_items(Alignment::Center)
            .push(common_button::toolbar(&mut toolbar.settings, "settings").on_press(Message::ChangePage));
        let toolbar = Row::new()
            .align_items(Alignment::Center)
            .width(Length::Fill)
            .height(Length::FillPortion(1))
            .spacing(20)
            .push(Text::new("One"))
            .push(Text::new("Two"))
            .push(Text::new("Three"))
            .push(Text::new("Four"))
            .push(Text::new("There will be a toolbar here..."));

        let view_picker = Row::new()
            .height(Length::FillPortion(9))
            .push(image_box.view())
            .push(
                Column::new()
                    .width(Length::FillPortion(2))
                    .push(Text::new("a picker here")),
            );

        Column::new()
            .align_items(Alignment::Center)
            .push(toolbar)
            .push(settings)
            .push(view_picker)
            .into()
    }

    fn settings(back: &'a mut button::State) -> Element<'a, Message> {
        Column::new()
            .push(common_button::toolbar(back, "back to mainpage").on_press(Message::ChangePage))
            .into()
    }
}

pub struct ToolBar {
    settings: button::State,
}

impl ToolBar {
    pub fn new() -> ToolBar {
        ToolBar {
            settings: button::State::new(),
        }
    }
}
