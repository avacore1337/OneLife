use super::{stat::StatTypes, Recordable};
use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
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
    BagageBoy,
    Slinger,
    Peltasts,
    Pikeman,
    FootCompanion,
    Hypaspists,
    LightCavalery,
    // ThessalianCavalery,
    // CompanionCavalery,
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

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
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

pub const WORK_SIZE: usize = variant_count::<WorkTypes>();

impl Recordable for WorkTypes {
    fn to_record_key(&self) -> String {
        format!("Set Work {:#?}", self)
    }
}
