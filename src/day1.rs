use std::fs;

use regex::Regex;

pub fn part_one() -> Result<i32, Box<dyn std::error::Error>> {
    let file_contents = fs::read_to_string("day_one_test.txt")?;

    let mut dial = Dial::new();
    let mut zero_count = 0;
    for line in file_contents.lines() {
        let rotation = Rotation::new(line).unwrap();
        zero_count += dial.rotate(&rotation);
    }
    
    Ok(zero_count)
}

struct Dial {
    position: i32,
}

impl Dial {
    fn new() -> Dial {
        Dial { position: 50 }
    }

    fn rotate(&mut self, rotation: &Rotation) -> i32 {
        let rotation = match rotation {
            Rotation::Left(ticks) => -(*ticks as i32),
            Rotation::Right(ticks) => *ticks as i32,
        };

        let dial_long = self.position + rotation;
        let mut passes_through_zero = (dial_long / 100).abs();
        
        if self.position != 0 && dial_long <= 0 {
            passes_through_zero += 1;
        }
        
        self.position = (dial_long).rem_euclid(100);
        passes_through_zero
    }
}

#[derive(Debug, PartialEq)]
enum Rotation {
    Left(u32),
    Right(u32)
}

impl Rotation {
    fn new(string_code: &str) -> Option<Rotation> {
        let parse_regex: Regex = Regex::new(r"([LR])(\d+)").ok()?;
        if let Some(captures) = parse_regex.captures(string_code) {
            let direction_code: &str = captures.get(1)?.as_str();
            let ticks: u32 = captures.get(2)?.as_str().parse::<u32>().ok()?;

            return match direction_code{
                "L" => Some(Rotation::Left(ticks)),
                "R" => Some(Rotation::Right(ticks)),
                _ => None,
            };
        }

        None
    }
}

#[cfg(test)]
mod rotation_tests {
    use super::*;

    #[test]
    fn create_rotation() {
        let test_rotation = Rotation::new("L32");
        assert!(test_rotation.is_some());
        assert_eq!(test_rotation.unwrap(), Rotation::Left(32));
    }

    #[test]
    fn fail_to_create_rotation() {
        let test_rotation = Rotation::new("G234");
        assert!(test_rotation.is_none());
    }
}