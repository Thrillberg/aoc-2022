pub fn day_five<'a>(input: std::str::Lines<'a>) {
    let mut stacks: Vec<Vec<&str>> = Vec::new();
    stacks.push(Vec::from(["F", "C", "P", "G", "Q", "R"]));
    stacks.push(Vec::from(["W", "T", "C", "P"]));
    stacks.push(Vec::from(["B", "H", "P", "M", "C"]));
    stacks.push(Vec::from(["L", "T", "Q", "S", "M", "P", "R"]));
    stacks.push(Vec::from(["P", "H", "J", "Z", "V", "G", "N"]));
    stacks.push(Vec::from(["D", "P", "J"]));
    stacks.push(Vec::from(["L", "G", "P", "Z", "F", "J", "T", "R"]));
    stacks.push(Vec::from(["N", "L", "H", "C", "F", "P", "T", "J"]));
    stacks.push(Vec::from(["G", "V", "Z", "Q", "H", "T", "C", "W"]));

    let mut is_instruction = false;
    for line in input {
        if line.is_empty() {
            is_instruction = true;
            continue;
        }

        if is_instruction {
            let quantity_to_move: usize = line.split(" ").collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap();
            let from: usize = line.split(" ").collect::<Vec<&str>>()[3]
                .parse::<usize>()
                .unwrap()
                - 1;
            let to: usize = line.split(" ").collect::<Vec<&str>>()[5]
                .parse::<usize>()
                .unwrap()
                - 1;
            let from_stack_index = stacks[from].len() - quantity_to_move;
            for _ in 0..quantity_to_move {
                let popped_crate = stacks[from].remove(from_stack_index);
                stacks[to].push(popped_crate);
            }
        }
    }

    let mut out = String::new();
    for mut stack in stacks {
        out = out + stack.pop().unwrap();
    }
    println!("{}", out);
}
