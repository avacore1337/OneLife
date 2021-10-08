use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct ValueGains {
    pub name: &'static str,
    pub base_gain: f64,
    pub multipliers: Vec<Multiplier>,
}

impl ValueGains {
    pub fn calculate_value(self: &Self) -> f64 {
        let sum = self
            .multipliers
            .iter()
            .fold(1.0, |acc: f64, elem: &Multiplier| acc * elem.factor);
        self.base_gain * sum
    }
}

#[derive(Serialize)]
pub struct Multiplier {
    factor: f64,
    source_descriptor: &'static str,
}

#[derive(Serialize)]
pub struct IntermediateState {
    pub value_gains: HashMap<&'static str, ValueGains>,
}

impl IntermediateState {
    pub fn new() -> IntermediateState {
        IntermediateState {
            value_gains: HashMap::new(),
        }
    }
}
