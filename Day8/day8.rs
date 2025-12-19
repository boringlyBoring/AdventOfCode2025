use std::{collections::HashMap, fs, i64};

fn parse(input: &String) -> Vec<(i64, i64, i64)> {
    input
        .lines()
        .map(|row| {
            let points: Vec<i64> = row
                .split(",")
                .map(|point| point.parse::<i64>().unwrap())
                .collect();
            (points[0], points[1], points[2])
        })
        .collect()
}

fn distance(a: (i64, i64, i64), b: (i64, i64, i64)) -> i64 {
    return (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2);
}

fn find(parent: &mut Vec<usize>, i: usize) -> usize {
    if parent[i] == i {
        return i;
    }
    parent[i] = find(parent, parent[i]);
    parent[i]
}

fn union(parent: &mut Vec<usize>, i: usize, j: usize) {
    let (root_i, root_j) = (find(parent, i), find(parent, j));
    if root_i != root_j {
        parent[root_i] = root_j;
    }
}

fn solve(points: &mut Vec<(i64, i64, i64)>) -> i64 {
    let n = points.len();
    let mut distances: Vec<(i64, usize, usize)> = vec![];

    for i in 0..n {
        for j in (i + 1)..n {
            if i == j {
                continue;
            }
            let distance = distance(points[i], points[j]);
            distances.push((distance, i, j));
        }
    }
    distances.sort_by_key(|k| k.0);

    let mut parent = (0..n).collect();

    let limit = if n <= 20 { 10 } else { 1000 };
    for k in 0..limit.min(distances.len()) {
        let (_, u, v) = distances[k];
        union(&mut parent, u, v);
    }

    let mut size: HashMap<usize, i64> = HashMap::new();

    for i in 0..n {
        let parent = find(&mut parent, i);
        *size.entry(parent).or_insert(0) += 1;
    }

    let mut answer: Vec<i64> = size.values().cloned().collect();
    answer.sort_by(|a, b| b.cmp(a));

    answer.iter().take(3).product()
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input");
    let mut points: Vec<(i64, i64, i64)> = parse(&input);
    let answer = solve(&mut points);
    println!("Answer: {}", answer);
}
