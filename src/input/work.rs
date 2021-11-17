use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug)]
pub enum WorkTypes {
    Mines,
    Fields,
    Servant,
    Teacher,
    Farm,
}

pub const WORK_SIZE: usize = variant_count::<WorkTypes>();
