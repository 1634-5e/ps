use iced::{Font, Length, Text};

const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("fonts/icons.ttf"),
};

// const ICONS: Font = Font::External {
//     name: "Icons",
//     bytes: include_bytes!("fonts/firacode_m.ttf"),
// };

fn icon(unicode: char) -> Text {
    Text::new(&unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .size(20)
}

pub fn delete() -> Text {
    icon('\u{F1F8}')
}

pub fn check() -> Text {
    icon('\u{e806}')
}

pub fn save() -> Text {
    icon('\u{e800}')
}

pub fn load() -> Text {
    icon('\u{f115}')
}

pub fn rectangle() -> Text {
    icon('\u{25ad}')
}

pub fn triangle() -> Text {
    icon('\u{25b3}')
}

pub fn quadratic_bezier() -> Text {
    icon('\u{25af}')
}

pub fn duplicate() -> Text {
    icon('\u{f0c5}')
}

// pub fn edit() -> Text {
//     icon('\u{e803}')
// }

// pub fn export() -> Text {
//     icon('\u{e81d}')
// }

// pub fn new() -> Text {
//     icon('\u{e810}')
// }

// pub fn add() -> Text {
//     icon('\u{e80c}')
// }

// pub fn help() -> Text {
//     icon('\u{f128}')
// }

// pub fn up_arrow() -> Text {
//     icon('\u{e816}')
// }

// pub fn down_arrow() -> Text {
//     icon('\u{e813}')
// }

// pub fn cubic_bezier() -> Text {
//     icon('\u{25af}')
// }
