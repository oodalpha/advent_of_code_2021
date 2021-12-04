use std::fs;

pub fn get_answer() {
    let inputs = fs::read_to_string("./inputs/day_1.txt")
        .expect("Something went wrong reading the file");
    let inputs : Vec<i32> = inputs.lines().map(|x| {x.parse().unwrap()}).collect();
    let mut input_increase_count = 0;
    let mut previous_value = inputs[0];
    for index in 1..inputs.len() {
        let current_value = inputs[index];
        if current_value > previous_value {
            input_increase_count += 1;
        }
        previous_value = current_value;
    }
    println!("The answer is: {}", input_increase_count);
}