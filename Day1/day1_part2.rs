use std::fs;

fn parse(input: &String) -> Vec<(&str, i64)> {
    input
        .lines()
        .map(|line| {
            let direction = &line[0..1];
            let num = line[1..].parse::<i64>().unwrap();
            (direction, num)
        })
        .collect()
}

fn solve(instructions: &Vec<(&str, i64)>) -> i64 {
    let mut position = 50;
    let mut ans = 0;
    for instruction in instructions {
        let (dir, num) = (instruction.0, instruction.1);
        match dir {
            "R" => {
                ans += (num + position) / 100;
                position = (position + num) % 100;
            }
            _ => {
                let reversed = (100 - position) % 100;
                ans += (reversed + num) / 100;
                position = (position - num).rem_euclid(100);
            }
        }
    }
    ans
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input file");
    let instructions = parse(&input);
    println!("Answer: {}", solve(&instructions));
}
