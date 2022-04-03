//save session when exiting normally and restore it next time.

use crate::State;

#[derive(Debug, Clone, Copy)]
enum Error {}

pub async fn save() {}

pub async fn load() -> Option<State> {
    None
}
