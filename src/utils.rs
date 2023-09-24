use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;

// color codes
pub const COLOR_RED: &'static str = "\x1B[31m";
pub const COLOR_GREEN: &'static str = "\x1B[32m";

// text modes
pub const TEXTMODE_BOLD: &'static str = "\x1B[1m";
pub const TEXTMODE_DIM: &'static str = "\x1B[2m";
pub const TEXTMODE_UNDERLINE: &'static str = "\x1B[4m";

// reset styles
pub const RESET_STYLES: &'static str = "\x1B[0m";


// Top level struct to hold the TOML data.
#[derive(Deserialize)]
pub struct Data {
    pub config: Config,
}

// Config struct holds to data from the `[config]` section.
#[derive(Deserialize)]
pub struct Config {
    pub ignore: Vec<String>,
    pub ignore_beginning: Vec<String>,
}

pub fn parse_toml() -> Data {
    /*
    Parsing the toml config and returning the Data struct.
    Only finds the config if in the same dir as the executable.
    */
    let toml_filepath ="/Users/pbr/Documents/dev/simil/simil.toml";
    let contents = match fs::read_to_string(toml_filepath) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not find simil.toml in program directory.");
            exit(1);
        }
    };
    //println!("{}", contents);
    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{}Unable to read data from simil.toml ({e}){}", COLOR_RED, RESET_STYLES);
            exit(1);
        }
    };
    return data;
}