use iced::{
    button, image, Alignment, Application, Button, Column, Command, Container, Element, Length,
    Row, Settings, Svg, Text,
};

pub fn main() -> iced::Result {
    Ps::run(Settings::default())
}

struct Ps {
    pages: Pages,
}

#[derive(Debug, Clone)]
enum Message {
    ImageLoaded(Result<ImageBox, Error>),
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
            Command::perform(ImageBox::load(), Message::ImageLoaded),
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
                    imagebox: ImageBox::Loading,
                },
                Page::Settings,
            ],
            current: 0,
        }
    }

    fn title(&self) -> String {
        match self.pages[self.current] {
            Page::MainPage { .. } => "MainPage".to_owned(),
            Page::Settings => "Settings".to_owned(),
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ImageLoaded(Ok(ib)) => {
                if let Page::MainPage { imagebox } = &mut self.pages[0] {
                    *imagebox = ib;
                }

                Command::none()
            }
            Message::ImageLoaded(Err(_error)) => Command::none(),
            Message::PickImage => Command::none(),
        }
    }

    fn view(&mut self) -> Element<Message> {
        self.pages[self.current].view()
    }
}

enum Page {
    MainPage { imagebox: ImageBox },
    Settings,
}

impl<'a> Page {
    fn view(&mut self) -> Element<Message> {
        match self {
            Page::MainPage { imagebox } => Self::main_page(imagebox),
            Page::Settings => Self::settings(),
        }
    }

    fn container(title: &str) -> Column<'a, Message> {
        Column::new().spacing(20).push(Text::new(title).size(50))
    }

    fn main_page(imagebox: &mut ImageBox) -> Element<Message> {
        let toolbar = Row::new()
            .width(Length::Fill)
            .height(Length::FillPortion(1))
            .spacing(20)
            .push(Text::new("1"))
            .push(Text::new("2"))
            .push(Text::new("3"))
            .push(Text::new("4"))
            .push(Text::new("5"))
            .push(Text::new("There will be a toolbar here..."));

        let view_picker = Row::new()
            .height(Length::FillPortion(5))
            .push(imagebox.view())
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
    Loading,
    Loaded { image_type: ImageType },
    Errored(Error),
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
            ImageBox::Loading => Self::basic_layout(Text::new("正在加载图片...").size(40)),
            ImageBox::Loaded { image_type } => match image_type {
                ImageType::Bitmap(image, state) => {
                    Self::basic_layout(image::Viewer::new(state, image.clone()))
                }
                ImageType::Vector(image) => Self::basic_layout(image.clone()),
            },
            ImageBox::Errored(..) => Self::basic_layout(Text::new("加载失败").size(40)),
        }
    }

    async fn load() -> Result<ImageBox, Error> {
        let image_path = format!("{}/images/tiger.svg", env!("CARGO_MANIFEST_DIR"));

        if image_path.ends_with(".svg") {
            Ok(ImageBox::Loaded {
                image_type: ImageType::Vector(Svg::from_path(image_path)),
            })
        } else {
            Ok(ImageBox::Loaded {
                image_type: ImageType::Bitmap(
                    image::Handle::from_path(image_path),
                    image::viewer::State::new(),
                ),
            })
        }
    }
}

#[derive(Debug, Clone)]
enum ImageType {
    Bitmap(image::Handle, image::viewer::State),
    Vector(Svg),
}

#[derive(Debug, Clone)]
enum Error {
    NotFoundError,
    OtherError,
}

fn button<'a>(state: &'a mut button::State, text: &str) -> Button<'a, Message> {
    Button::new(state, Text::new(text))
        .padding(10)
        .style(style::Button::Primary)
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }
}
