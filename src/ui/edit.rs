use iced::{
    button,
    canvas::event::{self, Event},
    canvas::{self, Canvas as IcedCanvas, Cursor, Fill, Frame, Geometry, Path, Stroke},
    keyboard::KeyCode,
    mouse, slider, text_input, Alignment, Button, Color, Column, Element, Length, Point,
    Rectangle as IcedRectangle, Row, Slider, Text,
};

use serde::{Deserialize, Serialize};
use svg::node::element::Path as SvgPath;
use svg::Document;

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
                    b.circle(*point, 5.0);
                }
            });

            frame.fill(&selection_highlight, Fill { ..Fill::default() });
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
    drawing_board: DrawingBoard,

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
            EditMessage::InputColorR(r) => {
                if let Ok(r) = r.parse::<f32>() {
                    if is_valid_rgb(r) {
                        if let Some(selected) = self.drawing_board.selected_curve {
                            self.drawing_board.curves[selected].color.r = r / 255.0;
                        } else {
                            self.drawing_board.pending.color.r = r / 255.0;
                        }
                    }
                }
            }
            EditMessage::InputColorG(g) => {
                if let Ok(g) = g.parse::<f32>() {
                    if is_valid_rgb(g) {
                        if let Some(selected) = self.drawing_board.selected_curve {
                            self.drawing_board.curves[selected].color.g = g / 255.0;
                        } else {
                            self.drawing_board.pending.color.g = g / 255.0;
                        }
                    }
                }
            }
            EditMessage::InputColorB(b) => {
                if let Ok(b) = b.parse::<f32>() {
                    if is_valid_rgb(b) {
                        if let Some(selected) = self.drawing_board.selected_curve {
                            self.drawing_board.curves[selected].color.b = b / 255.0;
                        } else {
                            self.drawing_board.pending.color.b = b / 255.0;
                        }
                    }
                }
            }
            EditMessage::InputWidth(w) => {
                if let Ok(width) = w.parse::<f32>() {
                    if let Some(selected) = self.drawing_board.selected_curve {
                        self.drawing_board.curves[selected].width = width;
                    } else {
                        self.drawing_board.pending.width = width;
                    }
                }
            }
            EditMessage::RemoveCurve => self.remove_curve(),
            EditMessage::SlideColorR(r) => {
                if let Some(selected) = self.drawing_board.selected_curve {
                    self.drawing_board.curves[selected].color.r = r;
                } else {
                    self.drawing_board.pending.color.r = r;
                }
            }
            EditMessage::SlideColorG(g) => {
                if let Some(selected) = self.drawing_board.selected_curve {
                    self.drawing_board.curves[selected].color.g = g;
                } else {
                    self.drawing_board.pending.color.g = g;
                }
            }
            EditMessage::SlideColorB(b) => {
                if let Some(selected) = self.drawing_board.selected_curve {
                    self.drawing_board.curves[selected].color.b = b;
                } else {
                    self.drawing_board.pending.color.b = b;
                }
            }
        }
        self.drawing_board.redraw();
    }

    pub fn view(&mut self) -> Element<EditMessage> {
        println!("{:?}", self.drawing_board.curves);
        let (Curve { color, width, .. }, edit_title, remove_button) =
            if let Some(selected) = self.drawing_board.selected_curve {
                (
                    &self.drawing_board.curves[selected],
                    "selected curve",
                    "Delete",
                )
            } else {
                (&self.drawing_board.pending, "to add a curve", "Discard")
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

        let canvas = IcedCanvas::new(&mut self.drawing_board)
            .width(Length::Fill)
            .height(Length::Fill);

        Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(canvas)
            .push(editable)
            .into()
    }

    pub fn save(&self) {
        if let Some(pathbuf) = save_file() {
            let document = self
                .drawing_board
                .curves
                .iter()
                .fold(Document::new(), |acc, x| acc.add(x.save()));

            svg::save(pathbuf, &document).unwrap();
        }
    }

    pub fn reset(&mut self) {
        self.drawing_board.pending.points.clear();
        self.drawing_board.curves.clear();
        self.drawing_board.redraw();
    }

    pub fn remove_curve(&mut self) {
        if let Some(selected) = self.drawing_board.selected_curve {
            self.drawing_board.curves.remove(selected);
            self.drawing_board.selected_curve = None;
        } else {
            self.drawing_board.pending.points.clear();
        }
        self.drawing_board.redraw();
    }

    pub fn change_shape(&mut self, s: Box<dyn Shape>) {
        if let Some(index) = self.drawing_board.selected_curve {
            self.drawing_board.curves[index].shape = s;
            if self.drawing_board.curves[index].points.len()
                < self.drawing_board.curves[index].shape.labor().into()
            {
                self.drawing_board.pending = self.drawing_board.curves.remove(index);
                self.drawing_board.selected_curve = None;
            }
        } else {
            self.drawing_board.pending.shape = s;
            if self.drawing_board.pending.points.len()
                == self.drawing_board.pending.shape.labor().into()
            {
                self.drawing_board.curves.push(Curve {
                    points: self.drawing_board.pending.points.drain(..).collect(),
                    ..self.drawing_board.pending.clone()
                });
            }
        }
        self.drawing_board.redraw();
    }
}

#[derive(Debug, Default)]
struct DrawingBoard {
    pending: Curve,
    curves: Vec<Curve>,
    cache: canvas::Cache,
    selected_curve: Option<usize>,
    curve_to_select: Option<usize>,

    pressed_point: Option<Point>,
    nearest_point: Option<(usize, usize)>, //（curves下标，points下标

    ctrl_pressed: bool,
}

