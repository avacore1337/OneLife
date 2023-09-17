extern crate variant_count;

use super::game::Game;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use strum::IntoEnumIterator;
use variant_count::VariantCount;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Info {
    pub tutorial_step: TutorialStep,
    pub show_tutorial: bool,
    pub disable_tutorial: bool,
    pub version_build_data: String,
    pub version_commit_data: String,
}

impl Default for Info {
    fn default() -> Self {
        Self::new()
    }
}

impl Info {
    pub fn new() -> Info {
        let build_version: String = option_env!("CARGO_PKG_VERSION")
            .unwrap_or("dev-build")
            .to_string();
        let build_date: String = option_env!("VERGEN_BUILD_DATE")
            .unwrap_or("now")
            .to_string();
        let commit_hash: String = option_env!("VERGEN_RUSTC_COMMIT_HASH")
            .unwrap_or("abc123xyz789")
            .chars()
            .take(7)
            .collect();
        let commit_date: String = option_env!("VERGEN_GIT_COMMIT_DATE")
            .unwrap_or("unknown")
            .to_string();

        Info {
            tutorial_step: TutorialStep::Welcome,
            show_tutorial: should_show_tutorial(),
            disable_tutorial: true,
            version_build_data: format!("v{} ({})", build_version, build_date),
            version_commit_data: format!("{} ({})", commit_hash, commit_date),
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
    Serialize,
    Deserialize,
    EnumIter,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    FromPrimitive,
    VariantCount,
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

pub const TUTORIAL_STEP_SIZE: usize = TutorialStep::VARIANT_COUNT;

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
