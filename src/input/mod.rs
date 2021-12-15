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

use activity::ActivityTypes;
use housing::HousingTypes;
use work::WorkTypes;

use serde::{Deserialize, Serialize};

use crate::{state::state_container::StateContainer, world_content::world::World};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Input {
    pub work: WorkTypes,
    pub housing: HousingTypes,
    pub activity: ActivityTypes,
}

impl Input {
    pub fn new(state: &StateContainer, world: &World) -> Input {
        Input {
            work: world.tiers[state.rebirth_stats.class_tier as usize].starting_work,
            housing: HousingTypes::StoneFloor,
            activity: ActivityTypes::Run,
        }
    }
}

pub trait Recordable {
    fn to_record_key(&self) -> String;
}
