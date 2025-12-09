use std::fs;

use regex::Regex;

pub fn part_one() -> Result<i32, Box<dyn std::error::Error>> {
    let file_contents = fs::read_to_string("day_one_test.txt")?;

    let mut dial: Dial = Dial::new();
    let mut zero_count: i32 = 0;
    for line in file_contents.lines() {
        let rotation: Rotation = Rotation::new(line).unwrap_or(Rotation::Left(0));
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
        // Convert rotation direction to positive/negative values
        let rotation: i32 = match rotation {
            Rotation::Left(ticks) => -(*ticks as i32),
            Rotation::Right(ticks) => *ticks as i32,
        };

        // Get the un-wrapped new position of the dial
        let non_wrapped_position: i32 = self.position + rotation;

        // Start passes through zero as the total number of rotations
        let mut passes_through_zero: i32 = (non_wrapped_position / 100).abs();

        // If the dial didn't start at zero, but went negative, add an extra pass through zero
        if self.position != 0 && non_wrapped_position <= 0 {
            passes_through_zero += 1;
        }

        // wrap the position to be between 0-99 again
        self.position = (non_wrapped_position).rem_euclid(100);

        passes_through_zero
    }
}

#[derive(Debug, PartialEq)]
enum Rotation {
    Left(u32),
    Right(u32),
}

impl Rotation {
    fn new(string_code: &str) -> Option<Rotation> {
        // Regex that can parse rotations from file
        let parse_regex = Regex::new(r"([LR])(\d+)").ok()?;

        // set captures if the regex matches the rotation code, otherwise return None
        let captures = parse_regex.captures(string_code)?;

        // Extract the direction and ticks from the code
        let direction_code = captures.get(1)?.as_str();
        let ticks = captures.get(2)?.as_str().parse::<u32>().ok()?;

        // Return a rotation struct
        match direction_code {
            "L" => Some(Rotation::Left(ticks)),
            "R" => Some(Rotation::Right(ticks)),
            _ => None,
        }
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
