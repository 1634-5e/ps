use iced::{
    button::State as ButtonState,
    canvas::event::{self, Event},
    canvas::{self, Canvas as IcedCanvas, Cursor, Frame, Geometry, Path, Stroke},
    mouse, Alignment, Button, Column, Command, Element, Length, Point, Rectangle, Text,
};

use crate::app::message::CanvasMessage as Message;
use crate::app::utils::get_size;

#[derive(Default)]
pub struct Canvas {
    rect: State,
    curves: Vec<Curve>,
    button_state: ButtonState,
}

impl Canvas {
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::AddCurve(curve) => {
                self.curves.push(curve);
                self.rect.request_redraw();
            }
            Message::Clear => {
                self.rect = State::default();
                self.curves.clear();
            }
        }
        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Alignment::Center)
            .push(self.rect.view(&self.curves).map(Message::AddCurve))
            .push(
                Button::new(&mut self.button_state, Text::new("Clear"))
                    .padding(8)
                    .on_press(Message::Clear),
            )
            .into()
    }
}

#[derive(Default)]
pub struct State {
    pending: Option<Pending>,
    cache: canvas::Cache,
}

impl State {
    pub fn view<'a>(&'a mut self, curves: &'a [Curve]) -> Element<'a, Curve> {
        IcedCanvas::new(Rect {
            state: self,
            curves,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear()
    }
}

struct Rect<'a> {
    state: &'a mut State,
    curves: &'a [Curve],
}

impl<'a> canvas::Program<Curve> for Rect<'a> {
    fn update(
        &mut self,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (event::Status, Option<Curve>) {
        let cursor_position = if let Some(position) = cursor.position_in(&bounds) {
            position
        } else {
            return (event::Status::Ignored, None);
        };

        match event {
            Event::Mouse(mouse_event) => {
                let message = match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => match self.state.pending {
                        None | Some(Pending::Two { from: _, to: _ }) => {
                            self.state.pending = Some(Pending::One {
                                from: cursor_position,
                            });

                            None
                        }
                        Some(Pending::One { from }) => {
                            self.state.pending = Some(Pending::Two {
                                from,
                                to: cursor_position,
                            });

                            Some(Curve {
                                from,
                                to: cursor_position,
                            })
                        }
                    },
                    _ => None,
                };

                (event::Status::Captured, message)
            }
            _ => (event::Status::Ignored, None),
        }
    }

    fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry> {
        let content = self.state.cache.draw(bounds.size(), |frame: &mut Frame| {
            Curve::draw_all(self.curves, frame);

            frame.stroke(
                &Path::rectangle(Point::ORIGIN, frame.size()),
                Stroke::default(),
            );
        });

        if let Some(pending) = &self.state.pending {
            let pending_curve = pending.draw(bounds, cursor);

            vec![content, pending_curve]
        } else {
            vec![content]
        }
    }

    fn mouse_interaction(&self, bounds: Rectangle, cursor: Cursor) -> mouse::Interaction {
        if cursor.is_over(&bounds) {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Curve {
    from: Point,
    to: Point,
}

impl Curve {
    fn draw_all(curves: &[Curve], frame: &mut Frame) {
        let curves = Path::new(|p| {
            for curve in curves {
                p.rectangle(curve.from, get_size(curve.from, curve.to));
            }
        });

        frame.stroke(&curves, Stroke::default().with_width(2.0));
    }
}

#[derive(Debug, Clone, Copy)]
enum Pending {
    One { from: Point },
    Two { from: Point, to: Point },
}

impl Pending {
    fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Geometry {
        let mut frame = Frame::new(bounds.size());

        if let Some(cursor_position) = cursor.position_in(&bounds) {
            match *self {
                Pending::One { from } => {
                    let line = Path::rectangle(from, get_size(from, cursor_position));
                    frame.stroke(&line, Stroke::default().with_width(2.0));
                    let curve = Curve {
                        from,
                        to: cursor_position,
                    };
                    Curve::draw_all(&[curve], &mut frame);
                }
                _ => {}
            };
        }

        frame.into_geometry()
    }
}
