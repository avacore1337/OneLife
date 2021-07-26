use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RebirthStats {
    pub class_tier: u32,
}
