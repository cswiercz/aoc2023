use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(17);

type Grid = Vec<Vec<usize>>;
type Point = (i32, i32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Node {
    point: Point,
    direction: Direction,
    num_consecutive: usize,
}

const NEIGHBORS: [(Point, Direction); 4] = [
    ((-1, 0), Direction::Up),
    ((1, 0), Direction::Down),
    ((0, -1), Direction::Left),
    ((0, 1), Direction::Right),
];

pub fn neighbors<'a>(node: &'a Node, grid: &'a Grid) -> impl Iterator<Item = Node> + 'a {
    let nrows = grid.len() as i32;
    let ncols = grid[0].len() as i32;
    let (i, j) = node.point;

    NEIGHBORS
        .iter()
        .map(move |((di, dj), dir)| ((i + di, j + dj), dir))
        .filter(move |&((u, v), _)| (0 <= u) & (u < nrows) & (0 <= v) & (v < ncols))
        .map(|((u, v), dir)| {
            let num_consecutive = if node.direction == *dir {
                node.num_consecutive + 1
            } else {
                0
            };
            Node {
                point: (u, v),
                direction: *dir,
                num_consecutive: num_consecutive,
            }
        })
        .filter(|n| match node.direction {
            Direction::Up => n.direction != Direction::Down,
            Direction::Down => n.direction != Direction::Up,
            Direction::Left => n.direction != Direction::Right,
            Direction::Right => n.direction != Direction::Left,
        })
        .filter(|n| n.num_consecutive < 3)
}

pub fn solve(grid: &Grid) -> usize {
    let mut queue: VecDeque<Node> = VecDeque::new();
    let mut costs: HashMap<Point, usize> = HashMap::new();
    queue.push_back(Node {
        point: (0, 0),
        direction: Direction::Right,
        num_consecutive: 0,
    });
    queue.push_back(Node {
        point: (0, 0),
        direction: Direction::Down,
        num_consecutive: 0,
    });

    println!("=====  GRID = {:?}", grid);
    println!("===== QUEUE = {:?}", queue);

    while let Some(node) = queue.pop_front() {
        println!("\n{:?}", node);
        let current_cost = costs.get(&node.point).unwrap_or(&0).clone();

        println!("\tcurrent cost = {}", current_cost);
        for neighbor in neighbors(&node, &grid) {
            let (u, v) = neighbor.point;
            let new_cost = current_cost + grid[u as usize][v as usize];
            println!("\tnew cost for {:?}: {}", (u, v), new_cost);
            if costs.get(&neighbor.point).is_some_and(|&c| c <= new_cost) {
                println!(
                    "\tabandoning path: old cost {} <= new cost {}",
                    costs.get(&neighbor.point).unwrap(),
                    new_cost
                );
                continue; // skip neighbors with higher costs
            }
            costs.insert(neighbor.point, new_cost);
            queue.push_back(neighbor);
        }
    }

    let mut all_costs: Vec<Vec<usize>> = Vec::new();
    for row in 0..grid.len() {
        all_costs.push(vec![]);
        for col in 0..grid[0].len() {
            let cost = costs.get(&(row as i32, col as i32)).unwrap_or(&0);
            all_costs[row].push(*cost);
        }
    }
    for row in all_costs {
        println!("{:?}", row);
    }

    let end = (grid.len() as i32 - 1, grid[0].len() as i32 - 1);
    *costs.get(&end).unwrap()
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| String::from(c).parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    Some(solve(&grid))
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
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
