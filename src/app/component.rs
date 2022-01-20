use std::path::PathBuf;

use super::message::MainPageMessage;
use super::{file_dialog, message::ImageBoxMessage, UserSettings};
use crate::common::button::{entry, toolbar};
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
    images: Vec<ImageData>,
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
pub struct ImageData {
    content: ImageType,
    path: PathBuf,
}

#[derive(Debug, Clone)]
enum ImageType {
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
            match &mut self.images[self.current].content {
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
                    if self.current < self.images.len() {
                        self.images.remove(self.current);
                    }
                }
            }
            ImageBoxMessage::Navigate(n) => {
                self.navigate(n);
            }
            ImageBoxMessage::PickImage(dp) => match Self::pick_image(dp) {
                Some(path) => {
                    self.loading = true;
                    return Command::perform(
                        Self::load(path, settings.automatic_load),
                        ImageBoxMessage::ImageLoaded,
                    );
                }
                None => {}
            },
        }
        Command::none()
    }
}

impl ImageBox {
    //这东西虽然严格上不需要，但是补充了逻辑
    fn pick_image(dialog_type: file_dialog::DialogType) -> Option<file_dialog::PathBuf> {
        file_dialog::pick(dialog_type)
    }

    pub async fn load(
        path: file_dialog::PathBuf,
        automatical_load: bool,
    ) -> (Vec<ImageData>, usize) {
        if path.is_dir() || automatical_load {
            Self::automatic(path)
        } else {
            Self::strict(path)
        }
    }

    fn content(path: PathBuf) -> Option<ImageData> {
        match path.extension() {
            Some(ext) => match ext.to_str() {
                Some(s) => {
                    if s == "png" {
                        return Some(ImageData {
                            content: ImageType::Bitmap(
                                image::Handle::from_path(path.clone()),
                                image::viewer::State::new(),
                            ),
                            path,
                        });
                    }
                    if s == "svg" {
                        return Some(ImageData {
                            content: ImageType::Vector(Svg::from_path(path.clone())),
                            path,
                        });
                    }
                }
                None => {}
            },
            None => {}
        }

        None
    }

    //只有不是文件夹且没开选项才会用到，因此简化了许多
    fn strict(path: file_dialog::PathBuf) -> (Vec<ImageData>, usize) {
        match Self::content(path) {
            Some(content) => (vec![content], 0),
            None => (vec![], 0),
        }
    }

    //直接
    fn automatic(path: file_dialog::PathBuf) -> (Vec<ImageData>, usize) {
        let picked = path.clone();
        let mut current: usize = 0;
        let mut images: Vec<ImageData> = vec![];
        let parent = if path.is_dir() {
            path.as_path()
        } else {
            match path.parent() {
                Some(p) => p,
                None => return (vec![], 0), //FIXME:这里可能是parent获取值？（应该不会，没有提示类型冲突）
            }
        };
        for entry in parent.read_dir().expect("failed") {
            match entry {
                Ok(d) => {
                    let path = d.path();
                    match Self::content(d.path()) {
                        Some(i) => {
                            images.push(i);
                        }
                        None => {}
                    }
                    if path == picked {
                        current = images.len() - 1;
                    }
                }
                Err(_) => {}
            }
        }

        (images, current)
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
//TODO:像close这种按钮需要有禁用的情况，目前貌似不自带，得自己手动实现。。
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
                .push(toolbar(&mut self.close_this, "close this").on_press(
                    MainPageMessage::ImageBoxMessage(ImageBoxMessage::CloseImage { whole: false }),
                ))
                .push(toolbar(&mut self.close_all, "close all").on_press(
                    MainPageMessage::ImageBoxMessage(ImageBoxMessage::CloseImage { whole: true }),
                )),
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
