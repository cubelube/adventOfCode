use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed");
    let mut coords: Vec<String> = Vec::new();
    let mut santax: i32 = 0;
    let mut santay: i32 = 0;
    let mut robox: i32 = 0;
    let mut roboy: i32 = 0;
    let mut isSanta: bool = false;

    coords.push("0|0".to_string());

    for char in contents.chars() {

        isSanta = !isSanta;
        if char == '^' && isSanta == true {
            santay += 1;
        } else if char == 'v' && isSanta == true {
            santay -= 1;
        } else if char == '>' && isSanta == true{
            santax += 1;
        } else if char == '<' && isSanta == true {
            santax -= 1;
        } else if char == '^' && isSanta == false {
            roboy += 1;
        } else if char == 'v' && isSanta == false {
            roboy -= 1;
        } else if char == '>' && isSanta == false {
            robox += 1;
        } else if char == '<' && isSanta == false {
            robox -= 1;
        }

        if isSanta == true {
            let visit: String = santax.to_string() + "|" + &santay.to_string();
            for element in coords.clone() {
                if element.to_string() == visit {
                    break;
                }

                if element.to_string() == coords.last().expect("Failed").to_string() {
                    coords.push(visit.clone());
                }
            }
        } else {
            let visit: String = robox.to_string() + "|" + &roboy.to_string();
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
}
