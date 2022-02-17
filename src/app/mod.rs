use std::path::PathBuf;

pub use iced::{button, Application, Command, Element};

mod component;
mod error;
mod file_dialog;
pub mod message;
pub mod page;

use message::{MainPageMessage, Message, UserSettingsMessage};
use page::{MainPage, Page, UserSettingsPage};

pub struct Ps {
    main_page: MainPage,
    settings_page: UserSettingsPage,
    current: CurrentPage,
    settings: UserSettings,
}

#[derive(Debug, Clone)]
enum CurrentPage {
    MainPage,
    UserSettingsPage,
}

#[derive(Debug, Clone, Default)]
pub struct Flags {
    pub(crate) env_args: Vec<PathBuf>,
    pub(crate) user_settings: UserSettings,
}

#[derive(Debug, Clone, Default)]
pub struct UserSettings {
    pub automatic_load: bool,
}

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
                settings: UserSettings {
                    automatic_load: true,
                },
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
                        .update(mm, &mut self.settings)
                        .map(Message::MainPageMessage)
                }
            },
            Message::UserSettingsMessage(um) => match um {
                UserSettingsMessage::GoToMainPage => self.current = CurrentPage::MainPage,
                _ => {
                    return self
                        .settings_page
                        .update(um, &mut self.settings)
                        .map(Message::UserSettingsMessage)
                }
            },
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self.current {
            CurrentPage::MainPage => self
                .main_page
                .view(&mut self.settings)
                .map(Message::MainPageMessage),
            CurrentPage::UserSettingsPage => self
                .settings_page
                .view(&mut self.settings)
                .map(Message::UserSettingsMessage),
        }
    }
}
