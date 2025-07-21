mod player;
mod roster;
mod composition;
mod evaluator;
mod pick;
mod lineup;

use player::Player;
use roster::read_roster;
use composition::parse_composition;
use pick::to_pick_data;
use lineup::{get_initial_lineup, optimize_lineup, StartingPosition};

fn main() -> std::io::Result<()> {
    let players = read_roster("team_data.txt")?;
    let composition = parse_composition("composition.txt")?;

    println!("Loaded {} players", players.len());
    for p in &players {
        println!("- {}", p.name);
    }
    println!("Offensive positions: {:?}", composition.attacking);
    println!("Defensive positions: {:?}", composition.defensive);
    println!("Position formulas:");
    for (pos, expr) in &composition.position_to_calculation {
        println!("  {pos} = {expr}");
    }
    

    let mut all_pick_data: Vec<_> = players.iter()
        .map(|p| to_pick_data(p, &composition))
        .collect();

    // Sort by max total score
    all_pick_data.sort_by(|a, b| b.max_score.partial_cmp(&a.max_score).unwrap());

    // Get initial N-best
    let team_size = composition.attacking.len();
    let initial = all_pick_data[..team_size].to_vec();

    let (optimized_team, total_score) = optimize_lineup(&all_pick_data, initial, &composition);

    println!("Best team (total score = {:.2}):", total_score);
    println!("{:<15} {:<8} {:<8} {:>6}", "Name", "Offense", "Defense", "Score");

    let mut sorted_team = optimized_team.clone();
    sorted_team.sort_by(|a, b| b.total_score.partial_cmp(&a.total_score).unwrap());

    for p in sorted_team {
        println!("{:<15} {:<8} {:<8} {:>6.2}",
            p.name,
            p.offense.position,
            p.defense.position,
            p.total_score,
        );
    }

    Ok(())
}
