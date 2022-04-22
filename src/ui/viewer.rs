use std::path::PathBuf;

use iced::{button, Alignment, Button, Column, Container, Element, Image, Length, Row, Svg, Text};

use super::style;

#[derive(Debug, Clone)]
pub enum ViewerMessage {
    ImageLoaded((Vec<PathBuf>, Option<usize>)),
    Navigate(i32),
    CloseNotFound,
    JumpToImage(usize),
}

#[derive(Debug, Default, Clone)]
pub struct Viewer {
    //TODO:改成HashMap::<String, HashSet>的结构以缩小内存占用
    pub images: Vec<PathBuf>,
    pub on_view: Option<usize>,
    pub on_preview: Option<(usize, usize)>,

    pub previous: button::State,
    pub next: button::State,
    pub close_not_found: button::State,
    pub preview_navigators: [button::State; 8],
}

impl Viewer {
    pub fn update(&mut self, message: ViewerMessage) {
        match message {
            ViewerMessage::ImageLoaded((mut images, on_view)) => {
                let old_length = self.images.len();
                if let (Some(pre), Some(new)) = (&mut self.on_view, on_view) {
                    *pre = old_length + new;
                }
                if self.on_view.is_none() {
                    self.on_view = on_view;
                }
                self.images.append(&mut images);
                self.update_preview();
            }
            ViewerMessage::CloseNotFound => self.close(),
            ViewerMessage::Navigate(i) => self.navigate(i),
            ViewerMessage::JumpToImage(index) => {
                if let Some(on_view) = &mut self.on_view {
                    *on_view = index;
                }
                self.update_preview();
            }
        }
    }

    pub fn view(&mut self) -> Element<ViewerMessage> {
        Container::new(match self.on_view {
            Some(index) => {
                let mut row = Row::new().align_items(Alignment::Center);

                row = row.push(
                    Button::new(&mut self.previous, Text::new("<"))
                        .style(style::Button::Navigator)
                        .on_press(ViewerMessage::Navigate(-1)),
                );

                let current_image = self.images[index].as_path();
                let image_column = if current_image.exists() {
                    match current_image.extension() {
                        Some(e) if e.eq("png") || e.eq("jpg") => Column::new()
                            .push(Image::new(current_image).height(Length::FillPortion(11))),
                        Some(e) if e.eq("svg") => Column::new()
                            .push(Svg::from_path(current_image).height(Length::FillPortion(11))),

                        _ => Column::new()
                            .push(Text::new("Internal Error.").height(Length::FillPortion(11))),
                    }
                } else {
                    Column::new()
                        .push(Text::new("Not Found. Maybe Deleted."))
                        .push(
                            Button::new(&mut self.close_not_found, Text::new("Close"))
                                .style(style::Button::Entry)
                                .on_press(ViewerMessage::CloseNotFound),
                        )
                }
                .spacing(7)
                .align_items(Alignment::Center)
                .push(Row::new().height(Length::Units(35)).push(Text::new(format!(
                    "{} / {}",
                    index + 1,
                    self.images.len()
                ))))
                .width(Length::FillPortion(8));

                row.push(image_column)
                    .push(
                        Button::new(&mut self.next, Text::new(">"))
                            .style(style::Button::Navigator)
                            .on_press(ViewerMessage::Navigate(1)),
                    )
                    .push(if let Some((start, end)) = self.on_preview {
                        //用迭代器
                        self.images[start..end]
                            .iter()
                            .enumerate()
                            .zip(self.preview_navigators.iter_mut())
                            .fold(
                                Column::new()
                                    .width(Length::FillPortion(1))
                                    .spacing(10)
                                    .padding(20)
                                    .align_items(Alignment::Center),
                                |acc, ((i, image), state)| {
                                    let mut preview_button = match image.as_path().extension() {
                                        Some(e) if e.eq("png") || e.eq("jpg") => {
                                            Button::new(state, Image::new(image))
                                        }
                                        Some(e) if e.eq("svg") => {
                                            Button::new(state, Svg::from_path(image))
                                        }
                                        _ => Button::new(state, Image::new("assets/blank.png")),
                                    }
                                    .style(style::Button::PreviewNavigator);

                                    if i + start != index {
                                        preview_button = preview_button
                                            .on_press(ViewerMessage::JumpToImage(i + start));
                                    }

                                    acc.push(preview_button)
                                },
                            )

                        //循环
                        // for image in self.images[start..end + 1].iter().enumerate() {
                        //     match image.as_path().extension() {
                        //         Some(e) if e.eq("png") || e.eq("jpg") => {
                        //             preview_navigators = preview_navigators.push(Image::new(image))
                        //         }
                        //         Some(e) if e.eq("svg") => {
                        //             preview_navigators = preview_navigators.push(Svg::from_path(image))
                        //         }

                        //         _ => preview_navigators = preview_navigators.push(Text::new("Internal Error.")),
                        //     }
                        // }
                    } else {
                        Column::new()
                            .width(Length::FillPortion(1))
                            .spacing(10)
                            .padding(20)
                            .align_items(Alignment::Center)
                    })
            }
            None => Row::new().push(Text::new("Pick an image.")),
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    #[inline]
    pub fn navigate(&mut self, i: i32) {
        if let Some(index) = &mut self.on_view {
            //如果是一个一个切换，则允许从第一个跳到最后一个
            let target = *index as i32 + i;
            if 0 <= target && target < self.images.len() as i32 {
                *index = target as usize;
            } else if i == -1 || i > 1 {
                *index = self.images.len() - 1;
            } else {
                *index = 0;
            }
        }
        self.update_preview();
    }

    #[inline]
    pub fn close(&mut self) {
        if let Some(index) = &mut self.on_view {
            self.images.remove(*index);
            if !self.images.is_empty() {
                *index %= self.images.len();
            }
        }

        if self.images.is_empty() {
            self.on_view = None;
        }
        self.update_preview();
    }

    #[inline]
    pub fn clear(&mut self) {
        self.images.clear();
        self.on_view = None;
        self.update_preview();
    }

    #[inline]
    fn update_preview(&mut self) {
        if let Some(center) = self.on_view {
            self.on_preview = Some(get_centered_slice(
                &self.images,
                self.preview_navigators.len(),
                center,
            ));
        }
    }
}

fn get_centered_slice<T>(array: &[T], slice_len: usize, center: usize) -> (usize, usize) {
    let len = array.len();

    let mut start = center;
    let mut end = center;

    while end - start < slice_len && (start > 0 || end < len) {
        if start > 0 {
            start -= 1;
        }
        if end < len {
            end += 1;
        }
    }

    //左闭右开
    (start, end)
}
