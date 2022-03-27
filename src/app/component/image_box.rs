use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use iced::{button, image, Button, Command, Element, Length, Row, Svg, Text};

use super::{Component, ToolbarButton};
use crate::app::error::Error;
use crate::app::file_dialog::{pick as pick_in_dialog, DialogType};
use crate::app::{message::ImageBoxMessage, Flags, UserSettings};
use crate::common::button::{entry, navigator};
use crate::common::custom_element::column_with_blanks;

// 展示图片以及未来的编辑区域
//因为toolbar触发的事件经常会跟imagebox里的东西相关，所以在考虑是否合并

//使用iced-widget-canvas，只需要一个struct、impl canvas
//参照bezier_tool example
#[derive(Debug)]
pub struct ImageBox {
    buttons: Buttons,
    images: Vec<ImageData>,
    current: usize,
    status: Status,
}

#[derive(Default, Debug, Clone)]
pub struct Buttons {
    open_image: button::State,
    open_dir: button::State,
    previous: button::State,
    next: button::State,
    close_this: ToolbarButton,
    close_all: ToolbarButton,
    new: ToolbarButton,
}

#[derive(Debug, Clone)]
pub struct ImageData {
    content: ImageType,
    path: PathBuf,
}

#[derive(Default, Debug, Clone, Copy)]
enum Status {
    #[default]
    Loading,
    View,
    Errored(Error),
}

#[derive(Debug, Clone)]
enum ImageType {
    Bitmap(image::Handle, image::viewer::State),
    Vector(Svg),
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
            .padding(20);
        let main_content = match self.status {
            Status::Loading => basic_layout.push(Text::new("Loading...")).into(),
            Status::View => {
                if self.images.is_empty() {
                    basic_layout
                        .push(
                            entry(&mut self.buttons.open_image, "Open an image")
                                .on_press(ImageBoxMessage::PickImage(DialogType::File)),
                        )
                        .push(
                            entry(&mut self.buttons.open_dir, "Directory")
                                .on_press(ImageBoxMessage::PickImage(DialogType::Dir)),
                        )
                        .into()
                } else {
                    basic_layout = basic_layout.push(column_with_blanks(
                        navigator(&mut self.buttons.previous, "<")
                            .on_press(ImageBoxMessage::Navigate(Navigate::Previous)),
                        1,
                        1,
                    ));
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
                        .push(column_with_blanks(
                            navigator(&mut self.buttons.next, ">")
                                .on_press(ImageBoxMessage::Navigate(Navigate::Next)),
                            1,
                            1,
                        ))
                        .into()
                }
            }
            Status::Errored(e) => Row::new().push(Text::new(e.explain())).into(),
        };

        (
            main_content,
            Self::toolbar(
                self.buttons
                    .close_this
                    .view("close this", ImageBoxMessage::CloseThis),
                self.buttons
                    .close_all
                    .view("close all", ImageBoxMessage::CloseAll),
                self.buttons.new.view("new", ImageBoxMessage::New),
            ),
        )
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
        if self.images.is_empty() {
            self.current = 0;
        } else {
            self.current = self.current % self.images.len();
        }
    }

    pub fn close_all(&mut self) {
        self.images.clear();
        self.current = 0;
    }

    pub fn toolbar<'a>(
        close_this: Button<'a, ImageBoxMessage>,
        close_all: Button<'a, ImageBoxMessage>,
        new: Button<'a, ImageBoxMessage>,
    ) -> Element<'a, ImageBoxMessage> {
        Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(close_this)
            .push(close_all)
            .push(new)
            .into()
    }
}

//FIXME:由于iced内部Event:window，对应的FileDropped事件，每一次只有一个path，也就是说，尽管多选的时候是有序的，放进去的时候顺序却是随机的
pub async fn open(
    path: Vec<PathBuf>,
    automatic_load: bool,
) -> Result<(Vec<ImageData>, usize), Error> {
    //要处理两个情况，
    //1：用户使用按钮打开文件或者文件夹，目前还只能打开单个文件/文件夹
    //2：用户使用拖拽方式打开，这时可能有多个路径需要处理

    let mut images = vec![];
    let mut current = 0;
    for p in path {
        if p.is_dir() || automatic_load {
            let picked = p.clone();
            let parent;
            if p.is_dir() {
                parent = p.as_path();
            } else {
                parent = match p.parent() {
                    Some(pt) => pt,
                    None => {
                        return Err(Error::ReadFileError);
                    }
                };
            }

            for entry in parent.read_dir()? {
                match entry {
                    Ok(d) => {
                        let path = d.path();
                        match get_image_data_by_extension(d.path()) {
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
        } else {
            if let Some(content) = get_image_data_by_extension(p) {
                images.push(content);
            };
        }
    }
    Ok((images, current))
}

fn get_image_data_by_extension(path: PathBuf) -> Option<ImageData> {
    let ext = path.extension()?;
    if ext.eq("png") {
        return Some(ImageData {
            content: ImageType::Bitmap(
                image::Handle::from_path(path.clone()),
                image::viewer::State::new(),
            ),
            path,
        });
    }
    if ext.eq("svg") {
        return Some(ImageData {
            content: ImageType::Vector(Svg::from_path(path.clone())),
            path,
        });
    }
    None
}
