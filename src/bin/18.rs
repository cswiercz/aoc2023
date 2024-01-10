use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(18);

type Point = (i32, i32);
pub struct Grid {
    nodes: HashMap<Point, Node>,
    row_range: (i32, i32),
    col_range: (i32, i32),
}

impl Grid {
    pub fn contains(&self, point: &Point) -> bool {
        (self.row_range.0 <= point.0)
            & (self.row_range.1 <= point.1)
            & (self.col_range.0 <= point.1)
            & (point.1 <= self.col_range.1)
    }
}

#[derive(Debug)]
pub enum Node {
    Empty,
    Filled(String),
}

const NEIGHBORS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub fn neighbors<'a>(point: &'a Point, _grid: &'a Grid) -> impl Iterator<Item = Point> + 'a {
    let i = point.0 as i32;
    let j = point.1 as i32;
    NEIGHBORS.iter().map(move |(di, dj)| (i + di, j + dj))
}
pub fn parse_line(line: &str) -> (char, usize, String) {
    let mut iter = line.split_whitespace();
    let direction: char = iter.next().unwrap().chars().next().unwrap();
    let num_steps: usize = iter.next().unwrap().parse().unwrap();
    let color: String = String::from(iter.next().unwrap());
    (direction, num_steps, color)
}

pub fn parse_grid(input: &str) -> Grid {
    let mut row_range = (i32::MAX, i32::MIN);
    let mut col_range = (i32::MAX, i32::MIN);
    let mut nodes: HashMap<Point, Node> = HashMap::new();
    let mut point: Point = (0, 0);

    for line in input.lines() {
        let (direction, num_steps, color) = parse_line(line);
        let (di, dj) = match direction {
            'U' => (1, 0),
            'D' => (-1, 0),
            'R' => (0, 1),
            'L' => (0, -1),
            _ => panic!("unexpected direction character: {}", direction),
        };

        // TODO: corners are ambiguous.
        for _ in 0..num_steps {
            point = (point.0 + di, point.1 + dj);
            nodes.insert(point, Node::Filled(color.clone()));
        }

        row_range.0 = i32::min(row_range.0, point.0);
        row_range.1 = i32::max(row_range.1, point.1);
        col_range.0 = i32::min(col_range.0, point.0);
        col_range.1 = i32::max(col_range.1, point.1);
    }

    Grid {
        nodes: nodes,
        row_range: row_range,
        col_range: col_range,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let (imin, imax) = grid.row_range;
    let (jmin, jmax) = grid.col_range;

    let is = (imin..imax).into_iter();
    let js = (jmin..jmax).into_iter();
    let points = js.flat_map(|j| is.clone().map(move |i| (i, j)));

    let mut total_area = 0;
    let mut visited: HashSet<Point> = HashSet::new();
    for point in grid.nodes.keys() {
        visited.insert(point.clone());
    }

    for point in points {
        // skip if already visitor or on the edge of the trench
        if visited.contains(&point) | grid.nodes.contains_key(&point) {
            continue;
        }

        // otherwise, fill the space. anything that touches the edge is
        // considered outside the trench
        let mut area = 0;
        let mut is_outside = false;
        let mut queue: VecDeque<Point> = VecDeque::new();
        queue.push_back(point.clone());
        while let Some(p) = queue.pop_front() {
            visited.insert(p);
            area += 1;
            for q in neighbors(&p, &grid) {
                if (q.0 <= imin) | (imax <= q.0) | (q.1 <= jmin) | (jmax <= q.1) {
                    is_outside = true;
                } else if !visited.contains(&q) {
                    queue.push_back(q);
                }
            }
        }

        if !is_outside {
            total_area += area;
        }
    }

    Some(total_area)
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
