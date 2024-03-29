use std::cell::RefCell;
use std::rc::Rc;

use iced::keyboard::{KeyCode, Modifiers};
use iced::pure::widget::{
    canvas::Canvas as IcedCanvas,
    canvas::{event, Cache, Cursor, Event, Frame, Geometry, Path, Program, Stroke},
    text_input, Column, PickList, Row, Slider, Space, Text,
};
use iced::pure::Element;
use iced::{keyboard, mouse, Alignment, Length, Point, Rectangle as IcedRectangle};

use svg::Document;

use super::{
    curve::*,
    shape::{Shape, ShapeEnum, ShapeMessage},
    style,
};
use crate::io::dialogs::save as save_file;

#[derive(Debug, Clone)]
pub enum EditMessage {
    AddWithClick(Point),
    AddFromPending,
    Curve(CurveMessage),
    ChangeShape(ShapeEnum),
    CurveCopied,
    CurvePasted(Point),
    Clear,
    RemoveCurve,
}

#[derive(Debug, Default)]
pub struct Edit {
    pub curves: Vec<Rc<RefCell<Curve>>>,
    pending: Rc<RefCell<Curve>>,
    pub dirty: bool,

    pub copied_curve: Option<Rc<RefCell<Curve>>>,
    selected: Rc<RefCell<(Option<usize>, Option<String>)>>,
    cache: Rc<RefCell<Cache>>,
}

impl Edit {
    pub fn new(curves: Vec<Rc<RefCell<Curve>>>) -> Self {
        Edit {
            curves,
            ..Edit::default()
        }
    }

    pub fn update(&mut self, message: EditMessage) {
        match message {
            EditMessage::Curve(cm) => {
                if let CurveMessage::CurveSelected(index) = cm {
                    self.selected.borrow_mut().0 = Some(index);
                } else {
                    if let Some(index) = self.selected.borrow().0 {
                        self.curves[index].borrow_mut().update(cm);
                    } else {
                        self.pending.borrow_mut().update(cm)
                    }
                }
            }
            EditMessage::AddWithClick(cursor_position) => {
                *self.selected.borrow_mut() = (None, None);
                self.pending
                    .borrow_mut()
                    .shape
                    .update(ShapeMessage::Labor(cursor_position));
            }
            EditMessage::ChangeShape(s) => {
                self.pending.borrow_mut().shape = s;
            }
            EditMessage::Clear => {
                self.curves.clear();
                *self.selected.borrow_mut() = (None, None);
            }
            EditMessage::RemoveCurve => {
                if let (Some(index), _) = self.selected.replace((None, None)) {
                    self.curves.remove(index);
                }
            }
            EditMessage::AddFromPending => {
                self.curves
                    .push(Rc::new(Rc::make_mut(&mut self.pending).to_owned()));
                self.pending.borrow_mut().shape.update(ShapeMessage::Reset);
                *self.selected.borrow_mut() = (Some(self.curves.len() - 1), None);
            }
            EditMessage::CurveCopied => {
                if let Some(index) = self.selected.borrow().0 {
                    self.copied_curve = Some(self.curves[index].clone());
                }
            }
            EditMessage::CurvePasted(point) => {
                if let Some(copied) = self.copied_curve.clone() {
                    let new = Rc::unwrap_or_clone(copied);
                    new.borrow_mut().shape.update(ShapeMessage::Centered(point));
                    self.curves.push(Rc::new(new));
                }
            }
        }

        if self.pending.borrow_mut().shape.is_complete() {
            self.update(EditMessage::AddFromPending);
        }

        //响应事件之后将“脏位”置为是，同时重绘画布
        self.dirty = true;
        self.redraw();
    }

    pub fn view(&self) -> Element<EditMessage> {
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
                        IcedCanvas::new(Pad {
                            pending: &self.pending,
                            curves: &self.curves,
                            selected: self.selected.clone(),
                            cache: self.cache.clone(),
                        })
                        .width(Length::Fill)
                        .height(Length::Fill),
                    )
                    .push(Space::with_height(Length::Units(10))),
            )
            .push(Space::with_width(Length::Units(10)));

        let editable = if let Some(index) = self.selected.borrow().0 {
            Editable {
                curves_len: self.curves.len(),
                curve: &self.curves[index],
                label: CurveLabel::Selected(index),
            }
        } else {
            Editable {
                curves_len: self.curves.len(),
                curve: &self.pending,
                label: CurveLabel::Pending,
            }
        };

        Row::new()
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(canvas)
            .push(editable.view().map(EditMessage::Curve))
            .into()
    }

    fn redraw(&mut self) {
        self.cache.borrow_mut().clear();
    }

    pub fn export(&self) {
        if let Some(pathbuf) = save_file() {
            let document = self.curves.iter().fold(Document::new(), |acc, x| {
                if let Some(path) = x.borrow().save() {
                    acc.add(path)
                } else {
                    acc
                }
            });

            svg::save(pathbuf, &document).unwrap();
        }
    }
}

