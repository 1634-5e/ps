use std::{
    fs,
    path::PathBuf,
};

use iced::{Color, Point, Size};
use json::JsonValue;
use serde::{Deserialize, Serialize};

use crate::ui::{shape::*, Curve, EqLineCap, EqLineJoin};

const FILE_NAME: &str = "last_place";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SavedState {
    pub is_editing: bool,

    //view
    pub images: Vec<PathBuf>,
    pub on_view: Option<usize>,
    //edit
    pub curves: Vec<Curve>,
}

pub async fn save_state(saved_state: SavedState, path: PathBuf) -> std::io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    let serialized = serde_json::to_string_pretty(&saved_state)?;

    fs::write(path.join(FILE_NAME), serialized)?;

    Ok(())
}

pub async fn load_state(path: PathBuf) -> std::io::Result<Option<SavedState>> {
    let mut res = SavedState::default();

    let content = fs::read_to_string(path.join(FILE_NAME))?;

    if let Ok(parsed) = json::parse(content.as_str()) {
        //构建curves
        if let JsonValue::Array(curves) = &parsed["curves"] {
            let mut restored_curves = vec![];
            for curve in curves {
                //这些是通用数据不用管类型；
                let color = if let (Some(r), Some(g), Some(b), Some(a)) = (
                    curve["color"]["r"].as_f32(),
                    curve["color"]["g"].as_f32(),
                    curve["color"]["b"].as_f32(),
                    curve["color"]["a"].as_f32(),
                ) {
                    Color { r, g, b, a }
                } else {
                    Color::default()
                };
                let width = if let Some(w) = curve["width"].as_f32() {
                    w
                } else {
                    2.0
                };
                let line_cap = match curve["line_cap"].as_str() {
                    Some("Butt") => EqLineCap::Butt,
                    Some("Round") => EqLineCap::Round,
                    Some("RouSquarend") => EqLineCap::Square,
                    _ => EqLineCap::default(),
                };
                let line_join = match curve["line_join"].as_str() {
                    Some("Round") => EqLineJoin::Round,
                    Some("Miter") => EqLineJoin::Miter,
                    Some("Bevel") => EqLineJoin::Bevel,
                    _ => EqLineJoin::default(),
                };

                let mut new_curve = Curve {
                    color,
                    width,
                    line_cap,
                    line_join,
                    ..Curve::default()
                };

                let points = &curve["shape"][2];
                let new: Option<Box<dyn Shape>> = if points.has_key("from") && points.has_key("to")
                {
                    let mut new = Line::default();
                    if let (Some(x), Some(y)) =
                        (points["from"]["x"].as_f32(), points["from"]["y"].as_f32())
                    {
                        new.from = Some(Point { x, y });
                    }
                    if let (Some(x), Some(y)) =
                        (points["to"]["x"].as_f32(), points["to"]["y"].as_f32())
                    {
                        new.to = Some(Point { x, y });
                    }
                    Some(Box::new(new))
                } else if points.has_key("top_left") && points.has_key("size") {
                    let mut new = Rectangle::default();
                    if let (Some(x), Some(y)) = (
                        points["top_left"]["x"].as_f32(),
                        points["top_left"]["y"].as_f32(),
                    ) {
                        new.top_left = Some(Point { x, y });
                    }
                    if let (Some(width), Some(height)) = (
                        points["size"]["width"].as_f32(),
                        points["size"]["height"].as_f32(),
                    ) {
                        new.size = Some(Size { width, height });
                    }
                    Some(Box::new(new))
                } else if points.has_key("a") && points.has_key("b") && points.has_key("c") {
                    let mut new = Triangle::default();
                    if let (Some(x), Some(y)) =
                        (points["a"]["x"].as_f32(), points["a"]["y"].as_f32())
                    {
                        new.a = Some(Point { x, y });
                    }
                    if let (Some(x), Some(y)) =
                        (points["b"]["x"].as_f32(), points["b"]["y"].as_f32())
                    {
                        new.b = Some(Point { x, y });
                    }
                    if let (Some(x), Some(y)) =
                        (points["c"]["x"].as_f32(), points["c"]["y"].as_f32())
                    {
                        new.c = Some(Point { x, y });
                    }
                    Some(Box::new(new))
                } else if points.has_key("a") && points.has_key("b") && points.has_key("control") {
                    let mut new = QuadraticBezier::default();
                    if let (Some(x), Some(y)) =
                        (points["a"]["x"].as_f32(), points["a"]["y"].as_f32())
                    {
                        new.a = Some(Point { x, y });
                    }
                    if let (Some(x), Some(y)) =
                        (points["b"]["x"].as_f32(), points["b"]["y"].as_f32())
                    {
                        new.b = Some(Point { x, y });
                    }
                    if let (Some(x), Some(y)) = (
                        points["control"]["x"].as_f32(),
                        points["control"]["y"].as_f32(),
                    ) {
                        new.control = Some(Point { x, y });
                    }
                    Some(Box::new(new))
                } else if points.has_key("center") && points.has_key("radius") {
                    let mut new = Circle::default();
                    if let (Some(x), Some(y)) = (
                        points["center"]["x"].as_f32(),
                        points["center"]["y"].as_f32(),
                    ) {
                        new.center = Some(Point { x, y });
                    }
                    if let Some(radius) = points["radius"].as_f32() {
                        new.radius = Some(radius);
                    }
                    Some(Box::new(new))
                } else {
                    None
                };
                if let Some(new) = new {
                    new_curve.shape = new;
                    restored_curves.push(new_curve);
                }
            }
            res.curves = restored_curves;
        }

        //构建images
        if let JsonValue::Array(images) = &parsed["images"] {
            let mut restored_images = vec![];
            for image in images {
                if let Some(path) = image.as_str() {
                    restored_images.push(PathBuf::from(path));
                }
            }
        }

        //构建is_editing
        if let Some(ie) = &parsed["is_editing"].as_bool() {
            res.is_editing = *ie;
        }

        //构建on_view
        if let Some(ov) = &parsed["on_view"].as_usize() {
            res.on_view = Some(*ov);
        }
        Ok(Some(res))
    } else {
        Ok(None)
    }
}
