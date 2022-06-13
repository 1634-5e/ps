use iced::{button, Alignment, Button, Column, Element, Length, Row, Text};

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
pub struct Toolbar {
    //view
    open: button::State,
    close: button::State,
    clear_images: button::State,
    new: button::State,

    //edit
    back: button::State,
    clear_canvas: button::State,
    save: button::State,

    line: button::State,
    rectangle: button::State,
    triangle: button::State,
    quadratic_bezier: button::State,
    circle: button::State,
}

impl Toolbar {
    pub fn editing(&mut self) -> Element<ToolbarMessage> {
        Row::new()
            .padding(20)
            .spacing(7)
            .height(Length::Units(100))
            .push(button(
                &mut self.back,
                icons::check(),
                "back",
                style::Button::Toolbar,
                Some(ToolbarMessage::Back),
            ))
            .push(button(
                &mut self.save,
                icons::save(),
                "export",
                style::Button::Toolbar,
                Some(ToolbarMessage::Export),
            ))
            .push(button(
                &mut self.line,
                icons::rectangle(),
                "line",
                style::Button::Toolbar,
                Some(ToolbarMessage::Edit(EditMessage::ChangeShape(
                    Line::default().into(),
                ))),
            ))
            .push(button(
                &mut self.rectangle,
                icons::rectangle(),
                "rect",
                style::Button::Toolbar,
                Some(ToolbarMessage::Edit(EditMessage::ChangeShape(
                    Rectangle::default().into(),
                ))),
            ))
            .push(button(
                &mut self.triangle,
                icons::triangle(),
                "triangle",
                style::Button::Toolbar,
                Some(ToolbarMessage::Edit(EditMessage::ChangeShape(
                    Triangle::default().into(),
                ))),
            ))
            .push(button(
                &mut self.circle,
                icons::triangle(),
                "circle",
                style::Button::Toolbar,
                Some(ToolbarMessage::Edit(EditMessage::ChangeShape(
                    Circle::default().into(),
                ))),
            ))
            .push(button(
                &mut self.quadratic_bezier,
                icons::quadratic_bezier(),
                "Bezier",
                style::Button::Toolbar,
                Some(ToolbarMessage::Edit(EditMessage::ChangeShape(
                    QuadraticBezier::default().into(),
                ))),
            ))
            .push(button(
                &mut self.clear_canvas,
                icons::delete(),
                "clear",
                style::Button::Delete,
                Some(ToolbarMessage::Edit(EditMessage::Clear)),
            ))
            .into()
    }

    pub fn viewing(&mut self) -> Element<ToolbarMessage> {
        Row::new()
            .padding(20)
            .spacing(7)
            .height(Length::Units(100))
            .push(button(
                &mut self.open,
                icons::load(),
                "open",
                style::Button::Toolbar,
                Some(ToolbarMessage::Open),
            ))
            .push(button(
                &mut self.close,
                icons::delete(),
                "close",
                style::Button::Toolbar,
                Some(ToolbarMessage::Close),
            ))
            .push(button(
                &mut self.clear_images,
                icons::delete(),
                "clear",
                style::Button::Toolbar,
                Some(ToolbarMessage::ClearImages),
            ))
            .push(button(
                &mut self.new,
                icons::duplicate(),
                "edit",
                style::Button::Toolbar,
                Some(ToolbarMessage::New),
            ))
            .into()
    }
}

fn button<'a>(
    state: &'a mut button::State,
    icon: Text,
    text: &str,
    style: style::Button,
    message: Option<ToolbarMessage>,
) -> Button<'a, ToolbarMessage> {
    let button = Button::new(
        state,
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
