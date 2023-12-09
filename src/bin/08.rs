use std::collections::{HashMap, HashSet};
use std::iter::repeat;

advent_of_code::solution!(8);

type Graph<'a> = HashMap<&'a str, (&'a str, &'a str)>;

pub fn parse(input: &str) -> (&str, Graph) {
    let mut graph = Graph::new();
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();

    lines
        .skip(1)
        .map(|line| line.split_once(" = ").unwrap())
        .for_each(|(key, s)| {
            let neighbors = s[1..s.len() - 1].split_once(", ").unwrap();
            graph.insert(key, neighbors);
        });

    (instructions, graph)
}

pub fn num_steps<F>(graph: &Graph, instructions: &str, start: &str, termination_condition: F) -> u64
where
    F: Fn(&str) -> bool,
{
    let mut instruction = repeat(instructions.chars()).flatten();
    let mut num_steps = 0;
    let mut element = start.clone();
    while !termination_condition(element) {
        let next_elements = graph.get(element).unwrap();
        match instruction.next() {
            Some('L') => element = next_elements.0,
            Some('R') => element = next_elements.1,
            _ => panic!("Invalid instruction"),
        }
        // println!("{}", element);
        num_steps += 1;
    }

    num_steps
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (instructions, graph) = parse(input);
    let answer = num_steps(&graph, instructions, "AAA", |s| s.ends_with('Z'));
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, graph) = parse(input);
    let answer = graph
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|&start| num_steps(&graph, instructions, start, |s| s.ends_with('Z')))
        .fold(1, |acc, x| lcm(acc, x));

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
