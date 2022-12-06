use std::collections::HashSet;
use std::fs;

pub fn day_six() {
    if let Ok(input) = fs::read_to_string("input/day_6") {
        let mut out = 0;
        let chars = input.chars();
        let mut last_13_chars: Vec<char> = Vec::new();

        for (index, char) in chars.enumerate() {
            if last_13_chars.len() < 14 {
                last_13_chars.push(char);
            } else {
                last_13_chars.remove(0);
                last_13_chars.push(char);
                if HashSet::<char>::from_iter(last_13_chars.clone()).len() < 14 {
                    continue;
                } else {
                    out = index + 1;
                    break;
                }
            }
        }

        println!("{}", out);
    }
}
