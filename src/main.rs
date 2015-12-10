mod one;
mod two;
mod three;
mod four;
mod five;

fn main() {
    println!("Day 1: {:?}", one::solve(one::INPUT));
    println!("Day 2: {:?}", two::solve(two::INPUT));
    println!("Day 3: {:?}", three::solve(three::INPUT));
    println!("Day 4: {:?}", four::solve(four::INPUT));
    println!("Day 5: {:?}", five::solve(five::INPUT));
}
