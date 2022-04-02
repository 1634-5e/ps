use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use iced::{
    button, Alignment, Button, Column, Command, Container, Element, Image, Length, Row, Svg, Text,
};

use crate::{io::dialogs::open, Flags, Message, UserSettings};

use super::style;

#[derive(Debug, Clone)]
pub enum ViewerMessage {
    NavigateBack,
    NavigateNext,
    CloseNotFound,
}

#[derive(Debug, Default, Clone)]
pub struct Viewer {
    previous: button::State,
    next: button::State,
    close_not_found: button::State,
    is_loading: bool,
    index: String,
}

impl Viewer {
    fn new(flags: &mut Flags) -> (Viewer, Command<Message>) {
        let command = match flags.user_settings.try_borrow() {
            Ok(us) => Command::perform(
                open(flags.env_args[1..].to_vec(), us.automatic_load),
                Message::ImageLoaded,
            ),
            Err(_) => Command::none(),
        };
        (Viewer::default(), command)
    }

    fn view(
        &mut self,
        _settings: Rc<RefCell<UserSettings>>,
        image: Option<PathBuf>,
    ) -> Element<ViewerMessage> {
        Container::new(if self.is_loading {
            Row::new().push(Text::new("Loading..."))
        } else {
            match image {
                Some(i) => {
                    let mut row = Row::new().align_items(Alignment::Center);

                    row = row.push(
                        Button::new(&mut self.previous, Text::new("<"))
                            .style(style::Button::Navigator)
                            .on_press(ViewerMessage::NavigateBack),
                    );

                    let current_image = i.as_path();
                    let image_column = if current_image.exists() {
                        match current_image.extension() {
                            Some(e) if e.eq("png") || e.eq("jpg") => {
                                Column::new().push(Image::new(current_image))
                            }
                            Some(e) if e.eq("svg") => {
                                Column::new().push(Svg::from_path(current_image))
                            }

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
                    .align_items(Alignment::Center)
                    .push(Text::new(self.index.as_str()));

                    row.push(
                        Button::new(&mut self.next, Text::new(">"))
                            .style(style::Button::Navigator)
                            .on_press(ViewerMessage::NavigateNext),
                    )
                }
                None => Row::new().push(Text::new("Pick an image.")),
            }
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}
