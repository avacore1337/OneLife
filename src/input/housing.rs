use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy)]
pub enum Housing {
    StoneFloor,
    ComfortableSpot,
    FilthyBarracks,
    CrampedBarracks,
}
