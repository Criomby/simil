use std::fs::File;
use std::io::{BufRead, BufReader, Seek, Read};
use std::path::Path;
use crate::utils;
use std::process::exit;

pub fn similarities(
    filepath1: &Path,
    filepath2: &Path,
    config: &utils::Config,
) {
    /*
    Check the content of the files line by line,
    printing the lines which are the same.
    */
    if filepath1 == filepath2 {
        eprintln!("Cannot compare file with each other.");
        exit(1);
    }
    let file1 = BufReader::new(
        File::open(filepath1)
        .unwrap_or_else(|e| {
            eprintln!("{0}error:{2} Unable to open file {1}'{3}'{2}\n{e}",
                utils::COLOR_RED,
                utils::COLOR_YELLOW,
                utils::RESET_STYLES,
                filepath1.file_name().unwrap().to_str().unwrap(),
            );
            utils::print_usage(true);
            exit(1);
            }
        )
    );
    let mut file2 = BufReader::new(
        File::open(filepath2)
        .unwrap_or_else(|e| {
            eprintln!("{0}error:{2} Unable to open file {1}'{3:?}'{2}\n{e}",
                utils::COLOR_RED,
                utils::COLOR_YELLOW,
                utils::RESET_STYLES,
                filepath2.file_name().unwrap(),
            );
            utils::print_usage(true);
            exit(1);
            }
        )
    );
    let mut fl1 = 0;
    let mut fl2;
    let mut fl1_last_found = 0;
    let mut fl2_last_found = 0;
    let mut n_found = 0;
    'outer1: for line in file1.lines() {
        fl1 += 1;
        let mut text1 = line.expect("error");
        if config.trim_whitespace {
            text1 = text1.trim().to_string();
        }
        // ignore
        for i in &config.ignore {
            if text1 == *i {
                continue 'outer1;
            }
        }
        // end ignore
        // ignore starts with
        for i in &config.ignore_beginning {
            if text1.starts_with(i) {
                continue 'outer1;
            }
        }
        // end ignore starts with
        let _ = file2.rewind();
        fl2 = 0;
        'outer2: for line2 in file2.by_ref().lines() {
            fl2 += 1;
            let mut text2 = line2.expect("error");
            if config.trim_whitespace {
                text2 = text2.trim().to_string();
            }
            // ignore
            for i in &config.ignore {
                if text2 == *i {
                    continue 'outer2;
                }
            }
            // end ignore
            // ignore starts with
            for i in &config.ignore_beginning {
                if text2.starts_with(i) {
                    continue 'outer2;
                }
            }
            // end ignore starts with
            if text1 == text2 {
                // print subsequent finds formatted with "..."
                // ignore for first line
                if (fl1 == fl1_last_found + 1 || fl2 == fl2_last_found + 1) && !(fl1_last_found == 0 || fl2_last_found == 0) {
                    if text1.is_empty() {
                        println!("... empty");
                    } else {
                        println!("... {}", text1);
                    }
                } else {
                    println!(
                        "\n{4}{5}{0}{2} {3}({fl1}){2}\n{4}{5}{1}{2} {3}({fl2}){2}",
                        filepath1.file_name().unwrap().to_str().unwrap(),
                        filepath2.file_name().unwrap().to_str().unwrap(),
                        utils::RESET_STYLES,
                        utils::TEXTMODE_BOLD,
                        utils::TEXTMODE_DIM,
                        utils::TEXTMODE_UNDERLINE,
                    );
                    if text1.is_empty() {
                        println!(">>> empty");
                    } else {
                        println!(">>> {}", text1);
                    }
                }
                n_found += 1;
                fl1_last_found = fl1;
                fl2_last_found = fl2;
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