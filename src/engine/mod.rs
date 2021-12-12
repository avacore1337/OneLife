pub mod auto_functions;
pub mod intermediate_state;
pub mod value_keys;

use crate::engine::value_keys::KeyValues;
use crate::game::Game;
use crate::input::activity::ActivityTypes;
use crate::input::blessing::BlessingTypes;
use crate::input::boost_item::BoostItemTypes;
use crate::input::housing::HousingTypes;
use crate::input::rebirth_upgrade::RebirthUpgradeTypes;
use crate::input::skill::SkillTypes;
use crate::input::stat::StatTypes;
use crate::input::tomb::TombTypes;
use crate::input::work::WorkTypes;
use crate::state::stats::Stat;
use crate::state::work::Work as StateWork;
use crate::world_content::activity::{
    should_be_visible_activity, should_unlock_activity, translate_activity,
};
use crate::world_content::blessing::{should_be_visible_blessing, should_unlock_blessing};
use crate::world_content::boost_item::{
    should_be_visible_boost_item, should_unlock_boost_item, translate_boost_item,
};
use crate::world_content::housing::translate_housing;
use crate::world_content::rebirth_upgrade::{
    should_be_visible_rebirth_upgrade, should_unlock_rebirth_upgrade, unlock,
};
use crate::world_content::skill::should_be_visible_skill;
use crate::world_content::stat::{get_stats_gains, should_be_visible_stat};
use crate::world_content::tomb::{should_be_visible_tomb, should_unlock_tomb};
use crate::world_content::work::{
    should_be_visible_work, should_unlock_work, translate_work, Work as WorkWorld,
};
use crate::TICK_RATE;
use intermediate_state::IntermediateState;
use strum::IntoEnumIterator;

use self::auto_functions::{auto_buy_item, auto_buy_tomb, auto_living, auto_work};

pub fn engine_run(game: &mut Game) {
    if game.state.life_stats.dead {
        return;
    }
    let run_count = game.meta_data.handle_run_count();
    for _ in 0..run_count {
        internal_run(game);
    }
}

fn internal_run(game: &mut Game) {
    game.intermediate_state = calculate_intermediate_state(game);
    let _old_state = game.state.clone(); //TODO use for delta?
    if game.state.life_stats.replaying {
        game.replay_input();
    }
    game.state.life_stats.current_tick += 1;

    auto_input_update(game);
    // Apply all modifiers to intermediate
    apply_housing(game);
    apply_items(game);
    apply_work(game);
    apply_activities(game);
    apply_tombs(game);
    apply_stats(game);

    apply_active_work(game);

    // Get the gains
    calculate_works_income(game);
    do_work(game.input.work, game);
    gain_work_xp(game);
    gain_stat_xp(game);

    // update frontend read values
    update_rebirth_unlocks(game);
    update_unlocks(game);
    update_life_stats(game);
}

fn auto_input_update(game: &mut Game) {
    if game.input.options.auto_work {
        auto_work(game);
    }
    if game.input.options.auto_living {
        auto_living(game);
    }
    if game.input.options.auto_buy_item {
        auto_buy_item(game);
    }
    if game.input.options.auto_buy_tomb {
        auto_buy_tomb(game);
    }
}

fn get_happiness(game: &Game) -> f64 {
    game.intermediate_state.get_multiplier(KeyValues::Happiness)
}

fn update_life_stats(game: &mut Game) {
    game.state.life_stats.happiness = get_happiness(game);
    let life_stats = &mut game.state.life_stats;
    let mut health_rate = 0.000_08 * game.intermediate_state.get_value(KeyValues::Health);
    if life_stats.health > 0.0 && health_rate > 0.0 {
        health_rate *= (1.0 - life_stats.health).powi(2);
    }
    life_stats.health_rate = health_rate;
    life_stats.health += health_rate / TICK_RATE;

    // Base gamespeed is that one life should take 30min for 0.0 health, the game runs in 30 ticks/second
    // Days/tick = total_days / (ticks in 30 min)
    // 52*365/(30*60*30) = 0.351
    let time_progression = 52.0 * 365.0 / (30.0 * 60.0 * TICK_RATE);
    life_stats.age += time_progression * game.state.rebirth_stats.time_factor;
    life_stats.lifespan = crate::BASE_LIFESPAN * (1.0 + life_stats.health);

    let should_die = life_stats.age + life_stats.health >= life_stats.lifespan;
    if should_die {
        life_stats.is_dying = true;
    }
}

