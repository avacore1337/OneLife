use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum ActivityTypes {
    Run,
    Studying,
    Training,
    Flirt,
    Praying,
    Acrobatics,
}

pub const ACTIVITY_SIZE: usize = variant_count::<ActivityTypes>();
