advent_of_code::solution!(5);

use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

type Range = (u32, u32, u32);
#[derive(Clone)]
struct Map {
    src: String,
    dst: String,
    ranges: Vec<Range>
}

fn parse_input(input: &str) -> () {
    let re_seeds = Regex::new(r"(?:[0-9]+").unwrap();
    let re_map = Regex::new(r"(?<src>[a-z]+)-to-(?<dst>[a-z]+)").unwrap();
    let re_range = Regex::new(r"(?<from>[0-9]+)\s(?<to>[0-9]+)\s(?<len>[0-9]+").unwrap();

    let mut lines = input
        .split_terminator('\n')
        .map(|line| line.trim())
        .peekable();
    
    let seeds: Vec<u32> = re_seeds
        .find_iter(lines.next().unwrap())
        .map(|s| s.as_str().parse::<u32>().unwrap())
        .collect();
    lines.next();

    let mut maps: HashMap<String, Map> = HashMap::new();
    let mut map: Map;

    while lines.peek().is_some() {
        let cap = re_map.captures(lines.next().unwrap()).unwrap();
        let mut map = Map { src: String::from(&cap["src"]), dst: String::from(&cap["dst"]), ranges: Vec::new() };

        while let Some(line) = lines.next() {
            if line.is_empty() {
                let m = map.clone();
                maps.insert(m.src.clone(), m);
            } else {
                let cap = re_range.captures(line).unwrap();
                let range: Range = cap
                    .iter()
                    .map(|s| s.unwrap().as_str().parse::<u32>().unwrap())
                    .collect_tuple()
                    .unwrap();
                map.ranges.push(range);
            }
        }
    }

    
}

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
