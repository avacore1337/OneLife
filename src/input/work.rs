use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy)]
pub enum Work {
    Mines,
    Fields,
    Servant,
    Teacher,
    Farm,
}
