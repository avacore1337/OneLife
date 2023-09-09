extern crate variant_count;

use serde::{Deserialize, Serialize};
use strum::EnumIter;
use variant_count::VariantCount;

#[derive(
    Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd, VariantCount,
)]
pub enum SkillTypes {
    Mindful,
    Tactics,
}

pub const SKILL_SIZE: usize = SkillTypes::VARIANT_COUNT;
