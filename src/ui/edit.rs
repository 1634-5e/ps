use std::vec;

use iced::{
    canvas::event::{self, Event},
    canvas::{self, Canvas as IcedCanvas, Cursor, Frame, Geometry, Path, Stroke},
    mouse, Alignment, Color, Column, Element, Length, Point, Rectangle,
};

use svg::node::element::path::Data;
use svg::node::element::Path as SvgPath;
use svg::Document;

// use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};

use super::utils::get_size;
use crate::io::dialogs::save as save_file;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Shape {
    Rectangle,
    Triangle,
    #[default]
    QuadraticBezier,
    CubicBezier,
}

#[derive(Debug, Clone)]
pub struct Curve {
    points: Vec<Point>,
    shape: Shape,
    color: Color,
    width: f32,
}

impl Default for Curve {
    fn default() -> Self {
        Curve {
            points: vec![],
            shape: Shape::Rectangle,
            color: Color::BLACK,
            width: 2.0,
        }
    }
}

impl Curve {
    pub fn new(kind: Shape, color: Color, width: f32) -> Self {
        Curve {
            points: vec![],
            shape: kind,
            color,
            width,
        }
    }

    pub fn labor(&self) -> u16 {
        match self.shape {
            Shape::Rectangle => 2,
            Shape::Triangle => 3,
            Shape::QuadraticBezier => 3,
            Shape::CubicBezier => 4,
        }
    }

