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
    //dbg!(&args);

    let data = utils::parse_toml();
    //dbg!(&data);

    // construct paths to files to analyze from args
    let (path1, path2) = utils::construct_filepaths(&args);
    //dbg!(&path1);
    //dbg!(&path2);

    //let path1 = "/Users/pbr/PycharmProjects/bids_programming/data_science_script.py";
    //let path2 = "/Users/pbr/PycharmProjects/bids_programming/exercises_script.py";
    
    similarities(Path::new(&path1), Path::new(&path2), &data.config);
}