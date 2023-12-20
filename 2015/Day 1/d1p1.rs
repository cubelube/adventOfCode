use std::fs;

fn main() {
    let mut counter:i32 = 0;
    let file_text = fs::read_to_string("input.txt")
        .expect("Failed");

    for char in file_text.chars(){
        if char == '(' {
            counter += 1;
        } else {
            counter -= 1;
        }
    }
    
    println!("{counter}");
}
