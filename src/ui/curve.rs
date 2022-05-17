
use iced::{canvas::{LineCap, LineJoin, LineDash, Frame, Stroke, Fill}, Color, Point};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use svg::node::element::Path as SvgPath;

use std::fmt::Display;

use crate::utils::{get_format_color, is_valid_rgb, SerdeColor};

use super::{ShapeMessage, Shape, Line, EditMessage};

#[derive(Debug, Clone)]
pub enum CurveMessage {
    Shape(ShapeMessage),

    InputColorR(String),
    InputColorG(String),
    InputColorB(String),
    InputColorA(String),

    SlideColorR(f32),
    SlideColorG(f32),
    SlideColorB(f32),
    SlideColorA(f32),

    InputWidth(String),

    LineCapSelected(EqLineCap),
    LineJoinSelected(EqLineJoin),

    CurveSelected(usize),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EqLineCap {
    Butt,
    #[default]
    Round,
    Square,
}

impl Into<LineCap> for EqLineCap {
    fn into(self) -> LineCap {
        match self {
            EqLineCap::Butt => LineCap::Butt,
            EqLineCap::Round => LineCap::Round,
            EqLineCap::Square => LineCap::Square,
        }
    }
}

impl Display for EqLineCap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EqLineCap::Butt => "butt",
                EqLineCap::Round => "round",
                EqLineCap::Square => "square",
            }
        )
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EqLineJoin {
    Miter,
    #[default]
    Round,
    Bevel,
}

impl Into<LineJoin> for EqLineJoin {
    fn into(self) -> LineJoin {
        match self {
            EqLineJoin::Miter => LineJoin::Miter,
            EqLineJoin::Round => LineJoin::Round,
            EqLineJoin::Bevel => LineJoin::Bevel,
        }
    }
}

impl Display for EqLineJoin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EqLineJoin::Miter => "miter",
                EqLineJoin::Round => "round",
                EqLineJoin::Bevel => "bevel",
            }
        )
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Curve {
    #[serde(with = "serde_traitobject")]
    pub shape: Box<dyn Shape>,
    #[serde_as(as = "SerdeColor")]
    pub color: Color,
    pub width: f32,
    pub line_cap: EqLineCap, //这里其实应该叫Fmt~，因为使用这个类型的原因主要是要能够转化成字符串，用于svg设置样式，而序列化只需要引入这个类型就够了
    pub line_join: EqLineJoin,
    // pub segments: Vec<f32>,
    // pub offset: usize,
}

impl Default for Curve {
    fn default() -> Self {
        let line_dash = LineDash::default();
        Curve {
            shape: Box::new(Line::default()),
            color: Color::BLACK,
            width: 2.0,
            line_cap: EqLineCap::Round,
            line_join: EqLineJoin::Round,
            // segments: line_dash.segments.to_vec(),
            // offset: line_dash.offset,
        }
    }
}

impl Curve {
    #[inline(always)]
    pub fn update(&mut self, message: CurveMessage) -> Option<EditMessage> {
        match message {
            CurveMessage::Shape(sm) => {
                self.shape.update(sm);
            }

            CurveMessage::InputColorR(r) => {
                if let Ok(r) = r.parse::<f32>() {
                    if is_valid_rgb(r) {
                        self.color.r = r / 255.0;
                    }
                }
            }
            CurveMessage::InputColorG(g) => {
                if let Ok(g) = g.parse::<f32>() {
                    if is_valid_rgb(g) {
                        self.color.g = g / 255.0;
                    }
                }
            }
            CurveMessage::InputColorB(b) => {
                if let Ok(b) = b.parse::<f32>() {
                    if is_valid_rgb(b) {
                        self.color.b = b / 255.0;
                    }
                }
            }
            CurveMessage::InputColorA(a) => {
                if let Ok(a) = a.parse::<f32>() {
                    if (0.0..=1.0).contains(&a) {
                        self.color.a = a;
                    }
                }
            }
            CurveMessage::InputWidth(w) => {
                if let Ok(width) = w.parse::<f32>() {
                    self.width = width;
                }
            }
            CurveMessage::SlideColorR(r) => {
                self.color.r = r;
            }
            CurveMessage::SlideColorG(g) => {
                self.color.g = g;
            }
            CurveMessage::SlideColorB(b) => {
                self.color.b = b;
            }
            CurveMessage::SlideColorA(a) => {
                self.color.a = a;
            }
            CurveMessage::LineCapSelected(lc) => self.line_cap = lc,
            CurveMessage::LineJoinSelected(lj) => self.line_join = lj,
            CurveMessage::CurveSelected(index) => return Some(EditMessage::CurveSelected(index)),
        }
        None
    }

    #[inline(always)]
    pub fn preview(&self, frame: &mut Frame, cursor_position: Point) {
        if let Some(path) = self.shape.preview(cursor_position) {
            frame.stroke(
                &path,
                Stroke {
                    color: self.color,
                    width: self.width,
                    line_cap: self.line_cap.into(),
                    line_join: self.line_join.into(),
                    line_dash: LineDash::default(),
                },
            );
        }
    }

    #[inline(always)]
    pub fn draw(&self, frame: &mut Frame, selected: bool) {
        if let (Some(path), selected) = self.shape.draw(selected) {
            frame.stroke(
                &path,
                Stroke {
                    color: self.color,
                    width: self.width,
                    line_cap: self.line_cap.into(),
                    line_join: self.line_join.into(),
                    line_dash: LineDash::default(),
                },
            );

            if let Some(selection_highlight) = selected {
                frame.fill(&selection_highlight, Fill { ..Fill::default() });
            }
        }
    }

    #[inline(always)]
    pub fn save(&self) -> Option<SvgPath> {
        //小写大写貌似不区分
        let data = self.shape.export_as_svg()?;
        Some(
            SvgPath::new()
                .set("fill", "none")
                .set("stroke", get_format_color(self.color))
                .set("stroke-width", self.width)
                .set("stroke-linecap", self.line_cap.to_string())
                .set("stroke-linejoin", self.line_join.to_string())
                .set("d", data),
        )
    }
}