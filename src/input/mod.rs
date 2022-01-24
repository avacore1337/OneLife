pub mod activity;
pub mod blessing;
pub mod boost_item;
pub mod housing;
pub mod options;
pub mod rebirth_upgrade;
pub mod skill;
pub mod stat;
pub mod tomb;
pub mod work;

use crate::{
    world_content::boost_item::{translate_boost_item, BoostItem},
    WORLD,
};
use activity::ActivityTypes;
use housing::HousingTypes;
use work::WorkTypes;

use serde::{Deserialize, Serialize};

use crate::state::state_container::StateContainer;

use self::boost_item::BoostItemTypes;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Input {
    pub work: WorkTypes,
    pub housing: HousingTypes,
    pub activity: ActivityTypes,
    pub item_queue: Vec<BoostItemTypes>,
}

impl Input {
    pub fn new(state: &StateContainer) -> Input {
        Input {
            work: WORLD.tiers[state.rebirth_stats.tier as usize].starting_work,
            housing: HousingTypes::StoneFloor,
            activity: ActivityTypes::Run,
            item_queue: vec![],
        }
    }

    pub fn dequeue_item(&mut self, item: BoostItemTypes) {
        self.item_queue.drain_filter(|item_type| *item_type == item);
    }

    pub fn queue_item(&mut self, item: BoostItemTypes) {
        if !self.item_queue.contains(&item) {
            self.item_queue.push(item);
        }
    }

    pub fn get_world_item_queue(&self) -> Vec<BoostItem> {
        self.item_queue
            .iter()
            .map(|item| translate_boost_item(item.clone()))
            .collect()
    }
}

pub trait Recordable {
    fn to_record_key(&self) -> String;
}
