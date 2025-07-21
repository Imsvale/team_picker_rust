// src/pick.rs

use std::collections::HashMap;
use crate::player::Player;
use crate::composition::PositionRequirements;
use crate::evaluator::{evaluate_position, EvalResult};

#[derive(Debug, Clone)]
pub struct PickTempData {
    pub name: String,
    pub position_scores: HashMap<String, f64>,
    pub max_score: f64,
}

pub fn to_pick_data(player: &Player, reqs: &PositionRequirements) -> PickTempData {
    let mut scores = HashMap::new();
    let mut max_offense = f64::MIN;
    let mut max_defense = f64::MIN;

    let mut add_scores = |positions: &[String], max: &mut f64| {
        for pos in positions {
            if !scores.contains_key(pos) {
                match evaluate_position(player, pos, reqs) {
                    Ok(score) => {
                        scores.insert(pos.clone(), score);
                        *max = f64::max(*max, score);
                    }
                    Err(err) => {
                        eprintln!("Error evaluating {} for {}: {:?}", pos, player.name, err);
                        scores.insert(pos.clone(), 0.0);
                    }
                }
            }
        }
    };

    add_scores(&reqs.attacking, &mut max_offense);
    add_scores(&reqs.defensive, &mut max_defense);

    PickTempData {
        name: player.name.clone(),
        position_scores: scores,
        max_score: max_offense + max_defense,
    }
}
