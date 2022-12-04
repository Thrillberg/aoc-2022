mod day_one;
mod day_three;
mod day_two;
use day_one::day_one;
use day_three::day_three_part_one;
use day_three::day_three_part_two;
use day_two::day_two;

fn main() {
    println!("Day one solution:");
    day_one();
    println!("Day two solution:");
    day_two();
    println!("Day three solution:");
    day_three_part_one();
    day_three_part_two();
}
