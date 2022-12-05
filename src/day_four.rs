pub fn day_four<'a>(input: std::str::Lines<'a>) {
    let lines = input.clone();
    let mut count_of_redundant_elves = 0;
    for line in lines {
        let elves: Vec<&str> = line.split(",").collect();
        let first_elf = elves[0];
        let second_elf = elves[1];
        let first_elf_bounds: Vec<&str> = first_elf.split("-").collect();
        let second_elf_bounds: Vec<&str> = second_elf.split("-").collect();
        let first_elf_bounds_ints: Vec<i32> = Vec::from([
            first_elf_bounds[0].parse::<i32>().unwrap(),
            first_elf_bounds[1].parse::<i32>().unwrap(),
        ]);
        let second_elf_bounds_ints: Vec<i32> = Vec::from([
            second_elf_bounds[0].parse::<i32>().unwrap(),
            second_elf_bounds[1].parse::<i32>().unwrap(),
        ]);

        if first_elf_bounds_ints[0] >= second_elf_bounds_ints[0]
            && first_elf_bounds_ints[0] <= second_elf_bounds_ints[1]
            || second_elf_bounds_ints[0] >= first_elf_bounds_ints[0]
                && second_elf_bounds_ints[0] <= first_elf_bounds_ints[1]
        {
            count_of_redundant_elves = count_of_redundant_elves + 1
        }
    }
    println!("{}", count_of_redundant_elves);
}
