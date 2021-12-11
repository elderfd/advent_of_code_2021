use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

fn day_one() -> (i32, i32) {
    let in_file_name = "inputs/ex1.txt";

    let file = File::open(in_file_name);

    if file.is_err() {
        panic!("Could not open {}", in_file_name);
    }

    let mut challenge_input: Vec<String> = Vec::new();

    for line in io::BufReader::new(file.unwrap()).lines() {
        match line {
            Ok(v) => challenge_input.push(v),
            Err(_) => panic!("Failed to read line"),
        }
    }

    let input_numbers: Vec<i32> = challenge_input
        .iter()
        .map(|i| match i32::from_str(i) {
            Ok(v) => v,
            Err(_) => panic!("Invalid input"),
        })
        .collect();

    let number_of_increases = input_numbers
        .iter()
        .fold((0, None), |acc, v| match acc {
            (count, None) => (count, Some(v)),
            (count, Some(last_v)) => (if last_v < v { count + 1 } else { count }, Some(v)),
        })
        .0;

    let number_of_summed_increases = input_numbers
        .windows(3)
        .fold((0, None), |acc: (i32, Option<i32>), v: &[i32]| {
            let triplet_sum = v.iter().sum();

            match acc {
                (count, None) => (count, Some(triplet_sum)),
                (count, Some(last_v)) => (
                    if last_v < triplet_sum {
                        count + 1
                    } else {
                        count
                    },
                    Some(triplet_sum),
                ),
            }
        })
        .0;

    return (number_of_increases, number_of_summed_increases);
}

enum Direction {
    Forward,
    Down,
    Up,
}

struct CourseInstruction {
    direction: Direction,
    distance: i32,
}

fn parse_instruction(input: &String) -> CourseInstruction {
    let parts: Vec<&str> = input.split(" ").collect();

    if parts.len() != 2 {
        panic!("Invalid direction {}", input);
    }

    let direction = match parts[0] {
        "forward" => Direction::Forward,
        "down" => Direction::Down,
        "up" => Direction::Up,
        _ => panic!("Invalid direction {}", input),
    };

    let distance = match i32::from_str(parts[1]) {
        Ok(v) => v,
        Err(_) => panic!("Invalid direction {}", input),
    };

    return CourseInstruction {
        direction,
        distance,
    };
}

fn day_two() -> i32 {
    let in_file_name = "inputs/ex2.txt";

    let file = File::open(in_file_name);

    if file.is_err() {
        panic!("Could not open {}", in_file_name);
    }

    let mut challenge_input: Vec<String> = Vec::new();

    for line in io::BufReader::new(file.unwrap()).lines() {
        match line {
            Ok(v) => challenge_input.push(v),
            Err(_) => panic!("Failed to read line"),
        }
    }

    let as_instructions: Vec<CourseInstruction> =
        challenge_input.iter().map(parse_instruction).collect();

    let final_position =
        as_instructions
            .iter()
            .fold((0, 0), |(x, y), instruction| match instruction.direction {
                Direction::Forward => (x + instruction.distance, y),
                Direction::Down => (x, y + instruction.distance),
                Direction::Up => (x, y - instruction.distance),
            });

    return final_position.0 * final_position.1;
}

fn main() {
    println!("Hello, world!");

    println!("Day one solution: {:?}", day_one());
    println!("Day two solution: {:?}", day_two());
}