#[derive(Debug, Default)]
pub struct Interaction {
    curve_to_select: Option<Rc<RefCell<Curve>>>, //这里的原因是Edit可能会改变向量长度，而这里是独立更新的，因此可能会越界
    pressed_point: Option<Point>,
    ctrl_pressed: bool,
}

#[derive(Debug)]
struct Pad<'a> {
    pending: &'a Rc<RefCell<Curve>>,
    curves: &'a Vec<Rc<RefCell<Curve>>>,

    selected: Rc<RefCell<(Option<usize>, Option<String>)>>,
    cache: Rc<RefCell<Cache>>,
}

impl<'a> Program<EditMessage> for Pad<'a> {
    type State = Interaction;
    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: IcedRectangle<f32>,
        cursor: Cursor,
    ) -> (event::Status, Option<EditMessage>) {
        let cursor_position = if let Some(position) = cursor.position_in(&bounds) {
            position
        } else {
            return (event::Status::Ignored, None);
        };

        if !self.pending.borrow().shape.is_empty() {
            //创建新的曲线，这个时候很多事件响应都取消了
            match event {
                //记下按下的位置，如果按下和放开的位置距离过远，则不响应
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    state.pressed_point = Some(cursor_position);
                }
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    if let Some(pressed) = state.pressed_point {
                        if pressed.distance(cursor_position) < Pad::DETERMINANT_DISTANCE {
                            return (
                                event::Status::Captured,
                                Some(EditMessage::AddWithClick(cursor_position)),
                            );
                        }
                    }
                    state.pressed_point = None;
                }
                //按下esc放弃这次添加
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code,
                    modifiers,
                }) => {
                    if key_code == KeyCode::Escape && modifiers.is_empty() {
                        return (
                            event::Status::Captured,
                            Some(EditMessage::Curve(CurveMessage::Shape(ShapeMessage::Reset))),
                        );
                    }
                }
                _ => {}
            }
        } else {
            match event {
                Event::Mouse(mouse_event) => match mouse_event {
                    mouse::Event::CursorMoved { position: _ } => {
                        //查看是否有最近的点，意味着已经按下左键但未松开
                        if state.pressed_point.is_some() {
                            if let Some(point) = self.selected.borrow().1.as_ref() {
                                if state.ctrl_pressed {
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
                        if let Some(to_select) = state.curve_to_select.clone() {
                            let mut to_cancel = true;
                            for (_, point) in to_select.borrow().shape.points() {
                                if point.distance(cursor_position) < Pad::DETERMINANT_DISTANCE {
                                    to_cancel = false;
                                }
                            }
                            if to_cancel {
                                state.curve_to_select = None;
                            }
                        }

                        //如果没有预览中的，则选择一个距离最近的curve
                        if state.curve_to_select.is_none() {
                            if let Some(index) = self.decide_which_curve(cursor_position).0 {
                                state.curve_to_select = Some(self.curves[index].clone());
                            }
                        }
                    }
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        state.pressed_point = Some(cursor_position);
                        *self.selected.borrow_mut() = self.decide_which_curve(cursor_position);
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        if let Some(pressed) = state.pressed_point {
                            if pressed.distance(cursor_position) < Pad::DETERMINANT_DISTANCE {
                                *self.selected.borrow_mut() =
                                    self.decide_which_curve(cursor_position);

                                if self.selected.borrow().0.is_none() {
                                    return (
                                        event::Status::Captured,
                                        Some(EditMessage::AddWithClick(cursor_position)),
                                    );
                                }
                            }
                        }
                        state.pressed_point = None;
                    }
                    _ => {}
                },
                Event::Keyboard(ke) => match ke {
                    keyboard::Event::KeyPressed {
                        key_code,
                        modifiers,
                    } => {
                        if key_code == KeyCode::LControl || key_code == KeyCode::RControl {
                            state.ctrl_pressed = true;
                        }

                        if key_code == KeyCode::C && modifiers.contains(Modifiers::CTRL) {
                            return (event::Status::Captured, Some(EditMessage::CurveCopied));
                        }

                        if key_code == KeyCode::V && modifiers.contains(Modifiers::CTRL) {
                            return (
                                event::Status::Captured,
                                Some(EditMessage::CurvePasted(cursor_position)),
                            );
                        }

                        if key_code == KeyCode::Escape
                            && modifiers.is_empty()
                            && (self.selected.borrow().0.is_some()
                                || self.selected.borrow().1.is_some())
                        {
                            *self.selected.borrow_mut() = (None, None);
                        }
                    }
                    keyboard::Event::KeyReleased {
                        key_code,
                        modifiers,
                    } => {
                        if key_code == KeyCode::LControl || key_code == KeyCode::RControl {
                            state.ctrl_pressed = false;
                        }
                        if key_code == KeyCode::Delete && modifiers.is_empty() {
                            return (event::Status::Captured, Some(EditMessage::RemoveCurve));
                        }
                    }
                    _ => {}
                },
            }
        }

        self.redraw();
        (event::Status::Ignored, None)
    }

    fn draw(
        &self,
        state: &Self::State,
        bounds: IcedRectangle<f32>,
        cursor: Cursor,
    ) -> Vec<Geometry> {
        let content = self
            .cache
            .borrow()
            .draw(bounds.size(), |frame: &mut Frame| {
                let selected = self.selected.borrow().0.clone();
                self.curves.iter().enumerate().for_each(|(index, curve)| {
                    curve.borrow().draw(frame, Some(index) == selected);
                });

                //现在curve to select是交互内的逻辑，因此绘制也放在这里
                if let Some(curve_to_select) = &state.curve_to_select {
                    curve_to_select.borrow().draw(frame, true);
                }

                frame.stroke(
                    &Path::rectangle(Point::ORIGIN, frame.size()),
                    Stroke::default(),
                );

                if let Some(cursor_position) = cursor.position_in(&bounds) {
                    self.pending.borrow().preview(frame, cursor_position);
                }
            });

        vec![content]
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: IcedRectangle<f32>,
        cursor: Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(&bounds) {
            if self.selected.borrow().0.is_some() && state.ctrl_pressed {
                mouse::Interaction::Grabbing
            } else if state.curve_to_select.is_some() {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::Crosshair
            }
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a> Pad<'a> {
    const DETERMINANT_DISTANCE: f32 = 10.0;

    fn decide_which_curve(&self, cursor_position: Point) -> (Option<usize>, Option<String>) {
        let mut res = (None, None);
        let mut last_distance = Pad::DETERMINANT_DISTANCE;
        for (curves_index, curve) in self.curves.iter().enumerate() {
            for (points_index, point) in curve.borrow().shape.points() {
                let distance = point.distance(cursor_position);
                if distance < Pad::DETERMINANT_DISTANCE && distance < last_distance {
                    last_distance = distance;
                    res = (Some(curves_index), Some(points_index));
                }
            }
        }
        res
    }

    fn redraw(&self) {
        self.cache.borrow_mut().clear();
    }
}

#[derive(Debug)]
enum CurveLabel {
    Pending,
    Selected(usize),
}

impl CurveLabel {
    fn text(&self) -> &str {
        match self {
            CurveLabel::Pending => "Creating",
            CurveLabel::Selected(_) => "Selected curve",
        }
    }

    fn index(&self) -> Option<usize> {
        match self {
            CurveLabel::Pending => None,
            CurveLabel::Selected(index) => Some(*index),
        }
    }
}

#[derive(Debug)]
struct Editable<'a> {
    curves_len: usize,
    curve: &'a Rc<RefCell<Curve>>,
    label: CurveLabel,
}

impl<'a> Editable<'a> {
    fn view(self) -> Element<'a, CurveMessage> {
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
        ) = (
            self.curve.borrow().shape.points(),
            self.curve.borrow().shape.attributes(),
            *self.curve.borrow(),
        );

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
            .push(
                Text::new(self.label.text())
                    .height(Length::Units(40))
                    .size(25),
            );

        editable = editable.push(
            Row::new()
                .align_items(Alignment::Center)
                .spacing(10)
                .push(Text::new("Index:"))
                .push(
                    PickList::new(
                        (0..self.curves_len).collect::<Vec<usize>>(),
                        self.label.index(),
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

        editable
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
                    .push(Slider::new(0.0..=1.0, color.r, CurveMessage::SlideColorR).step(0.01))
                    .push(
                        text_input::TextInput::new("red", r.as_str(), CurveMessage::InputColorR)
                            .style(style::TextInput::EditAttribute)
                            .width(Length::Units(50)),
                    ),
            )
            .push(
                Row::new()
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(Slider::new(0.0..=1.0, color.g, CurveMessage::SlideColorG).step(0.01))
                    .push(
                        text_input::TextInput::new("green", g.as_str(), CurveMessage::InputColorG)
                            .style(style::TextInput::EditAttribute)
                            .width(Length::Units(50)),
                    ),
            )
            .push(
                Row::new()
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(Slider::new(0.0..=1.0, color.b, CurveMessage::SlideColorB).step(0.01))
                    .push(
                        text_input::TextInput::new("blue", b.as_str(), CurveMessage::InputColorB)
                            .style(style::TextInput::EditAttribute)
                            .width(Length::Units(50)),
                    ),
            )
            .push(
                Row::new()
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(Slider::new(0.0..=1.0, color.a, CurveMessage::SlideColorA).step(0.01))
                    .push(
                        text_input::TextInput::new("a", a.as_str(), CurveMessage::InputColorA)
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
                            vec![EqLineCap::Butt, EqLineCap::Round, EqLineCap::Square],
                            Some(line_cap),
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
                            vec![EqLineJoin::Miter, EqLineJoin::Round, EqLineJoin::Bevel],
                            Some(line_join),
                            CurveMessage::LineJoinSelected,
                        )
                        .style(style::PickList),
                    ),
            )
            .into()
    }
}
