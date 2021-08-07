use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct ValueGains {
    name: &'static str,
    base_gain: f64,
    multipliers: Vec<Multiplier>,
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
