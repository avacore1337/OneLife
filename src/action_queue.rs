#![allow(dead_code)]

// use strum::IntoEnumIterator;

// use crate::game::Game;
// use crate::input::activity::ActivityTypes;
// use crate::input::blessing::BlessingTypes;
// use crate::input::boost_item::BoostItemTypes;
// use crate::input::housing::HousingTypes;
// use crate::input::options::AutoSettingTypes;
// use crate::input::tomb::TombTypes;
// use crate::input::work::WorkTypes;
use crate::input::Recordable;

// type Callback = Box<dyn Fn(&mut Game) + Send>;

struct ActionQueue {
    actions: Vec<String>,
}

impl ActionQueue {
    pub fn add_action<T: Recordable>(&mut self, key: T) {
        self.actions.push(key.to_record_key());
    }
}
