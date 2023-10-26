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

const VERSION: &str = env!("CARGO_PKG_VERSION");

const ACCEPTED_OPTIONS: [&'static str; 5] = [
        "--abspath",
        "--global-conf",
        "--noconf",
        "--ignore-empty",
        "--trim",
    ];

const USAGE_STR: &str = "
Usage: simil [...] file1 file2

positional arguments:
    file1
    file2

options:
    -h, --help      Show this help message and exit
    -V, --version   Show version number and exit
    --abspath       Using absolute filepaths (relative to cwd by default)
    --global-conf   Ignore local config
    --noconf        Do not use any simil.toml config
        + --ignore-empty  Omit empty lines in output
        + --trim          Trim whitespace

";


// Top level struct to hold the TOML data.
#[derive(Debug)]
#[derive(Deserialize)]
pub struct Data {
    pub config: Config,
}

// Config struct holds data from the `[config]` section.
#[derive(Debug)]
#[derive(Deserialize)]
pub struct Config {
    pub ignore: Vec<String>,
    pub ignore_beginning: Vec<String>,
    pub trim_whitespace: bool,
}

pub fn parse_toml(args_options: &Vec<String>) -> Data {
    /*
    Parsing the toml config and returning the Data struct.
    Only finds the config if in the same dir as the executable.
    */
    // check for flag to ignore config
    if args_options.contains(&"--noconf".to_string()) {
        // return empty Config struct
        return Data {
            config: Config {
                ignore: if args_options.contains(&"--ignore-empty".to_string()) {vec!["".to_string()]} else {vec![]},
                ignore_beginning: vec![],
                trim_whitespace: if args_options.contains(&"--trim".to_string()) {true} else {false},
            }
        }
    }

    let mut path: PathBuf;
    let toml_filename = Path::new("simil.toml");

    // check options for config
    if args_options.contains(&"--global-conf".to_string()) {
        // --global-conf (only search for global file)
        path = env::current_exe().unwrap().parent().unwrap().into();
        path.push(toml_filename);
        if !path.is_file() {
            eprintln!("{COLOR_RED}error:{RESET_STYLES} no file {COLOR_YELLOW}'simil.toml'{RESET_STYLES} in exe dir found
Consider removing {COLOR_YELLOW}'--global-config'{RESET_STYLES} or add a config file to the exe dir");
            exit(1);
        }
    } else {
        // first, search for toml in cwd and parent dirs
        path = env::current_dir().unwrap();
        path.push(toml_filename);
            loop {
                if path.is_file() {
                    // println!("{TEXTMODE_DIM}Using project config{RESET_STYLES}");
                    break;
                }
                if !(path.pop() && path.pop()) {
                    // no config in cwd/parents found
                    // check exe dir for config toml
                    path = env::current_exe().unwrap().parent().unwrap().into();
                    path.push(toml_filename);
                    if path.is_file() {
                        // println!("{TEXTMODE_DIM}Using global config{RESET_STYLES}");
                        break;
                    }
                    eprintln!("{COLOR_RED}error:{RESET_STYLES} no file {COLOR_YELLOW}'simil.toml'{RESET_STYLES} found
    Create a simil.toml file in the cwd or any parent dir for project settings,
    or the exe dir for global configuration.
    For additional options, see documentation or use --help.");
                    exit(1);
                }
            }
    }
    // dbg!(&path);
    
    // parse toml contents
    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{COLOR_RED}error:{RESET_STYLES} could not read simil.toml\n{e}");
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
    } else if args.contains(&"-V".to_string()) || args.contains(&"--version".to_string()) {
        println!("simil {}", VERSION);
        exit(0);
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

    // check n args
    // requires exactly 2 positional args (the filenames)
    // first arg (index 0) is always the path with which the exe was invoked with
    if positional.len() != 3 {
        if positional.len() < 3 {
            eprintln!("{COLOR_RED}error:{RESET_STYLES} missing required positional argument(s)");
        } else {
            eprintln!("{COLOR_RED}error:{RESET_STYLES} too many arguments");
        }
        print_usage(true);
        exit(1);
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