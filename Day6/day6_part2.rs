use std::fs;

fn parse(input: &String) -> Vec<Vec<&str>> {
    let mut grid: Vec<Vec<&str>> = vec![];

    for line in input.lines() {
        let row: Vec<&str> = line.split("").collect();
        grid.push(row);
    }

    grid
}

fn solve(grid: &mut Vec<Vec<&str>>) -> u64 {
    let n = grid[0].len() - 1;
    let m = grid.len();
    let mut ans = 0;
    let mut buffer = 0;
    let mut sign = "";

    for j in 1..n {
        let mut num = 0;
        for i in 0..m {
            match grid[i][j] {
                "+" => {
                    sign = "+";
                    buffer = 0;
                }
                "*" => {
                    sign = "*";
                    buffer = 1;
                }
                " " | "" => continue,
                _ => {
                    num = num * 10 + grid[i][j].parse::<u64>().unwrap();
                }
            }
        }
        if num == 0 {
            ans += buffer;
            sign = "";
            buffer = 0;
        } else {
            match sign {
                "+" => buffer += num,
                _ => buffer *= num,
            }
        }
    }
    ans + buffer
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input");
    let mut grid = parse(&input);
    println!("{}", solve(&mut grid));
}
