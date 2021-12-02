use crate::input::Input;
use crate::meta::MetaData;
use crate::state::state_container::{new_game, rebirth, StateContainer};
use crate::world_content::world::World;
use std::collections::BTreeMap;

pub fn get_presets(world: &World) -> BTreeMap<&'static str, (StateContainer, Input, MetaData)> {
    let mut presets = BTreeMap::new();
    presets.insert("1: poor death", make_poor_death(world));
    presets.insert("2: rich death", make_rich_death(world));
    presets.insert("3: full unlock", make_full_unlock(world));
    presets.insert("4: saved ticks", make_saved_ticks(world));

    presets
}

fn make_poor_death(world: &World) -> (StateContainer, Input, MetaData) {
    let input = Input::new();
    let mut state = new_game(world);
    let meta_data = MetaData::new();
    state.items.money = 10.0;
    state.life_stats.age = 70.0 * 365.0;

    (state, input, meta_data)
}

fn make_rich_death(world: &World) -> (StateContainer, Input, MetaData) {
    let input = Input::new();
    let mut state = new_game(world);
    let meta_data = MetaData::new();
    state.items.money = 10000000.0;
    state.life_stats.age = 70.0 * 365.0;

    (state, input, meta_data)
}

fn make_full_unlock(world: &World) -> (StateContainer, Input, MetaData) {
    let input = Input::new();
    let mut state = new_game(world);
    let meta_data = MetaData::new();
    state.rebirth_stats.class_tier = 3;
    state.rebirth_stats.coins = 8.0;
    state = rebirth(world, state.rebirth_stats.clone());
    state.items.money = 10000000.0;

    (state, input, meta_data)
}

fn make_saved_ticks(world: &World) -> (StateContainer, Input, MetaData) {
    let input = Input::new();
    let state = new_game(world);
    let mut meta_data = MetaData::new();
    meta_data.saved_ticks = 100_000.0;

    (state, input, meta_data)
}
