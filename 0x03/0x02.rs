use std::fs;
use std::collections::HashSet;

fn find_badge(s1: &str, s2: &str, s3: &str) -> char {
    let set = s1.chars()
    .filter(|c| s2.contains(*c) && s3.contains(*c))
    .collect::<HashSet<char>>()
    .iter()
    .map(|c| *c)
    .collect::<Vec<char>>();
    set[0]
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
    let rucksacks = input.split("\n").collect::<Vec<&str>>();
    let mut sum = 0;
    rucksacks
    .iter()
    .enumerate()
    .fold(vec![] as Vec<Vec<&str>>, |mut acc, (i, rucksack) | {
        if i % 3 == 0 {
            acc.push(vec![]);
        }
        acc[i / 3].push(rucksack);
        acc
    })
    .iter()
    .for_each(|rucksack| {
        let badge = find_badge(rucksack[0], rucksack[1], rucksack[2]);
        sum += get_score(badge);
    });
    println!("Sum: {}", sum);
}