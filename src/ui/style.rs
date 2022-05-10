use iced::{button, container, pick_list, text_input, Background, Color, Vector};

const ACTIVE: Color = Color::from_rgb(
    0x72 as f32 / 255.0,
    0x89 as f32 / 255.0,
    0xDA as f32 / 255.0,
);

const DESTRUCTIVE: Color = Color::from_rgb(
    0xC0 as f32 / 255.0,
    0x47 as f32 / 255.0,
    0x47 as f32 / 255.0,
);

const HOVERED: Color = Color::from_rgb(
    0x67 as f32 / 255.0,
    0x7B as f32 / 255.0,
    0xC4 as f32 / 255.0,
);

const BACKGROUND: Color = Color::from_rgb(
    0xe2 as f32 / 255.0,
    0xe1 as f32 / 255.0,
    0xe4 as f32 / 255.0,
);

const POSITIVE: Color = Color::from_rgb(230.0, 92.0, 92.0);

pub struct Container;

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(BACKGROUND)),
            text_color: Some(Color::BLACK),
            ..container::Style::default()
        }
    }
}

#[derive(Clone, Debug)]
pub enum Button {
    Toolbar,
    Navigator,
    Confirm,
    PreviewNavigator,
    Delete,
}

//可选：background, shadow_offset, border_radius, border_width, border_color, text_color
impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        match self {
            Button::Toolbar => button::Style {
                background: Some(Background::Color(ACTIVE)),
                border_radius: 5.0,
                ..button::Style::default()
            },
            Button::Navigator => button::Style {
                border_width: 5.0,
                border_radius: 5.0,
                ..button::Style::default()
            },
            Button::Confirm => button::Style {
                background: Some(Background::Color(POSITIVE)),
                shadow_offset: Vector::new(1.0, 1.0),
                border_radius: 5.0,
                ..button::Style::default()
            },
            Button::PreviewNavigator => button::Style {
                border_radius: 5.0,
                ..button::Style::default()
            },
            Button::Delete => button::Style {
                background: Some(Background::Color(DESTRUCTIVE)),
                shadow_offset: Vector::new(1.0, 1.0),
                border_radius: 5.0,
                ..button::Style::default()
            },
        }
    }

    fn disabled(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            ..self.active()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(HOVERED)),
            ..self.active()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            border_width: 1.0,
            border_color: Color::WHITE,
            ..self.hovered()
        }
    }
}

pub enum TextInput {
    EditAttribute,
}

impl text_input::StyleSheet for TextInput {
    fn active(&self) -> text_input::Style {
        match self {
            TextInput::EditAttribute => text_input::Style {
                border_radius: 0.0,
                ..Default::default()
            },
        }
    }

    // Produces the style of a focused text input.
    fn focused(&self) -> text_input::Style {
        text_input::Style {
            border_color: Color::from_rgb(0.5, 0.5, 0.5),
            ..self.active()
        }
    }

    fn placeholder_color(&self) -> Color {
        Color::from_rgb(0.7, 0.7, 0.7)
    }

    fn value_color(&self) -> Color {
        Color::from_rgb(0.3, 0.3, 0.3)
    }

    fn selection_color(&self) -> Color {
        Color::from_rgb(0.8, 0.8, 1.0)
    }

    // Produces the style of an hovered text input.
    // fn hovered(&self) -> Style;
}

pub struct PickList;

impl pick_list::StyleSheet for PickList {
    fn menu(&self) -> pick_list::Menu {
        pick_list::Menu {
            background: BACKGROUND.into(),
            border_width: 1.0,
            border_color: Color {
                a: 0.7,
                ..Color::BLACK
            },
            ..pick_list::Menu::default()
        }
    }

    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            background: BACKGROUND.into(),
            border_width: 1.0,
            border_color: Color {
                a: 0.6,
                ..Color::BLACK
            },
            border_radius: 2.0,
            icon_size: 0.5,
            ..pick_list::Style::default()
        }
    }

    fn hovered(&self) -> pick_list::Style {
        let active = self.active();

        pick_list::Style {
            border_color: Color {
                a: 0.9,
                ..Color::BLACK
            },
            ..active
        }
    }
}
