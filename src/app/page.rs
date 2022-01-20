use iced::{button, Alignment, Checkbox, Column, Command, Element, Length, Row, Text};

use crate::common::button::entry;

use super::component::{Component, ImageBox, ToolBar};
use super::message::{MainPageMessage, UserSettingsMessage};
use super::UserSettings;

pub trait Page {
    type Message;

    fn new() -> Self;
    fn view(&mut self, settings: &mut UserSettings) -> Element<Self::Message>;
    fn update(
        &mut self,
        message: Self::Message,
        settings: &mut UserSettings,
    ) -> Command<Self::Message>;
    fn title(&self) -> String;
}

//程序的每一个页面，预计只包含主页和设置页面，写成这样方便加入新的页面
pub struct MainPage {
    image_box: ImageBox,
    toolbar: ToolBar,
}

impl Page for MainPage {
    type Message = MainPageMessage;

    fn new() -> MainPage {
        MainPage {
            image_box: ImageBox::new(),
            toolbar: ToolBar::new(),
        }
    }

    fn title(&self) -> String {
        "MainPage".to_owned()
    }

    //自带的样式有点少，如果要让某个元素被放在末位，则让同等的元素随便有个Length::Fill或者Length::FillPortion（然后要放末位的那个不管），就会自动被挤过去。。（放中间同理，前后两个空白等值的FillPortion
    fn view(&mut self, settings: &mut UserSettings) -> Element<MainPageMessage> {
        //TODO:逐步加入按钮，先从关闭当前图片开始
        let toolbar = self.toolbar.view(settings);

        let view_picker = Row::new()
            .height(Length::FillPortion(9))
            .push(
                self.image_box
                    .view(settings)
                    .map(MainPageMessage::ImageBoxMessage),
            )
            .push(
                Column::new()
                    .width(Length::FillPortion(2))
                    .push(Text::new("a picker here")),
            );

        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .push(toolbar)
            .push(view_picker)
            .into()
    }

    fn update(
        &mut self,
        message: MainPageMessage,
        settings: &mut UserSettings,
    ) -> Command<MainPageMessage> {
        match message {
            MainPageMessage::ImageBoxMessage(im) => self
                .image_box
                .update(im, settings)
                .map(MainPageMessage::ImageBoxMessage),
            MainPageMessage::GoToSettings => Command::none(),
        }
    }
}

pub struct UserSettingsPage {
    back: button::State,
}

impl Page for UserSettingsPage {
    type Message = UserSettingsMessage;

    fn new() -> UserSettingsPage {
        UserSettingsPage {
            back: button::State::new(),
        }
    }

    fn title(&self) -> String {
        "Settings".to_owned()
    }

    fn view(&mut self, settings: &mut UserSettings) -> Element<UserSettingsMessage> {
        Column::new()
            .push(entry(&mut self.back, "Back").on_press(UserSettingsMessage::GoToMainPage))
            .push(Checkbox::new(
                settings.automatic_load,
                "Automatically load images under the same dir",
                UserSettingsMessage::AutomaticLoad,
            ))
            .into()
    }

    //设置项应该在Ps处完成
    fn update(
        &mut self,
        message: UserSettingsMessage,
        settings: &mut UserSettings,
    ) -> Command<UserSettingsMessage> {
        match message {
            UserSettingsMessage::AutomaticLoad(al) => {
                settings.automatic_load = al;
            }
            _ => {}
        }
        Command::none()
    }
}
