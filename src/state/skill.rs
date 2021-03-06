use crate::input::skill::{SkillTypes, SKILL_SIZE};
use serde::{Deserialize, Serialize};
use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Skill {
    pub name: SkillTypes,
    pub level: f64,
    pub next_level_progress: f64,
    pub next_level_required: f64,
    pub next_level_percentage: f64,
    pub xp_rate: f64,
    pub is_visible: bool,
}

impl Skill {
    pub fn new(skill: SkillTypes) -> Skill {
        Skill {
            name: skill,
            level: 0.0,
            next_level_progress: 0.0,
            next_level_required: 100.0,
            next_level_percentage: 0.0,
            xp_rate: 0.0,
            is_visible: false,
        }
    }
}

pub fn get_skills() -> [Skill; SKILL_SIZE] {
    let mut skills: [MaybeUninit<Skill>; SKILL_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in SkillTypes::iter() {
        skills[name as usize].write(Skill::new(name));
    }
    unsafe { mem::transmute(skills) }
}
