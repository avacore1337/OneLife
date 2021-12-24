#![allow(dead_code)]

use crate::game::Game;
use crate::input::activity::ActivityTypes;
use crate::input::blessing::BlessingTypes;
use crate::input::boost_item::BoostItemTypes;
use crate::input::housing::HousingTypes;
use crate::input::tomb::TombTypes;
use crate::input::work::WorkTypes;
use crate::input::Recordable;
use crate::{
    buy_blessing_internal, buy_item_internal, buy_tomb_internal, set_activity_internal,
    set_housing_internal, set_work_internal,
};
use std::collections::HashMap;
use strum::IntoEnumIterator;

type Callback = Box<dyn Fn(&mut Game)>;

pub struct InputMapping {
    pub user_function: HashMap<String, Callback>,
}

impl InputMapping {
    fn add<T: Recordable>(&mut self, key: T, callback: Callback) {
        self.user_function.insert(key.to_record_key(), callback);
    }
}

impl Default for InputMapping {
    fn default() -> InputMapping {
        let mut mapping = InputMapping {
            user_function: HashMap::new(),
        };
        // mapping.user_function.insert(
        //     "Set auto Work".into(),
        //     Box::new(|game: &mut Game| {
        //         set_auto_work(true, game);
        //     }),
        // );
        for blessing in BlessingTypes::iter() {
            mapping.add(
                blessing,
                Box::new(move |game: &mut Game| {
                    buy_blessing_internal(blessing, game);
                }),
            );
        }
        for tomb in TombTypes::iter() {
            mapping.add(
                tomb,
                Box::new(move |game: &mut Game| {
                    buy_tomb_internal(tomb, game);
                }),
            );
        }
        for item in BoostItemTypes::iter() {
            mapping.add(
                item,
                Box::new(move |game: &mut Game| {
                    buy_item_internal(item, game);
                }),
            );
        }
        for work in WorkTypes::iter() {
            mapping.add(
                work,
                Box::new(move |game: &mut Game| {
                    set_work_internal(work, game);
                }),
            );
        }
        for housing in HousingTypes::iter() {
            mapping.add(
                housing,
                Box::new(move |game: &mut Game| {
                    set_housing_internal(housing, game);
                }),
            );
        }
        for activity in ActivityTypes::iter() {
            mapping.add(
                activity,
                Box::new(move |game: &mut Game| {
                    set_activity_internal(activity, game);
                }),
            );
        }
        mapping
    }
}
