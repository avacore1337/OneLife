use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use strum::EnumIter;
use strum::IntoEnumIterator;

#[derive(Serialize, Debug, Clone)]
pub struct Icon {
    pub name: &'static str,
}

impl Icon {
    pub fn new(name: &'static str) -> Icon {
        Icon { name }
    }
}

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum IconType {
    Con,
    Int,
    Str,
    Cha,
    Faith,
    Dex,
    Mindful,
    Tactics,
    Coin,
    Money,
    DivineFavor,
    Happiness,
    Health,
    Question,
    Pause,
    Play,
    Soldier,
    Labor,
    Priest,
    Death,
    Tomb,
    Automate,
}

impl From<IconType> for Icon {
    fn from(icon: IconType) -> Self {
        Icon {
            name: get_icon_name(icon),
        }
    }
}

pub fn get_icon_name(icon: IconType) -> &'static str {
    match icon {
        IconType::Con => "running",
        IconType::Int => "brain",
        IconType::Str => "dumbbell",
        IconType::Cha => "glass-cheers",
        IconType::Dex => "dumbell",
        IconType::Faith => "church",
        IconType::Mindful => "hand-holding",
        IconType::Tactics => "map",
        IconType::Coin => "coins",
        IconType::Money => "money-bill",
        IconType::DivineFavor => "bible",
        IconType::Happiness => "laugh-beam",
        IconType::Health => "medkit",
        IconType::Question => "question",
        IconType::Pause => "pause",
        IconType::Play => "play",
        IconType::Soldier => "hammer", // should be a sword
        IconType::Labor => "hammer",
        IconType::Priest => "hammer", // should be something else
        IconType::Death => "skull-crossbones",
        IconType::Tomb => "skull", // should be a tombstone
        IconType::Automate => "cogs",
    }
}

pub fn get_icons() -> BTreeMap<String, Icon> {
    let mut icons = BTreeMap::new();
    for icon in IconType::iter() {
        icons.insert(format!("{:?}", icon), icon.into());
    }
    icons
}
