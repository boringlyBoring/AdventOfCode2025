use std::fs;

fn main() {
    let file_path = "./input.txt";

    let instructions = fs::read_to_string(file_path).expect("Should have read the file");

    let mut pointer = 50;
    let mut password = 0;

    for instruction in instructions.lines() {
        if instruction.len() == 0 {
            continue;
        }
        let direction = &instruction[0..1];
        let number = &instruction[1..]
            .parse::<i32>()
            .expect("Failed to parse the number");
        match direction {
            "L" => {
                pointer = (pointer - number) % 100;
            }
            "R" => {
                pointer = (pointer + number) % 100;
            }
            _ => !panic!("Invalid direction in instruction"),
        }

        if pointer == 0 {
            password += 1;
        }
    }

    println!("password: {}", password);
}
