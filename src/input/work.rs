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

pub const WORK_SIZE: usize = variant_count::<WorkTypes>();
