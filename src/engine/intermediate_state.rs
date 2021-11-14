use super::value_keys::KeyValues;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct ValueGains {
    pub key: KeyValues,
    pub base_gain: f64,
    pub multipliers: Vec<Multiplier>,
}

impl ValueGains {
    pub fn new(name: KeyValues) -> ValueGains {
        ValueGains {
            key: name,
            base_gain: 1.0,
            multipliers: Vec::new(),
        }
    }

    pub fn calculate_value(&self) -> f64 {
        let sum = self
            .multipliers
            .iter()
            .fold(1.0, |acc: f64, elem: &Multiplier| acc * elem.factor);
        self.base_gain * sum
    }
}

#[derive(Serialize, Debug)]
pub struct Multiplier {
    factor: f64,
    source_descriptor: &'static str,
}

#[derive(Serialize, Debug)]
pub struct IntermediateState {
    pub value_gains: HashMap<KeyValues, ValueGains>,
}

impl IntermediateState {
    pub fn new() -> IntermediateState {
        IntermediateState {
            value_gains: HashMap::new(),
        }
    }

    pub fn get_value(&mut self, key: KeyValues) -> f64 {
        self.value_gains
            .get(&key)
            .map(|value_gains| value_gains.calculate_value())
            .unwrap_or(0.0)
    }

    pub fn get_gains<T: Gain>(&mut self, source: &T) {
        source.gain(self);
    }

    pub fn add_multiplier(&mut self, key: KeyValues, value: f64, source: &'static str) {
        let values = self
            .value_gains
            .entry(key)
            .or_insert_with(|| ValueGains::new(key));
        values.multipliers.push(Multiplier {
            factor: value,
            source_descriptor: source,
        });
    }

    pub fn set_base(&mut self, key: KeyValues, value: f64) {
        let values = self
            .value_gains
            .entry(key)
            .or_insert_with(|| ValueGains::new(key));
        values.base_gain = value;
    }
}

pub trait Gain {
    fn gain(&self, intermediate: &mut IntermediateState);
}
