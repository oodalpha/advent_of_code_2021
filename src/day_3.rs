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
    let mut working_data = inputs.to_vec();
    for i in 0..inputs[0].len() { //assume first entry is same length as all the others
        if working_data.len()==1
        {
            break;
        }
        let majority_count = working_data.len() as f32/2.0;
        let mut number_of_ones = 0;
        for j in 0..working_data.len() {
            let bit = working_data[j][i];
            match bit {
                true => number_of_ones += 1,
                _ => (),
            }
        }
        let bit_to_keep = number_of_ones as f32 >= majority_count;
        let mut data_copy = Vec::with_capacity(working_data.len());
        for j in 0..working_data.len() {
            let bit = working_data [j][i];
            if bit == bit_to_keep {
                data_copy.push(working_data[j].to_vec());
            }
        }
        working_data = data_copy;
    }
    let oxygen_generator_bits = &working_data[0];
    let mut oxygen_generator_rating = 0;
    for i in 0..oxygen_generator_bits.len() {
        match oxygen_generator_bits[i] {
            true => oxygen_generator_rating += 1 << oxygen_generator_bits.len()-1-i,
            _ => ()
        }
    }

    //now find co2_scrubber
    let mut working_data = inputs.to_vec();
    for i in 0..inputs[0].len() { //assume first entry is same length as all the others
        if working_data.len()==1
        {
            break;
        }
        let majority_count = working_data.len() as f32/2.0;
        let mut number_of_ones = 0;
        for j in 0..working_data.len() {
            let bit = working_data[j][i];
            match bit {
                true => number_of_ones += 1,
                _ => (),
            }
        }
        let bit_to_keep = (number_of_ones as f32) < majority_count;
        let mut data_copy = Vec::with_capacity(working_data.len());
        for j in 0..working_data.len() {
            let bit = working_data [j][i];
            if bit == bit_to_keep {
                data_copy.push(working_data[j].to_vec());
            }
        }
        working_data = data_copy;
    }
    let co2_scrubber_bits = &working_data[0];
    let mut co2_scrubber_rating = 0;
    for i in 0..co2_scrubber_bits.len() {
        match co2_scrubber_bits[i] {
            true => co2_scrubber_rating += 1 << co2_scrubber_bits.len()-1-i,
            _ => ()
        }
    }
    println!("most common: {:b}({}), least common: {:b}({}).", oxygen_generator_rating, oxygen_generator_rating, co2_scrubber_rating, co2_scrubber_rating);
    let answer = oxygen_generator_rating * co2_scrubber_rating;
    println!("The answer to day 3 part 2 is: {}", answer);
}