use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    fmt::{Debug, Display},
};

use itertools::Itertools;

advent_of_code::solution!(7);

////////////////////////////////////////////////////////////////////////////////
/// Card and Hand Type
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum Rank {
    Joker = 0,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Card {
    rank: Rank,
}

impl Card {
    pub fn from(c: &char) -> Card {
        let rank = match *c {
            '@' => Rank::Joker,
            '2' => Rank::Two,
            '3' => Rank::Three,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            '9' => Rank::Nine,
            'T' => Rank::Ten,
            'J' => Rank::Jack,
            'Q' => Rank::Queen,
            'K' => Rank::King,
            'A' => Rank::Ace,
            _ => unreachable!("Invalid character: {}", c),
        };
        Card { rank: rank }
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeKind = 4,
    FullHouse = 5,
    FourKind = 6,
    FiveKind = 7,
}

////////////////////////////////////////////////////////////////////////////////
/// Hand
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn from(s: &str) -> Hand {
        let cards: Vec<Card> = s.chars().map(|c| Card::from(&c)).collect();
        Hand { cards: cards }
    }

    pub fn get_type(&self) -> HandType {
        let num_jokers = self.cards.iter().filter(|c| c.rank == Rank::Joker).count();
        let rank_counts = self
            .cards
            .iter()
            .map(|c| c.rank)
            .filter(|&r| r != Rank::Joker)
            .counts();
        let mut heap = BinaryHeap::from_iter(rank_counts.values());
        let first = *heap.pop().unwrap_or(&0) + num_jokers;
        let second = *heap.pop().unwrap_or(&0);

        match (first, second) {
            (5, _) => HandType::FiveKind,
            (4, _) => HandType::FourKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeKind,
            (2, 2) => HandType::TwoPair,
            (2, _) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type = self.get_type();
        let other_type = other.get_type();

        // use ordered card values to compare when types are equal
        if self_type == other_type {
            for (l, r) in self.cards.iter().zip(other.cards.iter()) {
                match l.cmp(&r) {
                    Ordering::Equal => continue,
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                }
            }
        }
        self_type.cmp(&other_type)
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.cards.iter().map(|c| format!("{:?}", c.rank)).join(" ");
        write!(f, "{}", s)
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Solutions
////////////////////////////////////////////////////////////////////////////////

pub fn solve(input: &str) -> u32 {
    let mut hands_and_bids: Vec<(Hand, u32)> = input
        .split('\n')
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bid)| (Hand::from(hand), bid.parse::<u32>().unwrap()))
        .collect();

    hands_and_bids.sort_unstable_by(|(l, _), (r, _)| l.cmp(r));
    hands_and_bids
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i as u32 + 1))
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input.replace('J', "@").as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cards() {
        let c1 = Card::from(&'A');
        let c2 = Card::from(&'K');
        assert!(c1 > c2);
    }

    #[test]
    fn test_hands() {
        let h = Hand::from(&"TTT98");
        assert!(h.get_type() == HandType::ThreeKind);

        let h = Hand::from(&"23432");
        assert!(h.get_type() == HandType::TwoPair);

        let h = Hand::from(&"A23A4");
        assert!(h.get_type() == HandType::OnePair);

        let h = Hand::from(&"23456");
        assert!(h.get_type() == HandType::HighCard);

        let h1 = Hand::from(&"33332");
        let h2 = Hand::from(&"2AAAA");
        assert!(h1.get_type() == HandType::FourKind);
        assert!(h2.get_type() == HandType::FourKind);
        assert!(h1 > h2);

        let h1 = Hand::from(&"77888");
        let h2 = Hand::from(&"77788");
        assert!(h1.get_type() == HandType::FullHouse);
        assert!(h2.get_type() == HandType::FullHouse);
        assert!(h1 > h2);

        let h = Hand::from(&"22222");
        assert!(h.get_type() == HandType::FiveKind);
    }

    #[test]
    fn test_jokers() {
        let h1 = Hand::from(&"T55@5");
        assert!(h1.get_type() == HandType::FourKind);

        let h2 = Hand::from(&"QQQ@A");
        assert!(h2.get_type() == HandType::FourKind);

        let h3 = Hand::from(&"KT@@T");
        assert!(h3.get_type() == HandType::FourKind);

        let h4 = Hand::from(&"@@@@@");
        assert!(h4.get_type() == HandType::FiveKind);

        let h5 = Hand::from(&"@@@@K");
        assert!(h5.get_type() == HandType::FiveKind);

        let h6 = Hand::from(&"@@@@A");
        assert!(h6.get_type() == HandType::FiveKind);

        assert!(h1 < h2);
        assert!(h2 < h3);
        assert!(h3 < h4);
        assert!(h4 < h5);
        assert!(h5 < h6);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
