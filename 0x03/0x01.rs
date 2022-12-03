use std::fs;
use std::collections::HashSet;

fn split_in_middle(s: &str) -> (&str, &str) {
    let mid = s.len() / 2;
    (&s[..mid], &s[mid..])
}

fn common_letters(s1: &str, s2: &str) -> HashSet<char> {
    s1.chars()
    .filter(|c| s2.contains(*c))
    .collect::<HashSet<char>>()
}

fn get_score(c: char) -> u32 {
    if c as u32 > 97 {
        c as u32 - 96
    } else {
        c as u32 - 64 + 26
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let rucksacks = input.split("\n").map(|rucksack| split_in_middle(rucksack)).collect::<Vec<(&str, &str)>>();
    let mut sum = 0;
    rucksacks.iter().for_each(|rucksack| {
        let common = common_letters(rucksack.0, rucksack.1);
        sum += common.iter().map(|c| get_score(*c)).sum::<u32>()
    });
    println!("Sum: {}", sum);
}