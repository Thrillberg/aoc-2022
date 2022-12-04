use std::fs;

pub fn day_one() {
    let file_path = "input/day_1";
    let mut current_calories = 0;
    let mut ultimate_calories = 0;
    let mut penultimate_calories = 0;
    let mut antepenultimate_calories = 0;
    let _input = fs::read_to_string(file_path);
    if let Ok(input) = fs::read_to_string(file_path) {
        let lines = input.lines();
        for line in lines {
            if line == "" {
                if current_calories > ultimate_calories {
                    antepenultimate_calories = penultimate_calories;
                    penultimate_calories = ultimate_calories;
                    ultimate_calories = current_calories;
                } else if current_calories > penultimate_calories {
                    antepenultimate_calories = penultimate_calories;
                    penultimate_calories = current_calories;
                } else if current_calories > antepenultimate_calories {
                    antepenultimate_calories = current_calories;
                }
                current_calories = 0;
            } else {
                let calories: i32 = line.parse().unwrap();
                current_calories = current_calories + calories;
            }
        }
    }
    println!(
        "{}",
        (ultimate_calories + penultimate_calories + antepenultimate_calories).to_string()
    )
}
