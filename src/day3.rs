use std::fs;

pub fn part_one() -> Result<u64, Box<dyn std::error::Error>> {
    let final_joltage: u64 = fs::read_to_string("day_three_input.txt")?
        .lines()
        .filter_map(get_joltage)
        .sum();

    Ok(final_joltage)
}

pub fn part_two() -> Result<u32, Box<dyn std::error::Error>> {
    todo!()
}

fn get_joltage(battery_bank: &str) -> Option<u64> {
    let count = battery_bank.chars().count();

    let (index,first) = battery_bank.chars().take(count-1).map(|c| c.to_digit(10)).enumerate().max_by_key(|&(_,val)| val)?;

    let second = battery_bank[index+1..].chars().map(|c| c.to_digit(10)).max()??;

    let thingy:u64 = format!("{}{}",first?,second).parse().ok()?;
    println!("{} => {}", battery_bank, thingy);
    Some(thingy)
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
