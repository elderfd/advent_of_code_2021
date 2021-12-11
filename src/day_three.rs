use super::read_input::read_input;

fn bits_to_number(bits: &String) -> i32 {
    return match i32::from_str_radix(bits, 2) {
        Ok(v) => v,
        Err(_) => panic!("Failed to parse mode string!"),
    };
}

pub fn solve() -> i32 {
    let challenge_input = read_input(&"inputs/day_three.txt".to_string());

    let number_bits = challenge_input[0].len();

    let most_common_bits = &(0..number_bits)
        .collect::<Vec<usize>>()
        .iter()
        .map(|position| {
            let counts = challenge_input.iter().fold((0, 0), |count, input| {
                match input.chars().nth(*position) {
                    Some(v) => match v {
                        '0' => (count.0 + 1, count.1),
                        '1' => (count.0, count.1 + 1),
                        v => panic!("Invalid input {}", v),
                    },
                    None => panic!("Tried to read out of range of string"),
                }
            });

            return if counts.0 > counts.1 { "0" } else { "1" };
        })
        .collect::<Vec<&str>>()
        .join("");

    let gamma_rate = bits_to_number(most_common_bits);
    let epsilon_rate = bits_to_number(
        &most_common_bits
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
            .join(""),
    );

    return gamma_rate * epsilon_rate;
}
