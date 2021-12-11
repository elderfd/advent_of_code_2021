use std::str::FromStr;

use super::read_input::read_input;

pub fn solve() -> (i32, i32) {
    let challenge_input = read_input(&"inputs/day_one.txt".to_string());

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_answer() {
        assert_eq!(solve(), (1228, 1257))
    }
}
