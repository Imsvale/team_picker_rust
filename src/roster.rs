// src/roster.rs

use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;

use crate::player::Player;

pub fn read_roster(path: &str) -> std::io::Result<Vec<Player>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut players = Vec::new();

    // Read header line
    let header_line = loop {
        match lines.next() {
            Some(Ok(line)) if !line.trim().is_empty() => break line,
            Some(_) => continue,
            None => return Ok(players), // empty file
        }
    };

    let mut header_fields: Vec<String> = header_line
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    // Remove "Name"
    if header_fields.first().map(|s| s.as_str()) != Some("Name") {
        panic!("First header column must be 'Name'");
    }
    header_fields.remove(0);

    // Read player entries
    while let Some(Ok(name_line)) = lines.next() {
        let name = name_line.trim().to_string();
        if name.is_empty() {
            continue;
        }

        let meta_line = match lines.next() {
            Some(Ok(line)) => line,
            _ => break,
        };

        // Skip token line like "#27 ..."
        let stat_line = match lines.next() {
            Some(Ok(line)) => line,
            _ => break,
        };

        let mut stat_values = Vec::new();
        for token in stat_line.split_whitespace() {
            if let Ok(num) = token.parse::<i32>() {
                stat_values.push(num);
            }
        }

        if stat_values.len() != header_fields.len() {
            eprintln!("Warning: stat count mismatch for player '{}'", name);
            continue;
        }

        let stats: HashMap<String, i32> = header_fields
            .iter()
            .cloned()
            .zip(stat_values.into_iter())
            .collect();

        players.push(Player { name, stats });
    }

    Ok(players)
}
