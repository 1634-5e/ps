mod component;
mod error;
mod file_dialog;
pub mod message;
pub mod page;

pub use iced::{button, Application, Command, Element};

use message::Message;
use page::{MainPage, UserSettingsPage};

use self::{
    message::{MainPageMessage, UserSettingsMessage},
    page::Page,
};

pub struct Ps {
    main_page: MainPage,
    settings_page: UserSettingsPage,
    current: CurrentPage,
    settings: UserSettings,
}

#[derive(Debug, Clone)]
pub struct UserSettings {
    automatic_load: bool,
}

impl Application for Ps {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Ps, Command<Message>) {
        (
            Ps {
                main_page: MainPage::new(),
                settings_page: UserSettingsPage::new(),
                current: CurrentPage::MainPage,
                settings: UserSettings {
                    automatic_load: true,
                },
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let subtitle = match self.current {
            CurrentPage::MainPage => self.main_page.title(),
            CurrentPage::UserSettings => self.settings_page.title(),
        };

        format!("{} - Ps", subtitle)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::MainPageMessage(mm) => match mm {
                MainPageMessage::GoToSettings => self.current = CurrentPage::UserSettings,
                _ => return self
                    .main_page
                    .update(mm, &mut self.settings)
                    .map(Message::MainPageMessage),
            },
            Message::UserSettingsMessage(um) => match um {
                UserSettingsMessage::GoToMainPage => self.current = CurrentPage::MainPage,
                _ => return self
                    .settings_page
                    .update(um, &mut self.settings)
                    .map(Message::UserSettingsMessage),
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
            CurrentPage::UserSettings => self
                .settings_page
                .view(&mut self.settings)
                .map(Message::UserSettingsMessage),
        }
    }
}

#[derive(Debug, Clone)]
enum CurrentPage {
    MainPage,
    UserSettings,
}
