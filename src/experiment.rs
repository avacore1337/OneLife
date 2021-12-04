#![allow(dead_code)]

use crate::input::boost_item::BoostItemTypes;
use crate::input::housing::HousingTypes;
use crate::input::tomb::TombTypes;
use crate::input::work::WorkTypes;
use crate::{
    buy_item_internal, buy_tomb_internal, set_housing_internal, set_work_internal, GLOBAL_DATA,
};
// use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::console;
type Callback = Box<dyn Fn() -> ()>;

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
        //     Box::new(|| {
        //         set_auto_work(true);
        //     }),
        // );
        for tomb in TombTypes::iter() {
            let name: String = format!("Buy Tomb {:#?}", tomb);
            mapping.user_function.insert(
                name.clone(),
                Box::new(move || {
                    buy_tomb_internal(tomb);
                }),
            );
        }
        for item in BoostItemTypes::iter() {
            let name: String = format!("Buy Item {:#?}", item);
            mapping.user_function.insert(
                name.clone(),
                Box::new(move || {
                    buy_item_internal(item);
                }),
            );
        }
        for work in WorkTypes::iter() {
            let name: String = format!("Set Work {:#?}", work);
            mapping.user_function.insert(
                name.clone(),
                Box::new(move || {
                    set_work_internal(work);
                }),
            );
        }
        for housing in HousingTypes::iter() {
            let name: String = format!("Set Housing {:#?}", housing);
            mapping.user_function.insert(
                name.clone(),
                Box::new(move || {
                    set_housing_internal(housing);
                }),
            );
        }
        console::log_1(&JsValue::from_str(&format!(
            "Mapping: {:#?}",
            mapping.user_function.keys()
        )));
        mapping
    }
}

#[wasm_bindgen]
pub fn test() {
    let _user_input_mapping = InputMapping::default();
    let game = GLOBAL_DATA.lock().unwrap();
    console::log_1(&JsValue::from_str(&format!("input: {:#?}", game.inputs)));
}
