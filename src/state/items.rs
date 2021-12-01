use crate::input::boost_item::{BoostItemTypes, BOOST_ITEM_SIZE};
use serde::{Deserialize, Serialize};

use std::mem::{self, MaybeUninit};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct BoostItem {
    pub name: BoostItemTypes,
    pub is_purchased: bool,
    pub is_unlocked: bool,
    pub is_visible: bool,
}

impl BoostItem {
    pub fn new(item_type: BoostItemTypes) -> BoostItem {
        BoostItem {
            name: item_type,
            is_purchased: false,
            is_unlocked: false,
            is_visible: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Items {
    pub money: f64,
    pub boost_items: [BoostItem; BOOST_ITEM_SIZE],
}

impl Items {
    pub fn new() -> Items {
        Items {
            money: 0.0,
            boost_items: get_boost_items(),
        }
    }
}

impl Default for Items {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_boost_items() -> [BoostItem; BOOST_ITEM_SIZE] {
    let mut works: [MaybeUninit<BoostItem>; BOOST_ITEM_SIZE] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for name in BoostItemTypes::iter() {
        works[name as usize].write(BoostItem::new(name));
    }
    unsafe { mem::transmute(works) }
}
