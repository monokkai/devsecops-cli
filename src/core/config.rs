use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ModuleConfig {
    pub enabled: bool,
    pub settings: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub version: String,
    pub modules: HashMap<String, ModuleConfig>,
}

// impl AppConfig {
//     pub fn load() -> Self {
//         // Loading from ... config.toml + env vars
//     }
// }
