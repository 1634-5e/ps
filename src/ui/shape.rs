use std::{collections::HashMap, fmt::Debug, hash::Hash};

use iced::{canvas::Path, Point, Size, Vector};

use svg::node::element::path::Data;

use dyn_clone::{clone_box, DynClone};

use crate::ui::utils::get_size;

#[derive(Debug, Clone)]
pub enum ShapeMessage {
    Labor(Point),
    MovePoint(String, Point),
    Move(f32, f32),
    Centered(Point),
    Reset,
}

pub trait Shape: Send + Debug + DynClone {
    //utils
    fn is_empty(&self) -> bool;
    fn is_complete(&self) -> bool;
    fn points(&self) -> HashMap<String, Point>;

    //manipulation
    fn update(&mut self, message: ShapeMessage);

    //drawing
    fn preview(&self, cursor_position: Point) -> Option<Path>;
    fn draw(&self, selected: bool) -> (Option<Path>, Option<Path>);
    fn save(&self) -> Option<Data>;
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        clone_box(&**self)
    }
}

impl Default for Box<dyn Shape> {
    fn default() -> Self {
        Box::new(Line::default())
    }
}

#[derive(Debug, Default, Clone)]
pub struct Line {
    from: Option<Point>,
    to: Option<Point>,
}

impl Shape for Line {
    fn is_complete(&self) -> bool {
        self.from.is_some() && self.to.is_some()
    }
    fn is_empty(&self) -> bool {
        self.from.is_none() && self.to.is_none()
    }
    fn points(&self) -> HashMap<String, Point> {
        let mut points = HashMap::new();

        if let Some(from) = self.from {
            points.insert(String::from("from"), from);
        }

        if let Some(to) = self.to {
            points.insert(String::from("to"), to);
        }

        points
    }

    fn update(&mut self, message: ShapeMessage) {
        match message {
            ShapeMessage::Labor(point) => {
                if self.from.is_none() {
                    self.from = Some(point);
                } else if self.to.is_none() {
                    self.to = Some(point);
                }
            }
            ShapeMessage::MovePoint(index, point) => match index.as_str() {
                "from" => {
                    if let Some(from) = &mut self.from {
                        *from = point;
                    }
                }
                "to" => {
                    if let Some(to) = &mut self.to {
                        *to = point;
                    }
                }
                _ => {}
            },
            ShapeMessage::Move(x, y) => {
                if let Some(from) = &mut self.from {
                    from.x += x;
                    from.y += y;
                }
                if let Some(to) = &mut self.to {
                    to.x += x;
                    to.y += y;
                }
            }
            ShapeMessage::Centered(p) => {
                if let (Some(from), Some(to)) = (self.from, self.to) {
                    let center_x = (from.x + to.x) / 2.0;
                    let center_y = (from.y + to.y) / 2.0;

                    let x_shift = p.x - center_x;
                    let y_shift = p.y - center_y;

                    self.update(ShapeMessage::Move(x_shift, y_shift));
                }
            }
            ShapeMessage::Reset => {
                if !self.is_empty() {
                    self.from = None;
                    self.to = None;
                }
            }
        }
    }

