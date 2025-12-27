use std::{cmp, fs};

fn parse(input: String) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<i64> = line
                .split(",")
                .map(|part| part.parse::<i64>().unwrap())
                .collect();
            (parts[0], parts[1])
        })
        .collect()
}

fn area(a: (i64, i64), b: (i64, i64)) -> i64 {
    let width = (a.0 - b.0).abs() + 1;
    let height = (a.1 - b.1).abs() + 1;
    width * height
}

fn solve(points: Vec<(i64, i64)>) -> i64 {
    let n = points.len();
    let mut max_area = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let area = area(points[i], points[j]);
            max_area = cmp::max(area, max_area);
        }
    }
    max_area
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to parse the input");
    let points = parse(input);
    let answer = solve(points);
    println!("Answer: {}", answer);
}
