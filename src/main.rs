use std::path::Path;
mod simil;
use simil::similarities;
mod utils;
use std::env;

// Ignore
// Ignore-starts-with

fn main() {
    // command line args
    let args = utils::check_args(env::args().collect());
    //dbg!(args);

    // search for toml in exe dir
    let mut exe_path = env::current_exe().unwrap().as_os_str().to_owned();
    println!("{:?}", exe_path);

    // create file paths from args
    let path1: String;
    let path2: String;
    if args.options.contains(&"--abspath".to_string()) {
        // use filepath args as given
        path1 = args.filepath1;
        path2 = args.filepath2;
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
    println!("{:?}", path1);
    println!("{:?}", path2);

    let path1 = "/Users/pbr/PycharmProjects/bids_programming/data_science_script.py";
    let path2 = "/Users/pbr/PycharmProjects/bids_programming/exercises_script.py";
    
    let data = utils::parse_toml();
    //dbg!(&data);
    similarities(Path::new(path1), Path::new(path2), &data.config);
}