fn update_rebirth_unlocks(game: &mut Game) {
    for rebirth_upgrade in RebirthUpgradeTypes::iter() {
        if game.state.rebirth_stats.rebirth_upgrades[rebirth_upgrade as usize].is_purchased {
            unlock(rebirth_upgrade, &mut game.state.rebirth_stats.unlocks);
        }
    }
}

fn update_unlocks(game: &mut Game) {
    for work in WorkTypes::iter() {
        game.state.works[work as usize].is_unlocked = should_unlock_work(work, game);
        game.state.works[work as usize].is_visible = should_be_visible_work(work, game);
    }
    for activity in ActivityTypes::iter() {
        game.state.activities[activity as usize].is_unlocked =
            should_unlock_activity(activity, game);
        game.state.activities[activity as usize].is_visible =
            should_be_visible_activity(activity, game);
    }
    for stat in StatTypes::iter() {
        game.state.stats[stat as usize].is_visible = should_be_visible_stat(stat, game);
    }
    for stat in SkillTypes::iter() {
        game.state.skills[stat as usize].is_visible = should_be_visible_skill(stat, game);
    }
    for rebirth_upgrade in RebirthUpgradeTypes::iter() {
        game.state.rebirth_stats.rebirth_upgrades[rebirth_upgrade as usize].is_unlocked =
            should_unlock_rebirth_upgrade(rebirth_upgrade, game);
        game.state.rebirth_stats.rebirth_upgrades[rebirth_upgrade as usize].is_visible =
            should_be_visible_rebirth_upgrade(rebirth_upgrade, game);
    }
    for tomb in TombTypes::iter() {
        game.state.tombs[tomb as usize].is_unlocked = should_unlock_tomb(tomb, game);
        game.state.tombs[tomb as usize].is_visible = should_be_visible_tomb(tomb, game);
    }
    for boost_item in BoostItemTypes::iter() {
        game.state.items.boost_items[boost_item as usize].is_unlocked =
            should_unlock_boost_item(boost_item, game);
        game.state.items.boost_items[boost_item as usize].is_visible =
            should_be_visible_boost_item(boost_item, game);
    }
    for blessing in BlessingTypes::iter() {
        game.state.blessings[blessing as usize].is_unlocked =
            should_unlock_blessing(blessing, game);
        game.state.blessings[blessing as usize].is_visible =
            should_be_visible_blessing(blessing, game);
    }
}

fn calculate_intermediate_state(_game: &Game) -> IntermediateState {
    IntermediateState::new()
}

pub fn character_death_update(game: &mut Game) {
    game.state.life_stats.dead = true;
    for (index, work) in game.state.works.iter().enumerate() {
        game.state.rebirth_stats.max_job_levels[index] =
            std::cmp::max(game.state.rebirth_stats.max_job_levels[index], work.level);
    }
    game.state.rebirth_stats.coins += game.intermediate_state.get_value(KeyValues::Coins);
}

fn apply_housing(game: &mut Game) {
    let mut housing = translate_housing(game.input.housing);
    if housing.upkeep > game.state.items.money {
        housing = translate_housing(HousingTypes::StoneFloor);
        // TODO signal to frontend that you are out of cash?
    }
    game.state.items.money -= housing.upkeep / TICK_RATE;
    game.intermediate_state.get_gains(&housing);
}

fn apply_items(game: &mut Game) {
    for item in game.state.items.boost_items {
        if item.is_purchased {
            let boost_item = translate_boost_item(item.name);
            game.intermediate_state.get_gains(&boost_item);
        }
    }
}

