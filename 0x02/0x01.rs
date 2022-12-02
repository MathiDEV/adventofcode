use std::fs;

const WIN_SCORE: i32 = 6;
const LOOSE_SCORE: i32 = 0;
const TIE_SCORE: i32 = 3;

fn main() {
    let input = fs::read_to_string("input").expect("Unable to read input");
    let rounds = input.split("\n").map(|round| round.split(" ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    let mut total_score = 0;

    rounds.iter().for_each(|round| {
        let loose_move: &str = match round[0] {
            "A" => "Z",
            "B" => "X",
            "C" => "Y",
            &_ => panic!("Unknown move")
        };
        let tie_move: &str = match round[0] {
            "A" => "X",
            "B" => "Y",
            "C" => "Z",
            &_ => panic!("Unknown move")
        };
        let win_move: &str = match round[0] {
            "A" => "Y",
            "B" => "Z",
            "C" => "X",
            &_ => panic!("Unknown move")
        };

        total_score += {
            if round[1] == loose_move {
                LOOSE_SCORE
            } else if round[1] == tie_move {
                TIE_SCORE
            } else if round[1] == win_move {
                WIN_SCORE
            } else {
                panic!("Unknown move")
            }
        };

        total_score += match round[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            &_ => panic!("Unknown move")
        };
    });
    println!("Total score: {}", total_score);
}