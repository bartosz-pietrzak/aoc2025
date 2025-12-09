use std::error::Error;
use std::fs;

type Err = Box<dyn Error>;

fn form_number_form_digits(digits_vec: Vec<u32>) -> u64 {
    let chars: Vec<char> = digits_vec
        .iter()
        .map(|&x| char::from_digit(x, 10).expect("Not a digit"))
        .collect();
    chars
        .iter()
        .collect::<String>()
        .parse()
        .expect("Invalid number")
}

fn find_highest_in_order(num_vec: &Vec<u32>, order: usize) -> (u32, Vec<u32>) {
    let len = num_vec.len();
    let short_by_order = num_vec[..len - order].to_vec();

    let max = short_by_order
        .iter()
        .max()
        .expect("Couldnt find max number");
    let max_idx = short_by_order
        .iter()
        .position(|x| x == max)
        .expect("No number in the vecotor");
    (*max, num_vec[max_idx + 1..].to_vec())
}

fn main() -> Result<(), Err> {
    let path = "input";
    let file = fs::read_to_string(path)?;
    let mut joltage_sum = 0;

    for bank in file.lines() {
        let num_vec: Vec<u32> = bank
            .chars()
            .map(|x| char::to_digit(x, 10).expect("Not a digit"))
            .collect();
        let num_of_digits = 12;
        let mut digits = Vec::new();
        let mut rest = num_vec;
        let mut digit: u32;
        for order in (0..num_of_digits).rev() {
            (digit, rest) = find_highest_in_order(&rest, order);
            digits.push(digit);
        }
        let highest_joltage = form_number_form_digits(digits);
        joltage_sum += highest_joltage;
    }

    println!("{}", joltage_sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_right() {}
}
