use std::{cmp, fs};

#[derive(Debug)]
struct Machine {
    target: u64,
    buttons: Vec<u64>,
}

fn solve(machine: Machine) -> Option<u64> {
    let n = machine.buttons.len();
    let mut min_press = None;

    // All 2^n combinations
    for i in 0..(1 << n) {
        let mut current_mask: u64 = 0;
        let mut count = 0;

        for j in 0..n {
            if (i >> j) & 1 == 1 {
                current_mask ^= machine.buttons[j];
                count += 1;
            }
        }

        if current_mask == machine.target {
            min_press = match min_press {
                Some(num) => Some(cmp::min(num, count)),
                _ => Some(count),
            }
        }
    }
    min_press
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split('{').collect(); // Split off joltage requirements
            let main_part = parts[0];

            let target_str = main_part
                .split(']')
                .next()
                .unwrap()
                .trim_matches(|c| c == '[' || c == ' ');
            let mut target: u64 = 0;
            for (i, ch) in target_str.chars().enumerate() {
                if ch == '#' {
                    target |= 1 << i;
                }
            }

            let mut buttons = Vec::new();
            // Regex or smarter split to find (0,1,2)
            for b_str in main_part.split('(').skip(1) {
                let clean = b_str.split(')').next().unwrap();
                let mut mask: u64 = 0;
                for num in clean.split(',') {
                    if let Ok(n) = num.trim().parse::<u64>() {
                        mask |= 1 << n;
                    }
                }
                buttons.push(mask);
            }

            Machine { target, buttons }
        })
        .collect()
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input from file");
    let machine_list = parse(&input);
    let mut ans = 0;

    for machine in machine_list {
        if let Some(min_press) = solve(machine) {
            ans += min_press;
        }
    }
    println!("Answer: {ans}");
}
