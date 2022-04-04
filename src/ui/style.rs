use iced::{button, Background, Color, Vector};

#[derive(Default, Clone, Debug)]
pub enum Button {
    Toolbar,
    Navigator,
    #[default]
    Entry,
    RemoveCurve,
    PreviewNavigator,
}

//可选：background, shadow_offset, border_radius, border_width, border_color, text_color
impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        match self {
            Button::Toolbar => button::Style {
                // background: Some(Background::Color(Color::from_rgb(0.58, 0.71, 0.81))),
                border_width: 5.0,
                border_radius: 5.0,
                ..button::Style::default()
            },
            Button::Navigator => button::Style {
                border_width: 5.0,
                border_radius: 5.0,
                ..button::Style::default()
            },
            Button::Entry => button::Style {
                background: Some(Background::Color(Color::from_rgb(230.0, 92.0, 92.0))),
                shadow_offset: Vector::new(1.0, 1.0),
                border_width: 5.0,
                border_radius: 5.0,
                ..button::Style::default()
            },
            Button::RemoveCurve => button::Style {
                background: Some(Background::Color(Color::from_rgb(230.0, 92.0, 92.0))),
                border_radius: 5.0,
                ..button::Style::default()
            },
            Button::PreviewNavigator => button::Style {
                border_radius: 5.0,
                border_width: 5.0,
                ..button::Style::default()
            },
        }
    }

    fn disabled(&self) -> button::Style {
        match self {
            Button::Toolbar => button::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                border_radius: 5.0,
                border_width: 5.0,
                ..button::Style::default()
            },
            Button::Navigator => button::Style::default(),
            Button::Entry => button::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                shadow_offset: Vector::new(1.0, 1.0),
                border_width: 5.0,
                border_radius: 5.0,
                ..button::Style::default()
            },
            Button::RemoveCurve => button::Style {
                // background: Some(Background::Color(Color::from_rgb(0.58, 0.71, 0.81))),
                border_width: 5.0,
                border_radius: 5.0,
                ..button::Style::default()
            },
            Button::PreviewNavigator => button::Style {
                border_radius: 5.0,
                border_width: 5.0,
                ..button::Style::default()
            },
        }
    }
}

// pub enum TextInput {
//     EditColor,
// }

// impl text_input::StyleSheet for TextInput {
//     fn active(&self) -> Style {
//         match self {
//             TextInput::EditColor => text_input::Style {
//                 border_radius: 0.0,
//                 shadow_offset: Vector::new(1.0, 0.0),
//                 ..Default::default()
//             },
//         }
//     }

//     /// Produces the style of a focused text input.
//     fn focused(&self) -> Style;

//     fn placeholder_color(&self) -> Color;

//     fn value_color(&self) -> Color;

//     fn selection_color(&self) -> Color;

//     /// Produces the style of an hovered text input.
//     fn hovered(&self) -> Style {
//         self.focused()
//     }
// }
