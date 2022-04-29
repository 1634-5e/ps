use iced::{Color, Point, Size};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, SerializeAs};

pub fn get_size(left_top: Point, right_bottom: Point) -> Size {
    Size::new(right_bottom.x - left_top.x, right_bottom.y - left_top.y)
}

pub fn is_valid_rgb(value: f32) -> bool {
    (0.0..256.0).contains(&value)
}

pub fn get_format_color(color: Color) -> String {
    let mut r = format!("{:x}", (color.r * 255.0) as i32);
    let mut g = format!("{:x}", (color.g * 255.0) as i32);
    let mut b = format!("{:x}", (color.b * 255.0) as i32);

    if r.len() != 1 || g.len() != 1 || b.len() != 1 {
        if r.len() == 1 {
            r = ["0".to_string(), r].concat();
        }
        if g.len() == 1 {
            g = ["0".to_string(), g].concat();
        }
        if b.len() == 1 {
            b = ["0".to_string(), b].concat();
        }
    }

    ["#".to_string(), r, g, b].concat()
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Point")]
pub struct SerdePoint {
    pub x: f32,
    pub y: f32,
}

impl SerializeAs<Point> for SerdePoint {
    fn serialize_as<S>(value: &Point, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SerdePoint::serialize(value, serializer)
    }
}

impl<'de> DeserializeAs<'de, Point> for SerdePoint {
    fn deserialize_as<D>(deserializer: D) -> Result<Point, D::Error>
    where
        D: Deserializer<'de>,
    {
        SerdePoint::deserialize(deserializer)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Color")]
pub struct SerdeColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl SerializeAs<Color> for SerdeColor {
    fn serialize_as<S>(value: &Color, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SerdeColor::serialize(value, serializer)
    }
}

impl<'de> DeserializeAs<'de, Color> for SerdeColor {
    fn deserialize_as<D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        SerdeColor::deserialize(deserializer)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Size")]
pub struct SerdeSize<T = f32> {
    pub width: T,
    pub height: T,
}

impl SerializeAs<Size> for SerdeSize {
    fn serialize_as<S>(value: &Size, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SerdeSize::serialize(value, serializer)
    }
}

impl<'de> DeserializeAs<'de, Size> for SerdeSize {
    fn deserialize_as<D>(deserializer: D) -> Result<Size, D::Error>
    where
        D: Deserializer<'de>,
    {
        SerdeSize::deserialize(deserializer)
    }
}
