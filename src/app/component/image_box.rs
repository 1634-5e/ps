use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use iced::{button, image, Alignment, Button, Column, Command, Element, Length, Row, Svg, Text};

use super::{Component, ControllableButton};
use crate::app::error::Error;
use crate::app::file_dialog::{pick as pick_in_dialog, DialogType};
use crate::app::{message::ImageBoxMessage, Flags, UserSettings};
use crate::common::custom_element::{column_with_spaces, row_with_spaces};
use crate::common::style;

// 展示图片以及未来的编辑区域
//因为toolbar触发的事件经常会跟imagebox里的东西相关，所以在考虑是否合并

//使用iced-widget-canvas，只需要一个struct、impl canvas
//参照bezier_tool example
#[derive(Debug)]
pub struct ImageBox {
    buttons: Buttons,
    images: Vec<PathBuf>,
    current: usize,
    status: Status,
    image_viewer: image::viewer::State,
}

#[derive(Default, Debug, Clone)]
pub struct Buttons {
    open_image: button::State,
    open_dir: button::State,
    previous: ControllableButton,
    next: ControllableButton,
    close_this: ControllableButton,
    close_all: ControllableButton,
    new: ControllableButton,
}

#[derive(Debug, Clone)]
enum ImageType {
    Bitmap(image::Handle),
    Vector(Svg),
}

#[derive(Default, Debug, Clone, Copy)]
enum Status {
    #[default]
    Loading,
    View,
    Errored(Error),
}

//TODO: 加入滚轮,用于切换图片
#[derive(Debug, Clone)]
pub enum Navigate {
    Previous,
    Next,
}

impl Component for ImageBox {
    type Message = ImageBoxMessage;

    fn new(flags: &mut Flags) -> (ImageBox, Command<ImageBoxMessage>) {
        let mut image_box = ImageBox {
            buttons: Buttons::default(),
            images: vec![],
            current: 0,
            status: Status::Loading,
            image_viewer: image::viewer::State::new(),
        };
        let command = match flags.user_settings.try_borrow() {
            Ok(us) => Command::perform(
                open(flags.env_args[1..].to_vec(), us.automatic_load),
                ImageBoxMessage::ImageLoaded,
            ),
            Err(e) => {
                image_box.status = Status::Errored(e.into());
                Command::none()
            }
        };
        (image_box, command)
    }

    fn view(
        &mut self,
        _settings: Rc<RefCell<UserSettings>>,
    ) -> (Element<ImageBoxMessage>, Element<ImageBoxMessage>) {
        let mut basic_layout = Row::new()
            .width(Length::FillPortion(5))
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .padding(10);
        let main_content = match self.status {
            Status::Loading => basic_layout
                .push(row_with_spaces(Text::new("Loading..."), 1, 1).align_items(Alignment::Center))
                .into(),
            Status::View => {
                if self.images.is_empty() {
                    basic_layout
                        .push(row_with_spaces(
                            Row::new()
                                .push(
                                    Button::new(
                                        &mut self.buttons.open_image,
                                        Text::new("Open an image"),
                                    )
                                    .style(style::Button::Entry)
                                    .on_press(ImageBoxMessage::PickImage(DialogType::File)),
                                )
                                .push(
                                    Button::new(&mut self.buttons.open_dir, Text::new("Directory"))
                                        .style(style::Button::Entry)
                                        .on_press(ImageBoxMessage::PickImage(DialogType::Dir)),
                                ),
                            1,
                            1,
                        ))
                        .into()
                } else {
                    basic_layout = basic_layout.push(column_with_spaces(
                        self.buttons.previous.view(
                            Text::new("<"),
                            style::Button::Navigator,
                            ImageBoxMessage::Navigate(Navigate::Previous),
                        ),
                        1,
                        1,
                    ));

                    let image_type = get_image_data_by_extension(&self.images[self.current]);
                    let image_column = match image_type {
                        Some(i) => match i {
                            ImageType::Bitmap(image) => Column::new()
                                .align_items(Alignment::Center)
                                .width(Length::FillPortion(8))
                                .push(
                                    image::Viewer::new(&mut self.image_viewer, image)
                                        .width(Length::Fill)
                                        .height(Length::Fill),
                                ),
                            ImageType::Vector(image) => {
                                Column::new().align_items(Alignment::Center).push(image)
                            }
                        },
                        None => {
                            //这个正常情况应该不可能出现
                            column_with_spaces(Text::new("Not a supported image file"), 1, 1)
                                .width(Length::Fill)
                                .align_items(Alignment::Center)
                        }
                    }
                    .push(Text::new(format!(
                        "{} / {}",
                        self.current + 1,
                        self.images.len()
                    )));

                    basic_layout
                        .push(image_column)
                        .push(column_with_spaces(
                            self.buttons.next.view(
                                Text::new(">"),
                                style::Button::Navigator,
                                ImageBoxMessage::Navigate(Navigate::Next),
                            ),
                            1,
                            1,
                        ))
                        .into()
                }
            }
            Status::Errored(e) => Row::new().push(Text::new(e.explain())).into(),
        };

        let toolbar = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(self.buttons.close_this.view(
                Text::new("close this"),
                style::Button::Toolbar,
                ImageBoxMessage::CloseThis,
            ))
            .push(self.buttons.close_all.view(
                Text::new("close all"),
                style::Button::Toolbar,
                ImageBoxMessage::CloseAll,
            ))
            .push(self.buttons.new.view(
                Text::new("New"),
                style::Button::Toolbar,
                ImageBoxMessage::New,
            ))
            .into();

        (main_content, toolbar)
    }

