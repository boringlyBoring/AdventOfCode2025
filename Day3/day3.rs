use std::fs;

fn main() {
    let file_path = "./input.txt";

    let battery_bank = fs::read_to_string(file_path).expect("Unable to read input from give path");
    let mut total_voltage: i32 = 0;

    for battery in battery_bank.lines() {
        if battery.len() == 0 {
            continue;
        }
        let mut largest_prefix: i32 = 0;
        let mut largest_volt: i32 = 0;

        for volt in battery.chars() {
            let volt_value: i32 = volt.to_digit(10).unwrap() as i32;
            let current_volt = largest_prefix * 10 + volt_value;
            if current_volt > largest_volt {
                largest_volt = current_volt;
            }
            if volt_value > largest_prefix {
                largest_prefix = volt_value;
            }
        }
        total_voltage += largest_volt;
    }
    println!("Total voltage: {}", total_voltage);
}
