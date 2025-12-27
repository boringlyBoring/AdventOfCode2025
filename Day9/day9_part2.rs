use std::{fs, i64::MIN};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

fn parse(input: String) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<i64> = line
                .split(",")
                .map(|row| row.parse::<i64>().unwrap())
                .collect();
            Point {
                x: parts[0],
                y: parts[1],
            }
        })
        .collect()
}

fn area(a: &Point, b: &Point) -> i64 {
    let width = (a.x - b.x).abs() + 1;
    let height = (a.y - b.y).abs() + 1;
    return width * height;
}

fn is_valid(a: &Point, b: &Point, points: &Vec<Point>, edges: &mut Vec<(Point, Point)>) -> bool {
    let (x1, x2) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };
    let (y1, y2) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };

    //Check if it is inside
    for point in points {
        if point.x > x1 && point.x < x2 && point.y > y1 && point.y < y2 {
            return false;
        }
    }

    //Horizontal and Vertical
    for (u, v) in &mut *edges {
        //Vertical edge
        if u.x == v.x {
            if x1 < u.x && x2 > u.x {
                let ey_min = u.y.min(v.y);
                let ey_max = u.y.max(v.y);
                if !(ey_max <= y1 || ey_min >= y2) {
                    return false;
                }
            }
        } else if u.y == v.y {
            //Horizontal edge
            if y1 < u.y && y2 > u.y {
                let ex_min = u.x.min(v.x);
                let ex_max = u.x.max(v.x);
                if !(ex_max <= x1 || ex_min >= x2) {
                    return false;
                }
            }
        }
    }

    //Ray Casting
    let cx = (x1 as f64 + x2 as f64) / 2.0;
    let cy = (y1 as f64 + y2 as f64) / 2.0;
    let mut inside = false;
    for (u, v) in edges {
        if ((u.y as f64 > cy) != (v.y as f64 > cy))
            && (cx < (v.x - u.x) as f64 * (cy - u.y as f64) / (v.y - u.y) as f64 + u.x as f64)
        {
            inside = !inside;
        }
    }
    inside
}

fn solve(points: Vec<Point>) -> i64 {
    let mut edges = vec![];
    // Closed path
    let n = points.len();
    for i in 0..n {
        edges.push((points[i], points[(i + 1) % n]));
    }
    let mut max_area = MIN;
    for i in 0..n {
        for j in (i + 1)..n {
            let (a, b) = (points[i], points[j]);
            let area = area(&a, &b);

            if is_valid(&a, &b, &points, &mut edges) && area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input");
    let points: Vec<Point> = parse(input);
    let ans = solve(points);
    println!("Answer: {}", ans);
}
