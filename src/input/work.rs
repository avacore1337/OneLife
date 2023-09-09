extern crate variant_count;

use super::{stat::StatTypes, Recordable};
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use variant_count::VariantCount;

#[derive(
    Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, VariantCount,
)]
pub enum WorkTypes {
    // Worker Types
    Mines,
    Latrine,
    GalleyRower,
    Fields,
    Mill,
    Weaver,
    Fisherman,
    Farmer,
    Woodcutter,
    LeatherWorker,
    Potter,
    Carpenter,
    BlacksmithApprentice,
    Blacksmith,
    Goldsmith,
    GlassBlower,
    Weaponsmith,
    // Trader,
    // Merchant,
    // Army Types
    BaggageBoy,
    Slinger,
    Peltast,
    Pikeman,
    FootCompanion,
    Hypaspist,
    LightCavalry,
    // ThessalianCavalry,
    // CompanionCavalry,
    // SquadCommander,
    // Commander,
    // General,
    // Intellectual Types
    // Servant,
    // Teacher,
    // Priest/Faith
    Priest,
    Bishop,
}

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum WorkCategoryTypes {
    Labor,
    Soldier,
    Intellectual,
    Priest,
}

impl TryFrom<StatTypes> for WorkCategoryTypes {
    type Error = ();
    fn try_from(stat: StatTypes) -> Result<Self, Self::Error> {
        match stat {
            StatTypes::Str => Ok(WorkCategoryTypes::Soldier),
            StatTypes::Con => Ok(WorkCategoryTypes::Labor),
            StatTypes::Int => Ok(WorkCategoryTypes::Intellectual),
            StatTypes::Faith => Ok(WorkCategoryTypes::Priest),
            _ => Err(()),
        }
    }
}

pub const WORK_SIZE: usize = WorkTypes::VARIANT_COUNT;

impl Recordable for WorkTypes {
    fn to_record_key(&self) -> String {
        format!("Set Work {:#?}", self)
    }
}
