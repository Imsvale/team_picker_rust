// src/cli.rs
use crate::file_handling::file_exists;

pub struct Config {
    pub team_file: String,
    pub comp_file: String,
    pub using_defaults: bool,
}

pub enum ArgParseResult {
    Exit,
    Config(Config),
}

static VALID_FLAGS: &[&str] = &[
    "-h", "--help", 
    "-c", "--composition",
    "-t", "--team-data"
];

pub fn print_help() {
    println!(
        "Usage: team_picker [-h] [-c <composition_file>] [-t <team_data_file>]

Options:
  -c, --composition <file>      Path to composition file
  -t, --team-data <file>        Path to team data file
  -h, --help                    Show this help text"
    );
}

pub fn argument_error(msg: &str) -> ArgParseResult {
    eprintln!("Error: {msg}");
    print_help();
    ArgParseResult::Exit
}

pub fn from_args() -> ArgParseResult {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let parser = ArgParser::new(args, VALID_FLAGS);

    if parser.has_flag("-h") || parser.has_flag("--help") {
        print_help();
        return ArgParseResult::Exit;
    }

    if let Some(bad_arg) = parser.check_unrecognized() {
        return argument_error(&format!("Unrecognized argument: {bad_arg}"));
    }

    let comp_user_specified = parser.value_of(&["-c", "--composition"]).is_some();
    let team_user_specified = parser.value_of(&["-t", "--team-data"]).is_some();

    let comp_file = parser.value_of(&["-c", "--composition"]).unwrap_or("composition.txt");
    let team_file = parser.value_of(&["-t", "--team-data"]).unwrap_or("team_data.txt");

    if comp_user_specified && file_exists(comp_file).is_err() {
        return argument_error(&format!("Composition file not found: {}", comp_file));
    }

    if team_user_specified && file_exists(team_file).is_err() {
        return argument_error(&format!("Team data file not found: {}", team_file));
    }

    let using_defaults = 
        !team_user_specified && 
        !comp_user_specified &&
        team_file == "team_data.txt" &&
        comp_file == "composition.txt";

    ArgParseResult::Config(Config {
        team_file: team_file.to_string(),
        comp_file: comp_file.to_string(),
        using_defaults
    })
}

pub struct ArgParser { 
    args: Vec<String>,
    valid_flags: &'static [&'static str],
}

impl ArgParser {
    pub fn new(args: Vec<String>, valid_flags: &'static [&'static str]) -> Self {
        Self { 
            args,
            valid_flags,
        }
    }

    pub fn has_flag(&self, flag: &str) -> bool {
        self.args.contains(&flag.to_string())
    }

    pub fn value_of(&self, keys: &[&str]) -> Option<&str> {
        for (i, arg) in self.args.iter().enumerate() {
            if keys.contains(&arg.as_str()) {
                let next = self.args.get(i + 1)?;
                if !next.starts_with('-') {
                    return Some(next);
                }
            }
        }
        None
    }

    pub fn check_unrecognized(&self) -> Option<String> {
        for arg in &self.args {
            if arg.starts_with('-') && !self.valid_flags.contains(&&arg.as_str()) {
                return Some(arg.clone());
            }
        }
        None
    }
}

