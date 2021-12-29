use crate::game::GameSave;
use crate::input::activity::ActivityTypes;
use crate::input::options::AutoSettingTypes;
use crate::input::rebirth_upgrade::RebirthUpgradeTypes;
use crate::input::work::WorkTypes;
use crate::input::Input;
use crate::state::rebirth_stats::RebirthStats;
use crate::state::state_container::rebirth;
use crate::util::{
    balance_activities, get_all_upgrades_up_to_current_tier, register_input, set_full_auto,
};
use crate::world_content::work::translate_work;
use std::collections::BTreeMap;
use strum::IntoEnumIterator;

fn set_lower_tier_jobs_to(rebirth_stats: &mut RebirthStats, level: u32) {
    let tier = rebirth_stats.class_tier;
    for work in WorkTypes::iter() {
        let work_world = translate_work(work);
        if work_world.required_tier < tier {
            rebirth_stats.max_job_levels[work as usize] = level;
        }
    }
}

pub fn get_presets() -> BTreeMap<&'static str, GameSave> {
    let mut presets = BTreeMap::new();
    presets.insert("00: Current Testing", make_current());
    presets.insert("01: T1 expected", make_t1());
    presets.insert("02: T2 expected", make_t2());
    presets.insert("03: T3 expected", make_t3());
    presets.insert("04: T4 expected", make_t4());
    presets.insert("05: T5 expected", make_t5());
    presets.insert("06: T5 Faith", make_t5_faith());
    presets.insert("07: saved ticks", make_saved_ticks());
    presets.insert("08: poor death", make_poor_death());
    presets.insert("09: rich death", make_rich_death());
    presets.insert("10: billion coins t0", make_only_coins());
    // presets.insert("Test_0: first rebirth", first_rebirth());
    presets.insert("Test_1: second rebirth", second_rebirth());
    presets.insert("Test_2: third rebirth", third_rebirth());

    presets
}

fn make_current() -> GameSave {
    let mut game_save = GameSave::default();
    let meta_data = &mut game_save.meta_data;
    let r = &mut game_save.state.rebirth_stats;
    r.class_tier = 1;
    get_all_upgrades_up_to_current_tier(r);
    r.rebirth_upgrades[RebirthUpgradeTypes::Skills as usize].is_purchased = true;
    r.rebirth_upgrades[RebirthUpgradeTypes::AcceptingDeath as usize].is_purchased = true;
    r.rebirth_upgrades[RebirthUpgradeTypes::StartingFunds1 as usize].is_purchased = true;
    r.rebirth_upgrades[RebirthUpgradeTypes::Privilege1 as usize].is_purchased = true;
    r.coins = 12.0;
    r.rebirth_count = 6;

    meta_data.options.auto_work = true;
    meta_data.options.auto_living = true;
    meta_data.options.auto_buy_item = true;

    set_lower_tier_jobs_to(r, 30);
    r.max_job_levels[WorkTypes::Fisherman as usize] = 30;
    r.max_job_levels[WorkTypes::Weaver as usize] = 30;

    game_save.state = rebirth(r.clone());
    game_save
}

fn make_poor_death() -> GameSave {
    let mut game_save = GameSave::default();
    let state = &mut game_save.state;
    state.items.money = 10.0;
    state.life_stats.age = 70.0 * 365.0;

    game_save.input = Input::new(&game_save.state);
    game_save
}

fn make_rich_death() -> GameSave {
    let mut game_save = GameSave::default();
    let state = &mut game_save.state;
    let r = &mut state.rebirth_stats;
    r.class_tier = 4;
    get_all_upgrades_up_to_current_tier(r);
    set_lower_tier_jobs_to(r, 70);
    game_save.state = rebirth(r.clone());
    game_save.state.items.money = 1.0e9;
    game_save.state.life_stats.age = 70.0 * 365.0;

    game_save.input = Input::new(&game_save.state);
    game_save
}

fn make_t1() -> GameSave {
    let mut game_save = GameSave::default();
    let r = &mut game_save.state.rebirth_stats;
    r.class_tier = 1;
    get_all_upgrades_up_to_current_tier(r);
    r.coins = 0.0;
    r.rebirth_count = 2;

    set_lower_tier_jobs_to(r, 20);
    game_save.state = rebirth(r.clone());

    game_save.input = Input::new(&game_save.state);
    game_save
}

