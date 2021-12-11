use super::read_input::read_input;

fn bits_to_number(bits: &String) -> i32 {
    return match i32::from_str_radix(bits, 2) {
        Ok(v) => v,
        Err(_) => panic!("Failed to parse mode string!"),
    };
}

fn evaluate_bits(
    input: &Vec<&String>,
    most_common: Option<bool>,
    zero_wins_draw: Option<bool>,
) -> String {
    let common = match most_common {
        Some(v) => v,
        None => true,
    };

    let zero_wins = match zero_wins_draw {
        Some(v) => v,
        None => false,
    };
    let number_bits = input[0].len();

    return (0..number_bits)
        .collect::<Vec<usize>>()
        .iter()
        .map(|position| {
            let counts =
                input
                    .iter()
                    .fold((0, 0), |count, input| match input.chars().nth(*position) {
                        Some(v) => match v {
                            '0' => (count.0 + 1, count.1),
                            '1' => (count.0, count.1 + 1),
                            v => panic!("Invalid input {}", v),
                        },
                        None => panic!("Tried to read out of range of string"),
                    });

            if counts.0 == counts.1 {
                return if zero_wins { "0" } else { "1" };
            }

            if common {
                return if counts.0 > counts.1 { "0" } else { "1" };
            }

            return if counts.0 < counts.1 { "0" } else { "1" };
        })
        .collect::<Vec<&str>>()
        .join("");
}

pub fn solve() -> (i32, i32) {
    let challenge_input = read_input(&"inputs/day_three.txt".to_string());

    let input_ref = challenge_input.iter().map(|v| v).collect::<Vec<&String>>();

    // TODO: above map must have an idiom somewhere

    let number_bits = input_ref[0].len();

    let most_common_bits = &evaluate_bits(&input_ref, None, None);

    let least_common_bits = &most_common_bits
        .chars()
        .map(|char| {
            match char {
                '0' => '1',
                '1' => '0',
                v => panic!("Somehow found {} in bit string", v),
            }
            .to_string()
        })
        .collect::<Vec<String>>()
        .join("");

    let gamma_rate = bits_to_number(most_common_bits);
    let epsilon_rate = bits_to_number(least_common_bits);

    let power_consumption = gamma_rate * epsilon_rate;

    let oxygen_rating = bits_to_number(
        &(0..number_bits)
            .collect::<Vec<usize>>()
            .iter()
            .fold(input_ref.clone(), |acc, position| {
                if acc.len() == 1 {
                    return acc;
                }

                let desired_value = evaluate_bits(&acc, Some(true), Some(false))
                    .chars()
                    .nth(*position);

                return acc
                    .iter()
                    .filter(|el| el.chars().nth(*position) == desired_value)
                    .map(|v| *v)
                    .collect();
            })
            .iter()
            .map(|&v| v.clone())
            .collect::<Vec<String>>()
            .join(""),
    );

    let co2_rating = bits_to_number(
        &(0..number_bits)
            .collect::<Vec<usize>>()
            .iter()
            .fold(input_ref.clone(), |acc, position| {
                if acc.len() == 1 {
                    return acc;
                }

                let desired_value = evaluate_bits(&acc, Some(false), Some(true))
                    .chars()
                    .nth(*position);

                return acc
                    .iter()
                    .filter(|el| el.chars().nth(*position) == desired_value)
                    .map(|v| *v)
                    .collect();
            })
            .iter()
            .map(|&v| v.clone())
            .collect::<Vec<String>>()
            .join(""),
    );

    let life_support_rating = oxygen_rating * co2_rating;

    return (power_consumption, life_support_rating);
}
