use std::fs;

fn load_ranges(input: &String) -> Vec<(u64, u64)> {
    input
        .lines()
        .filter(|&line| line.contains("-"))
        .map(|line| {
            let parts: Vec<u64> = line
                .split("-")
                .map(|part| part.parse::<u64>().unwrap())
                .collect();
            (parts[0], parts[1])
        })
        .collect()
}

fn load_ingredients(input: &String) -> Vec<u64> {
    input
        .lines()
        .filter(|&line| !line.contains("-") && line.len() > 0)
        .map(|line| line.parse::<u64>().unwrap())
        .collect()
}

fn find_fresh_ingredients(ranges: &Vec<(u64, u64)>, ingredients: &Vec<u64>) -> Vec<u64> {
    ingredients
        .iter()
        .copied()
        .filter(|&ingredient| {
            let mut in_range = false;
            for range in ranges {
                if ingredient >= range.0 && ingredient <= range.1 {
                    in_range = true;
                    break;
                }
            }
            in_range
        })
        .collect()
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input file");
    let ranges = load_ranges(&input);
    let ingredients = load_ingredients(&input);
    let fresh_ingredients = find_fresh_ingredients(&ranges, &ingredients);
    println!("Answer: {}", fresh_ingredients.len());
}
