// src/player.rs

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub stats: HashMap<String, i32>,
}
