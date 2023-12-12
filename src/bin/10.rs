use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

type Grid = Vec<Vec<char>>;
type Point = (i32, i32);

const NEIGHBORS_VALID: [(Point, [char; 3]); 4] = [
    ((-1, 0), ['|', '7', 'F']),
    ((1, 0), ['|', 'L', 'J']),
    ((0, -1), ['-', 'L', 'F']),
    ((0, 1), ['-', '7', 'J']),
];

pub fn neighbors<'a>(grid: &'a Grid, point: &'a Point) -> impl Iterator<Item = Point> + 'a {
    NEIGHBORS_VALID
        .iter()
        .map(|(dir, _)| (point.0 + dir.0, point.1 + dir.1))
        .filter(|new_point| {
            new_point.0 >= 0
                && new_point.0 < grid.len() as i32
                && new_point.1 >= 0
                && new_point.1 < grid[0].len() as i32
        })
}

pub fn neighbors_valid<'a>(grid: &'a Grid, point: &'a Point) -> impl Iterator<Item = Point> + 'a {
    NEIGHBORS_VALID
        .iter()
        .map(|(dir, valid_pipes)| ((point.0 + dir.0, point.1 + dir.1), valid_pipes))
        .filter(|(new_point, _)| {
            new_point.0 >= 0
                && new_point.0 < grid.len() as i32
                && new_point.1 >= 0
                && new_point.1 < grid[0].len() as i32
        })
        .filter(|(new_point, valid_pipes)| {
            let new_pipe = grid[new_point.0 as usize][new_point.1 as usize];
            valid_pipes.contains(&new_pipe)
        })
        .map(|(new_point, _)| new_point)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(row_num, row)| {
            row.iter()
                .position(|&c| c == 'S')
                .map(|col_num| (row_num as i32, col_num as i32))
        })
        .unwrap();

    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<(Point, u32)> = VecDeque::from([(start, 0)]);
    let mut max_steps: u32 = 0;
    while queue.len() > 0 {
        let (point, steps) = queue.pop_front().unwrap();
        if visited.contains(&point) {
            continue;
        }
        visited.insert(point);
        max_steps = max_steps.max(steps);
        for neighbor in neighbors_valid(&grid, &point) {
            queue.push_back((neighbor, steps + 1));
        }
    }

    Some(max_steps)
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
