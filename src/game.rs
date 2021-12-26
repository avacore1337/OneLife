use crate::engine::intermediate_state::IntermediateState;
use crate::input::{Input, Recordable};
use crate::input_mapping::InputMapping;
use crate::meta::MetaData;
use crate::state::state_container::{new_game, StateContainer};
use crate::world_content::world::World;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub struct Game {
    pub input: Input,
    pub world: World,
    pub state: StateContainer,
    pub intermediate_state: IntermediateState,
    pub meta_data: MetaData,
    pub inputs: BTreeMap<u32, Vec<String>>,
    pub previous_inputs: BTreeMap<u32, Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameSave {
    pub input: Input,
    pub state: StateContainer,
    pub meta_data: MetaData,
    pub inputs: BTreeMap<u32, Vec<String>>,
    pub previous_inputs: BTreeMap<u32, Vec<String>>,
}

impl From<&Game> for GameSave {
    fn from(game: &Game) -> Self {
        GameSave {
            input: game.input,
            state: game.state.clone(),
            meta_data: game.meta_data.clone(),
            inputs: game.inputs.clone(),
            previous_inputs: game.inputs.clone(),
        }
    }
}

impl Game {
    pub fn new() -> Game {
        let world = World::default();
        let state = new_game(&world);
        let input = Input::new(&state, &world);
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
        self.state = new_game(&self.world);
        self.input = Input::new(&self.state, &self.world);
    }

    pub fn load_game(&mut self, save: GameSave) {
        self.input = save.input;
        self.state = save.state;
        self.meta_data = save.meta_data;
        self.inputs = save.inputs;
        self.previous_inputs = save.previous_inputs;
    }
}
