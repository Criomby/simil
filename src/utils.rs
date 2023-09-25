use serde_derive::Deserialize;
use std::fs;
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

pub fn parse_toml() -> Data {
    /*
    Parsing the toml config and returning the Data struct.
    Only finds the config if in the same dir as the executable.
    */
    // search for toml in exe dir
    //let toml_filepath ="/Users/pbr/Documents/dev/simil/simil.toml";
    let toml_filepath = [
        env::current_exe().unwrap().parent().unwrap().to_str().unwrap(),
        "simil.toml"
        ].join("/");
    //dbg!(&toml_filepath);
    let contents = match fs::read_to_string(toml_filepath) {
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
    if args.len() < 2 {
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
    let accepted_options = vec![
        "--abspath"
    ];
    for option in &args.options {
        if !accepted_options.iter().any(|&i| i == option) {
            // option not recognized
            eprintln!("{0}error:{2} unexpected argument {1}'{3}'{2} found", COLOR_RED, COLOR_YELLOW, RESET_STYLES, option);
            print_usage(true);
            exit(1);
        }
    }
    return args;
}


const USAGE_STR: &str = "
Usage: simil [-h] [--abspath] file1 file2

positional arguments:
    file

options:
    -h, --help      Show this help message and exit
    -v, --version   Show version number and exit
    --abspath       Using absolute filepaths (relative to cwd by default)

";

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