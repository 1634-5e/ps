use std::cell::RefCell;
use std::rc::Rc;

use iced::{button, Alignment, Button, Checkbox, Column, Command, Element, Length, Row, Text};

use crate::app::component::image_box::ImageBox;
use crate::app::component::toolbar::ToolBar;
use crate::app::message::{MainPageMessage, UserSettingsMessage};
use crate::app::UserSettings;
use crate::common::style;

use super::component::canvas::Canvas;
use super::component::Component;
use super::error::Error;
use super::message::ToolBarMessage;
use super::Flags;

pub trait Page: Sized {
    type Message;

    fn new(flags: &mut Flags) -> (Self, Command<Self::Message>);
    fn view(&mut self, settings: Rc<RefCell<UserSettings>>) -> Element<Self::Message>;
    fn update(
        &mut self,
        message: Self::Message,
        settings: Rc<RefCell<UserSettings>>,
    ) -> Command<Self::Message>;
    fn title(&self) -> String;
}

//程序的每一个页面，预计只包含主页和设置页面，写成这样方便加入新的页面
pub struct MainPage {
    pub image_box: ImageBox,
    pub toolbar: ToolBar,
    pub canvas: Canvas,
    current: MainContent,
}

enum MainContent {
    Edit,
    View,
}

impl Page for MainPage {
    type Message = MainPageMessage;

    fn new(flags: &mut Flags) -> (MainPage, Command<MainPageMessage>) {
        let (image_box, c) = ImageBox::new(flags);
        let (toolbar, _) = ToolBar::new(flags);
        let canvas = Canvas::default();
        (
            MainPage {
                image_box,
                toolbar,
                canvas,
                current: MainContent::View,
            },
            c.map(MainPageMessage::ImageBoxMessage),
        )
    }

    fn title(&self) -> String {
        "MainPage".to_owned()
    }

    //自带的样式有点少，如果要让某个元素被放在末位，则让同等的元素随便有个Length::Fill或者Length::FillPortion（然后要放末位的那个不管），就会自动被挤过去。。（放中间同理，前后两个空白等值的FillPortion
    fn view(&mut self, settings: Rc<RefCell<UserSettings>>) -> Element<MainPageMessage> {
        //TODO:逐步加入按钮，先从关闭当前图片开始
        let toolbar = self
            .toolbar
            .view(settings.clone())
            .map(MainPageMessage::ToolBarMessage);
        let current = match self.current {
            MainContent::View => self
                .image_box
                .view(settings.clone())
                .map(MainPageMessage::ImageBoxMessage),
            MainContent::Edit => self
                .canvas
                .view(settings.clone())
                .map(MainPageMessage::CanvasMessage),
        };

        let view_picker = Row::new()
            .height(Length::FillPortion(9))
            .push(current)
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
        settings: Rc<RefCell<UserSettings>>,
    ) -> Command<MainPageMessage> {
        match message {
            MainPageMessage::ImageBoxMessage(im) => {
                return self
                    .image_box
                    .update(im, settings)
                    .map(MainPageMessage::ImageBoxMessage)
            }
            MainPageMessage::ToolBarMessage(tm) => match tm {
                ToolBarMessage::CloseThis => self.image_box.close_this(),
                ToolBarMessage::CloseAll => self.image_box.close_all(),
                ToolBarMessage::New => self.current = MainContent::Edit,
                ToolBarMessage::GoToSettings => {}
                ToolBarMessage::ShapeChanged(s) => {
                    self.canvas.pending.change_shape(s);
                    self.toolbar.pick_shape(s);
                }
            },
            MainPageMessage::CanvasMessage(cm) => {
                return self
                    .canvas
                    .update(cm, settings.clone())
                    .map(MainPageMessage::CanvasMessage)
            }
        }
        Command::none()
    }
}

pub struct UserSettingsPage {
    back: button::State,
}

impl Page for UserSettingsPage {
    type Message = UserSettingsMessage;

    fn new(_flags: &mut Flags) -> (UserSettingsPage, Command<UserSettingsMessage>) {
        (
            UserSettingsPage {
                back: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Settings".to_owned()
    }

    fn view(&mut self, settings: Rc<RefCell<UserSettings>>) -> Element<UserSettingsMessage> {
        match settings.try_borrow() {
            Ok(settings) => Column::new()
                .push(
                    Button::new(&mut self.back, Text::new("Back"))
                        .style(style::Button::PickImage)
                        .on_press(UserSettingsMessage::GoToMainPage),
                )
                .push(Checkbox::new(
                    settings.automatic_load,
                    "Automatically load images under the same dir",
                    UserSettingsMessage::AutomaticLoad,
                ))
                .into(),
            Err(e) => Column::new()
                .push(Text::new(Error::from(e).explain()))
                .into(),
        }
    }

    //设置项应该在Ps处完成
    fn update(
        &mut self,
        message: UserSettingsMessage,
        settings: Rc<RefCell<UserSettings>>,
    ) -> Command<UserSettingsMessage> {
        match message {
            UserSettingsMessage::AutomaticLoad(al) => {
                if let Ok(mut settings) = settings.try_borrow_mut() {
                    settings.automatic_load = al;
                }
            }
            _ => {}
        }
        Command::none()
    }
}
