use iced::{
    canvas::event::{self, Event},
    canvas::{self, Canvas as IcedCanvas, Cursor, Frame, Geometry, Path, Stroke},
    keyboard::{self, KeyCode, Modifiers},
    mouse, pick_list, slider, text_input, Alignment, Column, Element, Length, PickList, Point,
    Rectangle as IcedRectangle, Row, Slider, Space, Text,
};

use svg::Document;

use super::shape::*;
use super::{curve::*, style};
use crate::io::dialogs::save as save_file;

#[derive(Debug, Clone)]
pub enum EditMessage {
    AddFromPending,
    Curve(CurveMessage),
    ChangeShape(ShapeEnum),
    CurveSelected(usize),
    CurvePasted(Point),
    Clear,
    RemoveCurve,
}

#[derive(Debug, Default)]
pub struct Edit {
    pending: Curve,
    pub curves: Vec<Curve>,

    pub dirty: bool,

    pub copied_curve: Option<Curve>,           //用于复制粘贴
    selected: (Option<usize>, Option<String>), //（curves下标，points下标
    curve_to_select: Option<usize>,

    pressed_point: Option<Point>,

    ctrl_pressed: bool,

    cache: canvas::Cache, //缓存

    pick_curve: pick_list::State<usize>,

    input_color_r: text_input::State,
    input_color_g: text_input::State,
    input_color_b: text_input::State,
    input_color_a: text_input::State,
    input_width: text_input::State,

    slider_color_r: slider::State,
    slider_color_g: slider::State,
    slider_color_b: slider::State,
    slider_color_a: slider::State,

    pick_line_cap: pick_list::State<EqLineCap>,
    pick_line_join: pick_list::State<EqLineJoin>,
}

impl Edit {
    pub fn new(curves: Vec<Curve>) -> Self {
        Edit {
            curves,
            ..Edit::default()
        }
    }

    pub fn update(&mut self, message: EditMessage) {
        match message {
            EditMessage::Curve(cm) => {
                if let (Some(curve), _) = self.selected {
                    if let Some(new_message) = self.curves[curve].update(cm) {
                        self.update(new_message);
                    }
                } else if let Some(new_message) = self.pending.update(cm) {
                    self.update(new_message);
                }
            }
            EditMessage::ChangeShape(s) => {
                if self.pending.shape.is_empty() {
                    self.pending.shape = s;
                }
            }
            EditMessage::Clear => {
                self.curves.clear();
                self.selected = (None, None);
            }
            EditMessage::RemoveCurve => {
                if let (Some(curve), _) = self.selected {
                    self.curves.remove(curve);
                    self.selected = (None, None);
                    self.curve_to_select = None;
                }
            }
            EditMessage::AddFromPending => {
                self.curves.push(self.pending.clone());
                self.pending.shape.update(ShapeMessage::Reset);
                self.selected = (Some(self.curves.len() - 1), None);
            }
            EditMessage::CurvePasted(point) => {
                if let Some(copied) = &self.copied_curve {
                    let mut new = copied.clone();
                    new.shape.update(ShapeMessage::Centered(point));
                    self.curves.push(new);
                }
            }
            EditMessage::CurveSelected(index) => {
                if index < self.curves.len() {
                    self.selected.0 = Some(index)
                }
            }
        }

        if self.pending.shape.is_complete() {
            self.update(EditMessage::AddFromPending);
        }

        //响应事件之后将“脏位”置为是，同时重绘画布
        self.dirty = true;
        self.redraw();
    }

