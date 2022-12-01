use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Unable to read input");
    let mut elves: Vec<i32> = input
        .split("\n\n")
        .map(|items| items.split("\n").map(|item| item.parse::<i32>().unwrap()).sum())
        .collect::<Vec<i32>>();
    
    elves.sort();
    elves.reverse();
    
    println!("Best elf: {}", elves[0]);
}