use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug)]
pub enum Work {
    Mines,
    Fields,
    Servant,
    Teacher,
    Farm,
}

pub const WORK_SIZE: usize = variant_count::<Work>();
