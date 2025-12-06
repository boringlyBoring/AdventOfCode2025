use std::fs;

fn check_invalid_id(num: i64) -> bool {
    let num_str = num.to_string();
    let len = num_str.len();
    if len <= 1 {
        return false;
    }

    let first = &num_str[0..(len / 2)];
    let second = &num_str[(len / 2)..];
    first == second
}

fn main() {
    let file_path = "./input.txt";

    let content = fs::read_to_string(file_path).expect("Unable to read the file");

    let range_list: Vec<&str> = content.split(",").collect();
    let mut answer = 0;

    for range in range_list {
        let r: Vec<&str> = range.split("-").collect();
        let start = r[0]
            .replace("\n", "")
            .parse::<i64>()
            .expect("Unable to parse to Integer");
        let end = r[1]
            .replace("\n", "")
            .parse::<i64>()
            .expect("Unable to parse to Integer");

        for num in start..=end {
            if check_invalid_id(num) {
                answer += num;
            }
        }
    }

    println!("Answer: {}", answer);
}
