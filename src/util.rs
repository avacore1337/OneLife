use crate::game::Inputs;
use crate::input::options::Options;
use crate::input::Recordable;
use crate::state::rebirth_stats::RebirthStats;
use crate::world_content::rebirth_upgrade::translate_rebirth_upgrade;

pub fn get_all_upgrades_up_to_current_tier(rebirth_stats: &mut RebirthStats) {
    get_all_upgrades_up_to_tier(rebirth_stats, rebirth_stats.class_tier)
}

pub fn get_all_upgrades_up_to_tier(rebirth_stats: &mut RebirthStats, tier: u32) {
    for upgrade in rebirth_stats.rebirth_upgrades.iter_mut() {
        let upgrade_world = translate_rebirth_upgrade(upgrade.name);
        if upgrade_world.required_tier < tier {
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

pub fn register_input<T: Recordable>(inputs: &mut Inputs, tick: u32, key: T) {
    inputs.entry(tick).or_default().push(key.to_record_key());
}
