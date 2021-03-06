use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum SkillTypes {
    Mindfull,
    Tactics,
}

pub const SKILL_SIZE: usize = variant_count::<SkillTypes>();
