use std::path::Path;
mod simil;
use simil::similarities;
mod utils;
use std::env;  // current_dir & current_exe

// Ignore
// Ignore-starts-with

fn main() {
    let path1: &Path = Path::new("/Users/pbr/PycharmProjects/bids_programming/data_science_script.py");
    let path2: &Path = Path::new("/Users/pbr/PycharmProjects/bids_programming/exercises_script.py");
    
    let data = utils::parse_toml();
    similarities(
        path1,
        path2,
        &data.config.ignore,
        &data.config.ignore_beginning
    );
}