use std::fs;
use std::str::FromStr;
use std::string::ParseError;

enum Direction {
    Forward,
    Up,
    Down
}

impl FromStr for Direction {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => panic!("Invalid Direction")
        }
    }
}

struct SubCommand {
    pub direction: Direction,
    pub distance: i32
}

impl FromStr for SubCommand {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split_whitespace()
                                 .collect();

        let direction = coords[0].parse::<Direction>()?;
        let distance = match coords[1].parse::<i32>() {
            Ok(i) => i,
            Err(e) => panic!("Failed to parse distance: {}.\n{}", coords[1],e)
        };

        Ok(SubCommand { direction, distance })
    }
}

pub fn calculate() {
    let inputs = fs::read_to_string("./inputs/day_2.txt")
        .expect("Something went wrong reading the file");
    let inputs : Vec<SubCommand> = inputs.lines().map(
        |x| {x.parse().unwrap()}).collect();
    part1(&inputs);
    part2(&inputs);
}

fn part1(inputs: &Vec<SubCommand>) {
    let mut horizontal_position = 0;
    let mut vertical_position = 0; 
    for i in inputs {
        match i.direction {
            Direction::Forward => horizontal_position += i.distance,
            Direction::Up => vertical_position -= i.distance, //should really clamp this value to 0 unless the submarine can fly too...
            Direction::Down => vertical_position += i.distance
        }
    }
    let answer = horizontal_position * vertical_position;
    println!("The answer to day 2 part 1 is: {}", answer);
}

fn part2(inputs: &Vec<SubCommand>) {

}