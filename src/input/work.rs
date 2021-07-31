use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter)]
pub enum Work {
    Mines,
    Fields,
    Servant,
    Teacher,
    Farm,
}