    pub fn view(&mut self) -> Element<EditMessage> {
        let (
            points,
            attrs,
            Curve {
                color,
                width,
                line_cap,
                line_join,
                // segments,
                // offset,
                ..
            },
            edit_title,
        ) = if let (Some(index), _) = self.selected {
            (
                self.curves[index].shape.points(),
                self.curves[index].shape.attributes(),
                &self.curves[index],
                "Selected curve",
            )
        } else {
            (
                self.pending.shape.points(),
                self.pending.shape.attributes(),
                &self.pending,
                "Creating",
            )
        };

        let (r, g, b, a) = (
            (color.r * 255.0).to_string(),
            (color.g * 255.0).to_string(),
            (color.b * 255.0).to_string(),
            (color.a).to_string(),
        );
        let width = width.to_string();

        //排序points防止顺序一直变化
        let mut points = points.into_iter().collect::<Vec<(String, Point)>>();
        points.sort_by(|(a, _), (b, _)| a.cmp(b));
        let mut attrs = attrs.into_iter().collect::<Vec<(String, f32)>>();
        attrs.sort_by(|(a, _), (b, _)| a.cmp(b));

        let mut editable = Column::new()
            .width(Length::FillPortion(2))
            .align_items(Alignment::Start)
            .spacing(15)
            .push(Text::new(edit_title).height(Length::Units(40)).size(25));

        editable = editable.push(
            Row::new()
                .align_items(Alignment::Center)
                .spacing(10)
                .push(Text::new("Index:"))
                .push(
                    PickList::new(
                        &mut self.pick_curve,
                        (0..self.curves.len()).collect::<Vec<usize>>(),
                        self.selected.0,
                        CurveMessage::CurveSelected,
                    )
                    .style(style::PickList),
                ),
        );

        editable = points.into_iter().fold(editable, |acc, (index, p)| {
            acc.push(
                Row::new()
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(Text::new(format!("{:?}: ({:.2},{:.2})", index, p.x, p.y))),
            )
        });

        editable = attrs.into_iter().fold(editable, |acc, (index, attr)| {
            acc.push(
                Row::new()
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(Text::new(format!("{:?}: {:.2}", index, attr))),
            )
        });

        editable = editable
            // .push(
            //     Row::new()
            //         .align_items(Alignment::Center)
            //         .spacing(10)
            //         .push(Text::new(format!("Segments: {:?}", segments)))
            //         .push(Text::new(format!("offset: {:?}", offset))),
            // )
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
                        .style(style::TextInput::EditAttribute)
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
                        .style(style::TextInput::EditAttribute)
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
                        .style(style::TextInput::EditAttribute)
                        .width(Length::Units(50)),
                    ),
            )
            .push(
                Row::new()
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(
                        Slider::new(
                            &mut self.slider_color_a,
                            0.0..=1.0,
                            color.a,
                            CurveMessage::SlideColorA,
                        )
                        .step(0.01),
                    )
                    .push(
                        text_input::TextInput::new(
                            &mut self.input_color_a,
                            "a",
                            a.as_str(),
                            CurveMessage::InputColorA,
                        )
                        .style(style::TextInput::EditAttribute)
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
                        .style(style::TextInput::EditAttribute)
                        .width(Length::Units(50)),
                    ),
            )
            .push(
                Row::new()
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(Text::new("Line Cap:  "))
                    .push(
                        PickList::new(
                            &mut self.pick_line_cap,
                            vec![EqLineCap::Butt, EqLineCap::Round, EqLineCap::Square],
                            Some(*line_cap),
                            CurveMessage::LineCapSelected,
                        )
                        .style(style::PickList),
                    ),
            )
            .push(
                Row::new()
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(Text::new("Line Join:  "))
                    .push(
                        PickList::new(
                            &mut self.pick_line_join,
                            vec![EqLineJoin::Miter, EqLineJoin::Round, EqLineJoin::Bevel],
                            Some(*line_join),
                            CurveMessage::LineJoinSelected,
                        )
                        .style(style::PickList),
                    ),
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
                            selected: &mut self.selected,
                            curve_to_select: &mut self.curve_to_select,

                            pressed_point: &mut self.pressed_point,

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
    selected: &'a mut (Option<usize>, Option<String>), //（curves下标，points下标
    curve_to_select: &'a mut Option<usize>,

    pressed_point: &'a mut Option<Point>,

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
                        if self.pressed_point.is_some() {
                            if let (Some(_), Some(point)) = self.selected {
                                if *self.ctrl_pressed {
                                    return (
                                        event::Status::Captured,
                                        Some(EditMessage::Curve(CurveMessage::Shape(
                                            ShapeMessage::Move(point.clone(), cursor_position),
                                        ))),
                                    );
                                } else {
                                    return (
                                        event::Status::Captured,
                                        Some(EditMessage::Curve(CurveMessage::Shape(
                                            ShapeMessage::MovePoint(point.clone(), cursor_position),
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
                            *self.curve_to_select = self.decide_which_curve(cursor_position).0;
                        }
                    }
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        *self.pressed_point = Some(cursor_position);
                        *self.selected = self.decide_which_curve(cursor_position);
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        if let Some(pressed) = self.pressed_point {
                            if pressed.distance(cursor_position)
                                < DrawingBoard::DETERMINANT_DISTANCE
                            {
                                *self.selected = self.decide_which_curve(cursor_position);

                                if self.selected.0.is_none() {
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
                            if let (Some(selected), _) = self.selected {
                                *self.copied_curve = Some(self.curves[*selected].clone());
                            }
                        }

                        if key_code == KeyCode::V
                            && modifiers.contains(Modifiers::CTRL)
                            && self.copied_curve.is_some()
                        {
                            return (
                                event::Status::Captured,
                                Some(EditMessage::CurvePasted(cursor_position)),
                            );
                        }

                        if key_code == KeyCode::Escape
                            && modifiers.is_empty()
                            && (self.selected.0.is_some() || self.selected.1.is_some())
                        {
                            *self.selected = (None, None);
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

        //不返回事件的时候也需要重绘画布
        self.cache.clear();

        (event::Status::Ignored, None)
    }

    fn draw(&self, bounds: IcedRectangle, cursor: Cursor) -> Vec<Geometry> {
        let content = self.cache.draw(bounds.size(), |frame: &mut Frame| {
            let mut select = vec![];
            if let Some(ref selected) = self.selected.0 {
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
            if self.selected.0.is_some() && *self.ctrl_pressed {
                mouse::Interaction::Grabbing
            } else if self.curve_to_select.is_some() {
                mouse::Interaction::Pointer
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

    fn decide_which_curve(&self, cursor_position: Point) -> (Option<usize>, Option<String>) {
        let mut res = (None, None);
        let mut last_distance = DrawingBoard::DETERMINANT_DISTANCE;
        for (curves_index, curve) in self.curves.iter().enumerate() {
            for (points_index, point) in curve.shape.points() {
                let distance = point.distance(cursor_position);
                if distance < DrawingBoard::DETERMINANT_DISTANCE && distance < last_distance {
                    last_distance = distance;
                    res = (Some(curves_index), Some(points_index));
                }
            }
        }
        res
    }
}
