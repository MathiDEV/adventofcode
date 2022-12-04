use std::fs;

fn intersects(range1: (u32, u32), range2: (u32, u32)) -> bool {
   range1.0 <= range2.0 && range1.1 >= range2.0  || range2.0 <= range1.0 && range2.1 >= range1.0
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let intersects = input.split("\n")
    .fold(0 as u32, |acc, pair| {
        let elves = pair
        .split(",")
        .map(|elve| {
            let elve_range = elve.split("-").collect::<Vec<&str>>();
            (elve_range[0]
                .parse::<u32>()
                .expect("Unable to parse int"),
            elve_range[1]
                .parse::<u32>()
                .expect("Unable to parse int")
            )
        })
        .collect::<Vec<(u32, u32)>>();

        if intersects(elves[0], elves[1]) {
            acc + 1
        } else {
            acc
        }
    });
    println!("Nb of full intetsections: {}", intersects);
}