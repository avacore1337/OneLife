use crate::game::Game;
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
        1 => "So, you died",
        2 => "So, you died, but this time you brought money!",
        _ => "how did you get here?",
    }
}

pub fn check_for_tutorial_step(game: &mut Game) {
    let info = &mut game.meta_data.info;
    if info.tutorial_step == 1 && game.state.life_stats.dead {
        info.show_tutorial = true;
    }
    if info.tutorial_step == 2
        && game.state.life_stats.dead
        && game.state.rebirth_stats.coins == 2.0
    {
        info.show_tutorial = true;
    }
}