    pub fn preview(&self, bounds: Rectangle, cursor: Cursor) -> Geometry {
        let mut frame = Frame::new(bounds.size());

        if let Some(cursor_position) = cursor.position_in(&bounds) {
            let path = Path::new(|p| match self.shape {
                Shape::Rectangle => match &self.points[..] {
                    [left_top] => {
                        p.rectangle(*left_top, get_size(*left_top, cursor_position));
                    }
                    _ => {}
                },
                Shape::Triangle => match &self.points[..] {
                    [a] => {
                        p.move_to(*a);
                        p.line_to(cursor_position);
                    }
                    [a, b] => {
                        p.move_to(*a);
                        p.line_to(*b);
                        p.line_to(cursor_position);
                        p.line_to(*a);
                    }
                    _ => {}
                },
                Shape::QuadraticBezier => match &self.points[..] {
                    [from] => {
                        p.move_to(*from);
                        p.line_to(cursor_position);
                    }
                    [from, to] => {
                        p.move_to(*from);
                        p.quadratic_curve_to(cursor_position, *to);
                    }
                    _ => {}
                },
                Shape::CubicBezier => {
                    match &self.points[..] {
                        [from] => {
                            p.move_to(*from);
                            p.line_to(cursor_position);
                        }
                        [from, to] => {
                            p.move_to(*from);
                            p.quadratic_curve_to(cursor_position, *to);
                        }
                        [from, to, control_a] => {
                            p.move_to(*from);
                            p.bezier_curve_to(*control_a, cursor_position, *to);
                        }
                        _ => {}
                    }
                    // let len = self.points.len();
                    // if len == 1 {
                    //     p.move_to(self.points[0]);
                    //     p.line_to(cursor_position);
                    // } else if len == 2 {
                    //     p.move_to(self.points[0]);
                    //     p.quadratic_curve_to(cursor_position, self.points[1]);
                    // } else if len == 3 {
                    //     p.move_to(self.p)
                    // }
                }
            });
            frame.stroke(
                &path,
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
        assert!(self.points.len() == self.labor().into());
        let path = Path::new(|builder| match self.shape {
            Shape::Rectangle => {
                if let [top_left, right_bottom] = self.points[..] {
                    builder.rectangle(top_left, get_size(top_left, right_bottom));
                }
            }
            Shape::Triangle => {
                if let [a, b, c] = self.points[..] {
                    builder.move_to(a);
                    builder.line_to(b);
                    builder.line_to(c);
                    builder.line_to(a);
                }
            }
            Shape::QuadraticBezier => {
                if let [from, to, control] = self.points[..] {
                    builder.move_to(from);
                    builder.quadratic_curve_to(control, to);
                }
            }
            Shape::CubicBezier => {
                if let [from, to, control_a, control_b] = self.points[..] {
                    builder.move_to(from);
                    builder.bezier_curve_to(control_a, control_b, to);
                }
            }
        });
        frame.stroke(
            &path,
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
        let data = Data::new();

        let data = match self.shape {
            Shape::Rectangle if let [Point { x: x1, y: y1 }, Point { x: x2, y: y2 }] = self.points[..] => {
                 {
                    data
                        .move_to((x1, y1))
                        .line_to((x2, y1))
                        .line_to((x2, y2))
                        .line_to((x1, y2))
                        .line_to((x1, y1))
                }
            }
            Shape::Triangle if let [Point { x: ax, y: ay }, Point { x: bx, y: by }, Point { x: cx, y: cy }] =
            self.points[..] => {
                
                {
                    data
                        .move_to((ax, ay))
                        .line_to((bx, by))
                        .line_to((cx, cy))
                        .line_to((ax, ay))
                }
            }
            Shape::QuadraticBezier if let [Point { x: fx, y: fy }, Point { x: tx, y: ty }, Point { x: cx, y: cy }] =
            self.points[..] => {
                
                {
                    data
                        .move_to((fx, fy))
                        .quadratic_curve_to(vec![cx, cy, tx, ty])
                }
            }
            Shape::CubicBezier if let [Point { x: fx, y: fy }, Point { x: tx, y: ty }, Point { x: cax, y: cay }, Point { x: cbx, y: cby }] =
            self.points[..] => {
                
                {
                    data
                        .move_to((fx, fy))
                        .cubic_curve_to(vec![cax, cay, cbx, cby, tx, ty])
                }
            }
            _ => data
        };
        SvgPath::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", self.width)
        .set("d", data)
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
}

#[derive(Debug, Default)]
pub struct Edit {
    pub pending: Curve,
    curves: Vec<Curve>,
    cache: canvas::Cache,
    selected_curve: Option<usize>,
}

impl Edit {
    pub fn update(&mut self, message: EditMessage) {
        match message {
            EditMessage::AddCurve(c) => self.curves.push(c),
            EditMessage::SelectCurve(c) => self.selected_curve = c,
        }
        self.request_redraw();
    }

    pub fn view(&mut self) -> Element<EditMessage> {
        let main_content = Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Alignment::Center)
            .push(
                IcedCanvas::new(self)
                    .width(Length::Fill)
                    .height(Length::Fill),
            );

        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(main_content)
            .into()
    }

    pub fn reset(&mut self) {
        self.pending.points.clear();
        self.curves.clear();
        self.request_redraw();
    }

    pub fn save(&self) {
        if let Some(pathbuf) = save_file() {
            let document = self.curves.iter().fold(Document::new(), |acc, x| acc.add(x.save()));

            svg::save(pathbuf, &document).unwrap();
        }
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear()
    }

    pub fn change_shape(&mut self, s: Shape) {
        if let Some(index) = self.selected_curve {
            self.curves[index].shape = s;
        } else {
            self.pending.shape = s;
        }
    }
}

//FIXME:当Edit中新增了很多这里不需要的数据的时候，就应该换回原来的结构
impl canvas::Program<EditMessage> for Edit {
    fn update(
        &mut self,
        event: Event,
        bounds: Rectangle,
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

                        let labor: usize = self.pending.labor().into();
                        self.pending.points.push(cursor_position);
                        if self.pending.points.len() == labor {
                            return (
                                event::Status::Captured,
                                Some(EditMessage::AddCurve(Curve {
                                    points: self.pending.points.drain(..).collect(),
                                    ..self.pending
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

    fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry> {
        let content = self.cache.draw(bounds.size(), |frame: &mut Frame| {
            if let Some(selected) = self.selected_curve {
                self.curves.iter().enumerate().for_each(|(index, curve)| {
                    if index == selected {
                        curve.draw(frame, true);
                    } else {
                        curve.draw(frame, false);
                    }
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

    fn mouse_interaction(&self, bounds: Rectangle, cursor: Cursor) -> mouse::Interaction {
        if cursor.is_over(&bounds) {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }
}
