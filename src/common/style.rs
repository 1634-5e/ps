use iced::{button, Background, Color, Vector};

#[derive(Default, Clone, Debug)]
pub enum Button {
    Toolbar,
    Navigator,
    #[default]
    Entry,
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
                // background: Some(Background::Color(Color::from_rgb(0.71, 0.60, 0.63))),
                shadow_offset: Vector::new(1.0, 1.0),
                border_width: 5.0,
                border_radius: 5.0,
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
        }
    }
}
