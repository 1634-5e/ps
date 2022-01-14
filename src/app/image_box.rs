use super::super::common::button::{navigator, toolbar};
use super::app::Message;
use super::error;
use super::file_dialog;
use super::settings;
use iced::{button, image, Column, Container, Element, Length, Svg, Text};

// 展示图片以及未来的编辑区域
#[derive(Debug, Clone)]
pub enum ImageBox {
    Init {
        single: button::State,
        dir: button::State,
    },
    Loading,
    Loaded {
        image_type: Vec<ImageType>,
        current: usize,
        mode: super::settings::LoadMode,
    },
    Errored(error::Error),
}

impl<'a> ImageBox {
    fn basic_layout<E>(content: E) -> Element<'a, Message>
    where
        E: Into<Element<'a, Message>>,
    {
        Container::new(content)
            .width(Length::FillPortion(5))
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }

    pub fn view(&mut self) -> Element<Message> {
        match self {
            ImageBox::Init { single, dir } => Self::basic_layout(
                Column::new()
                    .push(
                        toolbar(single, "Open an image")
                            .on_press(Message::PickImage(file_dialog::DialogType::File)),
                    )
                    .push(
                        toolbar(dir, "Directory")
                            .on_press(Message::PickImage(file_dialog::DialogType::Dir)),
                    ),
            ),
            ImageBox::Loading => Self::basic_layout(Text::new("Loading...").size(40)),
            ImageBox::Loaded {
                image_type,
                current,
                ..
            } => {
                if image_type.len() == 0 {
                    Self::basic_layout(Text::new("Empty Folder."))
                } else {
                    match &mut image_type[current.clone()] {
                        ImageType::Bitmap(image, state) => {
                            Self::basic_layout(image::Viewer::new(state, image.clone()))
                        }
                        ImageType::Vector(image) => Self::basic_layout(image.clone()),
                    }
                }
            }
            ImageBox::Errored(e) => match e {
                error::Error::NameInvalid => {
                    Self::basic_layout(Text::new("Name Invalid!").size(40))
                }
                error::Error::NotFound => Self::basic_layout(Text::new("Not Exist!").size(40)),
            },
        }
    }

    //看不少软件都是打开一个图片自动就加载了同级及以下的其他图片。这里大概会做成一个选项。
    pub async fn load(path: file_dialog::PathBuf, load_mode: settings::LoadMode) -> ImageBox {
        let picked = path.clone();
        let mut current: usize = 0;
        if !path.exists() {
            return ImageBox::Errored(error::Error::NotFound);
        }

        let mut paths = match load_mode {
            settings::LoadMode::Strict => Self::strict(path),
            settings::LoadMode::Automatic => Self::automatic(path),
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

        ImageBox::Loaded {
            image_type: images,
            current,
            mode: load_mode,
        }
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
    pub fn pick_image(dialog_type: file_dialog::DialogType) -> Option<file_dialog::PathBuf> {
        file_dialog::pick(dialog_type)
    }
}

#[derive(Debug, Clone)]
pub enum ImageType {
    Bitmap(image::Handle, image::viewer::State),
    Vector(Svg),
}
