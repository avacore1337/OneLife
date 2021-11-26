use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Info {
    pub tutorial_step: u32,
    pub show_tutorial: bool,
}

impl Info {
    pub fn new() -> Info {
        Info {
            tutorial_step: 0,
            show_tutorial: true,
        }
    }
}
pub fn get_info(step: u32) -> &'static str {
    match step {
        0 => "Welcome to the tutorial",
        _ => "other",
    }
}
