use crate::input::Input;
use crate::state::state_container::{new_game, StateContainer};
use crate::world::world::World;
use std::collections::HashMap;

pub fn get_presets(world: &World) -> HashMap<&'static str, (StateContainer, Input)> {
    let mut presets = HashMap::new();
    presets.insert("poor death", make_poor_death(world));
    presets.insert("rich death", make_rich_death(world));

    presets
}

fn make_poor_death(world: &World) -> (StateContainer, Input) {
    let input = Input::new();
    let mut state = new_game(world);
    state.items.money = 10.0;
    state.life_stats.age = 70.0 * 365.0;

    (state, input)
}

fn make_rich_death(world: &World) -> (StateContainer, Input) {
    let input = Input::new();
    let mut state = new_game(world);
    state.items.money = 10000000.0;
    state.life_stats.age = 70.0 * 365.0;

    (state, input)
}