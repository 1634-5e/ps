use iced::{button, Background, Color, Vector};

pub enum Button {
    Toolbar,
    Navigator
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(match self {
                Button::Toolbar => Color::from_rgb(0.11, 0.42, 0.87),
                Button::Navigator => Color::WHITE
            })),
            border_radius: 12.0,
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::WHITE,
            ..button::Style::default()
        }
    }
}
