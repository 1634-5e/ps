use iced::{button, Button, Element, Length, Row, Text};

use super::{style, Shape};

#[derive(Debug, Clone)]
pub enum ToolbarMessage {
    //view
    Close,
    ClearImages,
    New,
    Open,

    //edit
    Back,
    ClearCanvas,
    Save,
    SelectShape(Shape),
}


//TODO:按钮要能改样式，但是不想每个都多一个bool
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
    rectangle: button::State,
    triangle: button::State,
}

impl Toolbar {
    pub fn editing(&mut self) -> Element<ToolbarMessage> {
        Row::new()
            .height(Length::Units(100))
            .push(button(&mut self.back, "back", Some(ToolbarMessage::Back)))
            .push(button(
                &mut self.clear_canvas,
                "clear",
                Some(ToolbarMessage::ClearCanvas),
            ))
            .push(button(&mut self.save, "save", Some(ToolbarMessage::Save)))
            .push(button(
                &mut self.rectangle,
                "rectangle",
                Some(ToolbarMessage::SelectShape(Shape::Rectangle)),
            ))
            .push(button(
                &mut self.triangle,
                "triangle",
                Some(ToolbarMessage::SelectShape(Shape::Triangle)),
            ))
            .into()
    }

    pub fn viewing(&mut self) -> Element<ToolbarMessage> {
        Row::new()
            .height(Length::Units(100))
            .push(button(&mut self.open, "open", Some(ToolbarMessage::Open)))
            .push(button(
                &mut self.close,
                "close",
                Some(ToolbarMessage::Close),
            ))
            .push(button(
                &mut self.clear_images,
                "clear",
                Some(ToolbarMessage::ClearImages),
            ))
            .push(button(&mut self.new, "new", Some(ToolbarMessage::New)))
            .into()
    }
}

fn button<'a>(
    state: &'a mut button::State,
    text: &str,
    message: Option<ToolbarMessage>,
) -> Button<'a, ToolbarMessage> {
    let button = Button::new(state, Text::new(text)).style(style::Button::Toolbar);
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