    fn update(
        &mut self,
        message: ImageBoxMessage,
        settings: Rc<RefCell<UserSettings>>,
    ) -> Command<ImageBoxMessage> {
        match message {
            ImageBoxMessage::ImageLoaded(res) => match res {
                Ok((mut images, current)) => {
                    self.current = current + self.images.len();
                    self.images.append(&mut images);
                    self.status = Status::View;
                    if self.images.len() > 1 {
                        self.buttons.previous.disabled = false;
                        self.buttons.next.disabled = false;
                    }
                }
                Err(e) => {
                    self.status = Status::Errored(e.into());
                }
            },
            ImageBoxMessage::Navigate(n) => {
                self.navigate(n);
            }
            ImageBoxMessage::PickImage(dp) => match pick_in_dialog(dp) {
                Some(path) => {
                    self.status = Status::Loading;
                    match settings.try_borrow() {
                        Ok(settings) => {
                            return Command::perform(
                                open(vec![path], settings.automatic_load),
                                ImageBoxMessage::ImageLoaded,
                            );
                        }
                        Err(e) => self.status = Status::Errored(e.into()),
                    }
                }
                None => {}
            },
            ImageBoxMessage::CloseThis => self.close_this(),
            ImageBoxMessage::CloseAll => self.close_all(),
            _ => {}
        }
        Command::none()
    }
}

impl ImageBox {
    //这里的卡顿比较的明显，优化之后速度可以接受，但是根本的问题——不缓存，没有解决
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

    pub fn close_this(&mut self) {
        if self.current < self.images.len() {
            self.images.remove(self.current);
        }
        if self.images.len() <= 1 {
            self.current = 0;
            self.buttons.previous.disabled = true;
            self.buttons.next.disabled = true;
        } else {
            self.current = self.current % self.images.len();
        }
    }

    pub fn close_all(&mut self) {
        self.images.clear();
        self.current = 0;
    }
}

//FIXME:由于iced内部Event:window，对应的FileDropped事件，每一次只有一个path，也就是说，尽管多选的时候是有序的，放进去的时候顺序却是随机的
pub async fn open(
    paths: Vec<PathBuf>,
    automatic_load: bool,
) -> Result<(Vec<PathBuf>, usize), Error> {
    //要处理两个情况，
    //1：用户使用按钮打开文件或者文件夹，目前还只能打开单个文件/文件夹
    //2：用户使用拖拽方式打开，这时可能有多个路径需要处理

    let mut images = vec![];
    let mut current = 0;
    for path in paths {
        if path.is_dir() || automatic_load {
            let parent;
            if path.is_dir() {
                parent = path.as_path();
            } else {
                parent = match path.parent() {
                    Some(pt) => pt,
                    None => {
                        return Err(Error::ReadFileError);
                    }
                };
            }

            for entry in parent.read_dir()? {
                match entry {
                    Ok(d) => {
                        let p = d.path();
                        match p.extension() {
                            Some(e) if (e.eq("png") || e.eq("svg")) => {
                                if p == path {
                                    current = images.len();
                                }
                                images.push(p);
                            }
                            _ => {}
                        }
                    }
                    Err(_) => {}
                }
            }
        } else {
            images.push(path);
        }
    }
    Ok((images, current))
}

fn get_image_data_by_extension(path: &PathBuf) -> Option<ImageType> {
    let ext = path.extension()?;
    if ext.eq("png") {
        return Some(ImageType::Bitmap(image::Handle::from_path(path.clone())));
    }
    if ext.eq("svg") {
        return Some(ImageType::Vector(Svg::from_path(path.clone())));
    }
    None
}
