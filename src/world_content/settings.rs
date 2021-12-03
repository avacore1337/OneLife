use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Settings {
    pub display_debug: bool,
}
impl Default for Settings {
    fn default() -> Settings {
        Settings {
            display_debug: should_display_debug(),
        }
    }
}

fn should_display_debug() -> bool {
    cfg!(debug_assertions)
}