fn make_t2() -> GameSave {
    let mut game_save = GameSave::default();
    let state = &mut game_save.state;
    let r = &mut state.rebirth_stats;
    r.class_tier = 2;
    get_all_upgrades_up_to_current_tier(r);
    r.coins = 8.0;
    r.rebirth_count = 8;

    set_lower_tier_jobs_to(r, 30);

    game_save.state = rebirth(r.clone());

    game_save.input = Input::new(&game_save.state);
    game_save
}

fn make_t3() -> GameSave {
    let mut game_save = GameSave::default();
    let state = &mut game_save.state;
    let r = &mut state.rebirth_stats;
    r.class_tier = 3;
    get_all_upgrades_up_to_current_tier(r);
    r.coins = 100.0;
    r.rebirth_count = 16;

    set_lower_tier_jobs_to(r, 50);

    game_save.state = rebirth(r.clone());

    game_save.input = Input::new(&game_save.state);
    game_save
}
pub fn make_t4() -> GameSave {
    let mut game_save = GameSave::default();
    let state = &mut game_save.state;
    let r = &mut state.rebirth_stats;
    r.class_tier = 4;
    get_all_upgrades_up_to_current_tier(r);
    r.coins = 1000.0;
    r.rebirth_count = 16;

    set_lower_tier_jobs_to(r, 70);

    game_save.state = rebirth(r.clone());

    game_save.input = Input::new(&game_save.state);
    game_save
}

fn make_t5() -> GameSave {
    let mut game_save = GameSave::default();
    let state = &mut game_save.state;
    let r = &mut state.rebirth_stats;
    r.class_tier = 5;
    get_all_upgrades_up_to_current_tier(r);
    r.coins = 10000.0;
    r.rebirth_count = 32;

    set_lower_tier_jobs_to(r, 80);

    game_save.state = rebirth(r.clone());

    game_save.input = Input::new(&game_save.state);
    game_save
}

fn make_t5_faith() -> GameSave {
    let mut game_save = GameSave::default();
    let state = &mut game_save.state;
    let r = &mut state.rebirth_stats;
    r.class_tier = 5;
    get_all_upgrades_up_to_current_tier(r);
    r.coins = 8000.0;
    r.rebirth_count = 32;

    set_lower_tier_jobs_to(r, 80);
    r.rebirth_upgrades[RebirthUpgradeTypes::TheDivine as usize].is_purchased = true;

    game_save.state = rebirth(r.clone());
    game_save.state.items.money = 1.0e9;
    game_save.state.items.divine_favor = 50.0;

    game_save.input = Input::new(&game_save.state);
    game_save
}

fn make_saved_ticks() -> GameSave {
    let mut game_save = GameSave::default();
    let meta_data = &mut game_save.meta_data;
    meta_data.saved_ticks = 100_000.0;

    game_save.input = Input::new(&game_save.state);
    game_save
}

fn make_only_coins() -> GameSave {
    let mut game_save = GameSave::default();
    let state = &mut game_save.state;
    state.rebirth_stats.coins = 1.0e9;

    game_save.input = Input::new(&game_save.state);
    game_save
}

pub fn second_rebirth() -> GameSave {
    let mut game_save = GameSave::default();
    let state = &mut game_save.state;
    let r = &mut state.rebirth_stats;
    set_full_auto(&mut game_save.meta_data.options);
    state.life_stats.replaying = true;
    register_input(
        &mut game_save.previous_inputs,
        12000,
        AutoSettingTypes::AutoBuyItemFalse,
    ); // Registering for next round

    for work in WorkTypes::iter() {
        if work < WorkTypes::Mill {
            r.max_job_levels[work as usize] = 10;
        }
    }

    game_save.input = Input::new(&game_save.state);
    game_save
}

pub fn third_rebirth() -> GameSave {
    let mut game_save = GameSave::default();
    let r = &mut game_save.state.rebirth_stats;
    r.rebirth_count = 8;
    r.class_tier = 1;
    set_lower_tier_jobs_to(r, 25);
    game_save.state = rebirth(r.clone());

    let state = &mut game_save.state;

    set_full_auto(&mut game_save.meta_data.options);
    state.life_stats.replaying = true;
    register_input(
        &mut game_save.previous_inputs,
        30000,
        AutoSettingTypes::AutoBuyItemFalse,
    );
    balance_activities(
        &mut game_save.previous_inputs,
        4000,
        30_000,
        &[ActivityTypes::Run, ActivityTypes::Studying],
    );
    register_input(&mut game_save.previous_inputs, 30000, ActivityTypes::Flirt);

    game_save.input = Input::new(&game_save.state);
    game_save
}
