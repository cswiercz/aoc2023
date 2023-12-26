use std::cmp::Ordering;

advent_of_code::solution!(19);

pub struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

pub struct Rule {
    f: fn(Part) -> bool,
    dst: String,
}
pub struct Workflow {
    name: String,
    rules: Vec<Rule>,
    end_dst: String,
}

pub fn parse_rule(s: &str) -> Rule {
    let (rule_str, dst) = s.split_once(':').unwrap();
    let ord = rule_str.chars().nth(1).unwrap();
    let (part, val) = rule_str.split_once(ord).unwrap();
    let val = val.parse::<u32>().unwrap();




    Rule { f: |p| p.a > 0, dst: String::from(dst) }
}

pub fn parse_workflow(s: &str) -> Workflow {
    let (name, rem) = s.split_once('{').unwrap();
    let rules_str: Vec<&str> = rem.strip_suffix('}').unwrap().split(',').collect();
    let (end_dst, rules_str) = rules_str[..].split_last().unwrap();
    let rules: Vec<Rule> = rules_str.iter().map(|&s| parse_rule(s)).collect();
    Workflow { name: String::from(name), rules: rules, end_dst: String::from(*end_dst) }
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
