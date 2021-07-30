use serde::{Deserialize, Serialize};

use crate::input::Input;
use crate::state::state_container::{new_game, StateContainer};
use crate::world::world::World;
#[derive(Serialize, Deserialize)]
pub struct Game {
    pub input: Input,
    pub world: World,
    pub state: StateContainer,
}

impl Game {
    pub fn new() -> Game {
        let world = World::new();
        let state = new_game(&world);
        let input = Input::new();
        Game {
            world,
            state,
            input,
        }
    }
    pub fn hard_reset(self: &mut Self) {
        self.state = new_game(&self.world);
        self.input = Input::new();
    }
}
