use std::fs;

pub fn day_three_part_one() {
    let file_path = "input/day_3";
    if let Ok(input) = fs::read_to_string(file_path) {
        let mut running_total = 0;
        let lines = input.lines();
        for line in lines {
            let half = line.chars().count() / 2;
            let mut first_compartment = Vec::new();
            for item in line.chars().take(half) {
                first_compartment.push(item);
            }

            let mut second_compartment = Vec::new();
            for item in line.chars().rev().take(half) {
                second_compartment.push(item);
            }

            for first_compartment_item in first_compartment {
                if second_compartment.contains(&first_compartment_item) {
                    running_total = running_total + get_priority(first_compartment_item);
                    break;
                }
            }
        }
        println!("{}", running_total);
    }
}

pub fn day_three_part_two() {
    let file_path = "input/day_3";
    if let Ok(input) = fs::read_to_string(file_path) {
        let mut running_total = 0;
        let mut rucksacks = Vec::new();
        let lines = input.lines();
        for line in lines {
            if rucksacks.len() < 3 {
                let mut rucksack = Vec::new();
                for item in line.chars() {
                    rucksack.push(item);
                }
                rucksacks.push(rucksack);
            }

            if rucksacks.len() == 3 {
                for first_rucksack_item in &rucksacks[0] {
                    if rucksacks[1].contains(&first_rucksack_item)
                        && rucksacks[2].contains(&first_rucksack_item)
                    {
                        running_total = running_total + get_priority(*first_rucksack_item);
                        rucksacks = Vec::new();
                        break;
                    }
                }
            }
        }
        println!("{}", running_total);
    }
}

fn get_priority(char: char) -> usize {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    for (index, letter) in alphabet.chars().enumerate() {
        if char == letter {
            return index + 1;
        } else if char.to_string() == letter.to_uppercase().collect::<String>() {
            return index + 27;
        }
    }
    return 0;
}
