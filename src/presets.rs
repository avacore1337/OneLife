use crate::input::rebirth_upgrade::RebirthUpgradeTypes;
use crate::input::work::WorkTypes;
use crate::input::Input;
use crate::meta::MetaData;
use crate::state::state_container::{new_game, rebirth, StateContainer};
use crate::world_content::world::World;
use std::collections::BTreeMap;

pub fn get_presets(world: &World) -> BTreeMap<&'static str, (StateContainer, Input, MetaData)> {
    let mut presets = BTreeMap::new();
    presets.insert("1: poor death", make_poor_death(world));
    presets.insert("2: rich death", make_rich_death(world));
    presets.insert("3: T2 expected", make_t2(world));
    presets.insert("4: T3", make_t3(world));
    presets.insert("4: T5 Faith", make_t5_faith(world));
    presets.insert("5: saved ticks", make_saved_ticks(world));

    presets
}

fn make_poor_death(world: &World) -> (StateContainer, Input, MetaData) {
    let mut state = new_game(world);
    let meta_data = MetaData::new();
    state.items.money = 10.0;
    state.life_stats.age = 70.0 * 365.0;

    let input = Input::new(&state, world);
    (state, input, meta_data)
}

fn make_rich_death(world: &World) -> (StateContainer, Input, MetaData) {
    let mut state = new_game(world);
    let meta_data = MetaData::new();
    state.items.money = 10000000.0;
    state.life_stats.age = 70.0 * 365.0;

    let input = Input::new(&state, world);
    (state, input, meta_data)
}

fn make_t2(world: &World) -> (StateContainer, Input, MetaData) {
    let mut state = new_game(world);
    let meta_data = MetaData::new();
    let r = &mut state.rebirth_stats;
    r.class_tier = 2;
    r.coins = 8.0;
    r.rebirth_count = 8;

    r.max_job_levels[WorkTypes::Mines as usize] = 30;
    r.max_job_levels[WorkTypes::Latrine as usize] = 30;
    r.max_job_levels[WorkTypes::GalleyRower as usize] = 30;
    r.max_job_levels[WorkTypes::Fields as usize] = 30;
    r.max_job_levels[WorkTypes::Mill as usize] = 30;

    r.rebirth_upgrades[RebirthUpgradeTypes::AcceptingDeath as usize].is_purchased = true;

    let input = Input::new(&state, world);
    (state, input, meta_data)
}

fn make_t3(world: &World) -> (StateContainer, Input, MetaData) {
    let mut state = new_game(world);
    let meta_data = MetaData::new();
    let r = &mut state.rebirth_stats;
    r.class_tier = 3;
    r.coins = 100.0;
    r.rebirth_count = 16;
    r.max_job_levels[WorkTypes::Fields as usize] = 50;
    r.max_job_levels[WorkTypes::Mill as usize] = 50;
    r.max_job_levels[WorkTypes::Weaver as usize] = 50;
    r.max_job_levels[WorkTypes::Fisherman as usize] = 50;
    r.max_job_levels[WorkTypes::Farmer as usize] = 50;

    state = rebirth(world, state.rebirth_stats.clone());
    state.items.money = 1_000_000.0;

    let input = Input::new(&state, world);
    (state, input, meta_data)
}

fn make_t5_faith(world: &World) -> (StateContainer, Input, MetaData) {
    let mut state = new_game(world);
    let meta_data = MetaData::new();
    let r = &mut state.rebirth_stats;
    r.class_tier = 5;
    r.coins = 8000.0;
    r.rebirth_count = 32;
    r.max_job_levels[WorkTypes::Fields as usize] = 80;
    r.max_job_levels[WorkTypes::Mill as usize] = 80;
    r.max_job_levels[WorkTypes::Weaver as usize] = 80;
    r.max_job_levels[WorkTypes::Fisherman as usize] = 80;
    r.max_job_levels[WorkTypes::Farmer as usize] = 80;
    r.rebirth_upgrades[RebirthUpgradeTypes::AcceptingDeath as usize].is_purchased = true;
    r.rebirth_upgrades[RebirthUpgradeTypes::TheDivine as usize].is_purchased = true;

    state = rebirth(world, state.rebirth_stats.clone());
    state.items.money = 1_000_000_000.0;
    state.items.divine_favor = 50.0;

    let input = Input::new(&state, world);
    (state, input, meta_data)
}

fn make_saved_ticks(world: &World) -> (StateContainer, Input, MetaData) {
    let state = new_game(world);
    let mut meta_data = MetaData::new();
    meta_data.saved_ticks = 100_000.0;

    let input = Input::new(&state, world);
    (state, input, meta_data)
}
