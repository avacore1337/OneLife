use crate::engine::intermediate_state::IntermediateState;
use crate::input::{Input, Recordable};
use crate::input_recording::Inputs;
use crate::meta::MetaData;
use crate::state::state_container::StateContainer;
use crate::world_content::world::World;
use crate::WORLD;
use serde::{Deserialize, Serialize};

pub struct Game {
    pub input: Input,
    pub world: &'static World,
    pub state: StateContainer,
    pub intermediate_state: IntermediateState,
    pub meta_data: MetaData,
    pub inputs: Inputs,
    pub previous_inputs: Inputs,
    pub just_loaded: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameSave {
    pub input: Input,
    pub state: StateContainer,
    pub meta_data: MetaData,
    pub inputs: Inputs,
    pub previous_inputs: Inputs,
}

impl Default for GameSave {
    fn default() -> GameSave {
        Game::new().into()
    }
}

impl From<&Game> for GameSave {
    fn from(game: &Game) -> Self {
        GameSave {
            input: game.input.clone(),
            state: game.state.clone(),
            meta_data: game.meta_data.clone(),
            inputs: game.inputs.clone(),
            previous_inputs: game.previous_inputs.clone(),
        }
    }
}

impl From<Game> for GameSave {
    fn from(game: Game) -> Self {
        GameSave {
            input: game.input,
            state: game.state,
            meta_data: game.meta_data,
            inputs: game.inputs,
            previous_inputs: game.previous_inputs,
        }
    }
}

impl Default for Game {
    fn default() -> Game {
        Game::new()
    }
}

impl Game {
    pub fn new() -> Game {
        let world = &WORLD;
        let state = StateContainer::default();
        let input = Input::new(&state);
        let intermediate_state = IntermediateState::new();
        let meta_data = MetaData::new();
        let inputs = Inputs::default();
        let previous_inputs = Inputs::default();
        Game {
            world,
            state,
            input,
            intermediate_state,
            meta_data,
            inputs,
            previous_inputs,
            just_loaded: false,
        }
    }

    pub fn register_input<T: Recordable>(&mut self, key: T) {
        let tick = self.state.life_stats.current_tick;
        self.inputs.register_input_on_tick(tick, key);
    }

    pub fn replay_input(&mut self) {
        let tick = self.state.life_stats.current_tick;
        if let Some(inputs) = self.previous_inputs.get(&tick) {
            for input in inputs.iter() {
                // log::info!("tick: {}, action: {:?}", tick, inputs);
                if let Some(function) = self
                    .world
                    .input_mapping
                    .lock()
                    .unwrap()
                    .user_function
                    .get(input)
                {
                    function(self);
                }
            }
        }
    }

    pub fn hard_reset(&mut self) {
        self.state = StateContainer::default();
        self.input = Input::new(&self.state);
        self.intermediate_state = IntermediateState::new();
        self.meta_data = MetaData::new();
        self.inputs = Inputs::default();
        self.previous_inputs = Inputs::default();
    }

    pub fn load_game(&mut self, save: GameSave) {
        let GameSave {
            input,
            state,
            meta_data,
            inputs,
            previous_inputs,
        } = save;
        self.input = input;
        self.state = state;
        self.meta_data = meta_data;
        self.inputs = inputs;
        self.previous_inputs = previous_inputs;
        self.just_loaded = true;
    }
}
