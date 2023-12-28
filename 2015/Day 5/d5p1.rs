use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    let mut nicecounter: i32 = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(value) = line {
                if check_if_nice(value) {
                    nicecounter += 1;
                }
            }
        }
    }

    println!("{}", nicecounter);
}

fn check_if_nice(input: String) -> bool {
    let naughtywords = Regex::new(r"ab|cd|pq|xy").unwrap();
    let vowels = Regex::new(r"a|e|i|o|u").unwrap();
    let doubleletters = Regex::new(r"aa|bb|cc|dd|ee|ff|gg|hh|ii|jj|kk|ll|mm|nn|oo|pp|qq|rr|ss|tt|uu|vv|ww|xx|yy|zz").unwrap();
    let mut vowelcount: i32 = 0;

    for char in input.chars() {
        if vowels.is_match(char.to_string().as_str()) {
            vowelcount += 1;
        }
    }

    if naughtywords.is_match(input.as_str()) || vowelcount < 3 || !doubleletters.is_match(input.as_str()) {
        return false;
    }

    return true;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
