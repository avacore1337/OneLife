use super::value_keys::KeyValues;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct ValueGains {
    pub key: KeyValues,
    pub bases: Vec<Base>,
    pub multipliers: Vec<Multiplier>,
}

impl ValueGains {
    pub fn new(name: KeyValues) -> ValueGains {
        ValueGains {
            key: name,
            bases: Vec::new(),
            multipliers: Vec::new(),
        }
    }

    pub fn calculate_value(&self) -> f64 {
        let sum = self
            .multipliers
            .iter()
            .fold(1.0, |acc: f64, elem: &Multiplier| acc * elem.factor);
        let base: f64 = self.bases.iter().map(|b| b.base).sum();
        base * sum
    }

    pub fn calculate_muliplier(&self) -> f64 {
        self.multipliers
            .iter()
            .fold(1.0, |acc: f64, elem: &Multiplier| acc * elem.factor)
    }
}

#[derive(Serialize, Debug)]
pub struct Base {
    base: f64,
    source_descriptor: &'static str,
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

impl Default for IntermediateState {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn get_multiplier(&self, key: KeyValues) -> f64 {
        self.value_gains
            .get(&key)
            .map(|value_gains| value_gains.calculate_muliplier())
            .unwrap_or(1.0)
    }

    pub fn get_gains<T: Gain>(&mut self, source: &T) {
        source.gain(self);
    }

    pub fn add_multiplier(&mut self, key: KeyValues, factor: f64, source_descriptor: &'static str) {
        let values = self
            .value_gains
            .entry(key)
            .or_insert_with(|| ValueGains::new(key));
        values.multipliers.push(Multiplier {
            factor,
            source_descriptor,
        });
    }

    pub fn set_base(&mut self, key: KeyValues, base: f64, source_descriptor: &'static str) {
        let values = self
            .value_gains
            .entry(key)
            .or_insert_with(|| ValueGains::new(key));
        values.bases = vec![Base {
            base,
            source_descriptor,
        }];
    }

    pub fn add_base(&mut self, key: KeyValues, base: f64, source_descriptor: &'static str) {
        let values = self
            .value_gains
            .entry(key)
            .or_insert_with(|| ValueGains::new(key));
        values.bases.push(Base {
            base,
            source_descriptor,
        });
    }
}

pub trait Gain {
    fn gain(&self, intermediate: &mut IntermediateState);
}
