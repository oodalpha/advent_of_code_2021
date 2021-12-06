use std::fs;

pub fn calculate() {
    let inputs = fs::read_to_string("./inputs/day_3.txt")
        .expect("Something went wrong reading the file");
    let inputs : Vec<&str> = inputs.lines().collect();
    let mut bits = Vec::with_capacity(inputs.len());
    for line in inputs {
        let line_of_bits = line.chars().map(|x| {
            match x {
                '0' => false,
                '1' => true,
                _ => panic!("invalid character: {}.", x)
            }
        }).collect();
        bits.push(line_of_bits);
    }
    part1(&bits);
    part2(&bits);
}

fn part1(inputs: &Vec<Vec<bool>>) {
    let majority_count = inputs.len()/2;
    let mut number_of_ones : Vec<usize> = vec![0; inputs[0].len()];//assume all data is the same length
    for i in 0..inputs.len() {
        let line = &inputs[i];
        for j in 0..line.len() {
            let bit = line[j];
            match bit {
                true => number_of_ones[j] += 1,
                _ => (),
            }
        }
    }
    println!("number of inputs: {} ", inputs.len());

    let mut most_common_values = 0;
    let mut least_common_values = 0; //can't just NOT most common values because it will fill up unused columns with 1s. 
    for i in 0..number_of_ones.len() {
        let count_at_index = number_of_ones[i];
        if count_at_index >= majority_count {
            most_common_values += 1 << number_of_ones.len()-1-i;
        }
        else {
            least_common_values += 1 << number_of_ones.len()-1-i;
        }
    }
    println!("most common: {:b}({}), least common: {:b}({}).", most_common_values, most_common_values, least_common_values, least_common_values);
    let answer = most_common_values * least_common_values;
    println!("The answer to day 3 part 1 is: {}", answer);
}

fn part2(inputs: &Vec<Vec<bool>>) {
    let answer = 0;
    println!("The answer to day 2 part 2 is: {}", answer);
}