use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed");
    let mut coords: Vec<String> = Vec::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    coords.push("0|0".to_string());

    for char in contents.chars() {
        if char == '^' {
            y += 1;
        } else if char == 'v' {
            y -= 1;
        } else if char == '>' {
            x += 1;
        } else {
            x -= 1;
        }

        let visit: String = x.to_string() + "|" + &y.to_string();

        for element in coords.clone() {
            if element.to_string() == visit {
                break;
            }

            if element.to_string() == coords.last().expect("failed").to_string() {
                coords.push(visit.clone());
            }
        }
    }

    println!("{}", coords.len());
}
