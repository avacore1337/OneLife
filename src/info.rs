use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Info {
    pub tutorial_step: u32,
    pub show_tutorial: bool,
    pub disable_tutorial: bool,
}

impl Default for Info {
    fn default() -> Self {
        Self::new()
    }
}

impl Info {
    pub fn new() -> Info {
        Info {
            tutorial_step: 0,
            show_tutorial: false,
            disable_tutorial: true,
        }
    }
}
