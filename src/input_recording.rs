use crate::input::Recordable;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Clone, Debug)]
pub struct RecordedInputEntry {
    pub id: u32,
    pub tick: u32,
    pub name: String,
}

impl From<&Inputs> for Vec<RecordedInputEntry> {
    fn from(inputs: &Inputs) -> Self {
        let mut ret = vec![];
        for (key, entries) in inputs.mapping.iter() {
            for entry in entries.iter() {
                ret.push(RecordedInputEntry {
                    tick: *key,
                    id: entry.id,
                    name: entry.name.clone(),
                });
            }
        }
        ret
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimedInput {
    pub id: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Inputs {
    pub mapping: BTreeMap<u32, Vec<TimedInput>>,
    current_id: u32,
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

    pub fn remove(&mut self, id: u32) -> Result<()> {
        for inputs in self.mapping.values_mut() {
            let mut found: Option<usize> = None;
            for (i, val) in inputs.iter().enumerate() {
                if val.id == id {
                    found = Some(i);
                }
            }
            if let Some(i) = found {
                inputs.swap_remove(i);
                return Ok(());
            }
        }
        Err(anyhow!("No entry found"))
    }

    pub fn clear(&mut self) {
        self.mapping.clear()
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
