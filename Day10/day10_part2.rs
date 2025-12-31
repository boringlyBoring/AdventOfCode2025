use std::{cmp, collections::HashMap, fs};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Machine {
    targets: Vec<i64>,
    buttons: Vec<u64>,
}

fn parse(input: &String) -> Vec<Machine> {
    input
        .lines()
        .filter(|f| !f.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split("{").collect();

            //Joltage Targets
            let joltage_part = parts[1].trim_end_matches("}");
            let targets: Vec<i64> = joltage_part
                .split(",")
                .map(|n| n.parse::<i64>().unwrap())
                .collect();

            //Buttons
            let mut buttons: Vec<u64> = vec![];

            for b_str in parts[0].split('(').skip(1) {
                let clean = b_str.split(')').next().unwrap();
                let mut mask: u64 = 0;
                for num in clean.split(',') {
                    mask |= 1 << num.parse::<u64>().unwrap();
                }
                buttons.push(mask);
            }
            Machine { targets, buttons }
        })
        .collect()
}

fn solve(
    targets: Vec<i64>,
    buttons: &Vec<u64>,
    cache: &mut HashMap<Vec<i64>, Option<u64>>,
) -> Option<u64> {
    if targets.iter().all(|&t| t == 0) {
        return Some(0);
    }
    if targets.iter().any(|&t| t < 0) {
        return None;
    };

    if let Some(&res) = cache.get(&targets) {
        return res;
    }

    let num_btns = buttons.len();
    let num_lights = targets.len();
    let mut min_press = None;

    for i in 0..(1 << num_btns) {
        let mut impacts = vec![0i64; num_lights];
        let mut current_press = 0;

        for j in 0..num_btns {
            if (i >> j) & 1 == 1 {
                current_press += 1;
                for bit in 0..num_lights {
                    if (buttons[j] >> bit) & 1 == 1 {
                        impacts[bit] += 1;
                    }
                }
            }
        }

        let mut next_targets = vec![0i64; num_lights];
        let mut valid_parity = true;
        for bit in 0..num_lights {
            let rem = targets[bit] - impacts[bit];
            if rem < 0 || rem % 2 != 0 {
                valid_parity = false;
                break;
            }
            next_targets[bit] = rem / 2;
        }

        if valid_parity {
            if let Some(sub_res) = solve(next_targets, buttons, cache) {
                let total = (sub_res * 2) + current_press;
                min_press = Some(match min_press {
                    Some(m) => cmp::min(m, total),
                    None => total,
                });
            }
        }
    }
    cache.insert(targets, min_press);
    min_press
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input from file");
    let machine_list = parse(&input);
    let mut total_press = 0;

    for machine in machine_list {
        let mut cache = HashMap::new();
        if let Some(res) = solve(machine.targets, &machine.buttons, &mut cache) {
            total_press += res;
        }
    }
    println!("Answer: {}", total_press);
}
