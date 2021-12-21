use super::Recordable;
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
}

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum WorkCategoryTypes {
    Labor,
    Soldier,
    Intellectual,
    //
}

pub const WORK_SIZE: usize = variant_count::<WorkTypes>();

impl Recordable for WorkTypes {
    fn to_record_key(&self) -> String {
        format!("Set Work {:#?}", self)
    }
}
