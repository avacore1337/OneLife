use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

use super::Recordable;

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

impl Recordable for ActivityTypes {
    fn to_record_key(&self) -> String {
        format!("Set Activity {:#?}", self)
    }
}
