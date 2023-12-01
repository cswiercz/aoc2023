advent_of_code::solution!(1);


pub fn part_one(input: &str) -> Option<u32> {
    /*
    let answer = input
        .split('\n')
        .map(|line: &str| line
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
        )
        .map(|digits| {
            let first = digits.chars().nth(0).unwrap();
            let last = digits.chars().last().unwrap();
            format!("{}{}", first, last)
        })
        .map(|number_string| number_string.parse::<u32>().unwrap())
        .sum();
    */
    let answer = input
        .split('\n')
        .map(|line| {
            let iter = line.chars().filter_map(|c| c.to_digit(10));
            let first = iter.clone().next();
            let last = iter.clone().last();
            format!("{}{}", first, last)
        });

    for x in answer {
        println!("{:?}", x)
    }
    Some(42)
}

pub fn part_two(input: &str) -> Option<u32> {
    
    Some(42)
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
