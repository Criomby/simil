use serde_derive::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;
use toml;
use std::env;

// color codes
pub const COLOR_RED: &'static str = "\x1B[31m";
pub const COLOR_GREEN: &'static str = "\x1B[32m";
pub const COLOR_YELLOW: &'static str = "\x1B[33m";

// text modes
pub const TEXTMODE_BOLD: &'static str = "\x1B[1m";
pub const TEXTMODE_DIM: &'static str = "\x1B[2m";
pub const TEXTMODE_UNDERLINE: &'static str = "\x1B[4m";

// reset styles
pub const RESET_STYLES: &'static str = "\x1B[0m";

const ACCEPTED_OPTIONS: [&'static str; 4] = [
        "--abspath",
        "--ignore-config",
        "--ignore-empty",
        "--trim",
    ];

const USAGE_STR: &str = "
Usage: simil [-h] [--abspath] file1 file2

positional arguments:
    file

options:
    -h, --help      Show this help message and exit
    -v, --version   Show version number and exit
    --abspath       Using absolute filepaths (relative to cwd by default)
    --ignore-config Do not use simil.toml config
        + --ignore-empty  Omit empty lines in output
        + --trim          Trim whitespace

";


// Top level struct to hold the TOML data.
#[derive(Debug)]
#[derive(Deserialize)]
pub struct Data {
    pub config: Config,
}

// Config struct holds to data from the `[config]` section.
#[derive(Debug)]
#[derive(Deserialize)]
pub struct Config {
    pub ignore: Vec<String>,
    pub ignore_beginning: Vec<String>,
    pub trim_whitespace: bool,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn parse_toml(args_options: &Vec<String>) -> Data {
    /*
    Parsing the toml config and returning the Data struct.
    Only finds the config if in the same dir as the executable.
    */
    // check for flag to ignore config
    if args_options.contains(&"--ignore-config".to_string()) {
        // return empty Config struct
        return Data {
            config: Config {
                ignore: if args_options.contains(&"--ignore-empty".to_string()) {vec!["".to_string()]} else {vec![]},
                ignore_beginning: vec![],
                trim_whitespace: if args_options.contains(&"--trim".to_string()) {true} else {false},
            }
        }
    }

    // search for toml in exe dir first
    let mut path: PathBuf = env::current_exe().unwrap().parent().unwrap().into();
    let toml_filename = Path::new("simil.toml");
    path.push(toml_filename);
    if !path.is_file() {
        // if file not in exe dir,
        // search cwd and parent dirs
        path = env::current_dir().unwrap();
        loop {
            path.push(toml_filename);
            if path.is_file() {
                break;
            }
            if !(path.pop() && path.pop()) {
                eprintln!("{COLOR_RED}error:{RESET_STYLES} no file {COLOR_YELLOW}'simil.toml'{RESET_STYLES} in exe dir or cwd & parent dirs found
Create a simil.toml file in the exe dir for global settings,
or the cwd or project (incl. any parent) dir for project-specific setttings.");
                exit(1);
            }
        }
    }
    //dbg!(&path);
    
    // parse toml contents
    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{COLOR_RED}error:{RESET_STYLES} could not find simil.toml in executable directory\n{e}");
            exit(1);
        }
    };
    //println!("{}", contents);
    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{COLOR_RED}error:{RESET_STYLES} unable to read data from simil.toml\n{e}");
            exit(1);
        }
    };
    return data;
}


#[derive(Debug)]
pub struct Args {
    pub filepath1: String,
    pub filepath2: String,
    pub options: Vec<String>,
}

pub fn check_args(args: Vec<String>) -> Args {
    /*
    Process command line arguments and returns an Args struct.
    Prints help message and exits if command supplied.
    */

    // check for help arg
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print_usage(false);
        exit(0);
    } else if args.contains(&"-v".to_string()) || args.contains(&"--version".to_string()) {
        println!("simil {}", VERSION);
        exit(0);
    }

    // check n args
    // requires min 2 args (the filenames)
    // (first arg is always the path with which the exe was invoked with)
    if args.len() < 3 {
        eprintln!("{}error:{} missing required positional argument(s)", COLOR_RED, RESET_STYLES);
        print_usage(true);
        exit(1);
    }

    // extract positional args and optional args
    // positional will include the relative exe path at index 0
    let mut positional: Vec<String> = vec![];
    let mut optional: Vec<String> = vec![];
    for arg in &args {
        if arg.starts_with("--") {
            optional.push(arg.to_string());
        } else if arg.starts_with("-") {
            optional.push(arg.to_string());
        } else {
            positional.push(arg.to_string());
        }
    }
    let args = Args {
        filepath1: positional[1].to_string(),
        filepath2: positional[2].to_string(),
        options: optional,
    };

    // check options provided
    for option in &args.options {
        if !ACCEPTED_OPTIONS.iter().any(|i| i == option) {
            // option not recognized
            eprintln!("{0}error:{2} unexpected argument {1}'{3}'{2} found", COLOR_RED, COLOR_YELLOW, RESET_STYLES, option);
            print_usage(true);
            exit(1);
        }
    }
    return args;
}

pub fn print_usage(as_error: bool) {
    if as_error {
        eprint!("{}", USAGE_STR);
    } else {
        print!("{}", USAGE_STR);
    }
}

pub fn construct_filepaths(args: &Args) -> (String, String) {
    // create file paths from args
    let path1: String;
    let path2: String;
    if args.options.contains(&"--abspath".to_string()) {
        // use filepath args as given
        path1 = args.filepath1.to_string();
        path2 = args.filepath2.to_string();
    } else {
        // append cwd to given filenames
        let cwd = env::current_dir()
                            .unwrap()
                            .as_os_str()
                            .to_owned()
                            .into_string()
                            .unwrap();
        // add filename to cwd
        path1 = [cwd.to_string(), args.filepath1.to_string()].join("/");
        path2 = [cwd.to_string(), args.filepath2.to_string()].join("/");
    }
    //println!("{:?}", path1);
    //println!("{:?}", path2);
    return (path1, path2);
}