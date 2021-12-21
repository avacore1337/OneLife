use crate::input::work::WorkCategoryTypes;
use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum StatTypes {
    Con,
    Int,
    Str,
    Cha,
    Faith,
    Dex,
}

impl From<WorkCategoryTypes> for StatTypes {
    fn from(stat: WorkCategoryTypes) -> Self {
        match stat {
            WorkCategoryTypes::Soldier => StatTypes::Str,
            WorkCategoryTypes::Labor => StatTypes::Con,
            WorkCategoryTypes::Intellectual => StatTypes::Int,
        }
    }
}

pub const STAT_SIZE: usize = variant_count::<StatTypes>();
