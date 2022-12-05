mod day_four;
mod day_one;
mod day_three;
mod day_two;
use day_four::day_four;
use day_one::day_one;
use day_three::day_three_part_one;
use day_three::day_three_part_two;
use day_two::day_two;
use std::fs;

fn main() {
    println!("Day one solution:");
    day_one();
    println!("Day two solution:");
    day_two();
    println!("Day three solution:");
    day_three_part_one();
    day_three_part_two();
    println!("Day four solution:");
    if let Ok(input) = fs::read_to_string("input/day_4") {
        let day_four_lines = input.lines().clone();
        day_four(day_four_lines);
    }
}
