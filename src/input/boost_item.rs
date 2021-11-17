use serde::{Deserialize, Serialize};
use std::mem::variant_count;
use strum::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Clone, Copy, Debug)]
pub enum BoostItemTypes {
    Book,
    Dumbell,
}

pub const BOOST_ITEM_SIZE: usize = variant_count::<BoostItemTypes>();