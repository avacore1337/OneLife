#![allow(dead_code)]

use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::game::Game;
use crate::input::activity::ActivityTypes;
use crate::input::blessing::BlessingTypes;
use crate::input::boost_item::BoostItemTypes;
use crate::input::housing::HousingTypes;
use crate::input::options::AutoSettingTypes;
use crate::input::tomb::TombTypes;
use crate::input::work::WorkTypes;
use crate::input::Recordable;
use crate::{
    buy_blessing_internal, buy_item_internal, buy_tomb_internal, set_activity_internal,
    set_auto_buy_item_internal, set_auto_buy_tomb_internal, set_auto_living_internal,
    set_auto_work_internal, set_housing_internal, set_work_internal,
};

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
        for auto_setting in AutoSettingTypes::iter() {
            match auto_setting {
                AutoSettingTypes::AutoWorkTrue => mapping.add(
                    auto_setting,
                    Box::new(move |game: &mut Game| {
                        set_auto_work_internal(true, game);
                    }),
                ),
                AutoSettingTypes::AutoWorkFalse => mapping.add(
                    auto_setting,
                    Box::new(move |game: &mut Game| {
                        set_auto_work_internal(false, game);
                    }),
                ),
                AutoSettingTypes::AutoLivingTrue => mapping.add(
                    auto_setting,
                    Box::new(move |game: &mut Game| {
                        set_auto_living_internal(true, game);
                    }),
                ),
                AutoSettingTypes::AutoLivingFalse => mapping.add(
                    auto_setting,
                    Box::new(move |game: &mut Game| {
                        set_auto_living_internal(false, game);
                    }),
                ),
                AutoSettingTypes::AutoBuyItemTrue => mapping.add(
                    auto_setting,
                    Box::new(move |game: &mut Game| {
                        set_auto_buy_item_internal(true, game);
                    }),
                ),
                AutoSettingTypes::AutoBuyItemFalse => mapping.add(
                    auto_setting,
                    Box::new(move |game: &mut Game| {
                        set_auto_buy_item_internal(false, game);
                    }),
                ),
                AutoSettingTypes::AutoBuyTombTrue => mapping.add(
                    auto_setting,
                    Box::new(move |game: &mut Game| {
                        set_auto_buy_tomb_internal(true, game);
                    }),
                ),
                AutoSettingTypes::AutoBuyTombFalse => mapping.add(
                    auto_setting,
                    Box::new(move |game: &mut Game| {
                        set_auto_buy_tomb_internal(false, game);
                    }),
                ),
            }
        }
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
