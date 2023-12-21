use std::fs::File;
use std::io::{self, BufRead};
use std::cmp;
use std::path::Path;

fn main() {
    let mut allpaper: i32 = 0;

        if let Ok(lines) = read_lines("input.txt") {
            for line in lines {
                if let Ok(ip) = line {

                    let length = ip.split('x').nth(0).unwrap();
                    let length: i32 = length.parse().unwrap();
                    let width = ip.split('x').nth(1).unwrap();
                    let width: i32 = width.parse().unwrap();
                    let height = ip.split('x').nth(2).unwrap();
                    let height: i32 = height.parse().unwrap();
                    
                    let area = (2 * length * width) + (2 * width * height) + (2 * length * height);
                    let extrapaper = cmp::min(length * width, width * height);
                    let extrapaper2 = cmp::min(extrapaper, length * height);
                    allpaper += area + extrapaper2;
                }
            }
        }

    println!("{:?}", allpaper);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
