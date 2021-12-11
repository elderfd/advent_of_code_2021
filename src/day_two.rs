use std::str::FromStr;

use super::read_input::read_input;

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

pub fn solve() -> (i32, i32) {
    let challenge_input = read_input(&"inputs/day_two.txt".to_string());

    let as_instructions: Vec<CourseInstruction> =
        challenge_input.iter().map(parse_instruction).collect();

    let simple_interpretation =
        as_instructions
            .iter()
            .fold((0, 0), |(x, y), instruction| match instruction.direction {
                Direction::Forward => (x + instruction.distance, y),
                Direction::Down => (x, y + instruction.distance),
                Direction::Up => (x, y - instruction.distance),
            });

    let complex_interpretation = as_instructions
        .iter()
        .fold(
            ((0, 0), 0),
            |((x, y), aim), instruction| match instruction.direction {
                Direction::Forward => (
                    (x + instruction.distance, y + aim * instruction.distance),
                    aim,
                ),
                Direction::Down => ((x, y), aim + instruction.distance),
                Direction::Up => ((x, y), aim - instruction.distance),
            },
        )
        .0;

    return (
        simple_interpretation.0 * simple_interpretation.1,
        complex_interpretation.0 * complex_interpretation.1,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_answer() {
        assert_eq!(solve(), (1815044, 1739283308))
    }
}
