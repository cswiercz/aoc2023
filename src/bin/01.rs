advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let answer = input
        .split('\n')
        .map(|line| {
            let iter = line.chars().filter_map(|c| c.to_digit(10));
            let first = iter.clone().next().unwrap();
            let last = iter.clone().last().unwrap();
            format!("{}{}", first, last)
        })
        .map(|number_string| number_string.parse::<u32>().unwrap())
        .sum();
    Some(answer)
}

fn insert_digits(line: &str) -> String {
    let mut s = String::from(line);
    s = s.replace("one", "o1ne");
    s = s.replace("two", "t2wo");
    s = s.replace("three", "t3hree");
    s = s.replace("four", "f4our");
    s = s.replace("five", "f5ive");
    s = s.replace("six", "s6ix");
    s = s.replace("seven", "s7even");
    s = s.replace("eight", "e8ight");
    s = s.replace("nine", "n9ine");
    s
}

pub fn part_two(input: &str) -> Option<u32> {
    let answer = input
        .split('\n')
        .map(|line| insert_digits(line))
        .map(|line| {
            let iter = line.chars().filter_map(|c| c.to_digit(10));
            let first = iter.clone().next().unwrap();
            let last = iter.clone().last().unwrap();
            format!("{}{}", first, last)
        })
        .map(|number_string| number_string.parse::<u32>().unwrap())
        .inspect(|n| println!("{}", n))
        .sum();
    Some(answer)
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
