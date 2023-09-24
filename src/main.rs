use std::fs::File;
use std::io::{BufRead, BufReader, Seek, Read};
use std::path::Path;

// color codes
const COLOR_BLACK: &'static str = "\033[30m";
const COLOR_RED: &'static str = "\033[30m";

// text modes
const TEXTMODE_BOLD: &'static str = "\033[1m";
const TEXTMODE_DIM: &'static str = "\033[2m";
const TEXTMODE_UNDERLINE: &'static str = "\033[4m";

// reset styles
const RESET_STYLES: &'static str = "\033[0m";

// Ignore
// Ignore-starts-with


fn similarities(file1: &Path, file2: &Path) {
    /*
    Check the content of the files,
    printing the lines which are the same.
    */
    let mut n_found = 0;
    let f1 = File::open(file1).expect("Unable to open file1");
    let file1 = BufReader::new(f1);
    let f2 = File::open(file2).expect("Unable to open file2");
    let mut file2 = BufReader::new(f2);
    let mut fl1 = 0;
    let mut fl2 = 0;
    let mut last_found = 0;
    for line in file1.lines() {
        let text = line.expect("error").trim().to_string();
        fl1 += 1;
        // ignore
        // ignore whitespace
        if text.is_empty() {
            continue;
        }
        // ignore starts with
        file2.rewind().expect("Cannot go to start of file 2");
        fl2 = 0;
        for line2 in file2.by_ref().lines() {
            let text2 = line2.expect("error").trim().to_string();
            // ignore
            // ignore whitespace
            if text2.is_empty() {
                continue;
            }
            // ignore starts with
            fl2 += 1;
            if text == text2 {
                println!("match!\n{}\n", text);
                // last found + 1 or not
                n_found += 1;
                last_found = fl1;
            }
        }
    }
    // print summary
    println!("found {} matches", n_found);
}    

fn main() {
    let path1: &Path = Path::new("/Users/pbr/PycharmProjects/bids_programming/data_science_script.py");
    let path2: &Path = Path::new("/Users/pbr/PycharmProjects/bids_programming/exercises_script.py");
    similarities(path1, path2);
}