impl<'a> canvas::Program<EditMessage> for &'a mut DrawingBoard {
    fn update(
        &mut self,
        event: Event,
        bounds: IcedRectangle,
        cursor: Cursor,
    ) -> (event::Status, Option<EditMessage>) {
        let cursor_position = if let Some(position) = cursor.position_in(&bounds) {
            position
        } else {
            self.pressed_point = None;
            self.nearest_point = None;
            return (event::Status::Ignored, None);
        };

        if self.pending.points.is_empty() {
            match event {
                Event::Mouse(mouse_event) => match mouse_event {
                    mouse::Event::CursorMoved { position: _ } => {
                        if let Some((curves_index, points_index)) = self.nearest_point {
                            let nearest_point = &mut self.curves[curves_index].points[points_index];
                            if nearest_point.distance(cursor_position)
                                > DrawingBoard::DETERMINANT_DISTANCE
                            {
                                if self.ctrl_pressed {
                                    let x_shift = cursor_position.x - nearest_point.x;
                                    let y_shift = cursor_position.y - nearest_point.y;

                                    for point in self.curves[curves_index].points.iter_mut() {
                                        point.x += x_shift;
                                        point.y += y_shift;
                                    }
                                } else {
                                    *nearest_point = cursor_position;
                                }
                            }
                        }

                        //如果离得远了就取消预览
                        if let Some(to_select) = self.curve_to_select {
                            let mut to_cancel = true;
                            for point in self.curves[to_select].points.iter() {
                                if point.distance(cursor_position)
                                    < DrawingBoard::DETERMINANT_DISTANCE
                                {
                                    to_cancel = false;
                                }
                            }
                            if to_cancel {
                                self.curve_to_select = None;
                            }
                        }
                        //如果没有预览中的，则选择一个距离最近的curve
                        if self.curve_to_select.is_none() {
                            self.curve_to_select = self.decide_which_curve(cursor_position);
                        }
                    }
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        self.pressed_point = Some(cursor_position);
                        let mut nearest_point = None;
                        let mut last_distance = DrawingBoard::DETERMINANT_DISTANCE;
                        for (curves_index, curve) in self.curves.iter().enumerate() {
                            for (points_index, point) in curve.points.iter().enumerate() {
                                let distance = point.distance(cursor_position);
                                if distance < DrawingBoard::DETERMINANT_DISTANCE
                                    && distance < last_distance
                                {
                                    last_distance = distance;
                                    nearest_point = Some((curves_index, points_index));
                                }
                            }
                        }
                        self.nearest_point = nearest_point;
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        if let Some(pressed) = self.pressed_point {
                            if pressed.distance(cursor_position)
                                < DrawingBoard::DETERMINANT_DISTANCE
                            {
                                self.selected_curve = self.decide_which_curve(cursor_position);

                                if self.selected_curve.is_none() {
                                    self.pending.points.push(cursor_position);
                                }
                            }
                        }
                        self.pressed_point = None;
                        self.nearest_point = None;
                    }
                    _ => {}
                },
                Event::Keyboard(ke) => match ke {
                    iced::keyboard::Event::KeyPressed {
                        key_code,
                        modifiers: _,
                    } => {
                        if key_code == KeyCode::LControl || key_code == KeyCode::RControl {
                            self.ctrl_pressed = true;
                        }
                    }
                    iced::keyboard::Event::KeyReleased {
                        key_code,
                        modifiers: _,
                    } => {
                        if key_code == KeyCode::LControl || key_code == KeyCode::RControl {
                            self.ctrl_pressed = false;
                        }
                    }
                    _ => {}
                },
            }
        } else {
            match event {
                Event::Mouse(mouse_event) => match mouse_event {
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        self.pending.points.push(cursor_position);
                        if self.pending.points.len() == self.pending.shape.labor().into() {
                            self.curves.push(Curve {
                                points: self.pending.points.drain(..).collect(),
                                ..self.pending.clone()
                            });
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        self.redraw();

        (event::Status::Ignored, None)
    }

    fn draw(&self, bounds: IcedRectangle, cursor: Cursor) -> Vec<Geometry> {
        let content = self.cache.draw(bounds.size(), |frame: &mut Frame| {
            let mut select = vec![];
            if let Some(ref selected) = self.selected_curve {
                select.push(*selected);
            }
            if let Some(ref to_select) = self.curve_to_select {
                select.push(*to_select);
            }

            self.curves.iter().enumerate().for_each(|(index, curve)| {
                curve.draw(frame, select.contains(&index));
            });

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
            if self.selected_curve.is_some() && self.ctrl_pressed {
                mouse::Interaction::Grabbing
            } else {
                mouse::Interaction::Crosshair
            }
        } else {
            mouse::Interaction::default()
        }
    }
}

impl DrawingBoard {
    const DETERMINANT_DISTANCE: f32 = 10.0;

    fn decide_which_curve(&self, cursor_position: Point) -> Option<usize> {
        let mut res: Option<usize> = None;
        let mut last_distance = Self::DETERMINANT_DISTANCE;
        for (index, curve) in self.curves.iter().enumerate() {
            for point in curve.points.iter() {
                let distance = point.distance(cursor_position);
                if distance < Self::DETERMINANT_DISTANCE && distance < last_distance {
                    last_distance = distance;
                    res = Some(index);
                }
            }
        }
        res
    }

    pub fn redraw(&mut self) {
        self.cache.clear()
    }
}

fn is_valid_rgb(value: f32) -> bool {
    0.0 <= value && value < 256.0
}

pub mod shape {
    use std::fmt::Debug;

    use iced::{canvas::Path, Point};
    use serde::{Deserialize, Serialize};
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

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
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

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
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

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
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
}
