use std::fs;

fn get_vertical(input: &String) -> Vec<i64> {
    let mut symbol: Vec<&str> = Vec::new();
    let mut number: Vec<i64> = Vec::new();
    let mut index = 0;
    for line in input.lines().rev() {
        if index == 0 {
            symbol = line.split(" ").filter(|&part| !part.is_empty()).collect();
        } else if index == 1 {
            number = line
                .split(" ")
                .filter(|&part| !part.is_empty())
                .map(|part| part.parse::<i64>().unwrap())
                .collect();
        } else {
            let next_numbers: Vec<i64> = line
                .split(" ")
                .filter(|&part| !part.is_empty())
                .map(|part| part.parse::<i64>().unwrap())
                .collect();
            for i in 0..symbol.len() {
                if symbol[i] == "+" {
                    number[i] = number[i] + next_numbers[i];
                } else {
                    number[i] = number[i] * next_numbers[i];
                }
            }
        }
        index += 1;
    }
    number
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read file");
    let numbers = get_vertical(&input);
    let mut answer = 0;
    for number in numbers {
        answer += number;
    }
    println!("Answer: {}", answer);
}
