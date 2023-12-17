use std::collections::{VecDeque, HashSet};

advent_of_code::solution!(16);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
    let (i, j) = beam.0;
    let c = grid[i][j];
    let directions: Vec<Direction> = match (c, beam.1) {
        ('.', d) => vec![d],
        ('/', d) => vec![match d {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up
        }],
        ('\\', d) => vec![match d {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }],
        ('|', d) => match d {
            Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
            _ => vec![d], 
        },
        ('-', d) => match d {
            Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
            _ => vec![d],
        },
        _ => unreachable!(),
    };

    directions
        .into_iter()
        .filter(|d| match d {
            Direction::Up => i > 0,
            Direction::Down => i < grid.len() - 1,
            Direction::Left => j > 0,
            Direction::Right => j < grid[0].len() - 1,
        })
        .map(|d| match d {
            Direction::Up => ((i-1, j), d),
            Direction::Down => ((i + 1, j), d),
            Direction::Left => ((i, j -1), d),
            Direction::Right => ((i, j + 1), d)
        })
        .collect()
}

pub fn solve(start: Beam, grid: &Grid) -> usize {
    let mut queue: VecDeque<Beam> = VecDeque::new();
    let mut energized: HashSet<Point> = HashSet::new();
    let mut visited: HashSet<Beam> = HashSet::new();
    queue.push_back(start);
    energized.insert(start.0);

    while let Some(beam) = queue.pop_back() {
        step(beam, &grid)
            .into_iter()
            .filter(|b| !visited.contains(b))
            .for_each(|b| queue.push_front(b));
        energized.insert(beam.0);
        visited.insert(beam);
    }

    energized.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = input.lines().map(|l| l.chars().collect()).collect();
    Some(solve(((0, 0), Direction::Right), &grid))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
