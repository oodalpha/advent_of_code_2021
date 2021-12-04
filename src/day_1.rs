use std::fs;

pub fn calculate() {
    let inputs = fs::read_to_string("./inputs/day_1.txt")
        .expect("Something went wrong reading the file");
    let inputs : Vec<i32> = inputs.lines().map(|x| {x.parse().unwrap()}).collect();
    part1(&inputs);
    part2(&inputs);
}

fn part1(inputs: &Vec<i32>) {
    let mut input_increase_count = 0;
    for index in 1..inputs.len() {
        let previous_value = inputs[index-1];
        let current_value = inputs[index];
        if current_value > previous_value {
            input_increase_count += 1;
        }
    }
    println!("The answer to day 1 part 1 is: {}", input_increase_count);
}

fn part2(inputs: &Vec<i32>) {
    let mut input_increase_count = 0;
    for index in 1..inputs.len()-2 {
        let previous_window_total = 
            inputs[index-1] + 
            inputs[index] + 
            inputs[index+1];
        let current_window_total = 
            inputs[index] + 
            inputs[index+1] + 
            inputs[index+2];
        if current_window_total > previous_window_total {
            input_increase_count += 1;
        }
    }
    println!("The answer to day 1 part 2 is: {}", input_increase_count);
}