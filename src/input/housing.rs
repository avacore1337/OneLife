use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter)]
pub enum Housing {
    StoneFloor,
    ComfortableSpot,
    FilthyBarracks,
    CrampedBarracks,
}
