//这部分只是为了测试

#![windows_subsystem = "windows"]
// #![allow(unused)]
#![feature(associated_type_bounds)]
#![feature(if_let_guard)]
#![feature(let_chains)]
#![feature(arc_unwrap_or_clone)]
#[allow(clippy::collapsible_match)]
#[allow(clippy::single_match)]

pub mod io {
    pub mod dialogs;
    pub mod last_place;

    pub use dialogs::{open, pick, save, PathBuf};
    pub use last_place::*;
}

pub mod ui {
    pub mod curve;
    pub mod edit;
    mod icons;
    pub mod shape;
    pub mod style;
    pub mod toolbar;
    pub mod utils;
    pub mod viewer;
    pub mod welcome;

    pub use curve::*;
    pub use edit::*;
    pub use shape::*;
    pub use toolbar::*;
    pub use viewer::*;
    pub use welcome::*;
}

use std::cell::RefCell;
use std::rc::Rc;

use app_dirs2::{get_app_dir, AppDataType, AppInfo};
use iced::keyboard::KeyCode;
use iced::mouse::ScrollDelta;
use iced::pure::widget::{Column, Container};
use iced::time::every;
// use iced::time::every;
use iced::pure::{Application, Element};
use iced::Length;
use iced::{Command, Subscription};
use iced_native::mouse::Event as MouseEvent;
use iced_native::window::Event as WindowEvent;
use iced_native::Event;

use io::*;

//用于决定Path,这里的生成的目录是/author/name/，但是只需要一级目录，所以稍微改了一点
const APP_INFO: AppInfo = AppInfo {
    name: "never use",
    author: "Ps",
};

#[derive(Debug, Clone, Default)]
pub struct Flags {
    pub env_args: Vec<PathBuf>,
    // pub(crate) user_settings: Rc<RefCell<UserSettings>>,
}

use io::dialogs::open;
use ui::*;

#[derive(Debug, Default)]
pub struct State {
    viewer: Viewer,
    pub edit: Edit,
    toolbar: Toolbar,
    is_editing: bool,
    is_saving: bool,
}

#[derive(Debug)]
pub enum Message {
    Viewer(ViewerMessage),
    Edit(EditMessage),
    Toolbar(ToolbarMessage),
    StateRestored(std::io::Result<Option<SavedState>>),
    ExternEvent(Event),
    SavedOrFailed(std::io::Result<()>),
    AutoSave,
}

#[derive(Debug)]
pub enum Ps {
    Loading,
    Loaded(Box<State>),
}

// assert_eq!(
//     PathBuf::from("C:\\Users\\86362\\AppData\\Local\\Ps\\"),
//     parent.to_path_buf()
// );
// (Ps::Loaded(Box::new(State::default())), Command::none())

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
            _ => {
                if let Ok(path) = get_app_dir(AppDataType::UserCache, &APP_INFO, "/") {
                    if let Some(parent) = path.parent() {
                        Command::perform(
                            last_place::load_state(parent.to_path_buf()),
                            Message::StateRestored,
                        )
                    } else {
                        Command::none()
                    }
                } else {
                    Command::none()
                }
            }
        };
        (Ps::Loading, command)
    }

    fn title(&self) -> String {
        String::from("Ps")
    }

    fn view(&self) -> Element<Message> {
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
                Container::new(
                    Column::new()
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .push(toolbar)
                        .push(main_content),
                )
                .style(style::Container)
                .into()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        match self {
            Ps::Loading => Subscription::none(),
            Ps::Loaded(state) => {
                //trait object即使能序列化在程序关闭之后也没法反序列化，因此放弃last_place
                let auto_save = if state.edit.dirty && !state.is_saving {
                    every(std::time::Duration::from_secs(2)).map(|_| Message::AutoSave)
                } else {
                    Subscription::none()
                };

                Subscription::batch(vec![
                    iced_native::subscription::events().map(Message::ExternEvent),
                    auto_save,
                ])
            }
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            Ps::Loading => match message {
                Message::StateRestored(state) => {
                    if let Ok(Some(state)) = state {
                        let SavedState {
                            is_editing,
                            images,
                            on_view,
                            curves,
                        } = state;
                        *self = Ps::Loaded(Box::new(State {
                            viewer: Viewer {
                                images,
                                on_view,
                                ..Viewer::default()
                            },
                            edit: Edit::new(
                                curves
                                    .into_iter()
                                    .map(|curve| Rc::new(RefCell::new(curve)))
                                    .collect(),
                            ),
                            is_editing,
                            ..State::default()
                        }));
                    } else {
                        *self = Ps::Loaded(Box::new(State::default()));
                    }
                }
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
                    ToolbarMessage::Export => state.edit.export(),
                    ToolbarMessage::Edit(em) => state.edit.update(em),
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
                            if !state.is_editing {
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
                    Event::Mouse(me) => match me {
                        MouseEvent::WheelScrolled { delta } => {
                            if let ScrollDelta::Lines { x: _, y } = delta {
                                state.viewer.navigate(-y as i32);
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                },
                Message::Viewer(vm) => state.viewer.update(vm),
                Message::Edit(em) => state.edit.update(em),

                Message::AutoSave => {
                    state.is_saving = true;
                    if let Ok(path) = get_app_dir(AppDataType::UserCache, &APP_INFO, "/") {
                        if let Some(parent) = path.parent() {
                            let saved_state = SavedState {
                                is_editing: state.is_editing,
                                images: state.viewer.images.clone(),
                                on_view: state.viewer.on_view,
                                curves: state
                                    .edit
                                    .curves
                                    .clone()
                                    .into_iter()
                                    .map(|mut rc| Rc::make_mut(&mut rc).to_owned().into_inner())
                                    .collect(),
                            };
                            return Command::perform(
                                save_state(saved_state, parent.to_path_buf()),
                                Message::SavedOrFailed,
                            );
                        }
                    }
                }
                Message::SavedOrFailed(result) => {
                    state.is_saving = false;
                    if result.is_ok() {
                        state.edit.dirty = false;
                    }
                }
                _ => {}
            },
        }
        Command::none()
    }

    // 让程序在启动之后立即退出
    fn should_exit(&self) -> bool {
        if let Ps::Loaded(_) = self {
            true
        } else {
            false
        }
    }
}

//用于响应外部事件，并传递到本地事件
// async fn do_nothing() {}
