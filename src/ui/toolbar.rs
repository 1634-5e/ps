use iced::pure::widget::{Button, Column, Row, Text};
use iced::pure::Element;
use iced::{Alignment, Length};

use super::icons;
use super::shape::*;
use super::style;
use super::EditMessage;

#[derive(Debug, Clone)]
pub enum ToolbarMessage {
    //view
    Close,
    ClearImages,
    New,
    Open,

    //edit
    Back,
    Export,
    Edit(EditMessage),
}

#[derive(Debug, Default, Clone)]
pub struct Toolbar {}

impl Toolbar {
    pub fn editing(&self) -> Element<ToolbarMessage> {
        Row::new()
            .padding(20)
            .spacing(7)
            .height(Length::Units(100))
            .push(button(
                icons::check(),
                "back",
                style::Button::Toolbar,
                Some(ToolbarMessage::Back),
            ))
            .push(button(
                icons::save(),
                "export",
                style::Button::Toolbar,
                Some(ToolbarMessage::Export),
            ))
            .push(button(
                icons::rectangle(),
                "line",
                style::Button::Toolbar,
                Some(ToolbarMessage::Edit(EditMessage::ChangeShape(
                    Line::default().into(),
                ))),
            ))
            .push(button(
                icons::rectangle(),
                "rect",
                style::Button::Toolbar,
                Some(ToolbarMessage::Edit(EditMessage::ChangeShape(
                    Rectangle::default().into(),
                ))),
            ))
            .push(button(
                icons::triangle(),
                "triangle",
                style::Button::Toolbar,
                Some(ToolbarMessage::Edit(EditMessage::ChangeShape(
                    Triangle::default().into(),
                ))),
            ))
            .push(button(
                icons::triangle(),
                "circle",
                style::Button::Toolbar,
                Some(ToolbarMessage::Edit(EditMessage::ChangeShape(
                    Circle::default().into(),
                ))),
            ))
            .push(button(
                icons::quadratic_bezier(),
                "Bezier",
                style::Button::Toolbar,
                Some(ToolbarMessage::Edit(EditMessage::ChangeShape(
                    QuadraticBezier::default().into(),
                ))),
            ))
            .push(button(
                icons::delete(),
                "clear",
                style::Button::Delete,
                Some(ToolbarMessage::Edit(EditMessage::Clear)),
            ))
            .into()
    }

    pub fn viewing(&self) -> Element<ToolbarMessage> {
        Row::new()
            .padding(20)
            .spacing(7)
            .height(Length::Units(100))
            .push(button(
                icons::load(),
                "open",
                style::Button::Toolbar,
                Some(ToolbarMessage::Open),
            ))
            .push(button(
                icons::delete(),
                "close",
                style::Button::Toolbar,
                Some(ToolbarMessage::Close),
            ))
            .push(button(
                icons::delete(),
                "clear",
                style::Button::Toolbar,
                Some(ToolbarMessage::ClearImages),
            ))
            .push(button(
                icons::duplicate(),
                "edit",
                style::Button::Toolbar,
                Some(ToolbarMessage::New),
            ))
            .into()
    }
}

fn button<'a>(
    icon: Text,
    text: &str,
    style: style::Button,
    message: Option<ToolbarMessage>,
) -> Button<ToolbarMessage> {
    let button = Button::new(
        Column::new()
            .align_items(Alignment::Center)
            .push(icon)
            .push(Text::new(text)),
    )
    .style(style)
    .width(Length::Units(70))
    .height(Length::Units(50));
    if let Some(m) = message {
        button.on_press(m)
    } else {
        button
    }
}

// impl Index<ShapeKind> for Shapes {
//     type Output = button::State;

//     fn index(&self, s: ShapeKind) -> &Self::Output {
//         match s {
//             ShapeKind::Rectangle => &self.rectangle,
//             ShapeKind::Triangle => &self.triangle,
//         }
//     }
// }

// impl IndexMut<ShapeKind> for Shapes {
//     fn index_mut(&mut self, s: ShapeKind) -> &mut Self::Output {
//         match s {
//             ShapeKind::Rectangle => &mut self.rectangle,
//             ShapeKind::Triangle => &mut self.triangle,
//         }
//     }
// }
