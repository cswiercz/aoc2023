use std::cmp::Ordering;
use std::collections::HashMap;

advent_of_code::solution!(19);

pub struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Debug)]
pub struct Rule {
    cat: char,
    ord: Ordering,
    val: u32,
    dst: String,
}

impl Rule {
    pub fn from(s: &str) -> Rule {
        let (rule_str, dst) = s.split_once(':').unwrap();
        let ord = rule_str.chars().nth(1).unwrap();
        let (part, val) = rule_str.split_once(ord).unwrap();

        let cat = part.chars().next().unwrap();
        let ord = match ord {
            '>' => Ordering::Greater,
            '<' => Ordering::Less,
            _ => panic!(),
        };
        let val = val.parse::<u32>().unwrap();

        Rule { cat: cat, ord: ord, val: val, dst: String::from(dst) }
    }

    pub fn eval(&self, p: &Part) -> bool {
        match self.cat {
            'x' => p.x.cmp(&self.val) == self.ord,
            'm' => p.m.cmp(&self.val) == self.ord,
            'a' => p.a.cmp(&self.val) == self.ord,
            's' => p.s.cmp(&self.val) == self.ord,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub struct Workflow {
    name: String,
    rules: Vec<Rule>,
    end_dst: String,
}

impl Workflow {
    pub fn from(s: &str) -> Workflow {
        let (name, rem) = s.split_once('{').unwrap();
        let rules_str: Vec<&str> = rem.strip_suffix('}').unwrap().split(',').collect();
        let (end_dst, rules_str) = rules_str[..].split_last().unwrap();
        let rules: Vec<Rule> = rules_str.iter().map(|&s| Rule::from(s)).collect();
        
        Workflow { name: String::from(name), rules: rules, end_dst: String::from(*end_dst) }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (s_workflows, s_parts) = input.split_once("\n\n").unwrap();
    let workflows: HashMap<String, Workflow> = s_workflows
        .lines()
        .map(Workflow::from)
        .map(|w| (w.name.clone(), w))
        .collect();
    let parts: Vec<Part> = s_parts
        .lines()
        .map(Part::from)
        .collect();

    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let rule = Rule::from(&"x>0:one");
        assert_eq!(rule.cat, 'x');
        assert_eq!(rule.ord, Ordering::Greater);
        assert_eq!(rule.val, 0);
        assert_eq!(rule.dst, "one");
    }

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
