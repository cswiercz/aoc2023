advent_of_code::solution!(5);

use core::num;
use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use regex::Regex;

type Seeds = Vec<u32>;
type Range = (u32, u32, u32);
type Maps = HashMap<String, Map>;

#[derive(Clone, Debug)]
struct Map {
    src: String,
    dst: String,
    ranges: Vec<Range>
}

fn parse_input(input: &str) -> (Seeds, Maps) {
    let re_nums = Regex::new(r"(?:[0-9]+)").unwrap();
    let re_map = Regex::new(r"(?<src>[a-z]+)-to-(?<dst>[a-z]+)").unwrap();

    let mut lines = input
        .split_terminator('\n')
        .map(|line| line.trim())
        .peekable();
    
    let seeds: Vec<u32> = re_nums
        .find_iter(lines.next().unwrap())
        .map(|s| s.as_str().parse::<u32>().unwrap())
        .collect();
    lines.next();

    let mut maps: Maps = HashMap::new();
    while lines.peek().is_some() {
        let line = lines.next().unwrap();
        let cap = re_map.captures(line).unwrap();
        let mut map = Map { src: String::from(&cap["src"]), dst: String::from(&cap["dst"]), ranges: Vec::new() };

        while let Some(line) = lines.next() {
            if line.is_empty() {
                let m = map.clone();
                maps.insert(m.src.clone(), m);
                break;
            } else {
                let r: Range = re_nums.find_iter(line).map(|s| s.as_str().parse::<u32>().unwrap()).collect_tuple().unwrap();
                map.ranges.push(r);
            }
        }
    }

    (seeds, maps)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, maps) = parse_input(input);
    let num_seeds = seeds.len();

    let mut src = String::from("seed");
    let mut dst: String;
    let mut stack: VecDeque<u32> = VecDeque::from(seeds);
    while src.as_str() != "location" {
        let map = maps.get(&src).unwrap();
        println!("stack: {:?}", stack);
        println!("map:   {:?}", map);
        println!("\t{} -> {}", src, &map.dst);
        for _ in 0..num_seeds {
            let val = stack.pop_front().unwrap();
            let next_val = map
                .ranges
                .iter()
                .filter(|&&(s, _, l)| (val >= s) & (val < s + l))
                .inspect()
                .map(|&(s, d, _)| d + (val - s))
                .next()
                .unwrap();

            println!("\t{} -> {}", val, next_val);
        }
    }

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
