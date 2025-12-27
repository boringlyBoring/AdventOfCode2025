use std::fs;

fn parse(input: String) -> Vec<(i64, i64, i64)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<i64> = line
                .split(",")
                .map(|part| part.parse::<i64>().unwrap())
                .collect();
            (parts[0], parts[1], parts[2])
        })
        .collect()
}

fn distance(a: (i64, i64, i64), b: (i64, i64, i64)) -> i64 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)
}

fn find(parent: &mut Vec<usize>, i: usize) -> usize {
    if parent[i] == i {
        return i;
    }
    parent[i] = find(parent, parent[i]);
    parent[i]
}

fn union(parent: &mut Vec<usize>, i: usize, j: usize) -> bool {
    let (root_i, root_j) = (find(parent, i), find(parent, j));
    if root_i != root_j {
        parent[root_i] = root_j;
        return true;
    }
    false
}

fn solve(points: Vec<(i64, i64, i64)>) -> i64 {
    //Sort by distance btw two points
    let n = points.len();
    let mut sorted_points: Vec<(i64, usize, usize)> = vec![];
    for i in 0..n {
        for j in (i + 1)..n {
            let distance = distance(points[i], points[j]);
            sorted_points.push((distance, i, j));
        }
    }
    sorted_points.sort_by_key(|k| k.0);
    let mut answer = 1;
    let mut num_of_connections = n;
    let mut parent = (0..n).collect();

    // Union each part untill last one
    for (_, u, v) in sorted_points {
        if union(&mut parent, u, v) {
            num_of_connections -= 1;
            if num_of_connections == 1 {
                answer = points[u].0 * points[v].0;
                break;
            }
        }
    }
    answer
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Error while reading input");
    let points = parse(input);
    let ans = solve(points);
    println!("Answer: {}", ans);
}
