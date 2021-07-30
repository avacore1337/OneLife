use crate::game::Game;
use crate::input::work::Work;
use crate::state::state_container::StateContainer;
use crate::world::work::translate_work;
pub fn engine_run(game: &mut Game) {
    let mut new_state = game.state.clone();
    do_work(&game.input.work, &mut new_state);

    game.state = new_state;
}

pub fn do_work(input_work: &Work, state: &mut StateContainer) {
    let work = translate_work(input_work);
    state.items.coins += work.money;
}
