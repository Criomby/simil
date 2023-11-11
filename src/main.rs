use std::path::Path;
mod simil;
use simil::similarities;
mod utils;
use std::env;

fn main() {
    // command line args
    let args = utils::check_args(env::args().collect());
    //dbg!(&args);

    let data = utils::parse_toml(&args.options);
    //dbg!(&data);

    // construct paths to files to analyze from args
    let (path1, path2) = utils::construct_filepaths(&args);
    //dbg!(&path1);
    //dbg!(&path2);
    
    similarities(Path::new(&path1), Path::new(&path2), &data.config, &args.options);
}