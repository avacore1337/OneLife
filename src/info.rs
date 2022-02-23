use super::game::Game;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Info {
    pub tutorial_step: TutorialStep,
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
            tutorial_step: TutorialStep::Welcome,
            show_tutorial: should_show_tutorial(),
            disable_tutorial: true,
        }
    }

    pub fn get_completed_steps(&self) -> Vec<TutorialStep> {
        TutorialStep::iter()
            .filter(|step| *step < self.tutorial_step)
            .collect()
    }
}

fn should_show_tutorial() -> bool {
    !cfg!(debug_assertions)
}

#[derive(
    Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd, FromPrimitive,
)]
pub enum TutorialStep {
    Welcome,
    GameStart,
    FirstDying,
    FirstDeath,
    FirstRestart,
    FirstCoin,
    SafeGuard,
}

pub const TUTORIAL_STEP_SIZE: usize = variant_count::<TutorialStep>();

impl TutorialStep {
    pub fn increment(&mut self) {
        if *self as usize + 1 < TUTORIAL_STEP_SIZE {
            *self = FromPrimitive::from_usize(*self as usize + 1).unwrap();
        }
    }
}

pub fn check_for_tutorial_step(game: &mut Game) {
    let info = &mut game.meta_data.info;
    if info.disable_tutorial {
        return;
    }
    if info.tutorial_step == TutorialStep::Welcome {
        info.show_tutorial = true;
    }
    if info.tutorial_step == TutorialStep::GameStart {
        info.show_tutorial = true;
    }
    if info.tutorial_step == TutorialStep::FirstDying && game.state.life_stats.is_dying {
        info.show_tutorial = true;
    }
    if info.tutorial_step == TutorialStep::FirstDeath && game.state.life_stats.dead {
        info.show_tutorial = true;
    }
    if info.tutorial_step == TutorialStep::FirstRestart
        && !game.state.life_stats.dead
        && game.state.rebirth_stats.rebirth_count >= 1
    {
        info.show_tutorial = true;
    }
    if info.tutorial_step == TutorialStep::FirstCoin
        && game.state.life_stats.dead
        && game.state.rebirth_stats.coins == 2.0
    {
        info.show_tutorial = true;
    }
}
