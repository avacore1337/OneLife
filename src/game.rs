use crate::engine::intermediate_state::IntermediateState;
use crate::input::Input;
use crate::state::state_container::{new_game, StateContainer};
use crate::world::world::World;

pub struct Game {
    pub input: Input,
    pub world: World,
    pub state: StateContainer,
    pub intermediate_state: IntermediateState,
}

impl Game {
    pub fn new() -> Game {
        let world = World::new();
        let state = new_game(&world);
        let input = Input::new();
        let intermediate_state = IntermediateState::new();
        Game {
            world,
            state,
            input,
            intermediate_state,
        }
    }
    pub fn hard_reset(self: &mut Self) {
        self.state = new_game(&self.world);
        self.input = Input::new();
    }
}
