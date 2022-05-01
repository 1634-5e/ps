use std::{collections::HashMap, fmt::Debug};

use iced::{canvas::Path, Point, Size};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use svg::node::element::path::Data;

use dyn_clone::{clone_box, DynClone};

use crate::ui::utils::get_size;
use crate::utils::SerdePoint;
use crate::utils::SerdeSize;

use super::utils::get_radius;

#[derive(Debug, Clone)]
pub enum ShapeMessage {
    Labor(Point),
    MovePoint(String, Point),
    Move(String, Point),
    Centered(Point),
    Reset,
}

pub trait Shape:
    Send + Debug + DynClone + serde_traitobject::Serialize + serde_traitobject::Deserialize
{
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

#[serde_as]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Line {
    #[serde_as(as = "Option<SerdePoint>")]
    pub(crate) from: Option<Point>,
    #[serde_as(as = "Option<SerdePoint>")]
    pub(crate) to: Option<Point>,
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
            ShapeMessage::Move(index, point) => {
                if let (Some(from), Some(to)) = (&mut self.from, &mut self.to) {
                    let (x, y) = match index.as_str() {
                        "from" => (point.x - from.x, point.y - from.y),
                        "to" => (point.x - to.x, point.y - to.y),
                        _ => (0.0, 0.0),
                    };
                    from.x += x;
                    from.y += y;
                    to.x += x;
                    to.y += y;
                }
            }
            ShapeMessage::Centered(p) => {
                if let (Some(from), Some(to)) = (self.from, self.to) {
                    let center_x = (from.x + to.x) / 2.0;
                    let center_y = (from.y + to.y) / 2.0;

                    let x = p.x - center_x;
                    let y = p.y - center_y;

                    if let (Some(from), Some(to)) = (&mut self.from, &mut self.to) {
                        from.x += x;
                        from.y += y;
                        to.x += x;
                        to.y += y;
                    }
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

#[serde_as]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Rectangle {
    #[serde_as(as = "Option<SerdePoint>")]
    pub(crate) top_left: Option<Point>,
    #[serde_as(as = "Option<SerdeSize>")]
    pub(crate) size: Option<Size>,
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
            ShapeMessage::Move(index, point) => {
                if let (Some(top_left), Some(size)) = (&mut self.top_left, &mut self.size) {
                    let (x, y) = match index.as_str() {
                        "top_left" => (point.x - top_left.x, point.y - top_left.y),
                        "bottom_left" => (point.x - top_left.x, point.y - top_left.y - size.height),
                        "top_right" => (point.x - top_left.x - size.width, point.y - top_left.y),
                        "bottom_right" => (point.x - top_left.x - size.width, point.y - top_left.y - size.height),
                        _ => (0.0, 0.0),
                    };
                    top_left.x += x;
                    top_left.y += y;
                }
            }
            ShapeMessage::Centered(p) => {
                if let (Some(top_left), Some(size)) = (self.top_left, self.size) {
                    let center_x = top_left.x + size.width / 2.0;
                    let center_y = top_left.y + size.height / 2.0;

                    let x = p.x - center_x;
                    let y = p.y - center_y;

                    if let (Some(top_left), Some(size)) = (&mut self.top_left, &mut self.size) {
                        top_left.x += x;
                        top_left.y += y;
                    }
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

#[serde_as]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Triangle {
    #[serde_as(as = "Option<SerdePoint>")]
    pub(crate) a: Option<Point>,
    #[serde_as(as = "Option<SerdePoint>")]
    pub(crate) b: Option<Point>,
    #[serde_as(as = "Option<SerdePoint>")]
    pub(crate) c: Option<Point>,
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
            ShapeMessage::Move(index, point) => {
                if let (Some(a), Some(b), Some(c)) = (&mut self.a, &mut self.b, &mut self.c) {
                    let (x, y) = match index.as_str() {
                        "a" => (point.x - a.x, point.y - a.y),
                        "b" => (point.x - b.x, point.y - b.y),
                        "c" => (point.x - c.x, point.y - c.y),
                        _ => (0.0, 0.0),
                    };
                    a.x += x;
                    a.y += y;
                    b.x += x;
                    b.y += y;
                    c.x += x;
                    c.y += y;
                }
            }
            ShapeMessage::Centered(p) => {
                if let (Some(a), Some(b), Some(c)) = (self.a, self.b, self.c) {
                    let center_x = (a.x + b.x + c.x) / 2.0;
                    let center_y = (a.y + b.y + c.y) / 2.0;

                    let x = p.x - center_x;
                    let y = p.y - center_y;

                    if let (Some(a), Some(b), Some(c)) = (&mut self.a, &mut self.b, &mut self.c) {
                        a.x += x;
                        a.y += y;
                        b.x += x;
                        b.y += y;
                        c.x += x;
                        c.y += y;
                    }
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

#[serde_as]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QuadraticBezier {
    #[serde_as(as = "Option<SerdePoint>")]
    pub(crate) a: Option<Point>,
    #[serde_as(as = "Option<SerdePoint>")]
    pub(crate) b: Option<Point>,
    #[serde_as(as = "Option<SerdePoint>")]
    pub(crate) control: Option<Point>,
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
            ShapeMessage::Move(index, point) => {
                if let (Some(a), Some(b), Some(control)) =
                    (&mut self.a, &mut self.b, &mut self.control)
                {
                    let (x, y) = match index.as_str() {
                        "a" => (point.x - a.x, point.y - a.y),
                        "b" => (point.x - b.x, point.y - b.y),
                        "control" => (point.x - control.x, point.y - control.y),
                        _ => (0.0, 0.0),
                    };
                    a.x += x;
                    a.y += y;
                    b.x += x;
                    b.y += y;
                    control.x += x;
                    control.y += y;
                }
            }
            ShapeMessage::Centered(p) => {
                if let (Some(a), Some(b)) = (self.a, self.b) {
                    let center_x = (a.x + b.x) / 2.0;
                    let center_y = (a.y + b.y) / 2.0;

                    let x = p.x - center_x;
                    let y = p.y - center_y;

                    if let (Some(a), Some(b), Some(control)) =
                        (&mut self.a, &mut self.b, &mut self.control)
                    {
                        a.x += x;
                        a.y += y;
                        b.x += x;
                        b.y += y;
                        control.x += x;
                        control.y += y;
                    }
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

#[serde_as]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Circle {
    #[serde_as(as = "Option<SerdePoint>")]
    pub(crate) center: Option<Point>,
    pub(crate) radius: Option<f32>,
}

impl Shape for Circle {
    fn is_complete(&self) -> bool {
        self.center.is_some() && self.radius.is_some()
    }
    fn is_empty(&self) -> bool {
        self.center.is_none() && self.radius.is_none()
    }
    fn points(&self) -> HashMap<String, Point> {
        let mut points = HashMap::new();

        if let Some(center) = self.center {
            points.insert(String::from("center"), center);
            if let Some(r) = self.radius {
                points.insert(
                    String::from("up"),
                    Point {
                        x: center.x,
                        y: center.y + r,
                    },
                );
                points.insert(
                    String::from("down"),
                    Point {
                        x: center.x,
                        y: center.y - r,
                    },
                );
                points.insert(
                    String::from("left"),
                    Point {
                        x: center.x - r,
                        y: center.y,
                    },
                );
                points.insert(
                    String::from("right"),
                    Point {
                        x: center.x + r,
                        y: center.y,
                    },
                );
            }
        }

        points
    }

    fn update(&mut self, message: ShapeMessage) {
        match message {
            ShapeMessage::Labor(point) => {
                if self.center.is_none() {
                    self.center = Some(point);
                } else if let Some(center) = self.center && self.radius.is_none() {
                    self.radius = Some(get_radius(center, point));
                }
            }
            ShapeMessage::MovePoint(index, point) => match index.as_str() {
                s if s =="center" => {
                    if let Some(center) = &mut self.center {
                        *center = point;
                    }
                }
                s => if let (Some(center), Some(radius)) = (&mut self.center, &mut self.radius) {
                    match s {
                        "up" => {
                                *radius = (point.y - center.y).abs();
                        }
                        "down" => {
                            *radius = (center.y - point.y).abs();
                        }
                        "left" => {
                            *radius = (center.x - point.x).abs();
                        }
                        "right" => {
                            *radius = (point.x - center.x).abs();
                        }
                        _ => {}
                    }
                }
            },
            ShapeMessage::Move(index, point) => {
                if let (Some(center), Some(radius)) = (&mut self.center, &mut self.radius) {
                    let (x, y) = match index.as_str() {
                        "center" => (point.x - center.x, point.y - center.y),
                        "up" => (point.x - center.x, point.y - center.y + *radius),
                        "down" => (point.x - center.x, point.y - center.y - *radius),
                        "left" => (point.x - center.x + *radius, point.y - center.y),
                        "right" => (point.x - center.x - *radius, point.y - center.y),
                        _ => (0.0, 0.0),
                    };
                    center.x+= x;
                    center.y += y;
                }
            }
            ShapeMessage::Centered(p) => {
                if let Some(center) = &mut self.center {
                    *center = p;
                }
            }
            ShapeMessage::Reset => {
                if !self.is_empty() {
                    self.center = None;
                    self.radius = None;
                }
            }
        }
    }

    fn preview(&self, cursor_position: Point) -> Option<Path> {
        if let Some(center) = self.center {
            if let Some(r) = self.radius {
                Some(Path::new(|p| {
                    p.circle(center, r);
                }))
            } else {
                Some(Path::new(|p| {
                    p.circle(center, get_radius(center, cursor_position));
                }))
            }
        } else {
            None
        }
    }
    fn draw(&self, selected: bool) -> (Option<Path>, Option<Path>) {
        if let (Some(center), Some(radius)) = (self.center, self.radius) {
            let to_fill = Path::new(|builder| {
                if selected {
                    for (_, point) in self.points() {
                        builder.circle(point, 5.0);
                    }
                }
            });
            (
                Some(Path::new(|builder| {
                    builder.circle(center, radius);
                })),
                Some(to_fill),
            )
        } else {
            (None, None)
        }
    }
    fn save(&self) -> Option<Data> {
        if let (Some(center), Some(radius)) = (self.center, self.radius) {
            Some(
                Data::new()
                    .move_to((center.x + radius, center.y))
                    .elliptical_arc_to(vec![
                        radius,
                        radius,
                        0.0,
                        0.0,
                        1.0,
                        center.x + radius,
                        center.y,
                    ]),
            )
        } else {
            None
        }
    }
}
