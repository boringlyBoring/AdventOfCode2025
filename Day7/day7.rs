use std::fs;

fn parse(input: &String) -> Vec<Vec<&str>> {
    let mut grid = vec![];
    for line in input.lines() {
        let row: Vec<&str> = line.split("").collect();
        grid.push(row);
    }
    grid
}

fn process(grid: &mut Vec<Vec<&str>>) -> u32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut count = 0;
    for i in 1..m {
        for j in 0..n {
            if grid[i][j] == "^" && grid[i - 1][j] == "|" {
                if j + 1 < n {
                    grid[i][j + 1] = "|";
                }
                if j - 1 >= 0 {
                    grid[i][j - 1] = "|";
                }
                count += 1;
            } else if grid[i - 1][j] == "S" || grid[i - 1][j] == "|" {
                grid[i][j] = "|";
            }
        }
    }
    count
}

fn print_grid(grid: &Vec<Vec<&str>>) {
    let (m, n) = (grid.len(), grid[0].len());

    for i in 0..m {
        for j in 0..n {
            print!("{}", grid[i][j]);
        }
        println!("");
    }
}

fn main() {
    let input = fs::read_to_string("./input_test.txt").expect("Unable to parse input");
    let mut grid = parse(&input);
    let answer = process(&mut grid);
    println!("Answer: {}", answer);
    print_grid(&grid);
}
