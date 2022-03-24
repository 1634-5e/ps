use std::{cell::RefCell, rc::Rc};

use super::{canvas::ShapeKind, Component};

use crate::{
    app::{message::ToolBarMessage, Flags, UserSettings},
    common::{button::toolbar, custom_element::row_with_blanks},
};
use iced::{button::State, Alignment, Button, Column, Command, Element, Length, Row};

//这里面按钮绑定的事件比较宽泛，所以内联的message是主页的
//TODO:像close这种按钮需要有禁用的情况，目前貌似不自带，得自己手动实现。。
#[derive(Debug, Clone)]
pub struct ToolBar {
    close_this: ToolBarButton,
    close_all: ToolBarButton,
    new: ToolBarButton,
    pub settings: ToolBarButton,
    pub shapes: Shapes,
}

impl Component for ToolBar {
    type Message = ToolBarMessage;

    fn new(_flags: &mut Flags) -> (ToolBar, Command<Self::Message>) {
        let toolbar = ToolBar {
            close_this: ToolBarButton::new(false),
            close_all: ToolBarButton::new(false),
            new: ToolBarButton::new(false),
            settings: ToolBarButton::new(false),
            shapes: Shapes::new(),
        };
        (toolbar, Command::none())
    }

    fn view(&mut self, _settings: Rc<RefCell<UserSettings>>) -> Element<Self::Message> {
        let settings_button = self.settings.view("settings", ToolBarMessage::GoToSettings);
        let close_this = self
            .close_this
            .view("close this", ToolBarMessage::CloseThis);
        let close_all = self.close_all.view("close all", ToolBarMessage::CloseAll);
        let new = self.new.view("new", ToolBarMessage::New);
        let shapes = self.shapes.view();

        Row::new()
            .height(Length::FillPortion(1))
            .push(Column::new().push(close_this).push(close_all))
            .push(new)
            .push(shapes)
            .push(
                row_with_blanks(
                    Row::new()
                        .align_items(Alignment::Center)
                        .push(settings_button),
                    1,
                    0,
                )
                .width(Length::FillPortion(2)),
            )
            .into()
    }

    fn update(
        &mut self,
        _message: Self::Message,
        _settings: Rc<RefCell<UserSettings>>,
    ) -> Command<Self::Message> {
        Command::none()
    }
}

impl ToolBar {
    pub fn pick_shape(&mut self, shape: ShapeKind) {
        let index = match shape {
            ShapeKind::Rectangle => 0,
        };
        assert!(index + 1 < self.shapes.choices.len());

        self.shapes.choices[self.shapes.picked].enable();
        self.shapes.choices[index].disable();
        self.shapes.picked = index;
    }
}

#[derive(Debug, Clone)]
pub struct ToolBarButton {
    state: State,
    disabled: bool,
}

impl ToolBarButton {
    pub fn new(disabled: bool) -> Self {
        ToolBarButton {
            state: State::new(),
            disabled,
        }
    }

    pub fn view<'a>(
        &'a mut self,
        text: &str,
        message: ToolBarMessage,
    ) -> Button<'a, ToolBarMessage> {
        let button = toolbar(&mut self.state, text);
        if !self.disabled {
            button.on_press(message)
        } else {
            button
        }
    }

    pub fn disable(&mut self) {
        self.disabled = true;
    }

    pub fn enable(&mut self) {
        self.disabled = false;
    }
}

#[derive(Debug, Clone)]
pub struct Shapes {
    choices: Vec<ToolBarButton>,
    picked: usize,
}

impl Shapes {
    pub fn new() -> Self {
        Shapes {
            choices: vec![ToolBarButton::new(true)],
            picked: 0,
        }
    }

    pub fn view<'a>(&'a mut self) -> Row<'a, ToolBarMessage> {
        let shapes = Row::new();

        shapes.push(self.choices[0].view(
            "rectangle",
            ToolBarMessage::ShapeChanged(ShapeKind::Rectangle),
        ))
    }
}