    fn preview(&self, cursor_position: Point) -> Option<Path> {
        if let Some(from) = self.from {
            Some(Path::new(|p| {
                p.move_to(from);
                p.line_to(cursor_position);
            }))
        } else {
            None
        }
    }
    fn draw(&self, selected: bool) -> (Option<Path>, Option<Path>) {
        if let (Some(from), Some(to)) = (self.from, self.to) {
            let selected = if selected {
                Some(Path::new(|b| {
                    b.circle(from, 5.0);
                    b.circle(to, 5.0);
                }))
            } else {
                None
            };
            (
                Some(Path::new(|builder| {
                    builder.move_to(from);
                    builder.line_to(to);
                })),
                selected,
            )
        } else {
            (None, None)
        }
    }
    fn save(&self) -> Option<Data> {
        if let (Some(Point { x: x1, y: y1 }), Some(Point { x: x2, y: y2 })) = (self.from, self.to) {
            {
                Some(Data::new().move_to((x1, y1)).line_to((x2, y2)))
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Rectangle {
    top_left: Option<Point>,
    size: Option<Size>,
}

impl Shape for Rectangle {
    fn is_complete(&self) -> bool {
        self.top_left.is_some() && self.size.is_some()
    }
    fn is_empty(&self) -> bool {
        self.top_left.is_none() && self.size.is_none()
    }
    fn points(&self) -> HashMap<String, Point> {
        let mut points = HashMap::new();

        if let Some(top_left) = self.top_left {
            points.insert(String::from("top_left"), top_left);

            if let Some(Size { width, height }) = self.size {
                points.insert(
                    String::from("bottom_left"),
                    Point {
                        x: top_left.x,
                        y: top_left.y + height,
                    },
                );
                points.insert(
                    String::from("top_right"),
                    Point {
                        x: top_left.x + width,
                        y: top_left.y,
                    },
                );
                points.insert(
                    String::from("bottom_right"),
                    Point {
                        x: top_left.x + width,
                        y: top_left.y + height,
                    },
                );
            }
        }

        points
    }

    fn update(&mut self, message: ShapeMessage) {
        match message {
            ShapeMessage::Labor(point) => {
                if self.top_left.is_none() {
                    self.top_left = Some(point);
                }

                else if let Some(top_left) = self.top_left && self.size.is_none() {
                    self.size = Some(get_size(top_left, point));
                }
            }
            ShapeMessage::MovePoint(index, point) => match index.as_str() {
                "top_left" => {
                    if let (Some(top_left), Some(size)) = (&mut self.top_left, &mut self.size) {
                        size.width += top_left.x - point.x;
                        size.height += top_left.y - point.y;
                        *top_left = point;
                    }
                }
                "bottom_left" => {
                    if let (Some(top_left), Some(size)) = (&mut self.top_left, &mut self.size) {
                        size.height = point.y - top_left.y;
                        size.width += top_left.x - point.x;
                        top_left.x = point.x;
                    }
                }
                "top_right" => {
                    if let (Some(top_left), Some(size)) = (&mut self.top_left, &mut self.size) {
                        size.height += top_left.y - point.y;
                        size.width = point.x - top_left.x;
                        top_left.y = point.y;
                    }
                }
                "bottom_right" => {
                    if let (Some(top_left), Some(size)) = (&mut self.top_left, &mut self.size) {
                        size.width = point.x - top_left.x;
                        size.height = point.y - top_left.y;
                    }
                }
                _ => {}
            },
            ShapeMessage::Move(x, y) => {
                if let Some(top_left) = &mut self.top_left {
                    top_left.x += x;
                    top_left.y += y;
                }
            }
            ShapeMessage::Centered(p) => {
                if let (Some(top_left), Some(size)) = (self.top_left, self.size) {
                    let center_x = top_left.x + size.width / 2.0;
                    let center_y = top_left.y + size.height / 2.0;

                    let x_shift = p.x - center_x;
                    let y_shift = p.y - center_y;

                    self.update(ShapeMessage::Move(x_shift, y_shift));
                }
            }
            ShapeMessage::Reset => {
                if !self.is_empty() {
                    self.top_left = None;
                    self.size = None;
                }
            }
        }
    }

    fn preview(&self, cursor_position: Point) -> Option<Path> {
        if let Some(top_left) = self.top_left {
            Some(Path::new(|p| {
                p.rectangle(top_left, get_size(top_left, cursor_position))
            }))
        } else {
            None
        }
    }
    fn draw(&self, selected: bool) -> (Option<Path>, Option<Path>) {
        if let (Some(top_left), Some(size)) = (self.top_left, self.size) {
            let selected = if selected {
                Some(Path::new(|b| {
                    for (_, point) in self.points() {
                        b.circle(point, 5.0);
                    }
                }))
            } else {
                None
            };
            (
                Some(Path::new(|builder| {
                    builder.rectangle(top_left, size);
                })),
                selected,
            )
        } else {
            (None, None)
        }
    }
    fn save(&self) -> Option<Data> {
        if let (Some(top_left), Some(size)) = (self.top_left, self.size) {
            let data = Data::new();
            let (x1, y1) = (top_left.x, top_left.y);
            let (x2, y2) = (x1 + size.width, y1 + size.height);
            Some(
                data.move_to((x1, y1))
                    .line_to((x2, y1))
                    .line_to((x2, y2))
                    .line_to((x1, y2))
                    .line_to((x1, y1)),
            )
        } else {
            None
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Triangle {
    a: Option<Point>,
    b: Option<Point>,
    c: Option<Point>,
}

impl Shape for Triangle {
    fn is_complete(&self) -> bool {
        self.a.is_some() && self.b.is_some() && self.c.is_some()
    }
    fn is_empty(&self) -> bool {
        self.a.is_none() && self.b.is_none() && self.c.is_none()
    }
    fn points(&self) -> HashMap<String, Point> {
        let mut points = HashMap::new();

        if let Some(a) = self.a {
            points.insert(String::from("a"), a);
        }

        if let Some(b) = self.b {
            points.insert(String::from("b"), b);
        }

        if let Some(c) = self.c {
            points.insert(String::from("c"), c);
        }

        points
    }

    fn update(&mut self, message: ShapeMessage) {
        match message {
            ShapeMessage::Labor(point) => {
                if self.a.is_none() {
                    self.a = Some(point);
                } else if self.b.is_none() {
                    self.b = Some(point);
                } else if self.c.is_none() {
                    self.c = Some(point);
                }
            }
            ShapeMessage::MovePoint(index, point) => match index.as_str() {
                "a" => {
                    if let Some(a) = &mut self.a {
                        *a = point;
                    }
                }
                "b" => {
                    if let Some(b) = &mut self.b {
                        *b = point;
                    }
                }
                "c" => {
                    if let Some(c) = &mut self.c {
                        *c = point;
                    }
                }
                _ => {}
            },
            ShapeMessage::Move(x, y) => {
                if let Some(a) = &mut self.a {
                    a.x += x;
                    a.y += y;
                }
                if let Some(b) = &mut self.b {
                    b.x += x;
                    b.y += y;
                }
                if let Some(c) = &mut self.c {
                    c.x += x;
                    c.y += y;
                }
            }
            ShapeMessage::Centered(p) => {
                if let (Some(a), Some(b), Some(c)) = (self.a, self.b, self.c) {
                    let center_x = (a.x + b.x + c.x) / 2.0;
                    let center_y = (a.y + b.y + c.y) / 2.0;

                    let x_shift = p.x - center_x;
                    let y_shift = p.y - center_y;

                    self.update(ShapeMessage::Move(x_shift, y_shift));
                }
            }
            ShapeMessage::Reset => {
                if !self.is_empty() {
                    self.a = None;
                    self.b = None;
                    self.c = None;
                }
            }
        }
    }

    fn preview(&self, cursor_position: Point) -> Option<Path> {
        if let Some(a) = self.a {
            if let Some(b) = self.b {
                Some(Path::new(|p| {
                    p.move_to(a);
                    p.line_to(b);
                    p.line_to(cursor_position);
                    p.close();
                }))
            } else {
                Some(Path::new(|builder| {
                    builder.move_to(a);
                    builder.line_to(cursor_position);
                }))
            }
        } else {
            None
        }
    }
    fn draw(&self, selected: bool) -> (Option<Path>, Option<Path>) {
        if self.is_complete() {
            let points = self.points();
            let selected = if selected {
                Some(Path::new(|builder| {
                    for (_, point) in points.iter() {
                        builder.circle(*point, 5.0);
                    }
                }))
            } else {
                None
            };
            (
                Some(Path::new(|builder| {
                    for (index, (_, point)) in points.iter().enumerate() {
                        if index == 0 {
                            builder.move_to(*point);
                        } else {
                            builder.line_to(*point);
                        }
                    }
                    builder.close();
                })),
                selected,
            )
        } else {
            (None, None)
        }
    }
    fn save(&self) -> Option<Data> {
        let points = self.points();
        if self.is_complete() {
            let data =
                points
                    .into_iter()
                    .enumerate()
                    .fold(Data::new(), |acc, (index, (_, point))| {
                        if index == 0 {
                            acc.move_to((point.x, point.y))
                        } else {
                            acc.line_to((point.x, point.y))
                        }
                    });
            Some(data.close())
        } else {
            None
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct QuadraticBezier {
    a: Option<Point>,
    b: Option<Point>,
    control: Option<Point>,
}

impl Shape for QuadraticBezier {
    fn is_complete(&self) -> bool {
        self.a.is_some() && self.b.is_some() && self.control.is_some()
    }
    fn is_empty(&self) -> bool {
        self.a.is_none() && self.b.is_none() && self.control.is_none()
    }
    fn points(&self) -> HashMap<String, Point> {
        let mut points = HashMap::new();

        if let Some(a) = self.a {
            points.insert(String::from("a"), a);
        }

        if let Some(b) = self.b {
            points.insert(String::from("b"), b);
        }

        if let Some(control) = self.control {
            points.insert(String::from("control"), control);
        }

        points
    }

    fn update(&mut self, message: ShapeMessage) {
        match message {
            ShapeMessage::Labor(point) => {
                if self.a.is_none() {
                    self.a = Some(point);
                } else if self.b.is_none() {
                    self.b = Some(point);
                } else if self.control.is_none() {
                    self.control = Some(point);
                }
            }
            ShapeMessage::MovePoint(index, point) => match index.as_str() {
                "a" => {
                    if let Some(a) = &mut self.a {
                        *a = point;
                    }
                }
                "b" => {
                    if let Some(b) = &mut self.b {
                        *b = point;
                    }
                }
                "control" => {
                    if let Some(control) = &mut self.control {
                        *control = point;
                    }
                }
                _ => {}
            },
            ShapeMessage::Move(x, y) => {
                if let Some(a) = &mut self.a {
                    a.x += x;
                    a.y += y;
                }
                if let Some(b) = &mut self.b {
                    b.x += x;
                    b.y += y;
                }
                if let Some(control) = &mut self.control {
                    control.x += x;
                    control.y += y;
                }
            }
            ShapeMessage::Centered(p) => {
                if let (Some(a), Some(b)) = (self.a, self.b) {
                    let center_x = (a.x + b.x) / 2.0;
                    let center_y = (a.y + b.y) / 2.0;

                    let x_shift = p.x - center_x;
                    let y_shift = p.y - center_y;

                    self.update(ShapeMessage::Move(x_shift, y_shift));
                }
            }
            ShapeMessage::Reset => {
                if !self.is_empty() {
                    self.a = None;
                    self.b = None;
                    self.control = None;
                }
            }
        }
    }

    fn preview(&self, cursor_position: Point) -> Option<Path> {
        if let Some(a) = self.a {
            if let Some(b) = self.b {
                Some(Path::new(|p| {
                    p.move_to(a);
                    p.quadratic_curve_to(cursor_position, b);
                }))
            } else {
                Some(Path::new(|p| {
                    p.move_to(a);
                    p.line_to(cursor_position);
                }))
            }
        } else {
            None
        }
    }
    fn draw(&self, selected: bool) -> (Option<Path>, Option<Path>) {
        if let (Some(a), Some(b), Some(control)) = (self.a, self.b, self.control) {
            let to_fill = if selected {
                Some(Path::new(|builder| {
                    builder.circle(a, 5.0);
                    builder.circle(b, 5.0);
                    builder.circle(control, 5.0);
                }))
            } else {
                None
            };
            (
                Some(Path::new(|builder| {
                    builder.move_to(a);
                    builder.quadratic_curve_to(control, b);

                    if selected {
                        builder.move_to(a);
                        builder.line_to(control);
                        builder.line_to(b);
                    }
                })),
                to_fill,
            )
        } else {
            (None, None)
        }
    }
    fn save(&self) -> Option<Data> {
        if let (Some(a), Some(b), Some(control)) = (self.a, self.b, self.control) {
            Some(
                Data::new()
                    .move_to((a.x, a.y))
                    .quadratic_curve_to(vec![control.x, control.y, b.x, b.y]),
            )
        } else {
            None
        }
    }
}

// #[derive(Debug, Default, Clone)]
// pub struct Ellipse {
//      center: Option<Point>,
//     radii: Option<Vector<f32>>,
//      rotation: Option<f32>,
//      start_angle: Option<f32>,
//      end_angle: Option<f32>,
// }

// impl Shape for Ellipse {
//     fn is_complete(&self) -> bool {
//         self.center.is_some() && self.radii.is_some() && self.rotation.is_some() && self.start_angle.is_some() && self.end_angle.is_some()
//     }
//     fn is_empty(&self) -> bool {
//         self.center.is_none() && self.radii.is_none() && self.rotation.is_none() && self.start_angle.is_none() && self.end_angle.is_none()
//     }
//     fn points(&self) -> HashMap<String, Point> {
//         let mut points = HashMap::new();

//         if let Some(center) = self.center {
//             points.insert(String::from("center"), center);
//             if let Some(r) = self.radius {
//                 points.insert(
//                     String::from("up"),
//                     Point {
//                         x: center.x,
//                         y: center.y + r,
//                     },
//                 );
//                 points.insert(
//                     String::from("down"),
//                     Point {
//                         x: center.x,
//                         y: center.y - r,
//                     },
//                 );
//                 points.insert(
//                     String::from("left"),
//                     Point {
//                         x: center.x - r,
//                         y: center.y,
//                     },
//                 );
//                 points.insert(
//                     String::from("right"),
//                     Point {
//                         x: center.x,
//                         y: center.y + r,
//                     },
//                 );
//             }
//         }

//         points
//     }

//     fn update(&mut self, message: ShapeMessage) {
//         match message {
//             ShapeMessage::Labor(point) => {
//                 if self.center.is_none() {
//                     self.center = Some(point);
//                 } else if let Some(center) = self.center && self.radius.is_none() {
//                     self.radius = Some(((point.x - center.x) * (point.x - center.x) + (point.y - center.y) * (point.y - center.y)).sqrt());
//                 }
//             }
//             ShapeMessage::MovePoint(index, point) => match index.as_str() {
//                 s if s =="center" => {
//                     if let Some(center) = &mut self.center {
//                         *center = point;
//                     }
//                 }
//                 s => if let (Some(center), Some(radius)) = (self.center, self.radius) {
//                     match s {
//                         "up" => {
                            
//                         }
//                         "down" => {
                            
//                         }"left" => {
                            
//                         }"right" => {
                            
//                         }
//                         _ => {}
//                     }
//                 }
                
//             },
//             ShapeMessage::Move(x, y) => {
//                 if let Some(a) = &mut self.a {
//                     a.x += x;
//                     a.y += y;
//                 }
//                 if let Some(b) = &mut self.b {
//                     b.x += x;
//                     b.y += y;
//                 }
//                 if let Some(control) = &mut self.control {
//                     control.x += x;
//                     control.y += y;
//                 }
//             }
//             ShapeMessage::Centered(p) => {
//                 if let (Some(a), Some(b)) = (self.a, self.b) {
//                     let center_x = (a.x + b.x) / 2.0;
//                     let center_y = (a.y + b.y) / 2.0;

//                     let x_shift = p.x - center_x;
//                     let y_shift = p.y - center_y;

//                     self.update(ShapeMessage::Move(x_shift, y_shift));
//                 }
//             }
//             ShapeMessage::Reset => {
//                 if !self.is_empty() {
//                     self.a = None;
//                     self.b = None;
//                     self.control = None;
//                 }
//             }
//         }
//     }

//     fn preview(&self, cursor_position: Point) -> Option<Path> {
//         if let Some(a) = self.a {
//             if let Some(b) = self.b {
//                 Some(Path::new(|p| {
//                     p.move_to(a);
//                     p.quadratic_curve_to(cursor_position, b);
//                 }))
//             } else {
//                 Some(Path::new(|p| {
//                     p.move_to(a);
//                     p.line_to(cursor_position);
//                 }))
//             }
//         } else {
//             None
//         }
//     }
//     fn draw(&self, selected: bool) -> (Option<Path>, Option<Path>) {
//         if let (Some(a), Some(b), Some(control)) = (self.a, self.b, self.control) {
//             let to_fill = if selected {
//                 Some(Path::new(|builder| {
//                     builder.circle(a, 5.0);
//                     builder.circle(b, 5.0);
//                     builder.circle(control, 5.0);
//                 }))
//             } else {
//                 None
//             };
//             (
//                 Some(Path::new(|builder| {
//                     builder.move_to(a);
//                     builder.quadratic_curve_to(control, b);

//                     if selected {
//                         builder.move_to(a);
//                         builder.line_to(control);
//                         builder.line_to(b);
//                     }
//                 })),
//                 to_fill,
//             )
//         } else {
//             (None, None)
//         }
//     }
//     fn save(&self) -> Option<Data> {
//         if let (Some(a), Some(b), Some(control)) = (self.a, self.b, self.control) {
//             Some(
//                 Data::new()
//                     .move_to((a.x, a.y))
//                     .quadratic_curve_to(vec![control.x, control.y, b.x, b.y]),
//             )
//         } else {
//             None
//         }
//     }
// }
