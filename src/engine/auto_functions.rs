use crate::{
    game::Game,
    world_content::{
        boost_item::translate_boost_item, housing::translate_housing, work::translate_work,
    },
};

pub fn auto_work(game: &mut Game) {
    let current_work = translate_work(game.input.work);
    for work in game.state.works.iter() {
        let work_world = translate_work(work.name);
        let same_type = current_work.main_stat == work_world.main_stat;
        if work.name > current_work.name && same_type && work.is_unlocked && work.is_visible {
            game.input.work = work.name;
        }
    }
}

pub fn auto_living(game: &mut Game) {
    let current_housing = translate_housing(game.input.housing);
    for housing in game.state.housing.iter() {
        let housing_world = translate_housing(housing.name);
        let can_afford = housing_world.upkeep < game.state.items.income;
        let better_housing = housing.name > current_housing.name;
        if better_housing && housing.is_visible && can_afford {
            game.input.housing = housing.name;
        }
    }
}

pub fn auto_buy_item(game: &mut Game) {
    for item in game.state.items.boost_items.iter_mut() {
        let world_item = translate_boost_item(item.name);
        let can_afford = game.state.items.money >= world_item.purchasing_cost;
        if !item.is_purchased && can_afford && item.is_visible {
            item.is_purchased = true;
            game.state.items.money -= world_item.purchasing_cost;
        }
    }
}
