use std::fs;

fn parse(input: &String) -> Vec<(i64, i64)> {
    input
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|range| {
            let r: Vec<i64> = range
                .split("-")
                .map(|i| i.parse::<i64>().unwrap())
                .collect();
            (r[0], r[1])
        })
        .collect()
}

fn is_invalid(num: i64) -> bool {
    let s = num.to_string();
    for i in 1..(s.len() / 2 + 1) {
        let substring = &s[0..i];
        let pattern = substring.repeat(s.len() / substring.len());
        if pattern == s {
            //println!("{}", s);
            return true;
        }
    }
    false
}

fn solve(ranges: Vec<(i64, i64)>) -> i64 {
    let mut ans: i64 = 0;
    for range in ranges {
        for n in range.0..=range.1 {
            if is_invalid(n) {
                ans += n;
            }
        }
    }
    ans
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input file");
    let ranges: Vec<(i64, i64)> = parse(&input.trim().to_string());
    println!("Answer: {}", solve(ranges));
}
