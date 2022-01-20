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
    fn from(category: WorkCategoryTypes) -> Self {
        match category {
            WorkCategoryTypes::Soldier => StatTypes::Str,
            WorkCategoryTypes::Labor => StatTypes::Con,
            WorkCategoryTypes::Intellectual => StatTypes::Int,
            WorkCategoryTypes::Priest => StatTypes::Faith,
        }
    }
}

pub const STAT_SIZE: usize = variant_count::<StatTypes>();
