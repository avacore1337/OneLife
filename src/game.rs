use crate::engine::intermediate_state::IntermediateState;
use crate::input::Input;
use crate::meta::MetaData;
use crate::state::state_container::{new_game, StateContainer};
use crate::world::world::World;
use serde::{Deserialize, Serialize};
use std::sync::MutexGuard;

pub struct Game {
    pub input: Input,
    pub world: World,
    pub state: StateContainer,
    pub intermediate_state: IntermediateState,
    pub meta_data: MetaData,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameSave {
    pub input: Input,
    pub state: StateContainer,
    pub meta_data: MetaData,
}

impl From<MutexGuard<'_, Game>> for GameSave {
    fn from(game: MutexGuard<'_, Game>) -> Self {
        GameSave {
            input: game.input,
            state: game.state,
            meta_data: game.meta_data,
        }
    }
}

impl From<Game> for GameSave {
    fn from(game: Game) -> Self {
        GameSave {
            input: game.input,
            state: game.state,
            meta_data: game.meta_data,
        }
    }
}

impl Game {
    pub fn new() -> Game {
        let world = World::new();
        let state = new_game(&world);
        let input = Input::new();
        let intermediate_state = IntermediateState::new();
        let meta_data = MetaData::new();
        Game {
            world,
            state,
            input,
            intermediate_state,
            meta_data,
        }
    }

    pub fn hard_reset(self: &mut Self) {
        self.state = new_game(&self.world);
        self.input = Input::new();
    }

    pub fn load_game(self: &mut Self, save: GameSave) {
        self.input = save.input;
        self.state = save.state;
        self.meta_data = save.meta_data;
    }
}
