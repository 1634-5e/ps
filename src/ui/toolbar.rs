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
                Some(ToolbarMessage::Back),
            ))
            .push(button(
                &mut self.clear_canvas,
                icons::delete(),
                "clear",
                Some(ToolbarMessage::Edit(EditMessage::Clear)),
            ))
            .push(button(
                &mut self.save,
                icons::save(),
                "export",
                Some(ToolbarMessage::Export),
            ))
            .push(button(
                &mut self.line,
                icons::rectangle(),
                "line",
                Some(ToolbarMessage::Edit(EditMessage::ChangeShape(Box::new(
                    Line::default(),
                )))),
            ))
            .push(button(
                &mut self.rectangle,
                icons::rectangle(),
                "rectangle",
                Some(ToolbarMessage::Edit(EditMessage::ChangeShape(Box::new(
                    Rectangle::default(),
                )))),
            ))
            .push(button(
                &mut self.triangle,
                icons::triangle(),
                "triangle",
                Some(ToolbarMessage::Edit(EditMessage::ChangeShape(Box::new(
                    Triangle::default(),
                )))),
            ))
            .push(button(
                &mut self.quadratic_bezier,
                icons::quadratic_bezier(),
                "2 Bezier",
                Some(ToolbarMessage::Edit(EditMessage::ChangeShape(Box::new(
                    QuadraticBezier::default(),
                )))),
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
                Some(ToolbarMessage::Open),
            ))
            .push(button(
                &mut self.close,
                icons::delete(),
                "close",
                Some(ToolbarMessage::Close),
            ))
            .push(button(
                &mut self.clear_images,
                icons::delete(),
                "clear",
                Some(ToolbarMessage::ClearImages),
            ))
            .push(button(
                &mut self.new,
                icons::duplicate(),
                "new",
                Some(ToolbarMessage::New),
            ))
            .into()
    }
}

fn button<'a>(
    state: &'a mut button::State,
    icon: Text,
    text: &str,
    message: Option<ToolbarMessage>,
) -> Button<'a, ToolbarMessage> {
    let button = Button::new(
        state,
        Column::new()
            .align_items(Alignment::Center)
            .spacing(5)
            .push(icon)
            .push(Text::new(text)),
    )
    .style(style::Button::Toolbar);
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
