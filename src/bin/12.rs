use core::num;
use std::fmt::Display;

use itertools::Itertools;

advent_of_code::solution!(12);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Data {
    springs: String,
    groups: Vec<usize>,
    current_run: usize,
    observed_springs: String, // used only in debugging
}

impl Data {
    pub fn new(s: &str, g: Vec<usize>) -> Data {
        Data {
            springs: String::from(s),
            groups: g,
            current_run: 0,
            observed_springs: String::new(),
        }
    }

    pub fn pop(&mut self) -> char {
        let c = self.springs.remove(0);
        self.observed_springs.push(c);
        c
    }

    pub fn push(&mut self, c: char) {
        self.springs = String::from(c) + &self.springs;
        self.observed_springs.pop();
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}|{} {:?} (current_run = {})",
            self.observed_springs,
            self.springs,
            &self.groups[..],
            self.current_run,
        )
    }
}

pub fn arrangements(springs: &str, groups: Vec<usize>) -> u32 {
    let mut stack: Vec<Data> = Vec::with_capacity(2 * springs.len());
    stack.push(Data::new(springs, groups));
    let mut num_arrangements: u32 = 0;

    while !stack.is_empty() {
        let mut d = stack.pop().unwrap();
        //println!("{}", d);

        if d.springs.is_empty() {
            if (d.current_run == 0) & (d.groups.len() > 0) {
                //println!("\tDONE (not enough remaining groups)");
            } else if (d.current_run > 0) & (d.groups.len() == 1) {
                if d.groups[0] == d.current_run {
                    num_arrangements += 1;
                    //println!("\tDONE (found valid arrangement)");
                }
            } else if (d.current_run > 0) & (d.groups.len() > 1) {
                //println!("\tDONE (too many remaining groups)");
            } else if (d.current_run == 0) & (d.groups.len() == 0) {
                num_arrangements += 1;
                //println!("\tDONE (found valid arrangement)")
            }
            continue;
        }

        match d.pop() {
            '#' => {
                // early exits:
                if d.groups.len() == 0 {
                    //println!("\tDONE (expected more groups)");
                    continue;
                }
                if d.current_run > d.groups[0] {
                    //println!("\tDONE (current run exceeded group size)");
                    continue;
                }
                d.current_run += 1;
                stack.push(d);
            }
            '?' => {
                // substitute current character with each option
                let mut d_left = d.clone();
                d_left.push('.');
                //println!("\t left: {}", d_left);
                stack.push(d_left);

                let mut d_right = d.clone();
                d_right.push('#');
                //println!("\tright: {}", d_right);
                stack.push(d_right);
            }
            '.' => {
                // if we just ended a run then check that the run length is valid
                if d.current_run > 0 {
                    if d.current_run == d.groups[0] {
                        d.groups.remove(0);
                    } else {
                        //println!("\tDONE (current run exceeded group size)");
                        continue;
                    }
                }
                d.current_run = 0;
                stack.push(d);
            }
            _ => unreachable!("unexpected character"),
        }
    }

    //println!();
    //println!("#arrangements = {}", num_arrangements);

    num_arrangements
}

pub fn part_one(input: &str) -> Option<u32> {
    let answer: u32 = input
        .split_terminator('\n')
        .map(|line| line.split_once(' ').unwrap())
        .map(|(springs, s_group_lengths)| {
            let group_lengths = s_group_lengths
                .split(',')
                .map(|si| si.parse::<usize>().unwrap())
                .collect();
            arrangements(springs, group_lengths)
        })
        .sum();

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let num_extensions = 6;
    let answer: u32 = input
        .split_terminator('\n')
        .map(|line| line.split_once(' ').unwrap())
        .map(|(springs, s_group_lengths)| {
            let group_lengths: Vec<usize> = s_group_lengths
                .split(',')
                .map(|si| si.parse::<usize>().unwrap())
                .collect();
            let mut extended_groups_lengths: Vec<usize> = Vec::new();
            for _ in 0..num_extensions {
                extended_groups_lengths.extend(group_lengths.iter());
            }
            let s = [springs].iter().cycle().take(num_extensions).join("?");
            /*
            println!("{}", springs);
            println!("{}", s);
            println!("{:?}", &extended_groups_lengths[..]);
            println!();
            */
            arrangements(&s, extended_groups_lengths)
        })
        .enumerate()
        .map(|t| t.1)
        .sum();

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        // test that pop() -> push() leaves the object invariant
        let d = Data::new(&".??..??...?##.", vec![1, 1, 3]);
        let mut d_new = d.clone();
        let c = d_new.pop();
        d_new.push(c);
        assert_eq!(d, d_new);
    }

    #[test]
    fn test_part_one_individual() {
        let result = part_one(&"???.### 1,1,3");
        assert_eq!(result, Some(1));

        let result = part_one(&".??..??...?##. 1,1,3");
        assert_eq!(result, Some(4));

        let result = part_one(&"?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(result, Some(1));

        let result = part_one(&"????.#...#... 4,1,1");
        assert_eq!(result, Some(1));

        let result = part_one(&"????.######..#####. 1,6,5");
        assert_eq!(result, Some(4));

        let result = part_one(&"?###???????? 3,2,1");
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two_individual() {
        // in all subsequent, "n" = num extensions. "n=1" is
        // equivalent to part 1
        //
        // seq = 4, 32, 256, 2048, 16384, 131072
        // a(n) = 4 * 8**(n-1)
        let s = &".??..??...?##. 1,1,3";
        println!("part 1: {:?}", part_one(s));
        println!("part 2: {:?}", part_two(s));

        // seq = 1, 2, 4, 8, 16, 32
        // a(n) = 1 * 2**(n-1)
        let s = &"????.#...#... 4,1,1";
        println!("part 1: {:?}", part_one(s));
        println!("part 2: {:?}", part_two(s));

        // seq = 4, 20, 100, 500, 2500, 12500
        // a(n) = 4 * 5**(n-1)
        let s = &"????.######..#####. 1,6,5";
        println!("part 1: {:?}", part_one(s));
        println!("part 2: {:?}", part_two(s));

        // seq = 10, 150, 2250, 33750, 506250, 7593750
        // a(n) = 10 * 15**(n-1)
        let s = &"?###???????? 3,2,1";
        println!("part 1: {:?}", part_one(s));
        println!("part 2: {:?}", part_two(s));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
