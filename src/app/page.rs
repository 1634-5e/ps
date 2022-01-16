use iced::{button, Alignment, Column, Element, Length, Row, Text};

use super::image_box::ImageBox;
use super::message::Message;
use crate::common::button as common_button;
use crate::common::custom_box::row_with_blanks;

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

    //自带的样式有点少，如果要让某个元素被放在末位，则让同等的元素随便有个Length::Fill或者Length::FillPortion（然后要放末位的那个不管），就会自动被挤过去。。（放中间同理，前后两个空白等值的FillPortion
    fn main_page(image_box: &'a mut ImageBox, toolbar: &'a mut ToolBar) -> Element<'a, Message> {
        let settings = row_with_blanks(
            Row::new().align_items(Alignment::Center).push(
                common_button::entry(&mut toolbar.settings, "settings")
                    .on_press(Message::ChangePage),
            ),
            1,
            0,
        )
        .width(Length::FillPortion(2));
        //TODO:逐步加入按钮，先从关闭当前图片开始
        let toolbar = Row::new()
            .align_items(Alignment::Center)
            .width(Length::FillPortion(8))
            .spacing(20)
            .push(Text::new("One"))
            .push(Text::new("Two"))
            .push(Text::new("Three"))
            .push(Text::new("Four"))
            .push(Text::new("There will be a toolbar here..."));

        let buttons = Row::new()
            .height(Length::FillPortion(1))
            .push(toolbar)
            .push(settings);

        let view_picker = Row::new()
            .height(Length::FillPortion(9))
            .push(image_box.view())
            .push(
                Column::new()
                    .width(Length::FillPortion(2))
                    .push(Text::new("a picker here")),
            );

        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .push(buttons)
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
