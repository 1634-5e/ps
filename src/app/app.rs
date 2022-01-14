pub use iced::{button, Application, Command, Element};

use super::file_dialog;
use super::image_box::ImageBox;
use super::page::{Page, ToolBar};
use super::settings;

pub struct Ps {
    pages: Pages,
    settings: UserSettings,
}

#[derive(Debug, Clone)]
struct UserSettings {
    load_mode: settings::LoadMode,
}

#[derive(Debug, Clone)]
pub enum Message {
    ImageLoaded(ImageBox),
    PickImage(file_dialog::DialogType),
    ChangePage,
    SettingsChanged(settings::SettingsType),
}

impl Application for Ps {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Ps, Command<Message>) {
        (
            Ps {
                pages: Pages::new(),
                settings: UserSettings {
                    load_mode: settings::LoadMode::Strict,
                },
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let subtitle = self.pages.title();

        format!("{} - Ps", subtitle)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        self.pages.update(message, &mut self.settings)
    }

    fn view(&mut self) -> Element<Message> {
        self.pages.view()
    }
}

struct Pages {
    pages: Vec<Page>,
    current: usize,
}

impl Pages {
    fn new() -> Pages {
        Pages {
            pages: vec![
                Page::MainPage {
                    image_box: ImageBox::Init {
                        single: button::State::new(),
                        dir: button::State::new(),
                    },
                    toolbar: ToolBar::new(),
                },
                Page::UserSettings {
                    back: button::State::new(),
                },
            ],
            current: 0,
        }
    }

    fn title(&self) -> String {
        match self.pages[self.current] {
            Page::MainPage { .. } => "MainPage".to_owned(),
            Page::UserSettings { .. } => "Settings".to_owned(),
        }
    }

    fn update(&mut self, message: Message, settings: &mut UserSettings) -> Command<Message> {
        match message {
            Message::ImageLoaded(ib) => {
                if let Page::MainPage { image_box, .. } = &mut self.pages[0] {
                    *image_box = ib;
                }
                Command::none()
            }
            Message::PickImage(dialog_type) => {
                let selected = ImageBox::pick_image(dialog_type);
                match selected {
                    Some(path) => {
                        if let Page::MainPage { image_box, .. } = &mut self.pages[self.current] {
                            *image_box = ImageBox::Loading;
                        }
                        Command::perform(
                            ImageBox::load(path, settings.load_mode),
                            Message::ImageLoaded,
                        )
                    }
                    None => Command::none(),
                }
            }
            Message::SettingsChanged(changed_settings) => {
                match changed_settings {
                    settings::SettingsType::LoadMode(load_mode) => settings.load_mode = load_mode,
                }
                Command::none()
            }
            //FIXME: 后面页面增加时，这里的逻辑就不适用了
            Message::ChangePage => {
                self.current = (self.current + 1) % 2;
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        self.pages[self.current].view()
    }
}
