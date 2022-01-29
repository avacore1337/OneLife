use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct Icon {
    pub name: &'static str,
}

impl Icon {
    pub fn new(name: &'static str) -> Icon {
        Icon { name }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum IconType {
    Con,
    Int,
    Str,
    Cha,
    Faith,
    Dex,
    Mindfull,
    Tactics,
    Coin,
    Money,
    Happiness,
    Health,
    Question,
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
        IconType::Mindfull => "hand-holding",
        IconType::Tactics => "map",
        IconType::Coin => "coins",
        IconType::Money => "money-bill",
        IconType::Happiness => "laugh-beam",
        IconType::Health => "medkit",
        IconType::Question => "question",
    }
}
