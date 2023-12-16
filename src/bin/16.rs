use std::collections::{VecDeque, HashSet};

advent_of_code::solution!(16);

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub type Point = (usize, usize);
pub type Beam = (Point, Direction);
pub type Grid = Vec<Vec<char>>;

pub fn step(beam: Beam, grid: &Grid) -> Vec<Beam> {
    let mut beams: Vec<Beam> = Vec::new();
    let (point, direction) = beam;
    let c = grid[point.0][point.1];
    match (c, direction) {
        ('.', Direction::Up) => beams.push(((point.0, point.1 - 1), direction)),
        _ => unreachable!(),
    }
    beams
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
    let mut queue: VecDeque<Beam> = VecDeque::new();
    let mut visited: HashSet<Point> = HashSet::new();
    queue.push_back(((0, 0), Direction::Right));
    visited.insert((0, 0));

    while let Some(beam) = queue.pop_back() {
        visited.insert(point);

    }

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
