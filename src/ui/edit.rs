use iced::{
    button,
    canvas::event::{self, Event},
    canvas::{self, Canvas as IcedCanvas, Cursor, Frame, Geometry, Path, Stroke},
    mouse, slider, text_input, Alignment, Button, Color, Column, Element, Length, Point,
    Rectangle as IcedRectangle, Row, Slider, Text,
};

use svg::node::element::Path as SvgPath;
use svg::Document;

// use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};

use self::shape::{Rectangle, Shape};

use crate::io::dialogs::save as save_file;

use super::style;

#[derive(Debug, Clone)]
pub struct Curve {
    points: Vec<Point>,
    shape: Box<dyn Shape>,
    color: Color,
    width: f32,
}

impl Default for Curve {
    fn default() -> Self {
        Curve {
            points: vec![],
            shape: Box::new(Rectangle),
            color: Color::BLACK,
            width: 2.0,
        }
    }
}

impl Curve {
    pub fn preview(&self, bounds: IcedRectangle, cursor: Cursor) -> Geometry {
        let mut frame = Frame::new(bounds.size());

        if let Some(cursor_position) = cursor.position_in(&bounds) {
            frame.stroke(
                &self.shape.preview(&self.points, cursor_position),
                Stroke {
                    width: self.width,
                    color: self.color,
                    ..Stroke::default()
                },
            )
        }

        frame.into_geometry()
    }

    #[inline(always)]
    pub fn draw(&self, frame: &mut Frame, selected: bool) {
        frame.stroke(
            &self.shape.draw(&self.points),
            Stroke {
                width: self.width,
                color: self.color,
                ..Stroke::default()
            },
        );

        if selected {
            let selection_highlight = Path::new(|b| {
                for point in self.points.iter() {
                    b.circle(*point, 8.0);
                }
            });

            frame.stroke(
                &selection_highlight,
                Stroke {
                    width: 1.0,
                    color: Color::from_rgb(255.0, 255.0, 0.0),
                    ..Stroke::default()
                },
            );
        }
    }

    #[inline(always)]
    pub fn save(&self) -> SvgPath {
        //小写大写貌似不区分
        SvgPath::new()
            .set("fill", "none")
            .set("stroke", Self::get_format_color(self.color))
            .set("stroke-width", self.width)
            .set("d", self.shape.save(&self.points))
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
}

// impl Serialize for Curve {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut s = serializer.serialize_struct("Curve", 4)?;
//         s.serialize_field("points", &self.points)?;
//         s.serialize_field("kind", &self.kind)?;
//         s.serialize_field("color", &self.color)?;
//         s.serialize_field("width", &self.width)?;
//         s.end()
//     }
// }

#[derive(Debug, Clone)]
pub enum EditMessage {
    AddCurve(Curve),
    SelectCurve(Option<usize>),

    //editable
    InputColorR(String),
    InputColorG(String),
    InputColorB(String),

    SlideColorR(f32),
    SlideColorG(f32),
    SlideColorB(f32),

    InputWidth(String),

    RemoveCurve,
}

#[derive(Debug, Default)]
pub struct Edit {
    pub pending: Curve,
    curves: Vec<Curve>,
    cache: canvas::Cache,
    selected_curve: Option<usize>,

    input_color_r: text_input::State,
    input_color_g: text_input::State,
    input_color_b: text_input::State,
    input_width: text_input::State,

    slider_color_r: slider::State,
    slider_color_g: slider::State,
    slider_color_b: slider::State,

