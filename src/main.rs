// #![allow(unused)]
#![feature(derive_default_enum)]
#![feature(associated_type_bounds)]
#![feature(if_let_guard)]
#![windows_subsystem = "windows"]
#![warn(clippy::all)]

//暂时放下用户设置部分

pub fn main() -> iced::Result {
    //处理拖拽事件,第一个值是程序的路径（可能是相对路径，也可能是绝对路径），后面的应该全是被拖拽文件（夹）的路径
    let env_args: Vec<PathBuf> = env::args().map(|s| PathBuf::from(s)).collect();
    // let user_settings = Rc::new(RefCell::new(UserSettings {
    //     automatic_load: true,
    // })); //恢复用户设置，目前没做

    Ps::run(Settings {
        flags: Flags {
            env_args,
            // user_settings,
        },
        antialiasing: true,
        ..Settings::default()
    })
}

mod io {
    pub mod dialogs;
    pub mod last_place;

    pub use dialogs::{open, pick, save, PathBuf};
    pub use last_place::*;
}

mod ui {
    pub mod edit;
    mod icons;
    pub mod style;
    pub mod toolbar;
    mod utils;
    pub mod viewer;
    pub mod welcome;

    pub use edit::*;
    pub use toolbar::*;
    pub use viewer::*;
    pub use welcome::welcome;
}

use std::env;

use iced::keyboard::KeyCode;
use iced::{Application, Column, Length, Settings};
use iced::{Command, Element, Subscription};
use iced_native::mouse::Event as MouseEvent;
use iced_native::window::Event as WindowEvent;
use iced_native::Event;

use io::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct Flags {
    pub(crate) env_args: Vec<PathBuf>,
    // pub(crate) user_settings: Rc<RefCell<UserSettings>>,
}

use io::dialogs::open;
use io::last_place;
use ui::*;

#[derive(Debug, Default)]
pub struct State {
    viewer: Viewer,
    edit: Edit,
    toolbar: Toolbar,
    is_editing: bool,
}

#[derive(Debug)]
pub enum Message {
    Viewer(ViewerMessage),
    Edit(EditMessage),
    Toolbar(ToolbarMessage),
    StateRestored(Option<SavedState>),
    ExternEvent(Event),
}

#[derive(Debug)]
enum Ps {
    Loading,
    Loaded(Box<State>),
}

impl Application for Ps {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = Flags;

    fn new(flags: Flags) -> (Ps, Command<Message>) {
        let command = match &flags.env_args[..] {
            [_, to_open @ ..] if !to_open.is_empty() => {
                Command::perform(open(to_open.to_vec(), false), ViewerMessage::ImageLoaded)
                    .map(Message::Viewer)
            }
            _ => Command::perform(last_place::load(), Message::StateRestored),
        };
        (Ps::Loading, command)
    }

    fn title(&self) -> String {
        String::from("Ps")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            Ps::Loading => match message {
                Message::StateRestored(state) => match state {
                    Some(s) => {
                        let SavedState {
                            is_editing,
                            images,
                            on_view,
                        } = s;
                        *self = Ps::Loaded(Box::new(State {
                            viewer: Viewer {
                                images,
                                on_view,
                                ..Viewer::default()
                            },
                            is_editing,
                            ..State::default()
                        }));
                    }
                    None => {
                        println!("Program failed to restore last state.");
                        *self = Ps::Loaded(Box::new(State::default()))
                    }
                },
                Message::Viewer(ViewerMessage::ImageLoaded(data)) => {
                    *self = Ps::Loaded(Box::new(State::default()));
                    if let Ps::Loaded(state) = self {
                        state.viewer.update(ViewerMessage::ImageLoaded(data));
                    }
                }
                _ => {}
            },
            Ps::Loaded(state) => match message {
                Message::Toolbar(tm) => match tm {
                    //view
                    ToolbarMessage::Close => {
                        state.viewer.close();
                    }
                    ToolbarMessage::ClearImages => {
                        state.viewer.clear();
                    }
                    ToolbarMessage::New => {
                        state.is_editing = true;
                    }

                    //edit
                    ToolbarMessage::Back => {
                        state.is_editing = false;
                    }
                    ToolbarMessage::ClearCanvas => {
                        state.edit.reset();
                    }
                    ToolbarMessage::Save => state.edit.save(),
                    ToolbarMessage::SelectShape(s) => {
                        state.edit.change_shape(s);
                    }
                    ToolbarMessage::Open => match pick() {
                        Some(p) => {
                            return Command::perform(open(p, true), ViewerMessage::ImageLoaded)
                                .map(Message::Viewer)
                        }
                        None => {}
                    },
                },
                Message::ExternEvent(ee) => match ee {
                    Event::Window(we) => match we {
                        WindowEvent::FileDropped(fd) => {
                            state.is_editing = false;
                            return Command::perform(
                                open(vec![fd], false),
                                ViewerMessage::ImageLoaded,
                            )
                            .map(Message::Viewer);
                        }
                        _ => {}
                    },
                    Event::Keyboard(ke) => match ke {
                        iced::keyboard::Event::KeyPressed {
                            key_code,
                            modifiers,
                        } => {
                            if state.is_editing {
                                match key_code {
                                    KeyCode::Delete => {
                                        if modifiers.is_empty() {
                                            state.edit.remove_curve();
                                        }
                                    }
                                    _ => {}
                                }
                            } else {
                                match key_code {
                                    KeyCode::Delete => {
                                        if modifiers.is_empty() {
                                            state.viewer.close();
                                        }
                                    }
                                    KeyCode::Up | KeyCode::Left => {
                                        if modifiers.is_empty() {
                                            state.viewer.navigate(-1);
                                        } else if modifiers.control() {
                                            state.viewer.navigate(-10);
                                        }
                                    }
                                    KeyCode::Down | KeyCode::Right => {
                                        if modifiers.is_empty() {
                                            state.viewer.navigate(1);
                                        } else if modifiers.control() {
                                            state.viewer.navigate(10);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    },
                    Event::Mouse(me) => {
                        match me {
                            MouseEvent::WheelScrolled { delta } => {
                                println!("{:?}", delta);
                                println!("event occured");
                            }
                            MouseEvent::ButtonPressed(_) => {
                                println!("button pressed/");
                            }
                            _ => {}
                        }
                        println!("mouse event");
                    }
                    _ => {}
                },
                Message::Viewer(vm) => state.viewer.update(vm),
                Message::Edit(em) => state.edit.update(em),

                _ => {}
            },
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            Ps::Loading => welcome(),
            Ps::Loaded(state) => {
                let (main_content, toolbar) = if state.is_editing {
                    (
                        state.edit.view().map(Message::Edit),
                        state.toolbar.editing().map(Message::Toolbar),
                    )
                } else {
                    (
                        state.viewer.view().map(Message::Viewer),
                        state.toolbar.viewing().map(Message::Toolbar),
                    )
                };
                Column::new()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .push(toolbar)
                    .push(main_content)
                    .into()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::ExternEvent)
    }
}

//用于响应外部事件，并传递到本地事件
// async fn do_nothing() {}
