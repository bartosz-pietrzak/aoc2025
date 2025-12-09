use regex::Regex;
use std::error::Error;
use std::fs;

fn merge_ranges(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    ranges.iter().fold(
        Vec::new(),
        |mut merged_ranges: Vec<(usize, usize)>, range| {
            // initialise the fold
            if merged_ranges.is_empty() {
                merged_ranges.push(*range);
                return merged_ranges;
            }
            // next range should always have higher start then prev range
            // check if the last end is lower than next start then put range into vec
            let last = merged_ranges.last().unwrap();
            let last_start = last.0;
            let last_end = last.1;
            let current_start = range.0;
            let current_end = range.1;
            // if current start is larger than prev end then put it into the vector
            if current_start > last_end {
                merged_ranges.push(*range)
            } else {
                // if current start is lower then last end then amend the range to new range
                let _ = merged_ranges.pop().unwrap();
                merged_ranges.push((last_start, current_end.max(last_end)));
            }
            merged_ranges
        },
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "input";
    let file_string = fs::read_to_string(path)?;
    let mut split_inputs = file_string.split("\n\n");
    let ranges_str = split_inputs.next().ok_or("No ranges found")?;
    let ids_str = split_inputs.next().ok_or("No ids found")?;
    let re_ranges = Regex::new(r"(\d+)-(\d+)")?;
    let ranges: Vec<(usize, usize)> = ranges_str
        .lines()
        .filter_map(|line| {
            let caputres = re_ranges.captures(line)?;
            let start = caputres.get(1)?.as_str().parse::<usize>().ok()?;
            let end = caputres.get(2)?.as_str().parse::<usize>().ok()?;
            Some((start, end))
        })
        .collect();
    let ranges = merge_ranges(ranges);
    let ids = ids_str
        .lines()
        .filter_map(|line| line.parse::<usize>().ok());

    let fresh_items = ids
        .filter(|id| ranges.iter().any(|range| (range.0..=range.1).contains(&id)))
        .count();
    println!("Part 1: {}", fresh_items);

    let all_fresh_items = ranges
        .into_iter()
        .flat_map(|(start, end)| start..=end)
        .count();

    println!("Part 2: {}", all_fresh_items);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_merge_ranges() {
        let v = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
        assert_eq!(merge_ranges(v), &[(3, 5), (10, 20)]);
    }
}
