use std::collections::VecDeque;

advent_of_code::solution!(15);

#[derive(Debug, PartialEq, Eq)]
pub struct Lens {
    name: String,
    focal: usize,
    hash: usize,
}
type Boxes = Vec<VecDeque<Lens>>;

#[inline]
pub fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| (17 * (acc + c as usize) % 256))
}

pub fn parse(s: &str) -> (Lens, char) {
    let n = s.find(|c| (c == '-') | (c == '=')).unwrap();
    (
        Lens {
            name: String::from(&s[0..n]),
            focal: String::from(&s[n + 1..]).parse::<usize>().unwrap_or(0),
            hash: hash(&s[0..n]),
        },
        s.chars().nth(n).unwrap(),
    )
}

pub fn print_boxes(boxes: &Boxes) {
    boxes.iter().enumerate().for_each(|(n, b)| {
        if !b.is_empty() {
            let names: Vec<(String, usize)> = b
                .iter()
                .map(|lens| (lens.name.clone(), lens.focal))
                .collect();
            println!("Box {}: {:?}", n, names);
        }
    });
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.split(',').map(hash).sum::<usize>())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut boxes: Boxes = Boxes::new();
    for _ in 0..256 {
        boxes.push(VecDeque::new());
    }

    input.split(',').map(parse).for_each(|(lens, op)| {
        //println!("\n\nAfter {}{}{}:", lens.name, op, lens.focal);

        match op {
            '-' => {
                if let Some(i) = boxes[lens.hash].iter().position(|x| x.name == lens.name) {
                    boxes[lens.hash].remove(i);
                }
            }
            '=' => {
                let hash = lens.hash;
                let index = boxes[hash].iter().position(|x| x.name == lens.name);
                boxes[hash].push_back(lens);
                if let Some(i) = index {
                    boxes[hash].swap_remove_back(i);
                }
            }
            _ => unreachable!(),
        };

        //print_boxes(&boxes);
    });

    let answer: usize = boxes
        .iter()
        .enumerate()
        .map(|(n, boxx)| {
            (n + 1)
                * boxx
                    .iter()
                    .enumerate()
                    .map(|(slot, lens)| (slot + 1) * lens.focal)
                    .sum::<usize>()
        })
        .sum();

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
