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
