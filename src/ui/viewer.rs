use std::path::PathBuf;

use iced::{
    button, Alignment, Button, Column, Container, Element, Image, Length, Row, Space, Svg, Text,
};

use super::style;

#[derive(Debug, Clone)]
pub enum ViewerMessage {
    ImageLoaded((Vec<PathBuf>, Option<usize>)),
    NavigateBack,
    NavigateNext,
    CloseNotFound,
    JumpToImage(usize),
}

#[derive(Debug, Default, Clone)]
pub struct Viewer {
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
                match (&mut self.on_view, on_view) {
                    (Some(pre), Some(new)) => *pre = old_length + new,
                    _ => {}
                }
                if self.on_view.is_none() {
                    self.on_view = on_view;
                }
                self.images.append(&mut images);
            }
            ViewerMessage::CloseNotFound => self.close(),
            ViewerMessage::NavigateBack => {
                if let Some(index) = &mut self.on_view {
                    if *index > 0 {
                        *index -= 1;
                    } else {
                        *index = self.images.len() - 1;
                    }
                }
            }
            ViewerMessage::NavigateNext => {
                if let Some(index) = &mut self.on_view {
                    *index += 1;
                    *index %= self.images.len();
                }
            }
            ViewerMessage::JumpToImage(index) => {
                if let Some(on_view) = &mut self.on_view {
                    *on_view = index;
                }
            }
        }
        self.update_preview();
    }

    pub fn view(&mut self) -> Element<ViewerMessage> {
        Container::new(match self.on_view {
            Some(index) => {
                let mut row = Row::new().align_items(Alignment::Center);

                row = row.push(
                    Button::new(&mut self.previous, Text::new("<"))
                        .style(style::Button::Navigator)
                        .on_press(ViewerMessage::NavigateBack),
                );

                let current_image = self.images[index].as_path();
                let image_column = if current_image.exists() {
                    match current_image.extension() {
                        Some(e) if e.eq("png") || e.eq("jpg") => {
                            Column::new().push(Image::new(current_image))
                        }
                        Some(e) if e.eq("svg") => Column::new().push(Svg::from_path(current_image)),

                        _ => Column::new().push(Text::new("Internal Error.")),
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
                .width(Length::Fill)
                .spacing(7)
                .align_items(Alignment::Center)
                .push(Text::new(format!("{} / {}", index + 1, self.images.len())))
                .push(Row::new().height(Length::Units(50)))
                .push(Space::with_height(Length::Units(10)));

                row.push(image_column)
                    .push(
                        Button::new(&mut self.next, Text::new(">"))
                            .style(style::Button::Navigator)
                            .on_press(ViewerMessage::NavigateNext),
                    )
                    .push(if let Some((start, end)) = self.on_preview {
                        //用迭代器
                        self.images[start..end]
                            .iter()
                            .enumerate()
                            .zip(self.preview_navigators.iter_mut())
                            .fold(
                                Column::new()
                                    .spacing(10)
                                    .padding(20)
                                    .align_items(Alignment::Center),
                                |acc, ((i, image), state)| {
                                    let mut preview_button = match image.as_path().extension() {
                                        Some(e) if e.eq("png") || e.eq("jpg") => Button::new(
                                            state,
                                            Image::new(image)
                                                .width(Length::Units(70))
                                                .height(Length::Units(70)),
                                        ),
                                        Some(e) if e.eq("svg") => Button::new(
                                            state,
                                            Svg::from_path(image)
                                                .width(Length::Units(70))
                                                .height(Length::Units(70)),
                                        ),
                                        _ => Button::new(
                                            state,
                                            Image::new("assets/blank.png")
                                                .width(Length::Units(70))
                                                .height(Length::Units(70)),
                                        ),
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
    }

    pub fn clear(&mut self) {
        self.images.clear();
        self.on_view = None;
    }

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
