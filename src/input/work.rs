use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Work {
    Mines,
    Fields,
    Servant,
    Teacher,
    Farm,
}
