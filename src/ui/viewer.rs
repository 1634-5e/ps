use std::path::PathBuf;

use iced::{button, Alignment, Button, Column, Container, Element, Image, Length, Row, Svg, Text};

use super::style;

#[derive(Debug, Clone)]
pub enum ViewerMessage {
    ImageLoaded((Vec<PathBuf>, Option<usize>)),
    NavigateBack,
    NavigateNext,
    CloseNotFound,
}

#[derive(Debug, Default, Clone)]
pub struct Viewer {
    pub images: Vec<PathBuf>,
    pub on_view: Option<usize>,

    pub previous: button::State,
    pub next: button::State,
    pub close_not_found: button::State,
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
        }
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
                .padding(5)
                .align_items(Alignment::Center)
                .push(Text::new(format!("{} / {}", index + 1, self.images.len())));

                row.push(image_column).push(
                    Button::new(&mut self.next, Text::new(">"))
                        .style(style::Button::Navigator)
                        .on_press(ViewerMessage::NavigateNext),
                )
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
}
