// src/composition.rs

use std::collections::HashMap;
use std::io::{BufRead, Result};
use crate::file_handling::open_file;

#[derive(Debug, Clone)]
pub struct PositionRequirements {
    pub attacking: Vec<String>,
    pub defensive: Vec<String>,
    pub position_to_calculation: HashMap<String, String>,
}

pub fn parse_composition(path: &str) -> Result<PositionRequirements> {
    let reader = open_file(path)?;

    let mut requirements = PositionRequirements {
        attacking: Vec::new(),
        defensive: Vec::new(),
        position_to_calculation: HashMap::new(),
    };

    for line in reader.lines() {
        let line = line?;
        let line = trim_comment(&line);

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((lhs, rhs)) = line.split_once('=') {

            // println!("Before: key='{}', value='{}'", lhs, rhs);

            let key = lhs.trim().to_string();
            let value = rhs.trim().to_string();

            // println!("After: key='{}', value='{}'", key, value);

            requirements.position_to_calculation.insert(key, value);
            continue;
        }

        if let Some((prefix, rest)) = line.split_once(':') {
            let target = match prefix.trim().to_lowercase().as_str() {
                "offense" => &mut requirements.attacking,
                "defense" => &mut requirements.defensive,
                _ => continue,
            };
            target.extend(rest.split_whitespace().map(|s| s.to_string()));
        }
    }

    Ok(requirements)
}

fn trim_comment(line: &str) -> &str {
    line
        // Trim # comment
        .split('#').next().unwrap()

        // Trim // comment
        .split("//").next().unwrap()

        // Trim ; comment
        .split(';').next().unwrap()

        // Trim whitespace
        .trim()
}
