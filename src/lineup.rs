// src/lineup.rs

use crate::pick::PickTempData;

#[derive(Debug, Clone)]
pub struct PositionDescription {
    pub position: String,
    pub score: f64,
}

#[derive(Debug, Clone)]
pub struct StartingPosition {
    pub name: String,
    pub offense: PositionDescription,
    pub defense: PositionDescription,
    pub total_score: f64,
}

use std::collections::HashSet;

pub fn find_best_positions(
    players: &[PickTempData],
    positions: &[String],
) -> (Vec<(String, PositionDescription)>, f64) {
    assert!(players.len() == positions.len());

    if players.is_empty() {
        return (Vec::new(), 0.0);
    }

    let first = &players[0];
    let rest = &players[1..];

    let mut best_score = f64::MIN;
    let mut best_result = Vec::new();

    let mut tried = HashSet::new();

    for pos in positions {
        if tried.contains(pos) {
            continue;
        }
        tried.insert(pos.clone());

        if let Some(&score) = first.position_scores.get(pos) {
            let mut remaining = positions.to_vec();
            if let Some(index) = remaining.iter().position(|p| p == pos) {
                remaining.remove(index);
            }

            let (mut sub_result, sub_score) = find_best_positions(rest, &remaining);
            let total = score + sub_score;

            if total > best_score {
                let entry = (first.name.clone(), PositionDescription {
                    position: pos.clone(),
                    score,
                });
                best_result = sub_result;
                best_result.push(entry);
                best_score = total;
            }
        }
    }

    (best_result, best_score)
}

use crate::lineup::{StartingPosition, PositionDescription};

pub fn make_lineup(
    starters: &[PickTempData],
    offense: &[(String, PositionDescription)],
    defense: &[(String, PositionDescription)],
) -> Vec<StartingPosition> {
    starters.iter().map(|p| {
        let offense_pos = offense.iter().find(|(name, _)| name == &p.name).unwrap().1.clone();
        let defense_pos = defense.iter().find(|(name, _)| name == &p.name).unwrap().1.clone();

        StartingPosition {
            name: p.name.clone(),
            offense: offense_pos.clone(),
            defense: defense_pos.clone(),
            total_score: offense_pos.score + defense_pos.score,
        }
    }).collect()
}

use crate::composition::PositionRequirements;

pub fn get_initial_lineup(
    all: &[PickTempData],
    reqs: &PositionRequirements,
) -> (Vec<StartingPosition>, f64) {
    let team_size = reqs.attacking.len();
    assert_eq!(team_size, reqs.defensive.len());

    let starters = &all[..team_size];

    let (offense, offense_score) = find_best_positions(starters, &reqs.attacking);
    let (defense, defense_score) = find_best_positions(starters, &reqs.defensive);

    let total_score = offense_score + defense_score;
    let lineup = make_lineup(starters, &offense, &defense);

    (lineup, total_score)
}

use crate::pick::PickTempData;
use crate::composition::PositionRequirements;

pub fn optimize_lineup(
    all_players: &[PickTempData],
    initial_lineup: Vec<PickTempData>,
    reqs: &PositionRequirements,
) -> (Vec<StartingPosition>, f64) {
    let mut starters = initial_lineup;
    let mut best_score;

    // Initial evaluation
    let (mut best_lineup, mut score) = get_initial_lineup(&starters, reqs);
    best_score = score;

    let mut improved = true;

    while improved {
        improved = false;

        let starter_names: Vec<String> = starters.iter().map(|p| p.name.clone()).collect();

        for bench_player in all_players {
            if starter_names.contains(&bench_player.name) {
                continue;
            }

            for i in 0..starters.len() {
                let mut trial = starters.clone();
                trial[i] = bench_player.clone();

                let (trial_lineup, trial_score) = get_initial_lineup(&trial, reqs);

                if trial_score > best_score {
                    best_score = trial_score;
                    starters = trial;
                    best_lineup = trial_lineup;
                    improved = true;
                    break; // restart loop with new baseline
                }
            }

            if improved {
                break;
            }
        }
    }

    (best_lineup, best_score)
}
