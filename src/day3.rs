use std::fs;

pub fn part_one() -> Result<u32, Box<dyn std::error::Error>> {
    let final_joltage = fs::read_to_string("day_three_input.txt")?
        .lines()
        .filter_map(get_joltage)
        .sum();

    Ok(final_joltage)
}

pub fn part_two() -> Result<u32, Box<dyn std::error::Error>> {
    todo!()
}

fn get_joltage(battery_bank: &str) -> Option<u32> {
    todo!()
}

#[cfg(test)]
mod day_three_test {
    use super::*;

    #[test]
    fn joltage_test_1() {
        let joltage = get_joltage("987654321111111");

        assert_eq!(joltage, Some(98));
    }

    #[test]
    fn joltage_test_2() {
        let joltage = get_joltage("811111111111119");

        assert_eq!(joltage, Some(89));
    }

    #[test]
    fn joltage_test_3() {
        let joltage = get_joltage("234234234234278");

        assert_eq!(joltage, Some(78));
    }

    #[test]
    fn joltage_test_4() {
        let joltage = get_joltage("818181911112111");

        assert_eq!(joltage, Some(92));
    }
}