    remove_curve: button::State,
}

impl Edit {
    pub fn update(&mut self, message: EditMessage) {
        match message {
            EditMessage::AddCurve(c) => {
                self.curves.push(c);
                self.selected_curve = Some(self.curves.len() - 1);
            }
            EditMessage::SelectCurve(c) => {
                if self.selected_curve == c {
                    self.selected_curve = None;
                } else {
                    self.selected_curve = c;
                }
            }
            EditMessage::InputColorR(r) => {
                if let Ok(r) = r.parse::<f32>() {
                    if is_valid_rgb(r) {
                        if let Some(selected) = self.selected_curve {
                            self.curves[selected].color.r = r / 255.0;
                        } else {
                            self.pending.color.r = r / 255.0;
                        }
                    }
                }
            }
            EditMessage::InputColorG(g) => {
                if let Ok(g) = g.parse::<f32>() {
                    if is_valid_rgb(g) {
                        if let Some(selected) = self.selected_curve {
                            self.curves[selected].color.g = g / 255.0;
                        } else {
                            self.pending.color.g = g / 255.0;
                        }
                    }
                }
            }
            EditMessage::InputColorB(b) => {
                if let Ok(b) = b.parse::<f32>() {
                    if is_valid_rgb(b) {
                        if let Some(selected) = self.selected_curve {
                            self.curves[selected].color.b = b / 255.0;
                        } else {
                            self.pending.color.b = b / 255.0;
                        }
                    }
                }
            }
            EditMessage::InputWidth(w) => {
                if let Ok(width) = w.parse::<f32>() {
                    if let Some(selected) = self.selected_curve {
                        self.curves[selected].width = width;
                    } else {
                        self.pending.width = width;
                    }
                }
            }
            EditMessage::RemoveCurve => self.remove_curve(),
            EditMessage::SlideColorR(r) => {
                if let Some(selected) = self.selected_curve {
                    self.curves[selected].color.r = r;
                } else {
                    self.pending.color.r = r;
                }
            }
            EditMessage::SlideColorG(g) => {
                if let Some(selected) = self.selected_curve {
                    self.curves[selected].color.g = g;
                } else {
                    self.pending.color.g = g;
                }
            }
            EditMessage::SlideColorB(b) => {
                if let Some(selected) = self.selected_curve {
                    self.curves[selected].color.b = b;
                } else {
                    self.pending.color.b = b;
                }
            }
        }
        self.request_redraw();
    }

    pub fn view(&mut self) -> Element<EditMessage> {
        let (Curve { color, width, .. }, edit_title, remove_button) =
            if let Some(selected) = self.selected_curve {
                (&self.curves[selected], "selected curve", "Delete")
            } else {
                (&self.pending, "to add a curve", "Discard")
            };
        let (r, g, b) = (
            (color.r * 255.0).to_string(),
            (color.g * 255.0).to_string(),
            (color.b * 255.0).to_string(),
        );
        let width = width.to_string();

        let editable = Column::new()
            .width(Length::Units(200))
            .align_items(Alignment::Center)
            .padding(20)
            .spacing(15)
            .push(Text::new(edit_title).height(Length::Units(40)).size(25))
            .push(Text::new("Color:  "))
            .push(
                Row::new()
                    .push(
                        Slider::new(
                            &mut self.slider_color_r,
                            0.0..=1.0,
                            color.r,
                            EditMessage::SlideColorR,
                        )
                        .step(0.01),
                    )
                    .push(
                        text_input::TextInput::new(
                            &mut self.input_color_r,
                            "red",
                            r.as_str(),
                            EditMessage::InputColorR,
                        )
                        .width(Length::Units(50)),
                    ),
            )
            .push(
                Row::new()
                    .push(
                        Slider::new(
                            &mut self.slider_color_g,
                            0.0..=1.0,
                            color.g,
                            EditMessage::SlideColorG,
                        )
                        .step(0.01),
                    )
                    .push(
                        text_input::TextInput::new(
                            &mut self.input_color_g,
                            "green",
                            g.as_str(),
                            EditMessage::InputColorG,
                        )
                        .width(Length::Units(50)),
                    ),
            )
            .push(
                Row::new()
                    .push(
                        Slider::new(
                            &mut self.slider_color_b,
                            0.0..=1.0,
                            color.b,
                            EditMessage::SlideColorB,
                        )
                        .step(0.01),
                    )
                    .push(
                        text_input::TextInput::new(
                            &mut self.input_color_b,
                            "blue",
                            b.as_str(),
                            EditMessage::InputColorB,
                        )
                        .width(Length::Units(50)),
                    ),
            )
            .push(
                Row::new().push(Text::new("Width:  ")).push(
                    text_input::TextInput::new(
                        &mut self.input_width,
                        width.as_str(),
                        width.as_str(),
                        EditMessage::InputWidth,
                    )
                    .width(Length::Units(50)),
                ),
            )
            .push(
                Button::new(&mut self.remove_curve, Text::new(remove_button))
                    .on_press(EditMessage::RemoveCurve)
                    .style(style::Button::RemoveCurve),
            );

        let canvas = IcedCanvas::new(DrawingBoard {
            pending: &mut self.pending,
            curves: &self.curves,
            cache: &self.cache,
            selected_curve: &self.selected_curve,
        })
        .width(Length::Fill)
        .height(Length::Fill);

        Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(canvas)
            .push(editable)
            .into()
    }

    #[inline]
    pub fn reset(&mut self) {
        self.pending.points.clear();
        self.curves.clear();
        self.request_redraw();
    }

    #[inline]
    pub fn remove_curve(&mut self) {
        if let Some(selected) = self.selected_curve {
            self.curves.remove(selected);
            self.selected_curve = None;
        } else {
            self.pending.points.clear();
        }
        self.request_redraw();
    }

