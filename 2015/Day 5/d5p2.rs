use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use fancy_regex::Regex;

fn main() {
    let mut nicecounter: i32 = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(value) = line {
                if check_if_nice(value.as_str()) {
                    nicecounter += 1;
                }
            }
        }
    }

    println!("{}", nicecounter);
}

fn check_if_nice(input: &str) -> bool {
    let pair = Regex::new(r"(..).*\1").unwrap();
    let between = Regex::new(r"(.).\1").unwrap();

    if pair.is_match(input).unwrap() && between.is_match(input).unwrap() {
        return true;
    }

    return false;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
