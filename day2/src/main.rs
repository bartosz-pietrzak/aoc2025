use regex::Regex;
use std::error::Error;
use std::fs;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_diff() {}
}

fn into_chunks(num_str: &str, parts: usize) -> Vec<&str> {
    let len = num_str.len();
    let size = len / parts;
    (0..len)
        .step_by(size)
        .map(|i| &num_str[i..(i + size)])
        .collect()
}

fn has_repeating_pattern(num: i64) -> i64 {
    let internal_sum = 0;
    let num_str = num.to_string();
    let num_range = 2..=num_str.len();
    for parts in num_range {
        if num_str.len() % parts == 0 {
            let r = into_chunks(&num_str, parts);
            let is_invalid = r.windows(2).all(|w| w[0] == w[1]);
            if is_invalid {
                return num;
            }
        }
    }
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "input";
    let f = fs::read_to_string(path)?;
    let mut invalid_sums = 0;
    for num_range in f.split(",") {
        let ids: Vec<i64> = num_range
            .split("-")
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect();
        let ids_range = ids[0]..=ids[1];
        for id in ids_range {
            invalid_sums += has_repeating_pattern(id);
        }
    }
    println!("{}", invalid_sums);
    Ok(())
}
