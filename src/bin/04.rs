use std::collections::{HashSet, VecDeque, HashMap};

advent_of_code::solution!(4);

pub fn card_intersection_counts(input: &str) -> impl Iterator<Item=(usize, u32)> + '_ {
    input
        .split('\n')
        .map(|line| line.split_once(": ").unwrap().1)
        .map(|line| line.split_once('|').unwrap())
        .map(|(s_winning, s_card)| {
            let mut winning: HashSet<u32> = HashSet::new();
            s_winning.split(' ').filter(|s| !s.is_empty()).for_each(|s| _ = winning.insert(s.parse().unwrap()));

            let mut card: HashSet<u32> = HashSet::new();
            s_card.split(' ').filter(|s| !s.is_empty()).for_each(|s| _ = card.insert(s.parse().unwrap()));

            return winning
                .intersection(&card)
                .count() as u32
        })
        .enumerate()
}

pub fn part_one(input: &str) -> Option<u32> {
    let answer = card_intersection_counts(input)
        .map(|(_, n)| if n > 0 { 1 << (n-1) } else { 0 })
        .sum();
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut win_counts: HashMap<usize, u32> = HashMap::new();
    let mut stack: VecDeque<usize> = VecDeque::new();
    let mut answer = 0;

    card_intersection_counts(input)
        .for_each(|(card, num_won)| {
            win_counts.insert(card, num_won);
            stack.push_back(card);
        });

    while !stack.is_empty() {
        answer += 1;
        let card = stack.pop_front().unwrap();
        let wins = *win_counts.get(&card).unwrap() as usize;
        for i in 0..wins {
            let next_card = card + i + 1;
            stack.push_back(next_card);
        }
    }

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
