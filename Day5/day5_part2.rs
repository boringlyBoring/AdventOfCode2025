use std::cmp;
use std::fs;

fn read_ranges(input: &String) -> Vec<(u64, u64)> {
    input
        .lines()
        .filter(|&line| line.contains("-"))
        .map(|line| {
            let parts: Vec<u64> = line
                .split("-")
                .map(|part| part.parse::<u64>().unwrap())
                .collect();
            (parts[0], parts[1])
        })
        .collect()
}

fn merge_ranges(ranges: &mut Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.sort_by_key(|&i| i.0);
    let mut merged_range: Vec<(u64, u64)> = vec![];

    for range in ranges {
        if merged_range.is_empty() || range.0 > merged_range.last_mut().unwrap().1 {
            merged_range.push(*range);
        } else {
            let last = merged_range.last_mut().unwrap();
            if range.0 <= last.1 {
                last.1 = cmp::max(range.1, last.1);
            }
        }
    }
    merged_range
}

fn count_ingredients(ranges: &Vec<(u64, u64)>) -> u64 {
    let mut count = 0;
    for range in ranges {
        count += range.1 - range.0 + 1;
    }
    count
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read file");
    let mut ranges = read_ranges(&input);
    let merged_range = merge_ranges(&mut ranges);
    let count = count_ingredients(&merged_range);
    println!("Answer: {}", count);
}
