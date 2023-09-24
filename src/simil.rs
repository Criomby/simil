use std::fs::File;
use std::io::{BufRead, BufReader, Seek, Read};
use std::path::Path;
use crate::utils;

pub fn similarities(
    filepath1: &Path,
    filepath2: &Path,
    ignore: &Vec<String>,
    ignore_beginning: &Vec<String>
) {
    /*
    Check the content of the files line by line,
    printing the lines which are the same.
    */
    let file1 = BufReader::new(
        File::open(filepath1)
        .expect(&format!("Unable to open file {:?}", filepath1.file_name().unwrap()))
    );
    let mut file2 = BufReader::new(
        File::open(filepath2)
        .expect(&format!("Unable to open file {:?}", filepath2.file_name().unwrap()))
    );
    let mut fl1 = 0;
    let mut fl2;
    let mut last_found = 0;
    let mut n_found = 0;
    'outer1: for line in file1.lines() {
        let text1 = line.expect("error").trim().to_string();
        fl1 += 1;
        // ignore
        for i in ignore {
            if &text1 == i {
                continue 'outer1;
            }
        }
        // end ignore
        // ignore starts with
        for i in ignore_beginning {
            if text1.starts_with(i) {
                continue 'outer1;
            }
        }
        // end ignore starts with
        let _ = file2.rewind();
        fl2 = 0;
        'outer2: for line2 in file2.by_ref().lines() {
            let text2 = line2.expect("error").trim().to_string();
            // ignore
            for i in ignore {
                if &text2 == i {
                    continue 'outer2;
                }
            }
            // end ignore
            // ignore starts with
            for i in ignore_beginning {
                if text2.starts_with(i) {
                    continue 'outer2;
                }
            }
            // end ignore starts with
            fl2 += 1;
            if text1 == text2 {
                if fl1 == last_found + 1 {
                    println!("... {}", text1);
                } else {
                    println!(
                        "\n{1}{3}{2} {0}({fl1}){2}\n{1}{4}{2} {0}({fl2}){2}",
                        utils::TEXTMODE_BOLD,
                        utils::TEXTMODE_DIM,
                        utils::RESET_STYLES,
                        filepath1.file_name().unwrap().to_str().unwrap(),
                        filepath2.file_name().unwrap().to_str().unwrap()
                    );
                    println!(">>> {}", text1);
                }
                n_found += 1;
                last_found = fl1;
            }
        }
    }
    if n_found == 0 {
        println!(
            "
------------------------------
{}{n_found} matches found{}
------------------------------
",
            utils::COLOR_GREEN,
            utils::RESET_STYLES
        );
    } else {
        println!(
            "
------------------------------
{}{n_found} matches found{}
------------------------------
",
            utils::COLOR_RED,
            utils::RESET_STYLES
        );
    }
}    