use std::path::PathBuf;

use iced::{
    button, image, Alignment, Application, Button, Column, Command, Container, Element, Length,
    Row, Settings, Svg, Text,
};
use native_dialog::FileDialog;

mod error;
mod style;

pub fn main() -> iced::Result {
    Ps::run(Settings::default())
}

struct Ps {
    pages: Pages,
}

#[derive(Debug, Clone)]
enum Message {
    ImageLoaded(ImageBox),
    PickImage,
}

impl Application for Ps {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Ps, Command<Message>) {
        (
            Ps {
                pages: Pages::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let subtitle = self.pages.title();

        format!("{} - Ps", subtitle)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        self.pages.update(message)
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
                    image_box: ImageBox::Init(button::State::new()),
                },
                Page::UserSettings,
            ],
            current: 0,
        }
    }

    fn title(&self) -> String {
        match self.pages[self.current] {
            Page::MainPage { .. } => "MainPage".to_owned(),
            Page::UserSettings => "Settings".to_owned(),
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ImageLoaded(ib) => {
                if let Page::MainPage { image_box } = &mut self.pages[0] {
                    *image_box = ib;
                }
                Command::none()
            }
            Message::PickImage => {
                if let Page::MainPage { image_box } = &mut self.pages[self.current] {
                    *image_box = ImageBox::Loading;
                }
                let selected = ImageBox::pick_image();
                match selected {
                    Some(path) => Command::perform(ImageBox::load(path), Message::ImageLoaded),
                    None => Command::none(),
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        self.pages[self.current].view()
    }
}

enum Page {
    MainPage { image_box: ImageBox },
    UserSettings,
}

impl<'a> Page {
    fn view(&mut self) -> Element<Message> {
        match self {
            Page::MainPage { image_box } => Self::main_page(image_box),
            Page::UserSettings => Self::settings(),
        }
    }

    fn container(title: &str) -> Column<'a, Message> {
        Column::new().spacing(20).push(Text::new(title).size(50))
    }

    fn main_page(image_box: &mut ImageBox) -> Element<Message> {
        let toolbar = Row::new()
            .width(Length::Fill)
            .height(Length::FillPortion(1))
            .spacing(20)
            .push(Text::new("One"))
            .push(Text::new("Two"))
            .push(Text::new("Three"))
            .push(Text::new("Four"))
            .push(Text::new("There will be a toolbar here..."));

        let view_picker = Row::new()
            .height(Length::FillPortion(5))
            .push(image_box.view())
            .push(
                Column::new()
                    .width(Length::FillPortion(2))
                    .push(Text::new("a picker here")),
            );

        Column::new()
            .align_items(Alignment::Start)
            .push(toolbar)
            .push(view_picker)
            .into()
    }

    fn settings() -> Element<'a, Message> {
        Self::container("设置").into()
    }
}

#[derive(Debug, Clone)]
enum ImageBox {
    Init(button::State),
    Loading,
    Loaded { image_type: ImageType },
    Errored(error::Error),
}

impl<'a> ImageBox {
    fn basic_layout<T>(content: T) -> Element<'a, Message>
    where
        T: Into<Element<'a, Message>>,
    {
        Container::new(content)
            .width(Length::FillPortion(5))
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            ImageBox::Init(state) => Self::basic_layout(
                Column::new().push(button(state, "Open an image").on_press(Message::PickImage)),
            ),
            ImageBox::Loading => Self::basic_layout(Text::new("Loading...").size(40)),
            ImageBox::Loaded { image_type } => match image_type {
                ImageType::Bitmap(image, state) => {
                    Self::basic_layout(image::Viewer::new(state, image.clone()))
                }
                ImageType::Vector(image) => Self::basic_layout(image.clone()),
            },
            ImageBox::Errored(e) => match e {
                error::Error::NameInvalid => Self::basic_layout(Text::new("Name Invalid!").size(40)),
                error::Error::NotFound => Self::basic_layout(Text::new("Not Exist!").size(40)),
            },
        }
    }

    async fn load(path: PathBuf) -> ImageBox {
        if !path.exists() {
            return ImageBox::Errored(error::Error::NotFound);
        }
        match path.extension() {
            Some(ext) => {
                if ext == "svg" {
                    ImageBox::Loaded {
                        image_type: ImageType::Vector(Svg::from_path(path)),
                    }
                } else {
                    ImageBox::Loaded {
                        image_type: ImageType::Bitmap(
                            image::Handle::from_path(path),
                            image::viewer::State::new(),
                        ),
                    }
                }
            }
            None => ImageBox::Errored(error::Error::NameInvalid),
        }
    }

    fn pick_image() -> Option<PathBuf> {
        let path = FileDialog::new()
            .set_location("D://Desktop")
            .add_filter("PNG Image", &["png"])
            .add_filter("JPEG Image", &["jpg", "jpeg"])
            .add_filter("SVG Image", &["svg"])
            .show_open_single_file()
            .unwrap();
        path
    }
}

#[derive(Debug, Clone)]
enum ImageType {
    Bitmap(image::Handle, image::viewer::State),
    Vector(Svg),
}

fn button<'a>(state: &'a mut button::State, text: &str) -> Button<'a, Message> {
    Button::new(state, Text::new(text))
        .padding(10)
        .style(style::button::Button::Primary)
}
