// #![allow(unused)]
#![feature(derive_default_enum)]
#![feature(associated_type_bounds)]

pub fn main() -> iced::Result {
    //处理拖拽事件,第一个值是程序的路径（可能是相对路径，也可能是绝对路径），后面的应该全是被拖拽文件（夹）的路径
    let env_args: Vec<PathBuf> = env::args().map(|s| PathBuf::from(s)).collect();
    let user_settings = Rc::new(RefCell::new(UserSettings {
        automatic_load: true,
    })); //恢复用户设置，目前没做

    Ps::run(Settings {
        flags: Flags {
            env_args,
            user_settings,
        },
        antialiasing: true,
        ..Settings::default()
    })
}

mod io {
    pub mod dialogs;
    pub mod last_place;
}

mod ui {
    pub mod edit;
    pub mod style;
    mod utils;
    pub mod viewer;

    pub use edit::*;
    pub use viewer::*;
}

#[derive(Debug, Clone, Default)]
pub struct Flags {
    pub(crate) env_args: Vec<PathBuf>,
    pub(crate) user_settings: Rc<RefCell<UserSettings>>,
}

//TODO: 这里应该使用Rc<RefCell>
#[derive(Debug, Clone, Default)]
pub struct UserSettings {
    pub(crate) automatic_load: bool, //这一项继续细分可以包括：按钮打开自动、拖拽到图标自动、拖拽到应用自动、以及全关
}

use std::{cell::RefCell, env, path::PathBuf, rc::Rc};

use iced::{Application, Settings};
use iced::{Command, Element, Subscription};
use iced_native::window::Event as WindowEvent;
use iced_native::Event;

use io::last_place;
use ui::*;

#[derive(Debug)]
pub struct State {
    viewer: Viewer,
    edit: Edit,
    images: Vec<PathBuf>,
    on_view: usize,
}

#[derive(Debug)]
enum Message {
    ImageBox(ViewerMessage),
    Edit(EditMessage),
    SessionRestored(Option<State>),
    ImageLoaded((Vec<PathBuf>, usize)),
    ExternEvent(Event),
}

#[derive(Debug)]
enum Ps {
    Loading,
    Loaded(Box<State>), //TODO:是否用Box还需要比较
}

impl Application for Ps {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = Flags;

    fn new(mut flags: Flags) -> (Ps, Command<Message>) {
        (
            Ps::Loading,
            Command::perform(last_place::load(), Message::SessionRestored),
        )
    }

    fn title(&self) -> String {
        todo!()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        todo!()
    }

    fn view(&mut self) -> Element<Message> {
        todo!()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::ExternEvent)
    }
}

//Viewer相应留下来的update
// fn update(
//     &mut self,
//     message: ViewerMessage,
//     settings: Rc<RefCell<UserSettings>>,
// ) -> Command<ViewerMessage> {
//     match message {
//         ViewerMessage::ImageLoaded(res) => match res {
//             Ok((mut images, current)) => {
//                 self.current = current + self.images.len();
//                 self.images.append(&mut images);
//                 self.status = Status::View;
//                 if self.images.len() > 1 {
//                     self.buttons.previous.disabled = false;
//                     self.buttons.next.disabled = false;
//                 }
//             }
//             Err(e) => {
//                 self.status = Status::Errored(e.into());
//             }
//         },
//         ViewerMessage::Navigate(n) => {
//             self.navigate(n);
//         }
//         ViewerMessage::PickImage(dp) => match pick_in_dialog(dp) {
//             Some(path) => {
//                 self.status = Status::Loading;
//                 match settings.try_borrow() {
//                     Ok(settings) => {
//                         return Command::perform(
//                             open(vec![path], settings.automatic_load),
//                             ViewerMessage::ImageLoaded,
//                         );
//                     }
//                     Err(e) => self.status = Status::Errored(e.into()),
//                 }
//             }
//             None => {}
//         },
//         ViewerMessage::Close => self.close_this(),
//         ViewerMessage::Clear => self.close_all(),
//         _ => {}
//     }
//     Command::none()
// }
