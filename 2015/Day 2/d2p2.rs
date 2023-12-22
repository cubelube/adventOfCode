use std::fs::File;
use std::io::{self, BufRead};
use std::cmp;
use std::path::Path;

fn main() {
    let mut allribbon: i32 = 0;

        if let Ok(lines) = read_lines("input.txt") {
            for line in lines {
                if let Ok(ip) = line {

                    let length: i32 = ip.split('x').nth(0).expect("REASON").parse().unwrap();
                    let width: i32 = ip.split('x').nth(1).expect("REASON").parse().unwrap();
                    let height: i32 = ip.split('x').nth(2).expect("REASON").parse().unwrap();

                    let wrappresent = cmp::min(2 * (length + width), 2 * (width + height));
                    let wrappresent2 =  cmp::min(wrappresent, 2 * (length + height));
                    let cubicvolume = length * width * height;
                    allribbon += wrappresent2 + cubicvolume;
                }
            }
        }

    println!("{:?}", allribbon);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
