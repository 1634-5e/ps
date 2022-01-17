//差错处理

#[derive(Debug, Clone)]
pub enum Error {
    NotFound,
    NameInvalid
}