    pub fn save(&self) {
        if let Some(pathbuf) = save_file() {
            let document = self
                .curves
                .iter()
                .fold(Document::new(), |acc, x| acc.add(x.save()));

            svg::save(pathbuf, &document).unwrap();
        }
    }

    #[inline]
    pub fn request_redraw(&mut self) {
        self.cache.clear()
    }

    #[inline]
    pub fn change_shape(&mut self, s: Box<dyn Shape>) {
        if let Some(index) = self.selected_curve {
            self.curves[index].shape = s;
            if self.curves[index].points.len() < self.curves[index].shape.labor().into() {
                self.pending = self.curves.remove(index);
                self.selected_curve = None;
            }
        } else {
            self.pending.shape = s;
            if self.pending.points.len() == self.pending.shape.labor().into() {
                self.curves.push(Curve {
                    points: self.pending.points.drain(..).collect(),
                    ..self.pending.clone()
                });
            }
        }
        self.request_redraw();
    }
}

struct DrawingBoard<'a> {
    pending: &'a mut Curve,
    curves: &'a Vec<Curve>,
    cache: &'a canvas::Cache,
    selected_curve: &'a Option<usize>,
}

impl<'a> canvas::Program<EditMessage> for DrawingBoard<'a> {
    fn update(
        &mut self,
        event: Event,
        bounds: IcedRectangle,
        cursor: Cursor,
    ) -> (event::Status, Option<EditMessage>) {
        let cursor_position = if let Some(position) = cursor.position_in(&bounds) {
            position
        } else {
            return (event::Status::Ignored, None);
        };

        match event {
            Event::Mouse(mouse_event) => {
                match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        //如果pending是空的，则判定是否落在已有curve的points上
                        if self.pending.points.is_empty() {
                            for (index, curve) in self.curves.iter().enumerate() {
                                for point in curve.points.iter() {
                                    if cursor_position.distance(*point) < 5.0 {
                                        return (
                                            event::Status::Captured,
                                            Some(EditMessage::SelectCurve(Some(index))),
                                        );
                                    }
                                }
                            }
                        }

                        let labor: usize = self.pending.shape.labor().into();
                        self.pending.points.push(cursor_position);
                        if self.pending.points.len() == labor {
                            return (
                                event::Status::Captured,
                                Some(EditMessage::AddCurve(Curve {
                                    points: self.pending.points.drain(..).collect(),
                                    ..self.pending.clone()
                                })),
                            );
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        (event::Status::Ignored, None)
    }

    fn draw(&self, bounds: IcedRectangle, cursor: Cursor) -> Vec<Geometry> {
        let content = self.cache.draw(bounds.size(), |frame: &mut Frame| {
            if let Some(selected) = self.selected_curve {
                self.curves.iter().enumerate().for_each(|(index, curve)| {
                    curve.draw(frame, index == *selected);
                });
            } else {
                self.curves
                    .iter()
                    .for_each(|curve| Curve::draw(curve, frame, false))
            }

            frame.stroke(
                &Path::rectangle(Point::ORIGIN, frame.size()),
                Stroke::default(),
            );
        });

        let pending_curve = self.pending.preview(bounds, cursor);

        vec![content, pending_curve]
    }

    fn mouse_interaction(&self, bounds: IcedRectangle, cursor: Cursor) -> mouse::Interaction {
        if cursor.is_over(&bounds) {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }
}

fn is_valid_rgb(value: f32) -> bool {
    0.0 <= value && value < 256.0
}

pub mod shape {
    use std::fmt::Debug;

    use iced::{canvas::Path, Point};
    use svg::node::element::path::Data;

    use crate::ui::utils::get_size;
    use dyn_clone::{clone_box, DynClone};

    pub trait Shape: Send + Debug + DynClone {
        fn labor(&mut self) -> u8;
        fn preview(&self, points: &[Point], cursor_position: Point) -> Path;
        fn draw(&self, points: &[Point]) -> Path;
        fn save(&self, points: &[Point]) -> Data;
    }

    impl Clone for Box<dyn Shape> {
        fn clone(&self) -> Self {
            clone_box(&**self)
        }
    }

    #[derive(Debug, Default, Clone)]
    pub struct Rectangle;

    impl Shape for Rectangle {
        fn labor(&mut self) -> u8 {
            2
        }
        fn preview(&self, points: &[Point], cursor_position: Point) -> Path {
            Path::new(|p| match points[..] {
                [left_top] => {
                    p.rectangle(left_top, get_size(left_top, cursor_position));
                }
                _ => {}
            })
        }

        fn draw(&self, points: &[Point]) -> Path {
            Path::new(|builder| {
                if let [top_left, right_bottom] = points[..] {
                    builder.rectangle(top_left, get_size(top_left, right_bottom));
                }
            })
        }
        fn save(&self, points: &[Point]) -> Data {
            let data = Data::new();

            if let [Point { x: x1, y: y1 }, Point { x: x2, y: y2 }] = points[..] {
                {
                    data.move_to((x1, y1))
                        .line_to((x2, y1))
                        .line_to((x2, y2))
                        .line_to((x1, y2))
                        .line_to((x1, y1))
                }
            } else {
                data
            }
        }
    }

    #[derive(Debug, Default, Clone)]
    pub struct Triangle;

    impl Shape for Triangle {
        fn labor(&mut self) -> u8 {
            3
        }
        fn preview(&self, points: &[Point], cursor_position: Point) -> Path {
            Path::new(|p| match points[..] {
                [a] => {
                    p.move_to(a);
                    p.line_to(cursor_position);
                }
                [a, b] => {
                    p.move_to(a);
                    p.line_to(b);
                    p.line_to(cursor_position);
                    p.line_to(a);
                }
                _ => {}
            })
        }

        fn draw(&self, points: &[Point]) -> Path {
            Path::new(|builder| {
                if let [a, b, c] = points[..] {
                    builder.move_to(a);
                    builder.line_to(b);
                    builder.line_to(c);
                    builder.line_to(a);
                }
            })
        }
        fn save(&self, points: &[Point]) -> Data {
            let data = Data::new();

            if let [Point { x: ax, y: ay }, Point { x: bx, y: by }, Point { x: cx, y: cy }] =
                points[..]
            {
                data.move_to((ax, ay))
                    .line_to((bx, by))
                    .line_to((cx, cy))
                    .line_to((ax, ay))
            } else {
                data
            }
        }
    }

    #[derive(Debug, Default, Clone)]
    pub struct QuadraticBezier;

    impl Shape for QuadraticBezier {
        fn labor(&mut self) -> u8 {
            3
        }
        fn preview(&self, points: &[Point], cursor_position: Point) -> Path {
            Path::new(|p| match points[..] {
                [from] => {
                    p.move_to(from);
                    p.line_to(cursor_position);
                }
                [from, to] => {
                    p.move_to(from);
                    p.quadratic_curve_to(cursor_position, to);
                }
                _ => {}
            })
        }

        fn draw(&self, points: &[Point]) -> Path {
            Path::new(|builder| {
                if let [from, to, control] = points[..] {
                    builder.move_to(from);
                    builder.quadratic_curve_to(control, to);
                }
            })
        }
        fn save(&self, points: &[Point]) -> Data {
            let data = Data::new();

            if let [Point { x: fx, y: fy }, Point { x: tx, y: ty }, Point { x: cx, y: cy }] =
                points[..]
            {
                data.move_to((fx, fy))
                    .quadratic_curve_to(vec![cx, cy, tx, ty])
            } else {
                data
            }
        }
    }

    //在选择基于点控制的逻辑下弃用，完全只用二次的就可以画出任意多次的贝塞尔曲线
    // #[derive(Debug, Default, Clone)]
    // pub struct CubicBezier;

    // impl Shape for CubicBezier {
    //     fn labor(&mut self) -> u8 {
    //         4
    //     }
    //     fn preview(&self, points: &[Point], cursor_position: Point) -> Path {
    //         Path::new(|p| match points[..] {
    //             [from] => {
    //                 p.move_to(from);
    //                 p.line_to(cursor_position);
    //             }
    //             [from, to] => {
    //                 p.move_to(from);
    //                 p.quadratic_curve_to(cursor_position, to);
    //             }
    //             [from, to, control_a] => {
    //                 p.move_to(from);
    //                 p.bezier_curve_to(control_a, cursor_position, to);
    //             }
    //             _ => {}
    //         })
    //     }

    //     fn draw(&self, points: &[Point]) -> Path {
    //         Path::new(|builder| {
    //             if let [from, to, control_a, control_b] = points[..] {
    //                 builder.move_to(from);
    //                 builder.bezier_curve_to(control_a, control_b, to);
    //             }
    //         })
    //     }
    //     fn save(&self, points: &[Point]) -> Data {
    //         let data = Data::new();

    //         if let [Point { x: fx, y: fy }, Point { x: tx, y: ty }, Point { x: cax, y: cay }, Point { x: cbx, y: cby }] =
    //             points[..]
    //         {
    //             data.move_to((fx, fy))
    //                 .cubic_curve_to(vec![cax, cay, cbx, cby, tx, ty])
    //         } else {
    //             data
    //         }
    //     }
    // }
}
