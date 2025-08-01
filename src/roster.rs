// src/roster.rs

use std::collections::HashMap;
use std::io::{BufRead, Result};

use crate::file_handling::open_file;
use crate::player::Player;

pub fn read_roster(path: &str) -> Result<Vec<Player>> {
    let reader = open_file(path)?;
    let mut lines = reader.lines().filter_map(Result::ok);

    let mut players = Vec::new();

    // Read and parse header line
    let header_line = loop {
        match lines.next() {
            Some(line) if !line.trim().is_empty() => break line,
            Some(_) => continue,
            None => return Ok(players), // empty or malformed file
        }
    };

    let mut header_fields: Vec<String> = header_line
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    if header_fields.first().map(|s| s.as_str()) != Some("Name") {
        panic!("First header column must be 'Name'");
    }
    header_fields.remove(0); // Remove "Name" column

    // Read player entries (two lines per player: name, stat line)
    while let Some(name_line) = lines.next() {
        let name = name_line.trim();

        if name.is_empty() {
            continue;
        }

        let name = name.strip_prefix("[CAPTAIN] ").unwrap_or(name).to_string();

        let stat_line = match lines.next() {
            Some(line) => line.trim().to_string(),
            None => break, // end of file
        };

        if stat_line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = stat_line.split('\t').collect();
        if parts.len() <= 1 {
            eprintln!("Warning: malformed stat line for player '{}'", name);
            continue;
        }

        let stat_tokens = &parts[1..]; // Skip metadata (first field)

        if stat_tokens.len() != header_fields.len() {
            eprintln!("Warning: stat count mismatch for player '{}'", name);
            continue;
        }

        let mut stats = HashMap::new();
        for (field, value_str) in header_fields.iter().zip(stat_tokens.iter()) {
            if let Ok(value) = value_str.trim().parse::<i32>() {
                stats.insert(field.clone(), value);
            } else {
                eprintln!("Warning: invalid number '{}' for player '{}'", value_str, name);
                continue;
            }
        }

        players.push(Player { name, stats });
    }

    Ok(players)
}
