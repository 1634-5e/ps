use iced::{
    canvas::event::{self, Event},
    canvas::{self, Canvas as IcedCanvas, Cursor, Frame, Geometry, Path, Stroke},
    mouse, Alignment, Color, Column, Element, Length, Point, Rectangle,
};

use svg::node::element::path::Data;
use svg::node::element::Path as SvgPath;
use svg::Document;

use super::{style, utils::get_size};
use crate::io::dialogs::save as save_file;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Shape {
    #[default]
    Rectangle,
    Triangle,
}

#[derive(Debug, Clone, Default)]
pub struct Curve {
    points: Vec<Point>,
    kind: Shape,
    color: Color,
    width: f32,
}

impl Curve {
    pub fn new(kind: Shape, color: Color, width: f32) -> Self {
        Curve {
            points: vec![],
            kind,
            color,
            width,
        }
    }

    pub fn labor(&self) -> u16 {
        match self.kind {
            Shape::Rectangle => 2,
            Shape::Triangle => 3,
        }
    }

    #[inline(always)]
    pub fn draw(curve: &Curve, frame: &mut Frame, selected: bool) {
        assert!(curve.points.len() == curve.labor().into());
        let path = Path::new(|builder| match curve.kind {
            Shape::Rectangle => {
                if let [top_left, right_bottom] = curve.points[..] {
                    builder.rectangle(top_left, get_size(top_left, right_bottom));
                }
            }
            Shape::Triangle => {
                if let [a, b, c] = curve.points[..] {
                    builder.move_to(a);
                    builder.line_to(b);
                    builder.line_to(c);
                    builder.line_to(a);
                }
            }
        });
        frame.stroke(
            &path,
            Stroke {
                width: curve.width,
                color: curve.color,
                ..Stroke::default()
            },
        );

        if selected {
            let selection_highlight = Path::new(|b| {
                for point in curve.points.iter() {
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
    pub fn save(&self, data: Data) -> Data {
        match self.kind {
            Shape::Rectangle => {
                assert!(self.points.len() == 2);
                let Point { x: x1, y: y1 } = self.points[0];
                let Point { x: x2, y: y2 } = self.points[1];
                data.move_to((x1, y1))
                    .line_to((x2, y1))
                    .line_to((x2, y2))
                    .line_to((x1, y2))
                    .line_to((x1, y1))
            }
            Shape::Triangle => {
                assert!(self.points.len() == 3);
                let Point { x: ax, y: ay } = self.points[0];
                let Point { x: bx, y: by } = self.points[1];
                let Point { x: cx, y: cy } = self.points[2];
                data.move_to((ax, ay))
                    .line_to((bx, by))
                    .line_to((cx, cy))
                    .line_to((ax, ay))
            }
        }
    }
}

#[derive(Debug)]
pub struct Pending {
    curve: Curve,
    cache: canvas::Cache,
}

impl Default for Pending {
    fn default() -> Self {
        Pending {
            curve: Curve::new(Shape::Rectangle, Color::BLACK, 2.0),
            cache: canvas::Cache::new(),
        }
    }
}

impl Pending {
    pub fn update(&mut self, new: Point) -> Option<Curve> {
        let labor: usize = self.curve.labor().into();
        self.curve.points.push(new);
        if self.curve.points.len() == labor {
            Some(Curve {
                points: self.curve.points.drain(..).collect(),
                ..self.curve
            })
        } else {
            None
        }
    }

    pub fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Geometry {
        let mut frame = Frame::new(bounds.size());

        if let Some(cursor_position) = cursor.position_in(&bounds) {
            let path = Path::new(|p| match self.curve.kind {
                Shape::Rectangle => {
                    if self.curve.points.len() == 1 {
                        let top_left = self.curve.points[0];
                        let right_bottom = cursor_position;
                        p.rectangle(top_left, get_size(top_left, right_bottom));
                    }
                }
                Shape::Triangle => {
                    let len = self.curve.points.len();
                    if len == 1 {
                        p.move_to(self.curve.points[0]);
                        p.line_to(cursor_position);
                    } else if len == 2 {
                        p.move_to(self.curve.points[0]);
                        p.line_to(self.curve.points[1]);
                        p.line_to(cursor_position);
                        p.line_to(self.curve.points[0]);
                    }
                }
            });
            frame.stroke(
                &path,
                Stroke {
                    width: self.curve.width,
                    color: self.curve.color,
                    ..Stroke::default()
                },
            )
        }

        frame.into_geometry()
    }

    pub fn view<'a>(
        &'a mut self,
        curves: &'a [Curve],
        selected: &'a Option<usize>,
    ) -> Element<'a, EditMessage> {
        IcedCanvas::new(Curves {
            pending: self,
            curves,
            selected,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear()
    }

    pub fn change_shape(&mut self, s: Shape) {
        self.curve.kind = s;
    }
}

pub struct Curves<'a> {
    pending: &'a mut Pending,
    curves: &'a [Curve],
    selected: &'a Option<usize>,
}

impl<'a> canvas::Program<EditMessage> for Curves<'a> {
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
                        if self.pending.curve.points.is_empty() {
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
                        if let Some(curve) = self.pending.update(cursor_position) {
                            return (event::Status::Captured, Some(EditMessage::AddCurve(curve)));
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
        let content = self.pending.cache.draw(bounds.size(), |frame: &mut Frame| {
            if let Some(selected) = self.selected {
                self.curves.iter().enumerate().for_each(|(index, curve)| {
                    if index == *selected {
                        Curve::draw(curve, frame, true);
                    } else {
                        Curve::draw(curve, frame, false);
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

        let pending_curve = self.pending.draw(bounds, cursor);

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

impl<'a> Curves<'a> {
    pub fn save(curves: &'a [Curve]) {
        if let Some(pathbuf) = save_file() {
            let data = curves.iter().fold(Data::new(), |acc, x| x.save(acc));

            let path = SvgPath::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("d", data.close());

            let document = Document::new().add(path);

            svg::save(pathbuf, &document).unwrap();
        }
    }
}

#[derive(Debug, Clone)]
pub enum EditMessage {
    AddCurve(Curve),
    SelectCurve(Option<usize>),
}

#[derive(Debug, Default)]
pub struct Edit {
    pub pending: Pending,
    curves: Vec<Curve>,
    selected_curve: Option<usize>,
}

impl Edit {
    pub fn update(&mut self, message: EditMessage) {
        match message {
            EditMessage::AddCurve(c) => self.curves.push(c),
            EditMessage::SelectCurve(c) => self.selected_curve = c,
        }
        self.pending.request_redraw();
    }

    pub fn view(&mut self) -> Element<EditMessage> {
        println!("{:?}", self.curves);
        let main_content = Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Alignment::Center)
            .push(self.pending.view(&self.curves, &self.selected_curve));

        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(main_content)
            .into()
    }

    pub fn reset(&mut self) {
        self.pending.curve.points.clear();
        self.curves.clear();
    }

    pub fn save(&self) {
        Curves::save(&self.curves);
    }
}
