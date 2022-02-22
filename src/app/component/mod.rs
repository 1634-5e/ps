//FIXME: 不应该把大部分函数放在结构体内
use std::cell::RefCell;
use std::rc::Rc;

use iced::{Command, Element};

use super::{Flags, UserSettings};

pub mod image_box;
pub mod toolbar;

//构成页面的组件，会有上方的按钮区，左下的显示区，和右下的区域
pub trait Component: Sized {
    type Message;

    fn new(flags: &mut Flags) -> (Self, Command<Self::Message>);
    fn view(
        &mut self,
        settings: Rc<RefCell<UserSettings>>,
    ) -> Element<Self::Message>;
    fn update(
        &mut self,
        message: Self::Message,
        settings: Rc<RefCell<UserSettings>>,
    ) -> Command<Self::Message>;
}
