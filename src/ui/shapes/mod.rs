/*
    Shape: Canvas中curve的实体，
    必须的数据包括：点、颜色、线宽
    提供自定义Message、自定义Editable，
*/

use iced::{
    canvas::{Cursor, Frame, Geometry, Path, Stroke},
    Color, Point, Rectangle as IcedRectangle,
};
use svg::node::element::path::Data;

pub trait Shape {
    fn draw(&self, frame: &mut Frame, selected: bool);
    fn save(&self, data: Data) -> Data;
    fn points(&self) -> Vec<Point>;
}

pub trait Pending {
    fn update(&mut self, new: Point) -> Option<&dyn Shape>;
    fn draw(&self, bounds: IcedRectangle, cursor: Cursor) -> Geometry;
}

#[derive(Default, Debug, Clone)]
pub struct Rectangle {
    left_top: Point,
    right_bottom: Point,
    color: Color,
    width: f32,
}

impl Shape for Rectangle {
    fn draw(&self, frame: &mut Frame, selected: bool) {
        let path = Path::new(|builder| {
            builder.rectangle(self.left_top, get_size(self.left_top, self.right_bottom));
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
                for point in self.points() {
                    b.circle(point, 8.0);
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
    fn save(&self, data: Data) -> Data {
        let Point { x: x1, y: y1 } = self.left_top;
        let Point { x: x2, y: y2 } = self.right_bottom;
        data.move_to((x1, y1))
            .line_to((x2, y1))
            .line_to((x2, y2))
            .line_to((x1, y2))
            .line_to((x1, y1))
    }

    fn points(&self) -> Vec<Point> {
        vec![self.left_top, self.right_bottom]
    }
}

#[derive(Default, Debug, Clone)]
pub enum PendingRectangle {
    #[default]
    None,
    One {
        left_top: Point,
    },
}

impl Pending for PendingRectangle {
    fn update(&mut self, new: Point) -> Option<&dyn Shape> {
        match self {
            PendingRectangle::None => {
                *self = PendingRectangle::One { left_top: new };
                None
            }
            PendingRectangle::One { left_top } => Some(&Rectangle {
                left_top: *left_top,
                right_bottom: new,
                ..Rectangle::default()
            }),
        }
    }

    fn draw(&self, bounds: IcedRectangle, cursor: Cursor) -> Geometry {
        let mut frame = Frame::new(bounds.size());

        if let Some(cursor_position) = cursor.position_in(&bounds) {
            let path = Path::new(|p| match self {
                PendingRectangle::One { left_top } => {
                    let right_bottom = cursor_position;
                    p.rectangle(*left_top, get_size(*left_top, right_bottom));
                }
                PendingRectangle::None => {}
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
}
