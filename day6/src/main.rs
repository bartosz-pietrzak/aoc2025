use regex::Regex;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "input";
    let file_string = fs::read_to_string(path)?;
    let lines: Vec<&str> = file_string.lines().collect();
    let re_signs = Regex::new(r"([+*-])")?;
    let numbers: Vec<usize> = lines
        .iter()
        .filter_map(|line| {
            let numbers: Vec<usize> = line
                .split_whitespace()
                .filter_map(|num_str| num_str.trim().parse::<usize>().ok())
                .collect();
            Some(numbers)
        })
        .flatten()
        .collect();

    let signs: Vec<char> = lines
        .iter()
        .filter_map(|line| {
            let signs = re_signs
                .find_iter(line)
                .filter_map(|sign| sign.as_str().parse::<char>().ok());
            Some(signs)
        })
        .flatten()
        .collect();
    let signs_len = signs.len();
    let sum_of_operations = (0..signs_len).fold(0, |sum, column_idx| {
        let num_in_column: usize = numbers.len() / signs_len;
        let nums_for_op = (0..num_in_column).map(|row_idx| {
            *numbers
                .get(row_idx * signs_len + column_idx)
                .expect("No number")
        });
        let sign = signs.get(column_idx);
        let res = nums_for_op
            .reduce(|acc, v| match sign {
                Some('+') => acc + v,
                Some('-') => acc - v,
                Some('*') => acc * v,
                Some(_) => 0,
                None => 0,
            })
            .expect("Failed to reduce values");

        sum + res
    });

    println!("part1: {}", sum_of_operations);

    let numbers_as_char_in_row: Vec<Vec<char>> = lines
        .iter()
        .filter_map(|line| Some(line.chars().collect::<Vec<char>>()))
        .collect();
    let chars_in_row_len = numbers_as_char_in_row.get(0).expect("No vector").len();
    //check all rows are same length
    let same_length = numbers_as_char_in_row
        .windows(2)
        .all(|pair| pair[0].len() == pair[1].len());
    assert!(same_length);
    let re_number = Regex::new(r"(\d+)")?;

    let numbers_and_signs_to_calculate: Vec<(Option<usize>, Option<char>)> = (0..chars_in_row_len)
        .map(|column_idx| {
            let numbers_str: String = (0..numbers_as_char_in_row.len())
                .filter_map(|row_idx| {
                    Some(
                        *numbers_as_char_in_row
                            .get(row_idx)
                            .expect("No char vectors")
                            .get(column_idx)
                            .expect("No char at index"),
                    )
                })
                .collect();
            let number = match re_number.find(numbers_str.as_str()) {
                Some(n) => n.as_str().parse::<usize>().ok(),
                None => None,
            };

            let sign = match re_signs.find(numbers_str.as_str()) {
                Some(s) => s.as_str().parse::<char>().ok(),
                None => None,
            };

            (number, sign)
        })
        .collect();

    let grouped = numbers_and_signs_to_calculate.iter().fold(
        vec![(vec![], '+')],
        |mut groups: Vec<(Vec<usize>, char)>, num_and_signs| {
            let last_group = groups.last_mut().unwrap();
            let (number, sign) = num_and_signs;
            match sign {
                Some(s) => last_group.1 = *s,
                None => (),
            }
            match number {
                Some(n) => last_group.0.push(*n),
                None => groups.push((vec![], '+')),
            }
            groups
        },
    );
    let sum_part_2 = grouped.into_iter().fold(0, |sum, group| {
        let (num, sign) = group;
        let ret = num
            .iter()
            .copied()
            .reduce(|acc, v| match sign {
                '+' => acc + v,
                '-' => acc - v,
                '*' => acc * v,
                _ => panic!("Wrong sign"),
            })
            .expect("Cant accumulate");
        sum + ret
    });
    println!("part2: {}", sum_part_2);
    Ok(())
}
