mod one;
mod two;
mod three;
mod four;
mod five;
mod six;
mod seven;
mod eight;
mod nine;

fn main() {
    // println!("Day 1: {:?}", one::solve(one::INPUT));
    // println!("Day 2: {:?}", two::solve(two::INPUT));
    // println!("Day 3: {:?}", three::solve(three::INPUT));
    // println!("Day 4: {:?}", four::solve(four::INPUT));
    // println!("Day 5: {:?}", five::solve(five::INPUT));
    // println!("Day 6: {:?}", six::solve(six::INPUT));
    // println!("Day 7: {:?}", seven::solve(seven::INPUT));
    // println!("Day 8: {:?}", eight::solve(&read_data("day8.txt")));
    println!("Day 9: {:?}", nine::solve(&read_data("day9.txt")));
}

fn read_data(name: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    let path = format!("data/{}", name);
    let path = Path::new(&path);
    if let Ok(mut file) = File::open(&path) {
        let mut s = String::new();
        if let Ok(_) = file.read_to_string(&mut s) {
            return s;
        } else {
            panic!("Error while reading from file at data/{}", name);
        }
    } else {
        panic!("Cannot open file at data/{}", name);
    }
}
