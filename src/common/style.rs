use iced::{button, Background, Color, Vector};

pub enum Button {
    Toolbar,
    Navigator,
    PickImage,
}

//可选：background, shadow_offset, border_radius, border_width, border_color, text_color
impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        match self {
            Button::Toolbar => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.11, 0.42, 0.87))),
                border_radius: 10.0,
                ..button::Style::default()
            },
            Button::Navigator => button::Style::default(),
            Button::PickImage => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.71, 0.60, 0.63))),
                shadow_offset: Vector::new(1.0, 1.0),
                ..button::Style::default()
            },
        }
    }
}
