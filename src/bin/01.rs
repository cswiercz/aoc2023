advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split_terminator('\n')
        .map(|line| {
            let mut moves = line.split_whitespace().take(2);
            let first = moves.next().unwrap();
            let second = moves.next().unwrap();
            let score: i32 = match (first, second) {
                ("A", "X") => 3 + 1,
                ("A", "Y") => 6 + 2,
                ("A", "Z") => 0 + 3,
                ("B", "X") => 0,
                ("B", "Y") => 0,
                ("B", "Z") => 0,
                ("C", "X") => 0,
                ("C", "Y") => 0,
                ("C", "Z") => 0,
            }
            return score;
        }
        )
        .sum()
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
