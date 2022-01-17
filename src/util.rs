use crate::engine::{character_death_update, engine_run};
use crate::game::Game;
use crate::input::activity::ActivityTypes;
use crate::input::options::Options;
use crate::input::work::WorkTypes;
use crate::input::Input;
use crate::input_recording::Inputs;
use crate::state::rebirth_stats::RebirthStats;
use crate::state::state_container::rebirth;
use crate::WORLD;
use strum::IntoEnumIterator;

pub fn get_upgrades_up_to_current_tier(rebirth_stats: &mut RebirthStats) {
    get_upgrades_up_to_tier(rebirth_stats, rebirth_stats.tier)
}

pub fn run_until_dead(game: &mut Game) {
    while !game.state.life_stats.is_dying {
        engine_run(game);
    }
    character_death_update(game);
    assert!(game.state.life_stats.dead);
}

pub fn do_test_rebirth(game: &mut Game) {
    game.state.rebirth_stats.rebirth_count += 1;
    game.state = rebirth(game.state.rebirth_stats.clone());
    game.input = Input::new(&game.state);
}

pub fn get_upgrades_up_to_tier(rebirth_stats: &mut RebirthStats, min_tier: u32) {
    get_upgrades_up_to_tier_max_cost(rebirth_stats, min_tier, 1.0e100)
}

pub fn get_upgrades_by_max_cost(rebirth_stats: &mut RebirthStats, max_cost: f64) {
    get_upgrades_up_to_tier_max_cost(rebirth_stats, 99, max_cost)
}

pub fn get_upgrades_up_to_tier_max_cost(
    rebirth_stats: &mut RebirthStats,
    min_tier: u32,
    max_cost: f64,
) {
    for upgrade in rebirth_stats.rebirth_upgrades.iter_mut() {
        let upgrade_world = WORLD.get_rebirth_upgrade(upgrade.name);
        if upgrade_world.required_tier < min_tier && upgrade_world.purchasing_cost <= max_cost {
            upgrade.is_purchased = true;
        }
    }
}

pub fn set_full_auto(options: &mut Options) {
    options.auto_work = true;
    options.auto_living = true;
    options.auto_buy_item = true;
    options.auto_buy_tomb = true;
}

pub fn balance_activities(
    inputs: &mut Inputs,
    start_tick: u32,
    end_tick: u32,
    activities: &[ActivityTypes],
) {
    let increment = 200;
    let mut tick = start_tick;
    let mut i = 0usize;
    while tick < end_tick - increment {
        inputs.register_input_on_tick(tick, activities[i % activities.len()]);
        tick += increment;
        i += 1;
    }
}

pub fn get_training_study_array() -> [ActivityTypes; 3] {
    [
        ActivityTypes::Training,
        ActivityTypes::Studying,
        ActivityTypes::Training,
    ]
}

pub fn get_run_study_array() -> [ActivityTypes; 3] {
    [
        ActivityTypes::Run,
        ActivityTypes::Studying,
        ActivityTypes::Run,
    ]
}

pub fn set_lower_tier_jobs_to(rebirth_stats: &mut RebirthStats, level: u32) {
    let tier = rebirth_stats.tier;
    for work in WorkTypes::iter() {
        let work_world = WORLD.get_work(work);
        if work_world.required_tier < tier {
            rebirth_stats.max_job_levels[work as usize] = level;
        }
    }
}

pub fn set_jobs_at_tier_to(rebirth_stats: &mut RebirthStats, tier: u32, level: u32) {
    for work in WorkTypes::iter() {
        let work_world = WORLD.get_work(work);
        if work_world.required_tier == tier {
            rebirth_stats.max_job_levels[work as usize] = level;
        }
    }
}
