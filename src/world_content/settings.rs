use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Settings {
    pub display_debug: bool,
}
impl Settings {
    pub fn new() -> Settings {
        Settings {
            display_debug: should_display_debug(),
        }
    }
}

fn should_display_debug() -> bool {
    if cfg!(debug_assertions) {
        true
    } else {
        false
    }
}
