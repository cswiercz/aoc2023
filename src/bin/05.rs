advent_of_code::solution!(5);

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

type Seeds = Vec<u64>;
type Range = (u64, u64, u64);
type Maps = HashMap<String, Map>;

#[derive(Clone, Debug)]
pub struct Map {
    src: String,
    dst: String,
    ranges: Vec<Range>,
}

pub fn parse_input(input: &str) -> (Seeds, Maps) {
    let re_nums = Regex::new(r"(?:[0-9]+)").unwrap();
    let re_map = Regex::new(r"(?<src>[a-z]+)-to-(?<dst>[a-z]+)").unwrap();

    let mut lines = input
        .split_terminator('\n')
        .map(|line| line.trim())
        .peekable();

    let seeds: Vec<u64> = re_nums
        .find_iter(lines.next().unwrap())
        .map(|s| s.as_str().parse::<u64>().unwrap())
        .collect();
    lines.next();

    let mut maps: Maps = HashMap::new();

    while lines.peek().is_some() {
        let line = lines.next().unwrap();
        let cap = re_map.captures(line).unwrap();
        let mut map = Map {
            src: String::from(&cap["src"]),
            dst: String::from(&cap["dst"]),
            ranges: Vec::new(),
        };

        while let Some(line) = lines.next() {
            if line.is_empty() {
                // TODO: this is bad
                let m = map.clone();
                maps.insert(m.src.clone(), m);
                break;
            } else {
                let r: Range = re_nums
                    .find_iter(line)
                    .map(|s| s.as_str().parse::<u64>().unwrap())
                    .collect_tuple()
                    .unwrap();
                map.ranges.push(r);
            }
        }
    }

    (seeds, maps)
}

////////////////////////////////////////////////////////////////////////////////
/// Solutions
////////////////////////////////////////////////////////////////////////////////

pub fn propagate_seed(seed: u64, maps: &Maps) -> u64 {
    let mut src = String::from("seed");
    let mut dst = String::from("foo");
    let mut value = seed;

    while dst != String::from("location") {
        let ranges = &maps.get(&src).unwrap().ranges;
        dst = maps.get(&src).unwrap().dst.clone();
        value = ranges
            .iter()
            .filter_map(|(d, s, l)| {
                if (value >= *s) & (value < *s + *l) {
                    Some(d + value - s)
                } else {
                    None
                }
            })
            .next()
            .unwrap_or(value);
        src = dst.clone();
    }

    value
}

pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, maps) = parse_input(input);

    seeds
        .iter()
        .map(|&s| propagate_seed(s, &maps))
        .min()
        .and_then(|v| Some(v as u32))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (seeds, maps) = parse_input(input);

    // brute forcing because I'm done with this problem. be sure to
    // run with --release flag. took X minutes on my machine.
    seeds
        .chunks_exact(2)
        .flat_map(|pair| (pair[0])..(pair[0] + pair[1]))
        .map(|s| propagate_seed(s, &maps))
        .min()
        .and_then(|v| Some(v as u32))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
