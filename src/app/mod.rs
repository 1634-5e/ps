use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use iced::{Application, Command, Element, Subscription};
use iced_native::window::Event as WindowEvent;
use iced_native::Event;

mod component;
pub mod error;
mod file_dialog;
pub mod message;
pub mod page;
mod utils;

use component::image_box::open;
use message::{MainPageMessage, Message, UserSettingsMessage};
use page::{MainPage, Page, UserSettingsPage};

use self::message::ImageBoxMessage;

pub struct Ps {
    main_page: MainPage,
    settings_page: UserSettingsPage,
    current: CurrentPage,
    flags: Flags,
}

#[derive(Debug, Clone)]
enum CurrentPage {
    MainPage,
    UserSettingsPage,
}

#[derive(Debug, Clone, Default)]
pub struct Flags {
    pub(crate) env_args: Vec<PathBuf>,
    pub(crate) user_settings: Rc<RefCell<UserSettings>>,
}

//TODO: 这里应该使用Rc<RefCell>
#[derive(Debug, Clone, Default)]
pub struct UserSettings {
    pub(crate) automatic_load: bool, //这一项继续细分可以包括：按钮打开自动、拖拽到图标自动、拖拽到应用自动、以及全关
}

//FIXME: update 目前一共要经历三层，Ps（程序主体）-> Page（程序页面）-> Component（页面的组成部分），
//而返回view又要逆向经过三层，这样原本不必要的传参多了4次，
//以后要是要减小程序内存的话，这里的层次应该全部去掉
impl Application for Ps {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = Flags;

    fn new(mut flags: Flags) -> (Ps, Command<Message>) {
        let (main_page, c) = MainPage::new(&mut flags);
        let (settings_page, _) = UserSettingsPage::new(&mut flags);
        (
            Ps {
                main_page,
                settings_page,
                current: CurrentPage::MainPage,
                flags,
            },
            c.map(Message::MainPageMessage),
        )
    }

    fn title(&self) -> String {
        let subtitle = match self.current {
            CurrentPage::MainPage => self.main_page.title(),
            CurrentPage::UserSettingsPage => self.settings_page.title(),
        };

        format!("{} - Ps", subtitle)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::MainPageMessage(mm) => match mm {
                MainPageMessage::GoToSettings => self.current = CurrentPage::UserSettingsPage,
                _ => {
                    return self
                        .main_page
                        .update(mm, self.flags.user_settings.clone())
                        .map(Message::MainPageMessage)
                }
            },
            Message::UserSettingsMessage(um) => match um {
                UserSettingsMessage::GoToMainPage => self.current = CurrentPage::MainPage,
                _ => {
                    return self
                        .settings_page
                        .update(um, self.flags.user_settings.clone())
                        .map(Message::UserSettingsMessage)
                }
            },
            //这里是从内置的Event事件中匹配，
            //因为不同的组件，也就是接下来套娃的部分，可能会响应相同的事件，所以这里不套娃应该好一点
            Message::ExternEvent(ee) => match ee {
                Event::Window(we) => match we {
                    WindowEvent::FileDropped(fd) => {
                        //FIXME:这个错误暂时没有处理
                        return Command::perform(
                            open(vec![fd], false),
                            ImageBoxMessage::ImageLoaded,
                        )
                        .map(MainPageMessage::ImageBoxMessage)
                        .map(Message::MainPageMessage);
                    }
                    _ => {}
                },
                _ => {}
            },
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self.current {
            CurrentPage::MainPage => self
                .main_page
                .view(self.flags.user_settings.clone())
                .map(Message::MainPageMessage),
            CurrentPage::UserSettingsPage => self
                .settings_page
                .view(self.flags.user_settings.clone())
                .map(Message::UserSettingsMessage),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::ExternEvent)
    }
}
