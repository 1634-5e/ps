// #![windows_subsystem = "windows"]
// #![allow(unused)]
#![feature(associated_type_bounds)]
#![feature(if_let_guard)]
#![feature(let_chains)]
#[allow(clippy::collapsible_match)]
#[allow(clippy::single_match)]

//暂时放下用户设置部分

pub fn main() -> iced::Result {
    //处理拖拽事件,第一个值是程序的路径（可能是相对路径，也可能是绝对路径），后面的应该全是被拖拽文件（夹）的路径
    let env_args: Vec<PathBuf> = env::args().map(PathBuf::from).collect();
    // let user_settings = Rc::new(RefCell::new(UserSettings {
    //     automatic_load: true,
    // })); //恢复用户设置，目前没做

    Ps::run(Settings {
        flags: Flags {
            env_args,
            // user_settings,
        },
        antialiasing: true,
        window: window::Settings {
            position: window::Position::Specific(200, 20),
            ..window::Settings::default()
        },
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
    pub mod curve;
    pub mod edit;
    mod icons;
    pub mod shape;
    pub mod style;
    pub mod toolbar;
    pub mod utils;
    pub mod viewer;
    pub mod welcome;

    pub use edit::*;
    pub use toolbar::*;
    pub use viewer::*;
    pub use welcome::*;
}

use std::env;

use app_dirs2::{get_app_dir, AppDataType, AppInfo};
use iced::keyboard::KeyCode;
use iced::mouse::ScrollDelta;
use iced::pure::widget::{Column, Container};
use iced::time::every;
// use iced::time::every;
use iced::pure::{Application, Element};
use iced::{window, Length, Settings};
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
    edit: Edit,
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
enum Ps {
    Loading,
    Loaded(State),
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
                        *self = Ps::Loaded(State {
                            viewer: Viewer {
                                images,
                                on_view,
                                ..Viewer::default()
                            },
                            edit: Edit::new(curves),
                            is_editing,
                            ..State::default()
                        });
                    } else {
                        //FIXME:
                        //如果文件存在，但是解析失败，则很难将文件修改到正确的格式（意味着之前的数据丢失）
                        //这一现象的原因是改变了数据结构，因此后面可能需要考虑版本之间的兼容性
                        *self = Ps::Loaded(State::default());
                    }
                }
                Message::Viewer(ViewerMessage::ImageLoaded(data)) => {
                    *self = Ps::Loaded(State::default());
                    if let Ps::Loaded(state) = self {
                        state.viewer.update(ViewerMessage::ImageLoaded(data));
                    }
                }
                _ => {}
            },
            Ps::Loaded(state) => match message {
                //将事件传递到下一级进行处理
                Message::Viewer(vm) => state.viewer.update(vm),
                Message::Edit(em) => state.edit.update(em),
                //工具栏的事件要在这里处理
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
                    ToolbarMessage::Open => {
                        if let Some(p) = pick() {
                            return Command::perform(open(p, true), ViewerMessage::ImageLoaded)
                                .map(Message::Viewer);
                        }
                    }
                },
                //外部设备事件的处理
                Message::ExternEvent(ee) => match ee {
                    //窗口事件
                    Event::Window(we) => {
                        //处理拖拽图片
                        if let WindowEvent::FileDropped(fd) = we {
                            state.is_editing = false;
                            return Command::perform(
                                open(vec![fd], false),
                                ViewerMessage::ImageLoaded,
                            )
                            .map(Message::Viewer);
                        }
                    }
                    //程序主动响应鼠标和键盘分在两处，包括这里和画布，后面应该把这里的分散到各个结构之内
                    Event::Keyboard(ke) => {
                        if let iced::keyboard::Event::KeyPressed {
                            key_code,
                            modifiers,
                        } = ke
                        {
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
                    }
                    //鼠标事件
                    Event::Mouse(me) => {
                        if let MouseEvent::WheelScrolled {
                            delta: ScrollDelta::Lines { x: _, y },
                        } = me
                        {
                            state.viewer.navigate(-y as i32);
                        }
                    }
                    _ => {}
                },
                //自动保存
                Message::AutoSave => {
                    state.is_saving = true;
                    if let Ok(path) = get_app_dir(AppDataType::UserCache, &APP_INFO, "/") {
                        if let Some(parent) = path.parent() {
                            let saved_state = SavedState {
                                is_editing: state.is_editing,
                                images: state.viewer.images.clone(),
                                on_view: state.viewer.on_view,
                                curves: state.edit.curves.clone(),
                            };
                            return Command::perform(
                                save_state(saved_state, parent.to_path_buf()),
                                Message::SavedOrFailed,
                            );
                        }
                    }
                }
                //响应自动保存的返回
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
}
