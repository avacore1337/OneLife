use crate::input::Recordable;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimedInput {
    pub id: u64, // Will have precision loss at higher numbers
    pub name: String,
}

type InputData = BTreeMap<u32, Vec<TimedInput>>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Inputs {
    mapping: InputData,
    current_id: u64,
}

impl Default for Inputs {
    fn default() -> Inputs {
        Inputs::new()
    }
}

impl Inputs {
    pub fn new() -> Inputs {
        Inputs {
            mapping: BTreeMap::new(),
            current_id: 0,
        }
    }

    pub fn register_input_on_tick<T: Recordable>(&mut self, tick: u32, key: T) {
        self.mapping.entry(tick).or_default().push(TimedInput {
            id: self.current_id,
            name: key.to_record_key(),
        });
        self.current_id += 1;
    }

    pub fn get(&self, tick: &u32) -> Option<Vec<String>> {
        self.mapping.get(tick).map(|inputs| {
            inputs
                .iter()
                .map(|input: &TimedInput| input.name.clone())
                .collect()
        })
    }

    // pub fn get_iter(&self, tick: &u32) -> impl Iterator<Item = String> + '_ {
    //     // self.mapping
    //     //     .get(tick)
    //     //     .map(|inputs| inputs.iter().map(|input: &TimedInput| input.name.clone()))
    //     //     .unwrap()
    //     match self.mapping.get(tick) {
    //         Some(inputs) => inputs.iter().map(|input: &TimedInput| input.name.clone()),
    //         None => std::iter::empty(),
    //     }
    // }
}
