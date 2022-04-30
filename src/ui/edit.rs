use std::fmt::Display;

use iced::{
    canvas::{
        self, Canvas as IcedCanvas, Cursor, Fill, Frame, Geometry, LineCap, LineJoin, Path, Stroke,
    },
    canvas::{
        event::{self, Event},
        LineDash,
    },
    keyboard::{self, KeyCode, Modifiers},
    mouse, pick_list, slider, text_input, Alignment, Color, Column, Element, Length, PickList,
    Point, Rectangle as IcedRectangle, Row, Slider, Space, Text,
};

use svg::node::element::Path as SvgPath;
use svg::Document;

use super::{
    shape::*,
    utils::{get_format_color, is_valid_rgb},
};
use crate::io::dialogs::save as save_file;

#[derive(Debug, Clone)]
pub enum CurveMessage {
    Shape(ShapeMessage),

    InputColorR(String),
    InputColorG(String),
    InputColorB(String),

    SlideColorR(f32),
    SlideColorG(f32),
    SlideColorB(f32),

    InputWidth(String),

    LineCapSelected(EqLineCap),
    LineJoinSelected(EqLineJoin),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EqLineCap {
    Butt,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EqLineJoin {
    Miter,
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

#[derive(Debug, Clone)]
pub struct Curve {
    shape: Box<dyn Shape>,
    color: Color,
    width: f32,
    line_cap: EqLineCap,
    line_join: EqLineJoin,
    segments: Vec<f32>,
    offset: usize,
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
            segments: line_dash.segments.to_vec(),
            offset: line_dash.offset,
        }
    }
}

impl Curve {
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
            CurveMessage::LineCapSelected(lc) => self.line_cap = lc,
            CurveMessage::LineJoinSelected(lj) => self.line_join = lj,
        }
        None
    }

    pub fn preview(&self, frame: &mut Frame, cursor_position: Point) {
        if let Some(path) = self.shape.preview(cursor_position) {
            frame.stroke(
                &path,
                Stroke {
                    color: self.color,
                    width: self.width,
                    line_cap: self.line_cap.into(),
                    line_join: self.line_join.into(),
                    line_dash: LineDash {
                        segments: &self.segments,
                        offset: self.offset,
                    },
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
                    line_dash: LineDash {
                        segments: &self.segments,
                        offset: self.offset,
                    },
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
        let data = self.shape.save()?;
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

#[derive(Debug, Clone)]
pub enum EditMessage {
    Curve(CurveMessage),
    ChangeShape(Box<dyn Shape>),
    AddFromPending,
    CurvePasted(Point),
    Clear,
    RemoveCurve,
}

#[derive(Debug, Default)]
pub struct Edit {
    pending: Curve,
    pub curves: Vec<Curve>,

    pub dirty: bool,

    copied_curve: Option<Curve>, //用于复制粘贴
    selected_curve: Option<usize>,
    curve_to_select: Option<usize>,

    pressed_point: Option<Point>,
    nearest_point: Option<(usize, String)>, //（curves下标，points下标,以及点坐标

    ctrl_pressed: bool,

    cache: canvas::Cache, //缓存

    input_color_r: text_input::State,
    input_color_g: text_input::State,
    input_color_b: text_input::State,
    input_width: text_input::State,

    slider_color_r: slider::State,
    slider_color_g: slider::State,
    slider_color_b: slider::State,

    pick_line_cap: pick_list::State<EqLineCap>,
    pick_line_join: pick_list::State<EqLineJoin>,
}

impl Edit {
    // pub fn new(curves: Vec<Curve>) -> Self {
    //     Edit {
    //         curves,
    //         ..Edit::default()
    //     }
    // }

    pub fn update(&mut self, message: EditMessage) {
        match message {
            EditMessage::Curve(cm) => {
                if let Some(selected) = self.selected_curve {
                    if let Some(new_message) = self.curves[selected].update(cm) {
                        self.update(new_message);
                    }
                } else {
                    if let Some(new_message) = self.pending.update(cm) {
                        self.update(new_message);
                    }
                }
            }
            EditMessage::ChangeShape(s) => {
                if self.pending.shape.is_empty() {
                    self.pending.shape = s;
                }
            }
            EditMessage::Clear => {
                self.curves.clear();
                self.selected_curve = None;
            }
            EditMessage::RemoveCurve => {
                if let Some(selected) = self.selected_curve {
                    self.curves.remove(selected);
                    self.selected_curve = None;
                }
            }
            EditMessage::AddFromPending => {
                self.curves.push(self.pending.clone());
                self.pending.shape.update(ShapeMessage::Reset);
                self.selected_curve = Some(self.curves.len() - 1);
            }
            EditMessage::CurvePasted(point) => {
                if let Some(copied) = &self.copied_curve {
                    let mut new = copied.clone();
                    new.shape.update(ShapeMessage::Centered(point));
                    self.curves.push(new);
                    self.selected_curve = Some(self.curves.len() - 1);
                }
            }
        }

        if self.pending.shape.is_complete() {
            self.update(EditMessage::AddFromPending);
        }

        self.dirty = true;
        self.redraw();
    }

    pub fn view(&mut self) -> Element<EditMessage> {
        let (
            points,
            Curve {
                color,
                width,
                line_cap,
                line_join,
                segments,
                offset,
                ..
            },
            edit_title,
        ) = if let Some(selected) = self.selected_curve {
            (
                self.curves[selected].shape.points(),
                &self.curves[selected],
                "Selected curve",
            )
        } else {
            (self.pending.shape.points(), &self.pending, "Creating")
        };

        let (r, g, b) = (
            (color.r * 255.0).to_string(),
            (color.g * 255.0).to_string(),
            (color.b * 255.0).to_string(),
        );
        let width = width.to_string();

        //排序points防止顺序一直变化
        let mut points = points.into_iter().collect::<Vec<(String, Point)>>();
        points.sort_by(|(a, _), (b, _)| a.cmp(b));

        let mut editable = Column::new()
            .width(Length::FillPortion(2))
            .align_items(Alignment::Start)
            .spacing(15)
            .push(Text::new(edit_title).height(Length::Units(40)).size(25));

        editable = points.into_iter().fold(editable, |acc, (index, p)| {
            acc.push(
                Row::new()
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(Text::new(format!("{:?}: ({:.2},{:.2})", index, p.x, p.y))),
            )
        });

        editable = editable
            .push(
                Row::new()
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(Text::new(format!("Segments: {:?}", segments)))
                    .push(Text::new(format!("offset: {:?}", offset))),
            )
            .push(Text::new("Color:  "))
            .push(
                Row::new()
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(
                        Slider::new(
                            &mut self.slider_color_r,
                            0.0..=1.0,
                            color.r,
                            CurveMessage::SlideColorR,
                        )
                        .step(0.01),
                    )
                    .push(
                        text_input::TextInput::new(
                            &mut self.input_color_r,
                            "red",
                            r.as_str(),
                            CurveMessage::InputColorR,
                        )
                        .width(Length::Units(50)),
                    ),
            )
            .push(
                Row::new()
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(
                        Slider::new(
                            &mut self.slider_color_g,
                            0.0..=1.0,
                            color.g,
                            CurveMessage::SlideColorG,
                        )
                        .step(0.01),
                    )
                    .push(
                        text_input::TextInput::new(
                            &mut self.input_color_g,
                            "green",
                            g.as_str(),
                            CurveMessage::InputColorG,
                        )
                        .width(Length::Units(50)),
                    ),
            )
            .push(
                Row::new()
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(
                        Slider::new(
                            &mut self.slider_color_b,
                            0.0..=1.0,
                            color.b,
                            CurveMessage::SlideColorB,
                        )
                        .step(0.01),
                    )
                    .push(
                        text_input::TextInput::new(
                            &mut self.input_color_b,
                            "blue",
                            b.as_str(),
                            CurveMessage::InputColorB,
                        )
                        .width(Length::Units(50)),
                    ),
            )
            .push(
                Row::new()
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(Text::new("Width:  "))
                    .push(
                        text_input::TextInput::new(
                            &mut self.input_width,
                            width.as_str(),
                            width.as_str(),
                            CurveMessage::InputWidth,
                        )
                        .width(Length::Units(50)),
                    ),
            )
            .push(
                Row::new()
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(Text::new("Line Cap:  "))
                    .push(PickList::new(
                        &mut self.pick_line_cap,
                        vec![EqLineCap::Butt, EqLineCap::Round, EqLineCap::Square],
                        Some(*line_cap),
                        CurveMessage::LineCapSelected,
                    )),
            )
            .push(
                Row::new()
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(Text::new("Line Join:  "))
                    .push(PickList::new(
                        &mut self.pick_line_join,
                        vec![EqLineJoin::Miter, EqLineJoin::Round, EqLineJoin::Bevel],
                        Some(*line_join),
                        CurveMessage::LineJoinSelected,
                    )),
            );

        let canvas = Row::new()
            .width(Length::FillPortion(7))
            .height(Length::Fill)
            .push(Space::with_width(Length::Units(10)))
            .push(
                Column::new()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .push(Space::with_height(Length::Units(10)))
                    .push(
                        IcedCanvas::new(DrawingBoard {
                            pending: &mut self.pending,
                            curves: &self.curves,

                            copied_curve: &mut self.copied_curve,
                            selected_curve: &mut self.selected_curve,
                            curve_to_select: &mut self.curve_to_select,

                            pressed_point: &mut self.pressed_point,
                            nearest_point: &mut self.nearest_point, //（curves下标，points下标

                            ctrl_pressed: &mut self.ctrl_pressed,

                            cache: &mut self.cache,
                        })
                        .width(Length::Fill)
                        .height(Length::Fill),
                    )
                    .push(Space::with_height(Length::Units(10))),
            )
            .push(Space::with_width(Length::Units(10)));

        Row::new()
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(canvas)
            .push((Element::<CurveMessage>::from(editable)).map(EditMessage::Curve))
            .into()
    }

    pub fn export(&self) {
        if let Some(pathbuf) = save_file() {
            let document = self.curves.iter().fold(Document::new(), |acc, x| {
                if let Some(path) = x.save() {
                    acc.add(path)
                } else {
                    acc
                }
            });

            svg::save(pathbuf, &document).unwrap();
        }
    }

    pub fn redraw(&mut self) {
        self.cache.clear()
    }
}

#[derive(Debug)]
struct DrawingBoard<'a> {
    pending: &'a mut Curve,
    curves: &'a Vec<Curve>,

    copied_curve: &'a mut Option<Curve>,
    selected_curve: &'a mut Option<usize>,
    curve_to_select: &'a mut Option<usize>,

    pressed_point: &'a mut Option<Point>,
    nearest_point: &'a mut Option<(usize, String)>, //（curves下标，points下标

    ctrl_pressed: &'a mut bool,

    cache: &'a mut canvas::Cache,
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

        if !self.pending.shape.is_empty() {
            //创建新的曲线，这个时候很多事件响应都取消了
            match event {
                //记下按下的位置，如果按下和放开的位置距离过远，则不响应
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    *self.pressed_point = Some(cursor_position);
                }
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    if let Some(pressed) = self.pressed_point {
                        if pressed.distance(cursor_position) < DrawingBoard::DETERMINANT_DISTANCE {
                            self.pending
                                .shape
                                .update(ShapeMessage::Labor(cursor_position));
                            if self.pending.shape.is_complete() {
                                return (
                                    event::Status::Captured,
                                    Some(EditMessage::AddFromPending),
                                );
                            }
                        }
                    }
                    *self.pressed_point = None;
                }
                //按下esc放弃这次添加
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code,
                    modifiers,
                }) => {
                    if key_code == KeyCode::Escape && modifiers.is_empty() {
                        self.pending.shape.update(ShapeMessage::Reset);
                    }
                }
                _ => {}
            }
        } else {
            match event {
                Event::Mouse(mouse_event) => match mouse_event {
                    mouse::Event::CursorMoved { position: _ } => {
                        //查看是否有最近的点，意味着已经按下左键但未松开
                        if let Some((curves_index, points_index)) = self.nearest_point {
                            if let Some(point) = self.curves[*curves_index].shape.points().get(points_index) && point.distance(cursor_position) > DrawingBoard::DETERMINANT_DISTANCE {
                                if *self.ctrl_pressed {
                                    let x_shift = cursor_position.x - point.x;
                                    let y_shift = cursor_position.y - point.y;
                                    return (
                                        event::Status::Captured,
                                        Some(EditMessage::Curve(CurveMessage::Shape(
                                            ShapeMessage::Move(x_shift, y_shift),
                                        ))),
                                    );
                                } else {
                                    return (
                                        event::Status::Captured,
                                        Some(EditMessage::Curve(CurveMessage::Shape(
                                            ShapeMessage::MovePoint(
                                                points_index.clone(),
                                                cursor_position,
                                            ),
                                        ))),
                                    );
                                }
                            }
                        }

                        //如果离得远了就取消预览
                        if let Some(to_select) = self.curve_to_select {
                            let mut to_cancel = true;
                            for (_, point) in self.curves[*to_select].shape.points() {
                                if point.distance(cursor_position)
                                    < DrawingBoard::DETERMINANT_DISTANCE
                                {
                                    to_cancel = false;
                                }
                            }
                            if to_cancel {
                                *self.curve_to_select = None;
                            }
                        }

                        //如果没有预览中的，则选择一个距离最近的curve
                        if self.curve_to_select.is_none() {
                            *self.curve_to_select = self.decide_which_curve(cursor_position);
                        }
                    }
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        *self.pressed_point = Some(cursor_position);
                        let mut nearest_point = None;
                        let mut selected_curve = None;
                        let mut last_distance = DrawingBoard::DETERMINANT_DISTANCE;
                        for (curves_index, curve) in self.curves.iter().enumerate() {
                            for (points_index, point) in curve.shape.points() {
                                let distance = point.distance(cursor_position);
                                if distance < DrawingBoard::DETERMINANT_DISTANCE
                                    && distance < last_distance
                                {
                                    last_distance = distance;
                                    nearest_point = Some((curves_index, points_index));
                                    selected_curve = Some(curves_index);
                                }
                            }
                        }
                        *self.nearest_point = nearest_point;
                        *self.selected_curve = selected_curve;
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        if let Some(pressed) = self.pressed_point {
                            if pressed.distance(cursor_position)
                                < DrawingBoard::DETERMINANT_DISTANCE
                            {
                                *self.selected_curve = self.decide_which_curve(cursor_position);

                                if self.selected_curve.is_none() {
                                    self.pending
                                        .shape
                                        .update(ShapeMessage::Labor(cursor_position));
                                    if self.pending.shape.is_complete() {
                                        return (
                                            event::Status::Captured,
                                            Some(EditMessage::AddFromPending),
                                        );
                                    }
                                }
                            }
                        }
                        *self.pressed_point = None;
                        *self.nearest_point = None;
                    }
                    _ => {}
                },
                Event::Keyboard(ke) => match ke {
                    keyboard::Event::KeyPressed {
                        key_code,
                        modifiers,
                    } => {
                        if key_code == KeyCode::LControl || key_code == KeyCode::RControl {
                            *self.ctrl_pressed = true;
                        }

                        if key_code == KeyCode::C && modifiers.contains(Modifiers::CTRL) {
                            if let Some(selected) = self.selected_curve {
                                *self.copied_curve = Some(self.curves[*selected].clone());
                            }
                        }

                        if key_code == KeyCode::V && modifiers.contains(Modifiers::CTRL) {
                            if self.copied_curve.is_some() {
                                return (
                                    event::Status::Captured,
                                    Some(EditMessage::CurvePasted(cursor_position)),
                                );
                            }
                        }
                    }
                    keyboard::Event::KeyReleased {
                        key_code,
                        modifiers,
                    } => {
                        if key_code == KeyCode::LControl || key_code == KeyCode::RControl {
                            *self.ctrl_pressed = false;
                        }
                        if key_code == KeyCode::Delete && modifiers.is_empty() {
                            return (event::Status::Captured, Some(EditMessage::RemoveCurve));
                        }
                    }
                    _ => {}
                },
            }
        }

        self.cache.clear();

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

            if let Some(cursor_position) = cursor.position_in(&bounds) {
                self.pending.preview(frame, cursor_position);
            }
        });

        vec![content]
    }

    fn mouse_interaction(&self, bounds: IcedRectangle, cursor: Cursor) -> mouse::Interaction {
        if cursor.is_over(&bounds) {
            if self.selected_curve.is_some() && *self.ctrl_pressed {
                mouse::Interaction::Grabbing
            } else {
                mouse::Interaction::Crosshair
            }
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a> DrawingBoard<'a> {
    const DETERMINANT_DISTANCE: f32 = 10.0;

    fn decide_which_curve(&self, cursor_position: Point) -> Option<usize> {
        let mut res: Option<usize> = None;
        let mut last_distance = Self::DETERMINANT_DISTANCE;
        for (index, curve) in self.curves.iter().enumerate() {
            for (_, point) in curve.shape.points() {
                let distance = point.distance(cursor_position);
                if distance < Self::DETERMINANT_DISTANCE && distance < last_distance {
                    last_distance = distance;
                    res = Some(index);
                }
            }
        }
        res
    }
}
