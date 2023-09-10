use crate::input::stat::StatTypes;
use crate::input::work::WorkTypes;
use crate::input::{skill::SkillTypes, work::WorkCategoryTypes};
use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyValues {
    //Other
    Happiness,
    Money,
    Coins,
    DivineFavor,
    Health,
    Skills,
    Stats,
    //WorkTypes
    LaborXp,
    SoldierXp,
    IntellectualXp,
    PriestXp,
    //Stats
    Str,
    Int,
    Cha,
    Con,
    Dex,
    Faith,
    //Skill
    Mindful,
    Tactics,
    //Work
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
    //Soldiers
    BaggageBoy,
    Slinger,
    Peltast,
    Pikeman,
    FootCompanion,
    Hypaspist,
    LightCavalry,
    //
    // Trader,
    // Merchant,
    // Priest/Faith
    Priest,
    Bishop,
}

impl From<SkillTypes> for KeyValues {
    fn from(stat: SkillTypes) -> Self {
        match stat {
            SkillTypes::Mindful => KeyValues::Mindful,
            SkillTypes::Tactics => KeyValues::Tactics,
        }
    }
}

impl From<StatTypes> for KeyValues {
    fn from(stat: StatTypes) -> Self {
        match stat {
            StatTypes::Str => KeyValues::Str,
            StatTypes::Cha => KeyValues::Cha,
            StatTypes::Con => KeyValues::Con,
            StatTypes::Int => KeyValues::Int,
            StatTypes::Dex => KeyValues::Dex,
            StatTypes::Faith => KeyValues::Faith,
        }
    }
}

impl From<WorkCategoryTypes> for KeyValues {
    fn from(category: WorkCategoryTypes) -> Self {
        match category {
            WorkCategoryTypes::Labor => KeyValues::LaborXp,
            WorkCategoryTypes::Soldier => KeyValues::SoldierXp,
            WorkCategoryTypes::Intellectual => KeyValues::IntellectualXp,
            WorkCategoryTypes::Priest => KeyValues::PriestXp,
        }
    }
}

impl From<WorkTypes> for KeyValues {
    fn from(work: WorkTypes) -> Self {
        match work {
            WorkTypes::Mines => KeyValues::Mines,
            WorkTypes::Latrine => KeyValues::Latrine,
            WorkTypes::GalleyRower => KeyValues::GalleyRower,
            WorkTypes::Fields => KeyValues::Fields,
            WorkTypes::Mill => KeyValues::Mill,
            WorkTypes::Weaver => KeyValues::Weaver,
            WorkTypes::Fisherman => KeyValues::Fisherman,
            WorkTypes::Farmer => KeyValues::Farmer,
            WorkTypes::Woodcutter => KeyValues::Woodcutter,
            WorkTypes::LeatherWorker => KeyValues::LeatherWorker,
            WorkTypes::Potter => KeyValues::Potter,
            WorkTypes::Carpenter => KeyValues::Carpenter,
            WorkTypes::BlacksmithApprentice => KeyValues::BlacksmithApprentice,
            WorkTypes::Blacksmith => KeyValues::Blacksmith,
            WorkTypes::Goldsmith => KeyValues::Goldsmith,
            WorkTypes::GlassBlower => KeyValues::GlassBlower,
            WorkTypes::Weaponsmith => KeyValues::Weaponsmith,
            // WorkTypes::Trader => KeyValues::Trader,
            WorkTypes::BaggageBoy => KeyValues::BaggageBoy,
            WorkTypes::Slinger => KeyValues::Slinger,
            WorkTypes::Peltast => KeyValues::Peltast,
            WorkTypes::Pikeman => KeyValues::Pikeman,
            WorkTypes::FootCompanion => KeyValues::FootCompanion,
            WorkTypes::Hypaspist => KeyValues::Hypaspist,
            WorkTypes::LightCavalry => KeyValues::LightCavalry,
            WorkTypes::Priest => KeyValues::Priest,
            WorkTypes::Bishop => KeyValues::Bishop,
        }
    }
}
