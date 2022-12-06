fn is_marker(part: &str) -> bool {
    let mut chars = part.chars().collect::<Vec<char>>();
    chars.sort();
    chars.dedup();
    chars.len() == 14
}

fn main() {
    let input = include_str!("input");
    for i in 0..input.len() - 14 {
        if is_marker(&input[i..i+14]) {
            println!("First marker pos: {}", i + 14);
            break;
        }
    }
}