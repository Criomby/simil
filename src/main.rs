use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

// color codes
const COLOR_CODES: HashMap<> = HashMap::new();

// text modes

// reset styles

// Ignore
// Ignore-starts-with

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn similarities(file1: &str, file2: &str) {
    /*
    Check the content of the files,
    printing the lines which are the same.
    */
    let mut n_found = 0;
    if let Ok(lines) = read_lines(file1) {
        if let Ok(lines2) = read_lines(file2) {
            let mut fl1 = 0;
            let mut last_found = 0;
            for line in lines {
                if let Ok(ip) = line {
                    let text = ip.to_string().trim();
                    fl1 += 1;
                    // if line in IGNORE
                    // if line starts with IGNORE_STARTS_WITH
                    
                }
            }
        }
    }
}

fn main() {
    similarities(file1, file2)
}
