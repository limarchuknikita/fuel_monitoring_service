use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vessel {
    pub id: String,
    pub name: String,
    pub fuel_level: f32,
}