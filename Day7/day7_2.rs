use std::fs;

fn parse(input: &String) -> Vec<Vec<&str>> {
    let mut grid = vec![];

    for line in input.lines() {
        let row: Vec<&str> = line.split("").collect();
        grid.push(row);
    }
    grid
}

fn solve(grid: &mut Vec<Vec<&str>>) -> u64 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut dp: Vec<Vec<u64>> = vec![vec![0; n]; m];

    for i in 0..n {
        if grid[0][i] == "S" {
            dp[0][i] = 1;
        }
    }

    for i in 1..m {
        for j in 0..n {
            if grid[i][j] == "^" {
                if j - 1 >= 0 {
                    dp[i][j - 1] += dp[i - 1][j];
                }
                if j + 1 < n {
                    dp[i][j + 1] += dp[i - 1][j];
                }
            } else {
                dp[i][j] += dp[i - 1][j];
            }
        }
    }
    dp[m - 1].iter().sum()
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input");
    let mut grid = parse(&input);
    let answer = solve(&mut grid);
    println!("Answer: {}", answer);
}
