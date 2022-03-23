#![allow(unused)]

use app::{Flags, Ps, UserSettings};

use iced::{Application, Settings};
use std::{cell::RefCell, env, path::PathBuf, rc::Rc};

mod app;
mod common;
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
