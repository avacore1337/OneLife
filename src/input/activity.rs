use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug)]
pub enum ActivityTypes {
    Training,
    Studying,
    Praying,
}

pub const ACTIVITY_SIZE: usize = variant_count::<ActivityTypes>();
