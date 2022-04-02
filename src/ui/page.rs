use std::cell::RefCell;
use std::rc::Rc;

use iced::{
    button, Alignment, Button, Checkbox, Column, Command, Container, Element, Length, Text,
};

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
    image_box: ImageBox,
    canvas: Canvas,
    current: MainContent,
    goto_settings: button::State,
}

enum MainContent {
    Edit,
    View,
}

impl Page for MainPage {
    type Message = MainPageMessage;

    fn new(flags: &mut Flags) -> (MainPage, Command<MainPageMessage>) {
        let (image_box, c1) = ImageBox::new(flags);
        let (canvas, c2) = Canvas::new(flags);
        (
            MainPage {
                image_box,
                canvas,
                current: MainContent::View,
                goto_settings: button::State::new(),
            },
            Command::batch([
                c1.map(MainPageMessage::ImageBoxMessage),
                c2.map(MainPageMessage::CanvasMessage),
            ]),
        )
    }

    fn title(&self) -> String {
        "MainPage".to_owned()
    }

    fn view(&mut self, settings: Rc<RefCell<UserSettings>>) -> Element<MainPageMessage> {
        let main_content = match self.current {
            MainContent::View => self
                .image_box
                .view(settings.clone())
                .map(MainPageMessage::ImageBoxMessage),
            MainContent::Edit => self
                .canvas
                .view(settings.clone())
                .map(MainPageMessage::CanvasMessage),
        };

        let settings_button = Button::new(&mut self.goto_settings, Text::new("settings"))
            .style(style::Button::Toolbar)
            .on_press(MainPageMessage::GoToSettings)
            .width(Length::Shrink);

        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::End)
            .push(settings_button)
            .push(main_content)
            .into()
    }

    fn update(
        &mut self,
        message: MainPageMessage,
        settings: Rc<RefCell<UserSettings>>,
    ) -> Command<MainPageMessage> {
        match message {
            MainPageMessage::ImageBoxMessage(im) => match im {
                ImageBoxMessage::New => self.current = MainContent::Edit,
                _ => {
                    return self
                        .image_box
                        .update(im, settings)
                        .map(MainPageMessage::ImageBoxMessage)
                }
            },
            MainPageMessage::CanvasMessage(cm) => match cm {
                CanvasMessage::Back => self.current = MainContent::View,
                _ => {
                    return self
                        .canvas
                        .update(cm, settings.clone())
                        .map(MainPageMessage::CanvasMessage)
                }
            },
            _ => {}
        }
        Command::none()
    }
}
