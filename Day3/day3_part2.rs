use std::fs;
use std::io::{self, Write};

fn solve(num: &str) -> u64 {
    let bytes = num.as_bytes();
    let (n, k) = (num.len(), 12);

    if n < k {
        return 0;
    }

    let mut ans: u64 = 0;
    let mut start_index: usize = 0;

    for i in 0..k {
        let remaining = k - i - 1;
        let end_index = n - remaining;

        let mut best_digit = 0;
        let mut best_index = start_index;

        for j in start_index..end_index {
            let digit = bytes[j] - b'0';
            if digit > best_digit {
                best_digit = digit;
                best_index = j;
                if best_digit == 9 {
                    break;
                }
            }
        }

        ans = (ans * 10) + best_digit as u64;
        start_index = best_index + 1;
    }
    ans
}

fn main() {
    let file_path = "./input.txt";
    let content = fs::read_to_string(file_path).expect("Unable to read file");
    let lines: Vec<&str> = content.lines().filter(|s| !s.is_empty()).collect();
    let total_lines = lines.len();

    let mut total_sum: u64 = 0;

    for (idx, line) in lines.iter().enumerate() {
        total_sum += solve(line.trim());

        // --- Status Bar Logic ---
        let progress = idx + 1;
        let percent = (progress * 100) / total_lines;
        let filled_width = (percent / 5) as usize; // 20-character wide bar
        let bar = format!(
            "[{}{}] {}/{}",
            "=".repeat(filled_width),
            " ".repeat(20 - filled_width),
            progress,
            total_lines
        );

        // \r moves cursor to start of line, print! does not add a newline
        print!("\rProcessing: {} {}%", bar, percent);

        // stdout is line-buffered; must flush manually to show progress immediately
        io::stdout().flush().unwrap();
    }

    // Move to a new line once finished
    println!("\nDone!");
    println!("Answer: {}", total_sum);
}