fn calculate_works_income(game: &mut Game) {
    for work_state in game.state.works.iter_mut() {
        work_state.effective_income = game.intermediate_state.get_value(work_state.name.into());
    }
}

fn apply_work(game: &mut Game) {
    for work_state in game.state.works.iter_mut() {
        let work = translate_work(work_state.name);

        let main_stat_level = game.state.stats[work.main_stat as usize].level;
        let stat_multiplier: f64 = 1.0 + (main_stat_level as f64 / 10.0);
        game.intermediate_state
            .add_multiplier(work.name.into(), stat_multiplier, "Stat");

        let level_multiplier: f64 = 1.0 + (work_state.level as f64 / 10.0);
        game.intermediate_state
            .add_multiplier(work.name.into(), level_multiplier, "Level");
        game.intermediate_state
            .set_base(work.name.into(), work.money);
    }
}
fn apply_active_work(game: &mut Game) {
    let work = translate_work(game.input.work);
    game.intermediate_state.get_gains(&work);
}

fn apply_activities(game: &mut Game) {
    let activity = translate_activity(game.input.activity);
    game.intermediate_state.get_gains(&activity);
}

fn apply_tombs(game: &mut Game) {
    for tomb in &game.state.tombs {
        if tomb.is_purchased {
            let world_tomb = game.world.tombs[tomb.name as usize].clone();
            game.intermediate_state.get_gains(&world_tomb);
        }
    }
}

fn apply_stats(game: &mut Game) {
    for stat in StatTypes::iter() {
        get_stats_gains(stat, game);
    }
}

fn gain_work_xp(game: &mut Game) {
    let input_work = game.input.work as usize;
    let work: &mut StateWork = &mut game.state.works[input_work];
    let work_world = &game.world.works[input_work];
    work.next_level_progress += 10.0
        * game.state.life_stats.happiness
        * (1.0 + f64::sqrt(game.state.rebirth_stats.max_job_levels[input_work] as f64))
        / TICK_RATE;
    let mut next_level_xp_needed = calculate_work_next_level_xp_neeeded(work, work_world);
    while work.next_level_progress > next_level_xp_needed {
        work.level += 1;
        work.next_level_progress -= next_level_xp_needed;
        next_level_xp_needed = calculate_work_next_level_xp_neeeded(work, work_world);
    }
    work.next_level_required = next_level_xp_needed;
    work.next_level_percentage = (work.next_level_progress * 100.0) / next_level_xp_needed;
}

fn gain_stat_xp(game: &mut Game) {
    let int_level = game.state.stats[StatTypes::Int as usize].level;
    let level_multiplier = 1.0 + (int_level as f64 / 10.0);
    for stat_type in StatTypes::iter() {
        let stat_xp = game.intermediate_state.get_value(stat_type.into()) * level_multiplier;
        let stat: &mut Stat = &mut game.state.stats[stat_type as usize];
        stat.xp_rate = stat_xp;
        stat.next_level_progress += stat_xp / TICK_RATE;
        let mut next_level_xp_needed = calculate_stat_next_level_xp_neeeded(stat);
        while stat.next_level_progress > next_level_xp_needed {
            stat.level += 1.0;
            stat.next_level_progress -= next_level_xp_needed;
            next_level_xp_needed = calculate_stat_next_level_xp_neeeded(stat);
        }
        stat.next_level_required = next_level_xp_needed;
        stat.next_level_percentage = (stat.next_level_progress * 100.0) / next_level_xp_needed;
    }
}

fn calculate_stat_next_level_xp_neeeded(stat: &mut Stat) -> f64 {
    (100.0 + (4.0 * stat.level * stat.level)) as f64
}

fn calculate_work_next_level_xp_neeeded(work: &mut StateWork, work_world: &WorkWorld) -> f64 {
    (100 + (4 * work.level * work.level)) as f64 * work_world.xp_req_modifier
}

fn do_work(input_work: WorkTypes, game: &mut Game) {
    let income = game.intermediate_state.get_value(input_work.into());
    game.state.items.income = income;
    game.state.items.money += income / TICK_RATE;
}
