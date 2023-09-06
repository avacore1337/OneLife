use crate::{
    game::Game,
    input::{boost_item::BoostItemTypes, options::AutoSettingTypes},
    world_content::boost_item::translate_boost_item,
    world_content::tomb::translate_tomb,
    WORLD,
};

pub fn auto_work(game: &mut Game) {
    let current_work = WORLD.get_work(game.input.work);
    for work in game.state.works.iter() {
        let work_world = WORLD.get_work(work.name);
        let same_type = current_work.work_type == work_world.work_type;
        if work.name > current_work.name && same_type && work.is_unlocked && work.is_visible {
            game.input.work = work.name;
        }
    }
}

pub fn auto_living(game: &mut Game) {
    let current_housing = WORLD.get_housing(game.input.housing);
    for housing in game.state.housing.iter() {
        let housing_world = WORLD.get_housing(housing.name);
        let can_afford = housing_world.upkeep < game.state.items.income;
        let better_housing = housing.name > current_housing.name;
        if better_housing && housing.is_unlocked && can_afford {
            game.input.housing = housing.name;
        }
    }
}

pub fn auto_buy_item(game: &mut Game) {
    for item in game.state.boost_items.iter_mut() {
        let world_item = translate_boost_item(item.name);
        let can_afford = game.state.items.money >= world_item.purchasing_cost;
        if !item.is_purchased && item.is_unlocked && item.is_visible && can_afford {
            item.is_purchased = true;
            game.state.items.money -= world_item.purchasing_cost;
        }
    }
}

pub fn auto_buy_blessing(game: &mut Game) {
    for blessing in game.state.blessings.iter_mut() {
        let can_afford = game.state.items.divine_favor >= blessing.next_level_cost;
        if blessing.is_unlocked && blessing.is_visible && can_afford {
            blessing.level += 1;
            game.state.items.divine_favor -= blessing.next_level_cost;
        }
    }
}

pub fn auto_buy_queued_item(game: &mut Game) {
    for item_type in game.input.item_queue.clone().into_iter() {
        let item_world = translate_boost_item(item_type);
        let item = &mut game.state.boost_items[item_type as usize];
        let can_afford = game.state.items.money >= item_world.purchasing_cost;
        if !item.is_purchased && item.is_unlocked && item.is_visible && can_afford {
            item.is_purchased = true;
            game.state.items.money -= item_world.purchasing_cost;
        } else {
            break;
        }
    }
    game.input
        .item_queue
        .retain(|item_type| !game.state.boost_items[*item_type as usize].is_purchased);
}

pub fn auto_buy_tomb(game: &mut Game) {
    for tomb in game.state.tombs.iter_mut() {
        let world_tomb = translate_tomb(tomb.name);
        let can_afford = game.state.items.money >= world_tomb.purchasing_cost;
        if !tomb.is_purchased && tomb.is_unlocked && tomb.is_visible && can_afford {
            tomb.is_purchased = true;
            game.state.items.money -= world_tomb.purchasing_cost;
        }
    }
}

pub fn register_auto_settings(game: &mut Game) {
    if game.meta_data.options.auto_work {
        game.register_input(AutoSettingTypes::AutoWorkTrue)
    } else {
        game.register_input(AutoSettingTypes::AutoWorkFalse)
    };
    if game.meta_data.options.auto_living {
        game.register_input(AutoSettingTypes::AutoLivingTrue)
    } else {
        game.register_input(AutoSettingTypes::AutoLivingFalse)
    };
    if game.meta_data.options.auto_buy_item {
        game.register_input(AutoSettingTypes::AutoBuyItemTrue)
    } else {
        game.register_input(AutoSettingTypes::AutoBuyItemFalse)
    };
    if game.meta_data.options.auto_buy_tomb {
        game.register_input(AutoSettingTypes::AutoBuyTombTrue)
    } else {
        game.register_input(AutoSettingTypes::AutoBuyTombFalse)
    };
}
