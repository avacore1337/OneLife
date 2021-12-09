use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

use super::Recordable;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum BlessingTypes {
    HeruclesStrength,
    AthenasWisdom,
}

pub const BLESSING_SIZE: usize = variant_count::<BlessingTypes>();

impl Recordable for BlessingTypes {
    fn to_record_key(&self) -> String {
        format!("Buy Blessing {:#?}", self)
    }
}
