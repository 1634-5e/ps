use super::message::MainPageMessage;
use super::{file_dialog, message::ImageBoxMessage, UserSettings};
use crate::common::button::entry;
use crate::common::custom_box::row_with_blanks;
use crate::common::style;
use iced::{button, image, Alignment, Command, Element, Length, Row, Svg, Text};
use iced::{Button, Column};

//构成页面的组件，会有上方的按钮区，左下的显示区，和右下的区域
pub trait Component {
    type Message;

    fn new() -> Self;
    fn view(&mut self, settings: &UserSettings) -> Element<Self::Message>;
    fn update(
        &mut self,
        message: Self::Message,
        settings: &mut UserSettings,
    ) -> Command<Self::Message>;
}

// 展示图片以及未来的编辑区域
//因为toolbar触发的事件经常会跟imagebox里的东西相关，所以在考虑是否合并
#[derive(Debug, Clone)]
pub struct ImageBox {
    buttons: Buttons,
    images: Vec<ImageType>,
    current: usize,
    loading: bool,
}

#[derive(Debug, Clone)]
pub struct Buttons {
    open_image: button::State,
    open_dir: button::State,
    previous: button::State,
    next: button::State,
}

impl Buttons {
    pub fn new() -> Buttons {
        Buttons {
            previous: button::State::new(),
            next: button::State::new(),
            open_image: button::State::new(),
            open_dir: button::State::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct ImageBoxSettings {
    automatic_load: bool,
}

#[derive(Debug, Clone)]
pub enum ImageType {
    Bitmap(image::Handle, image::viewer::State),
    Vector(Svg),
}

impl Component for ImageBox {
    type Message = ImageBoxMessage;

    fn new() -> ImageBox {
        ImageBox {
            buttons: Buttons::new(),
            images: vec![],
            current: 0,
            loading: false,
        }
    }

    fn view(&mut self, settings: &UserSettings) -> Element<ImageBoxMessage> {
        let mut basic_layout = Row::new()
            .width(Length::FillPortion(5))
            .height(Length::Fill)
            .padding(20);
        if self.loading {
            return basic_layout.push(Text::new("Loading...")).into();
        }
        if self.images.is_empty() {
            basic_layout
                .push(
                    entry(&mut self.buttons.open_image, "Open an image")
                        .on_press(ImageBoxMessage::PickImage(file_dialog::DialogType::File)),
                )
                .push(
                    entry(&mut self.buttons.open_dir, "Directory")
                        .on_press(ImageBoxMessage::PickImage(file_dialog::DialogType::Dir)),
                )
                .into()
        } else {
            basic_layout = basic_layout.push(
                navigator(&mut self.buttons.previous, "<")
                    .on_press(ImageBoxMessage::Navigate(Navigate::Previous)),
            );
            match &mut self.images[self.current] {
                ImageType::Bitmap(image, state) => {
                    basic_layout = basic_layout
                        .push(
                            image::Viewer::new(state, image.clone())
                                .width(Length::Fill)
                                .height(Length::Fill),
                        )
                        .into()
                }
                ImageType::Vector(image) => basic_layout = basic_layout.push(image.clone()),
            }
            basic_layout
                .push(
                    navigator(&mut self.buttons.next, ">")
                        .on_press(ImageBoxMessage::Navigate(Navigate::Next)),
                )
                .into()
        }
    }

    fn update(
        &mut self,
        message: ImageBoxMessage,
        settings: &mut UserSettings,
    ) -> Command<ImageBoxMessage> {
        match message {
            ImageBoxMessage::ImageLoaded((images, current)) => {
                self.images = images.clone();
                self.current = current;
                self.loading = false;
            }
            ImageBoxMessage::CloseImage { whole } => {
                if whole {
                    self.images.clear();
                } else {
                    self.images.remove(self.current);
                }
            }
            ImageBoxMessage::Navigate(n) => {
                let len = self.images.len();
                match n {
                    Navigate::Next => {
                        self.current += 1;
                    }
                    Navigate::Previous => {
                        self.current += len - 1;
                    }
                }
                self.current %= len;
            }
            ImageBoxMessage::PickImage(dp) => {
                self.loading = true;
                let path = Self::pick_image(dp).unwrap();
                return Command::perform(
                    Self::load(path, settings.automatic_load),
                    ImageBoxMessage::ImageLoaded,
                );
            }
        }
        Command::none()
    }
}

impl ImageBox {
    //看不少软件都是打开一个图片自动就加载了同级及以下的其他图片。这里大概会做成一个选项。
    pub async fn load(path: file_dialog::PathBuf, mode: bool) -> (Vec<ImageType>, usize) {
        let picked = path.clone();
        let mut current: usize = 0;

        let paths = match mode {
            true => Self::strict(path),
            false => Self::automatic(path),
        };

        let mut images: Vec<ImageType> = vec![];

        paths.into_iter().enumerate().for_each(|(i, path)| {
            if path == picked {
                current = i;
            }
            match path.extension() {
                Some(ext) => {
                    if ext == "svg" {
                        images.push(ImageType::Vector(Svg::from_path(path)));
                    } else {
                        images.push(ImageType::Bitmap(
                            image::Handle::from_path(path),
                            image::viewer::State::new(),
                        ));
                    }
                }
                None => {}
            }
        });
        (images, current)
    }

    fn strict(path: file_dialog::PathBuf) -> Vec<file_dialog::PathBuf> {
        let mut res: Vec<file_dialog::PathBuf> = vec![];
        if path.is_dir() {
            for entry in path.read_dir().expect("failed") {
                res.push(entry.unwrap().path());
            }
        } else {
            res.push(path);
        }
        res
    }

    fn automatic(path: file_dialog::PathBuf) -> Vec<file_dialog::PathBuf> {
        let mut res: Vec<file_dialog::PathBuf> = vec![];
        if path.is_dir() {
            for entry in path.read_dir().expect("failed") {
                res.push(entry.unwrap().path());
            }
        } else {
            res.push(path);
        }
        res
    }

    //这东西虽然严格上不需要，但是补充了逻辑
    fn pick_image(dialog_type: file_dialog::DialogType) -> Option<file_dialog::PathBuf> {
        file_dialog::pick(dialog_type)
    }

    //这里的卡顿比较的明显
    //TODO:让其他的图片隐藏而不是删除
    pub fn navigate(&mut self, navigate: Navigate) {
        let len = self.images.len();
        match navigate {
            Navigate::Previous => {
                self.current += len - 1;
                self.current %= len;
            }
            Navigate::Next => {
                self.current += 1;
                self.current %= len;
            }
        }
    }
}

//TODO: 加入滚轮
#[derive(Debug, Clone)]
pub enum Navigate {
    Previous,
    Next,
}

fn navigator<'a>(state: &'a mut button::State, text: &str) -> Button<'a, ImageBoxMessage> {
    Button::new(state, Text::new(text))
        .height(Length::Fill)
        .padding(10)
        .style(style::Button::Navigator)
}

//这里面按钮绑定的事件比较宽泛，所以内联的message是主页的
pub struct ToolBar {
    close_this: button::State,
    close_all: button::State,
    pub settings: button::State,
}

impl Component for ToolBar {
    type Message = MainPageMessage;

