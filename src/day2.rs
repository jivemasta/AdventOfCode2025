use std::fs;

use fancy_regex::Regex;

pub fn part_one() -> Result<u64, Box<dyn std::error::Error>> {
    let invalid_sum = fs::read_to_string("day_two_input.txt")?
        .lines()
        .filter_map(get_range)
        .filter_map(get_invalid_ids)
        .sum();

    Ok(invalid_sum)
}

pub fn part_two() -> Result<u64, Box<dyn std::error::Error>> {
    let invalid_sum = fs::read_to_string("day_two_input.txt")?
        .lines()
        .filter_map(get_range)
        .filter_map(get_invalid_ids_part_two)
        .sum();

    Ok(invalid_sum)
}

fn get_invalid_ids(range: (u64, u64)) -> Option<u64> {
    // Regex that can find a single repeating pattern in the id
    let id_parse_regex = Regex::new(r"^(\d+)(\1)$").unwrap();

    // Return the sum of ids in the range that match the pattern
    Some((range.0..=range.1)
        .filter(|id| id_parse_regex.is_match(&id.to_string()).unwrap_or(false))
        .sum())
}

fn get_invalid_ids_part_two(range: (u64, u64)) -> Option<u64> {
    // Regex that can fine multipl repeating patterns in the id
    let id_parse_regex = Regex::new(r"^(\d+)(\1)+$").unwrap();

    // Return the sum of ids in the range that match the pattern
    Some((range.0..=range.1)
        .filter(|id| id_parse_regex.is_match(&id.to_string()).unwrap_or(false))
        .sum())
}

fn get_range(line: &str) -> Option<(u64, u64)> {
    let range_regex = Regex::new(r"(\d+)-(\d+)").ok()?;

    if let Ok(Some(captures)) = range_regex.captures(line) {
        let start: u64 = captures.get(1)?.as_str().parse::<u64>().ok()?;
        let end: u64 = captures.get(2)?.as_str().parse::<u64>().ok()?;

        return Some((start, end));
    }

    None
}

#[cfg(test)]
mod day_two_test {
    use super::*;

    #[test]
    fn range_test_1() {
        let (start, end) = get_range("123-345").unwrap();

        assert_eq!(start, 123);
        assert_eq!(end, 345);
    }

    #[test]
    fn range_test_2() {
        let (start, end) = get_range("456-7891").unwrap();

        assert_eq!(start, 456);
        assert_eq!(end, 7891);
    }

    #[test]
    fn id_test_1() {
        let result = get_invalid_ids((11, 22));

        assert_eq!(result, Some(33));
    }

    #[test]
    fn id_test_2() {
        let result = get_invalid_ids((95, 115));

        assert_eq!(result, Some(99));
    }

    #[test]
    fn id_test_1_2() {
        let result = get_invalid_ids_part_two((11, 22));

        assert_eq!(result, Some(33));
    }

    #[test]
    fn id_test_2_2() {
        let result = get_invalid_ids_part_two((95, 115));

        assert_eq!(result, Some(210));
    }
}
