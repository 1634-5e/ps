//手动泛型。。。目前不支持在函数以外的地方用impl Trait，而使用generic Type又会导致和Application Trait冲突，所以只能用这个很难看的写法
#[derive(Debug, Clone, Copy)]
pub enum SettingsType {
    LoadMode(LoadMode),
}

pub trait SettingsItem {
    fn describe(&self) {}
}

#[derive(Debug, Clone, Copy)]
pub enum LoadMode {
    Strict,
    Automatic,
}

impl SettingsItem for LoadMode {
    //未来在设置页面展示的文字
    fn describe(&self) {}
}
