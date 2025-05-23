use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, JsonSchema, Serialize, Deserialize)]
pub struct Momentary {
    pub id: String,
    pub tag: Option<String>,
}

#[derive(Clone, JsonSchema, Serialize, Deserialize)]
pub struct Fader {
    pub level: f32,
    pub tag: Option<String>,
}
