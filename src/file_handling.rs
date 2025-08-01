// src/file_handling.rs

use std::io::{BufReader, Error, ErrorKind, Result};
use std::fs::{self, File};
use std::path::Path;

pub fn open_file(path: &str) -> Result<BufReader<File>> {
    if !Path::new(path).exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("File not found: {}", path),
        ));
    }

    let file = File::open(path)?;
    println!("Loaded file: {}", path);
    Ok(BufReader::new(file))
}

pub fn file_exists(path: &str) -> Result<()> {
    if !Path::new(path).exists() {
        Err(Error::new(
            ErrorKind::NotFound,
            format!("File not found: {}", path),
        ))
    } else {
        Ok(())
    }
}

pub fn check_default_files_exist() -> Result<()> {

    let c = "composition.txt";
    let t = "team_data.txt";
    
    if let Err(_e) = file_exists(c) {
        println!("{} not found. Creating default file.", c);
        fs::write(c, DEFAULT_COMPOSITION)?;
    }

    if let Err(_e) = file_exists(t) {
        println!("{} not found. Creating default file.", t);
        fs::write(t, DEFAULT_TEAM_DATA)?;
        return Err(Error::new(
            ErrorKind::Other,
            format!("Please paste your team roster into {}. See the README for further details.", t)
        ));
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
LB=LB";

const DEFAULT_TEAM_DATA: &str = "\
Name XP TV OVR RN HB QB GN BK DL LB CV Spd Str Agl Stm Tck Blk Ddg BrB Hnd Pas Vis Bru Dur Sal
";