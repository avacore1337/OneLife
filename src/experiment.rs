#![allow(dead_code)]

use crate::game::Game;
use crate::input::activity::ActivityTypes;
use crate::input::blessing::BlessingTypes;
use crate::input::boost_item::BoostItemTypes;
use crate::input::housing::HousingTypes;
use crate::input::tomb::TombTypes;
use crate::input::work::WorkTypes;
use crate::{
    buy_blessing_internal, buy_item_internal, buy_tomb_internal, set_activity_internal,
    set_housing_internal, set_work_internal, GLOBAL_DATA,
};
// use serde::{Deserialize, Serialize};
use log::info;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use wasm_bindgen::prelude::*;
type Callback = Box<dyn Fn(&mut Game)>;

pub struct InputMapping {
    pub user_function: HashMap<String, Callback>,
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
            let name: String = format!("Buy Blessing {:#?}", blessing);
            mapping.user_function.insert(
                name.clone(),
                Box::new(move |game: &mut Game| {
                    buy_blessing_internal(blessing, game);
                }),
            );
        }
        for tomb in TombTypes::iter() {
            let name: String = format!("Buy Tomb {:#?}", tomb);
            mapping.user_function.insert(
                name.clone(),
                Box::new(move |game: &mut Game| {
                    buy_tomb_internal(tomb, game);
                }),
            );
        }
        for item in BoostItemTypes::iter() {
            let name: String = format!("Buy Item {:#?}", item);
            mapping.user_function.insert(
                name.clone(),
                Box::new(move |game: &mut Game| {
                    buy_item_internal(item, game);
                }),
            );
        }
        for work in WorkTypes::iter() {
            let name: String = format!("Set Work {:#?}", work);
            mapping.user_function.insert(
                name.clone(),
                Box::new(move |game: &mut Game| {
                    set_work_internal(work, game);
                }),
            );
        }
        for housing in HousingTypes::iter() {
            let name: String = format!("Set Housing {:#?}", housing);
            mapping.user_function.insert(
                name.clone(),
                Box::new(move |game: &mut Game| {
                    set_housing_internal(housing, game);
                }),
            );
        }
        for activity in ActivityTypes::iter() {
            let name: String = format!("Set Activity {:#?}", activity);
            mapping.user_function.insert(
                name.clone(),
                Box::new(move |game: &mut Game| {
                    set_activity_internal(activity, game);
                }),
            );
        }
        mapping
    }
}

#[wasm_bindgen]
pub fn test() {
    // let user_input_mapping = InputMapping::default();
    // info!("Mapping: {:#?}", user_input_mapping.user_function.keys());
    let game = GLOBAL_DATA.lock().unwrap();
    info!("input: {:#?}", game.inputs);
}
