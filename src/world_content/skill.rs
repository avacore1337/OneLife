use crate::engine::value_keys::KeyValues;
use crate::game::Game;
use crate::icon::{Icon, IconType};
use crate::input::skill::{SkillTypes, SKILL_SIZE};
use serde::Serialize;
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(readonly)]
#[derive(Serialize, Clone)]
pub struct Skill {
    pub name: SkillTypes,
    #[wasm_bindgen(getter_with_clone)]
    pub description: String,
    #[wasm_bindgen(skip)]
    pub effect_description: &'static str,
    #[wasm_bindgen(skip)]
    pub display_name: &'static str,
    pub required_tier: u32,
    pub xp_req_modifier: f64,
    pub icon: Icon,
}

impl Skill {
    pub fn get_skills_gains(&self, game: &mut Game) {
        let skill_state = &mut game.state.skills[self.name as usize];

        match self.name {
            SkillTypes::Mindfull => {
                game.intermediate_state.add_multiplier(
                    KeyValues::Happiness,
                    0.05 * skill_state.level + 1.0,
                    self.display_name,
                );
            }
            SkillTypes::Tactics => {
                game.intermediate_state.add_multiplier(
                    KeyValues::SoldierXp,
                    0.05 * skill_state.level + 1.0,
                    self.display_name,
                );
            }
        }
    }
}

pub fn translate_skill(skill: SkillTypes) -> Skill {
    match skill {
        SkillTypes::Mindfull => Skill {
            name: skill,
            description: "Be one with the world".to_string(),
            effect_description: "Multiplies Happiness by 1 + level*0.05",
            display_name: "Mindefullness",
            required_tier: 1,
            xp_req_modifier: 4.0,
            icon: IconType::Mindfull.into(),
        },
        SkillTypes::Tactics => Skill {
            name: skill,
            description: "Flank them!".to_string(),
            effect_description: "Multiplies Soldier XP by 1 + level*0.05",
            display_name: "Military Tactics",
            required_tier: 3,
            xp_req_modifier: 1.0,
            icon: IconType::Tactics.into(),
        },
    }
}

pub fn should_be_visible_skill(input_skill: SkillTypes, game: &Game) -> bool {
    let skill = &game.world.skills[input_skill as usize];
    if game.state.rebirth_stats.tier < skill.required_tier {
        return false;
    }
    match input_skill {
        SkillTypes::Tactics => game.state.rebirth_stats.unlocks.has_military_tactics,
        SkillTypes::Mindfull => game.state.rebirth_stats.unlocks.has_meditation,
        // _ => true,
    }
}

pub fn get_skills() -> [Skill; SKILL_SIZE] {
    let mut skills: [MaybeUninit<Skill>; SKILL_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in SkillTypes::iter() {
        skills[name as usize].write(translate_skill(name));
    }
    unsafe { mem::transmute(skills) }
}
