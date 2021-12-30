use crate::engine::intermediate_state::IntermediateState;
use crate::input::{Input, Recordable};
use crate::input_mapping::InputMapping;
use crate::meta::MetaData;
use crate::state::state_container::StateContainer;
use crate::world_content::world::World;
use crate::WORLD;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type Inputs = BTreeMap<u32, Vec<String>>;

pub struct Game {
    pub input: Input,
    pub world: &'static World,
    pub state: StateContainer,
    pub intermediate_state: IntermediateState,
    pub meta_data: MetaData,
    pub inputs: Inputs,
    pub previous_inputs: Inputs,
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
            input: game.input,
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
        let inputs = BTreeMap::new();
        let previous_inputs = BTreeMap::new();
        Game {
            world,
            state,
            input,
            intermediate_state,
            meta_data,
            inputs,
            previous_inputs,
        }
    }

    pub fn register_input<T: Recordable>(&mut self, key: T) {
        let tick = self.state.life_stats.current_tick;
        self.inputs
            .entry(tick)
            .or_default()
            .push(key.to_record_key());
    }

    pub fn register_input_on_tick<T: Recordable>(&mut self, tick: u32, key: T) {
        self.inputs
            .entry(tick)
            .or_default()
            .push(key.to_record_key());
    }

    pub fn replay_input(&mut self) {
        let tick = self.state.life_stats.current_tick;
        // log::info!("{:?}", tick);
        if let Some(names) = self.previous_inputs.get(&tick) {
            log::info!("{:?}", names);
            for name in names.clone().iter() {
                let input_mapping = InputMapping::default();
                if let Some(function) = input_mapping.user_function.get(name) {
                    function(self);
                }
            }
        }
    }

    pub fn hard_reset(&mut self) {
        self.state = StateContainer::default();
        self.input = Input::new(&self.state);
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
    }
}
