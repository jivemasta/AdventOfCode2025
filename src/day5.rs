use std::fs;

use regex::Regex;

pub fn part_one() -> Result<u32, Box<dyn std::error::Error>> {
    let input_string = fs::read_to_string("day_four_input.txt")?;
    let (fresh_inventory_string, inventory_string) = get_freshness_lists(&input_string).unwrap();

    let fresh_inventory = get_freshness_range(fresh_inventory_string).unwrap();
    let inventory = dbg!(get_inventory(inventory_string).unwrap());

    println!("ranges: {}", fresh_inventory.iter().count());
    println!("inventory count: {}", inventory.iter().count());
    let fresh_count = inventory.into_iter().filter_map(|item| check_freshness(item, fresh_inventory.clone())).count();
    println!("Count: {}", fresh_count);
    Ok(fresh_count.try_into()?)
}

fn get_freshness_lists(input_string: &str) -> Option<(&str,&str)> {
    let list_parsing_regex = Regex::new(r"((?:(?:\d+)-(?:\d+)\r\n)+)\r\n((?:(?:\d+)(?:\r\n)?)+)").unwrap();
    let captures = list_parsing_regex.captures(input_string)?;


    Some((captures.get(1)?.as_str(), captures.get(2)?.as_str()))
}

fn get_freshness_range(freshness_range_string: &str) -> Option<Vec<(u64,u64)>> {
    let range_parsing_regex = Regex::new(r"(\d+)-(\d+)").unwrap();
    
    let mut ranges: Vec<(u64,u64)> = vec![];
    for line in freshness_range_string.lines(){
        let capture = range_parsing_regex.captures(line)?;
        ranges.push((capture.get(1)?.as_str().parse::<u64>().ok()?,capture.get(2)?.as_str().parse::<u64>().ok()?));
    }

    Some(ranges)
}

fn get_inventory(inventory_string: &str) -> Option<Vec<u64>> {
    let mut inventory: Vec<u64> = vec![];
    let inventory_parsing_regex = Regex::new(r"(\d+)").unwrap();
    for line in inventory_string.lines(){
        let capture = inventory_parsing_regex.captures(line)?;
        inventory.push(capture.get(1)?.as_str().parse::<u64>().ok()?);
    }

    Some(inventory)
}

fn check_freshness(inventory_item: u64, fresh_inventory: Vec<(u64,u64)>) -> Option<()> {
    for (start, end) in fresh_inventory{
        if inventory_item >= start && inventory_item <= end {
            return Some(());
        }
    }

    None
}