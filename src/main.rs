// src/main.rs
use std::io::{self, Write};

mod cli;
mod composition;
mod evaluator;
mod file_handling;
mod lineup;
mod pick;
mod player;
mod roster;
mod testing;

use cli::*;
use composition::parse_composition;
use lineup::optimize_lineup;
use pick::to_pick_data;
use roster::read_roster;
use file_handling::check_default_files_exist;

fn pause() {
    print!("\nPress 'Enter' to quit.");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn main() -> io::Result<()> {
    let config = match from_args() {
        ArgParseResult::Exit => return Ok(()),
        ArgParseResult::Config(config) => config,
    };
    
    // Create default files if they don't exist
    if config.using_defaults {
        if let Err(e) = check_default_files_exist() {
            println!("{e}");
            pause();
            return Ok(());
        }
    }

    let composition = parse_composition(&config.comp_file)?;
    let players = read_roster(&config.team_file)?;

    if players.len() < composition.attacking.len() {
        println!(
            "Not enough players in team_data.txt.\n\
            Found {}, but at least {} are required.",
            players.len(),
            composition.attacking.len()
        );
        println!("\nPlease paste your team roster into team_data.txt. See the README for further details.");
        pause();
        return Ok(());
    }

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
    pause();

    Ok(())
}