    fn new() -> ToolBar {
        ToolBar {
            close_this: button::State::new(),
            close_all: button::State::new(),
            settings: button::State::new(),
        }
    }

    fn view(&mut self, settings: &UserSettings) -> Element<MainPageMessage> {
        let settings_button = row_with_blanks(
            Row::new().align_items(Alignment::Center).push(
                entry(&mut self.settings, "settings").on_press(MainPageMessage::GoToSettings),
            ),
            1,
            0,
        )
        .width(Length::FillPortion(2));

        let function_buttons = Row::new().height(Length::FillPortion(1)).push(
            Column::new()
                .push(
                    Button::new(&mut self.close_this, Text::new("close this")).on_press(
                        MainPageMessage::ImageBoxMessage(ImageBoxMessage::CloseImage {
                            whole: false,
                        }),
                    ),
                )
                .push(
                    Button::new(&mut self.close_all, Text::new("close all")).on_press(
                        MainPageMessage::ImageBoxMessage(ImageBoxMessage::CloseImage {
                            whole: true,
                        }),
                    ),
                ),
        );

        function_buttons.push(settings_button).into()
    }

    fn update(
        &mut self,
        message: Self::Message,
        settings: &mut UserSettings,
    ) -> Command<MainPageMessage> {
        Command::none()
    }
}
