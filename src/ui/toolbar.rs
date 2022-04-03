use iced::{button, Button, Element, Row, Text, Length};

use super::{style, Shape};

#[derive(Debug, Clone)]
pub enum ToolbarMessage {
    //view
    Close,
    ClearImages,
    New,

    //edit
    Back,
    ClearCanvas,
    Save,
    SelectShape(Shape),
}

#[derive(Debug, Default, Clone)]
pub struct Toolbar {
    //view
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
            .push(
                Button::new(&mut self.back, Text::new("back"))
                    .style(style::Button::Toolbar)
                    .on_press(ToolbarMessage::Back),
            )
            .push(
                Button::new(&mut self.clear_canvas, Text::new("clear"))
                    .style(style::Button::Toolbar)
                    .on_press(ToolbarMessage::ClearCanvas),
            )
            .push(
                Button::new(&mut self.save, Text::new("save"))
                    .style(style::Button::Toolbar)
                    .on_press(ToolbarMessage::Save),
            )
            .push(
                Button::new(&mut self.rectangle, Text::new("rectangle"))
                    .style(style::Button::Toolbar)
                    .on_press(ToolbarMessage::SelectShape(Shape::Rectangle)),
            )
            .push(
                Button::new(&mut self.triangle, Text::new("triangle"))
                    .style(style::Button::Toolbar)
                    .on_press(ToolbarMessage::SelectShape(Shape::Triangle)),
            )
            .into()
    }

    pub fn viewing(&mut self) -> Element<ToolbarMessage> {
        Row::new()
            .height(Length::Units(100))
            .push(
                Button::new(&mut self.close, Text::new("close"))
                    .style(style::Button::Toolbar)
                    .on_press(ToolbarMessage::Close),
            )
            .push(
                Button::new(&mut self.clear_images, Text::new("clear"))
                    .style(style::Button::Toolbar)
                    .on_press(ToolbarMessage::ClearImages),
            )
            .push(
                Button::new(&mut self.new, Text::new("new"))
                    .style(style::Button::Toolbar)
                    .on_press(ToolbarMessage::New),
            )
            .into()
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
