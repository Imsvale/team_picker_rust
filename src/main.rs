// src/main.rs

mod player;
mod roster;
mod composition;
mod evaluator;
mod pick;
mod lineup;

use std::fs;
use std::io::{self, Write};
use std::path::Path;
use roster::read_roster;
use composition::parse_composition;
use pick::to_pick_data;
use lineup::optimize_lineup;


fn main() -> io::Result<()> {
    // Create default files if they don't exist
    check_files_exist()?;

    let players = read_roster("team_data.txt")?;
    let composition = parse_composition("composition.txt")?;

    let mut all_pick_data: Vec<_> = players.iter()
        .map(|p| to_pick_data(p, &composition))
        .collect();

    // Sort by best potential
    all_pick_data.sort_by(|a, b| b.max_score.partial_cmp(&a.max_score).unwrap());

    let team_size = composition.attacking.len();
    let initial = all_pick_data[..team_size].to_vec();

    let (optimized_team, _) = optimize_lineup(&all_pick_data, initial, &composition);

    
    let mut sorted_team = optimized_team;
    sorted_team.sort_by_key(|p| {
        let pos = &p.offense.position;
        let sort_key = match pos.as_str() {
            "RN" => 0,
            "GN" => 1,
            "BK" => 2,
            _ => 3,
        };
        // Pack into a tuple: first by role, then descending total_score
        (sort_key, -((p.total_score * 100.0) as i32))
    });
    
    // Find longest name. We'll use this to space things correctly.
    let longest_name = sorted_team.iter().map(|p| p.name.len()).max().unwrap_or(0);
    let padding = 3;
    
    // Headers
    println!("{:<10}{:<name_width$}{}", "Pos", "Name", "Score", name_width = longest_name + padding);
    
    let mut total_off = 0;
    let mut total_def = 0;
    
    for player in &sorted_team {
        let off_val = player.offense.score.round() as i32;
        let def_val = player.defense.score.round() as i32;
        let total = off_val + def_val;
        total_off += off_val;
        total_def += def_val;
    
        println!(
            "{} / {}   {:<name_width$}{:>2.0} + {:>2.0} = {:>3.0}",
            player.offense.position,
            player.defense.position,
            player.name,
            off_val,
            def_val,
            total,
            name_width = longest_name + padding
        );
    }
    
    println!("\n    Team total: {} + {} = {}", total_off, total_def, total_off + total_def);
    print!("\nPress 'Enter' to quit.");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();

    Ok(())
}

fn check_files_exist() -> io::Result<()> {
    if !Path::new("composition.txt").exists() {
        fs::write("composition.txt", DEFAULT_COMPOSITION)?;
    }

    if !Path::new("team_data.txt").exists() {
        fs::write("team_data.txt", DEFAULT_TEAM_DATA)?;
    }

    Ok(())
}

const DEFAULT_COMPOSITION: &str = "\
Offense: RN RN RN GN GN BK BK BK
Defense: DL DL DL CV CV LB LB LB

RN=max(HB,QB)
GN=GN
BK=BK
DL=DL
CV=CV
LB=LB
";

const DEFAULT_TEAM_DATA: &str = "\
Name XP TV OVR RN HB QB GN BK DL LB CV Spd Str Agl Stm Tck Blk Ddg BrB Hnd Pas Vis Bru Dur Sal
";