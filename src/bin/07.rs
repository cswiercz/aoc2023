use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Copy, Clone, Eq, Hash, PartialEq, PartialOrd)]
pub enum Rank {
    Two   = 2,
    Three = 3,
    Four  = 4,
    Five  = 5,
    Six   = 6,
    Seven = 7,
    Eight = 8,
    Nine  = 9,
    Ten   = 10,
    Jack  = 11,
    Queen = 12,
    King  = 13,
    Ace   = 14,
}

#[derive(PartialEq, PartialOrd)]
pub struct Card(Rank);

impl Card {
    pub fn from(c: &char) -> Card {
        let rank = match *c {
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
        Card(rank)
    }
}

#[derive(PartialEq, PartialOrd)]
pub enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

pub struct Hand(Vec<Card>);

impl Hand {
    pub fn from(s: &str) -> Hand {
        let cards: Vec<Card> = s
            .chars()
            .map(|c| Card::from(&c))
            .collect();
        Hand(cards)
    }

    pub fn get_type(&self) -> HandType {
        let mut rank_counts: HashMap<Rank, u32> = HashMap::new();
        self.0.iter().for_each(|card| 
            if let Some(count) = rank_counts.get_mut(&card.0) {
                *count += 1;
            } else {
                rank_counts.insert(card.0, 1);
            }
        );

        // special handling of two-pair detection
        let counts = rank_counts.values().map(|&v| v).collect_vec();
        let mut has_pair = false;
        let mut has_two_pair = false;
        counts.iter().for_each(|&v| if (v == 2) & has_pair { has_two_pair = true; } else if v == 2 { has_pair = true; });

        if counts.contains(&5) {
            return HandType::FiveOfAKind;
        } else if counts.contains(&4) {
            return HandType::FourOfAKind;
        } else if counts.contains(&3) & has_pair {
            return HandType::FullHouse;
        } else if counts.contains(&3) {
            return HandType::ThreeOfAKind;
        } else if has_two_pair {
            return HandType::TwoPair;
        } else if has_pair {
            return HandType::OnePair;
        } else {
            return HandType::HighCard;
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        (self.get_type() == other.get_type()) & (self.0[0] == other.0[0])
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_type = self.get_type();
        let other_type = other.get_type();

        // use first card rank rule if hand types are equal (e.g 3333A > 2AAAA
        // and 77888 > 77788 [third card is higher])
        if self_type == other_type {
            for (l, r) in self.0.iter().zip(other.0.iter()) {
                if l != r { return l.partial_cmp(r); }
            }
            return None
        }
        self_type.partial_cmp(&other_type)
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let (hands, bids): (Vec<Hand>, Vec<u32>) = input
        .split('\n')
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bid)| (Hand::from(hand), bid.parse::<u32>().unwrap()))
        .unzip();

    let mut indices = (0..hands.len()).collect::<Vec<usize>>();
    indices.sort_by_key(|&i| &hands[i]);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cards() {
        let c1 = Card::from(&'A');
        let c2 = Card::from (&'K');
        assert!(c1 > c2);
    }

    #[test]
    fn test_hands() {
        let h = Hand::from(&"TTT98");
        assert!(h.get_type() == HandType::ThreeOfAKind);

        let h = Hand::from(&"23432");
        assert!(h.get_type() == HandType::TwoPair);
        
        let h = Hand::from(&"A23A4");
        assert!(h.get_type() == HandType::OnePair);

        let h = Hand::from(&"23456");
        assert!(h.get_type() == HandType::HighCard);

        let h1 = Hand::from(&"33332");
        let h2 = Hand::from(&"2AAAA");
        assert!(h1.get_type() == HandType::FourOfAKind);
        assert!(h2.get_type() == HandType::FourOfAKind);
        assert!(h1 > h2);

        let h1 = Hand::from(&"77888");
        let h2 = Hand::from(&"77788");
        assert!(h1.get_type() == HandType::FullHouse);
        assert!(h2.get_type() == HandType::FullHouse);
        assert!(h1 > h2);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
