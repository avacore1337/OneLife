// use crate::engine::value_keys::KeyValues;
use crate::game::Game;
use crate::input::skill::{SkillTypes, SKILL_SIZE};
use serde::Serialize;
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize, Clone)]
pub struct Skill {
    pub name: SkillTypes,
    pub description: &'static str,
    pub display_name: &'static str,
    pub required_tier: u32,
}

pub fn get_skills_gains(skill_type: SkillTypes, game: &mut Game) {
    // let world_skill = &game.world.skills[skill_type as usize];
    let skill_state = &mut game.state.skills[skill_type as usize];
    if !skill_state.is_visible {
        return;
    }

    match skill_type {
        // SkillTypes::Con => {
        //     game.intermediate_state.add_multiplier(
        //         KeyValues::Health,
        //         0.05 * skill_state.level,
        //         "Constitution",
        //     );
        // }
        // SkillTypes::Cha => {
        //     game.intermediate_state.add_multiplier(
        //         KeyValues::Coins,
        //         1.0 + 0.05 * skill_state.level,
        //         "Charisma",
        //     );
        // }
        _ => {}
    }
}

pub const fn translate_skill(skill: SkillTypes) -> Skill {
    match skill {
        SkillTypes::Mindfull => Skill {
            name: skill,
            description: "Do you even lift?",
            display_name: "Strength",
            required_tier: 2,
        },
        SkillTypes::Tactics => Skill {
            name: skill,
            description: "Do you even lift?",
            display_name: "Strength",
            required_tier: 2,
        },
    }
}

pub fn should_be_visible_skill(input_skill: SkillTypes, game: &Game) -> bool {
    let skill = &game.world.skills[input_skill as usize];
    game.state.rebirth_stats.class_tier >= skill.required_tier
}

pub fn get_skills() -> [Skill; SKILL_SIZE] {
    let mut skills: [MaybeUninit<Skill>; SKILL_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in SkillTypes::iter() {
        skills[name as usize].write(translate_skill(name));
    }
    unsafe { mem::transmute(skills) }
}
