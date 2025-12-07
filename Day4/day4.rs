use std::fs;

fn check_access(grid: &Vec<Vec<i32>>, i: i32, j: i32, n: i32, m: i32) -> bool {
    if grid[i as usize][j as usize] == 0 {
        return false;
    }
    let dirs = vec![
        (-1, 0),
        (1, 0),
        (0, 1),
        (0, -1),
        (-1, 1),
        (-1, -1),
        (1, -1),
        (1, 1),
    ];
    let mut sum = 0;
    for (x, y) in dirs {
        let new_i = i + x;
        let new_j = j + y;

        if new_i < 0 || new_i == n || new_j < 0 || new_j == m {
            continue;
        }
        sum += grid[new_i as usize][new_j as usize];
    }
    sum < 4
}

fn main() {
    let file_path = "./input.txt";

    let diagram = fs::read_to_string(file_path).expect("Unable to parse input");

    let mut grid: Vec<Vec<i32>> = Vec::new();

    for row in diagram.lines() {
        if row.len() == 0 {
            continue;
        }
        let mut row_vector: Vec<i32> = Vec::new();
        for cell in row.chars() {
            match cell {
                '@' => row_vector.push(1),
                '.' => row_vector.push(0),
                _ => !panic!("Fuck, this should never have happened!"),
            }
        }
        grid.push(row_vector);
    }

    let n = grid.len();
    let m = grid.first().expect("Error").len();
    let mut ans = 0;

    for i in 0..n {
        for j in 0..m {
            if check_access(&grid, i as i32, j as i32, n as i32, m as i32) {
                ans += 1;
            }
        }
    }

    println!("answer: {}", ans);
}
