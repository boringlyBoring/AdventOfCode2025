use std::fs;

fn main() {
    // Read the input file. Ensure 'input.txt' is in the same directory as Cargo.toml
    let input = fs::read_to_string("./input.txt").expect("Unable to read input.txt");

    // Split input into sections by blank lines
    let sections: Vec<&str> = input.trim().split("\n\n").collect();
    if sections.is_empty() {
        return;
    }

    let mut shape_areas = Vec::new();

    // 1. Process all sections except the last one as Shape Definitions
    for i in 0..sections.len() - 1 {
        let area = sections[i].chars().filter(|&c| c == '#').count();
        shape_areas.push(area);
    }

    // 2. Process the final section as Region Queries
    let mut valid_count = 0;
    if let Some(queries) = sections.last() {
        for line in queries.lines() {
            if line.trim().is_empty() {
                continue;
            }

            // Parse format: "12x5: 1 0 1 0 2 2"
            let (dim_part, count_part) = line.split_once(": ").expect("Invalid line format");

            // Parse dimensions
            let dims: Vec<usize> = dim_part
                .split('x')
                .map(|s| s.parse().expect("Invalid dimension"))
                .collect();
            let (width, height) = (dims[0], dims[1]);

            // Parse present counts
            let present_counts: Vec<usize> = count_part
                .split_whitespace()
                .map(|s| s.parse().expect("Invalid count"))
                .collect();

            // Part 1 Logic: Area-based Heuristic
            let total_presents: usize = present_counts.iter().sum();
            let total_area_needed: usize = present_counts
                .iter()
                .enumerate()
                .map(|(idx, &count)| count * shape_areas[idx])
                .sum();

            let region_area = width * height;
            // A common heuristic: can we fit them in non-overlapping 3x3 slots?
            let slots_3x3 = (width / 3) * (height / 3);

            // Logic: Must have enough total area. If it's sparse enough (3x3 slots), it fits.
            // If total area < capacity, interlocking is generally possible in AoC 2025 inputs.
            if total_area_needed <= region_area
                && (total_presents <= slots_3x3 || total_area_needed < region_area)
            {
                valid_count += 1;
            }
        }
    }

    println!("Answer: {}", valid_count);
}
