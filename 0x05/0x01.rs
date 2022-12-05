
const MAX_STACK_SIZE: usize = 8;
const N_STACKS: usize = 9;

fn get_box(input: &Vec<&str>) -> Vec<Vec<char>> {
    let mut boxes = vec![vec![]; N_STACKS];
    input
    .iter()
    .take(MAX_STACK_SIZE)
    .for_each(|line| {
        line.chars().enumerate().for_each(|(i, c)| {
            if i % 4 == 1 && c != ' ' {
                boxes[i / 4].insert(0, c);
            }   
        });
    });
    boxes
}

fn main() {
    let input = include_str!("input").split("\n").collect::<Vec<&str>>();
    let mut boxes = get_box(&input);
    input.iter().skip(MAX_STACK_SIZE + 2).for_each(|line| {
        let line_data = line.split(" ").collect::<Vec<&str>>();
        let count = line_data[1].parse::<usize>().unwrap();
        let source = line_data[3].parse::<usize>().unwrap() - 1;
        let dest = line_data[5].parse::<usize>().unwrap() - 1;
        for _ in 0..count {
            if boxes[source].len() == 0 {
                break;
            }
            let c = boxes[source].pop().unwrap();
            boxes[dest].push(c);
        }
    });
    let mut result = String::new();
    for i in 0..N_STACKS {
        result.push(boxes[i].pop().unwrap());
    }
    println!("Result: {}", result);
}