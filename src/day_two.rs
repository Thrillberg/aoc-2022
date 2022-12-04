use std::collections::HashMap;
use std::fs;

pub fn day_two() {
    let file_path = "input/day_2";
    let mut running_score = 0;
    let shape_scores = HashMap::from([
        ("A".to_string(), 1),
        ("B".to_string(), 2),
        ("C".to_string(), 3),
    ]);
    let losing_shapes = HashMap::from([
        ("A".to_string(), "C".to_string()),
        ("B".to_string(), "A".to_string()),
        ("C".to_string(), "B".to_string()),
    ]);
    let winning_shapes = HashMap::from([
        ("A".to_string(), "B".to_string()),
        ("B".to_string(), "C".to_string()),
        ("C".to_string(), "A".to_string()),
    ]);
    if let Ok(input) = fs::read_to_string(file_path) {
        let lines = input.lines();
        for line in lines {
            let mut winner = "";
            let opponent_shape: String = line.chars().nth(0).unwrap().to_string();
            let desired_result: String = line.chars().nth(2).unwrap().to_string();
            let my_shape: String;

            if desired_result == "Y" {
                my_shape = opponent_shape.clone();
            } else if desired_result == "X" {
                my_shape = losing_shapes.get(&opponent_shape).unwrap().to_string();
            } else {
                my_shape = winning_shapes.get(&opponent_shape).unwrap().to_string();
            }
            running_score = running_score + shape_scores.get(&my_shape).unwrap();

            // Handle match according to score
            if shape_scores.get(&opponent_shape) > shape_scores.get(&my_shape) {
                winner = "opponent";
            } else if shape_scores.get(&opponent_shape) < shape_scores.get(&my_shape) {
                winner = "me";
            };
            // Handle the fact that scissors beats rock
            if opponent_shape == "C" && my_shape == "A" {
                winner = "me";
            };
            if opponent_shape == "A" && my_shape == "C" {
                winner = "opponent";
            }
            // Update score depending on winner
            if winner == "me" {
                running_score = running_score + 6;
            } else if winner == "" {
                running_score = running_score + 3;
            }
        }
    }
    println!("{}", running_score);
}
