use crate::input::rebirth_upgrade::RebirthUpgradeTypes;
use crate::input::work::WorkTypes;
use crate::input::Input;
use crate::meta::MetaData;
use crate::state::rebirth_stats::RebirthStats;
use crate::state::state_container::{new_game, rebirth, StateContainer};
use crate::world_content::rebirth_upgrade::translate_rebirth_upgrade;
use crate::world_content::work::translate_work;
use crate::world_content::world::World;
use std::collections::BTreeMap;
use strum::IntoEnumIterator;

fn get_all_upgrades_up_to_current_tier(rebirth_stats: &mut RebirthStats) {
    let tier = rebirth_stats.class_tier;
    for upgrade in rebirth_stats.rebirth_upgrades.iter_mut() {
        let upgrade_world = translate_rebirth_upgrade(upgrade.name);
        if upgrade_world.required_tier < tier {
            upgrade.is_purchased = true;
        }
    }
}

fn set_lower_tier_jobs_to(rebirth_stats: &mut RebirthStats, level: u32) {
    let tier = rebirth_stats.class_tier;
    for work in WorkTypes::iter() {
        let work_world = translate_work(work);
        if work_world.required_tier < tier {
            rebirth_stats.max_job_levels[work as usize] = level;
        }
    }
}

pub fn get_presets(world: &World) -> BTreeMap<&'static str, (StateContainer, Input, MetaData)> {
    let mut presets = BTreeMap::new();
    presets.insert("1: T1 expected", make_t1(world));
    presets.insert("2: T2 expected", make_t2(world));
    presets.insert("3: T3 expected", make_t3(world));
    presets.insert("4: T4 expected", make_t4(world));
    presets.insert("5: T5 expected", make_t5(world));
    presets.insert("6: T5 Faith", make_t5_faith(world));
    presets.insert("7: saved ticks", make_saved_ticks(world));
    presets.insert("8: poor death", make_poor_death(world));
    presets.insert("9: rich death", make_rich_death(world));

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

fn make_t1(world: &World) -> (StateContainer, Input, MetaData) {
    let mut state = new_game(world);
    let meta_data = MetaData::new();
    let r = &mut state.rebirth_stats;
    r.class_tier = 1;
    get_all_upgrades_up_to_current_tier(r);
    r.coins = 0.0;
    r.rebirth_count = 2;

    set_lower_tier_jobs_to(r, 20);

    state = rebirth(world, state.rebirth_stats.clone());

    let input = Input::new(&state, world);
    (state, input, meta_data)
}

fn make_t2(world: &World) -> (StateContainer, Input, MetaData) {
    let mut state = new_game(world);
    let meta_data = MetaData::new();
    let r = &mut state.rebirth_stats;
    r.class_tier = 2;
    get_all_upgrades_up_to_current_tier(r);
    r.coins = 8.0;
    r.rebirth_count = 8;

    set_lower_tier_jobs_to(r, 30);

    state = rebirth(world, state.rebirth_stats.clone());

    let input = Input::new(&state, world);
    (state, input, meta_data)
}

fn make_t3(world: &World) -> (StateContainer, Input, MetaData) {
    let mut state = new_game(world);
    let meta_data = MetaData::new();
    let r = &mut state.rebirth_stats;
    r.class_tier = 3;
    get_all_upgrades_up_to_current_tier(r);
    r.coins = 100.0;
    r.rebirth_count = 16;

    set_lower_tier_jobs_to(r, 50);

    state = rebirth(world, state.rebirth_stats.clone());

    let input = Input::new(&state, world);
    (state, input, meta_data)
}
pub fn make_t4(world: &World) -> (StateContainer, Input, MetaData) {
    let mut state = new_game(world);
    let meta_data = MetaData::new();
    let r = &mut state.rebirth_stats;
    r.class_tier = 4;
    get_all_upgrades_up_to_current_tier(r);
    r.coins = 1000.0;
    r.rebirth_count = 16;

    set_lower_tier_jobs_to(r, 70);

    state = rebirth(world, state.rebirth_stats.clone());

    let input = Input::new(&state, world);
    (state, input, meta_data)
}

fn make_t5(world: &World) -> (StateContainer, Input, MetaData) {
    let mut state = new_game(world);
    let meta_data = MetaData::new();
    let r = &mut state.rebirth_stats;
    r.class_tier = 5;
    get_all_upgrades_up_to_current_tier(r);
    r.coins = 10000.0;
    r.rebirth_count = 32;

    set_lower_tier_jobs_to(r, 80);

    state = rebirth(world, state.rebirth_stats.clone());

    let input = Input::new(&state, world);
    (state, input, meta_data)
}

fn make_t5_faith(world: &World) -> (StateContainer, Input, MetaData) {
    let mut state = new_game(world);
    let meta_data = MetaData::new();
    let r = &mut state.rebirth_stats;
    r.class_tier = 5;
    get_all_upgrades_up_to_current_tier(r);
    r.coins = 8000.0;
    r.rebirth_count = 32;

    set_lower_tier_jobs_to(r, 80);
    r.rebirth_upgrades[RebirthUpgradeTypes::TheDivine as usize].is_purchased = true;

    state = rebirth(world, state.rebirth_stats.clone());
    state.items.money = 1.0e9;
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
