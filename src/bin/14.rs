use std::collections::HashMap;

advent_of_code::solution!(14);

type Grid = Vec<Vec<char>>;

pub enum Direction {
    North,
    South,
    East,
    West,
}

pub fn step(grid: &mut Grid, direction: &Direction) -> bool {
    let mut grid_changed = false;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let is_rock = grid[row][col] == 'O';
            if !is_rock {
                continue;
            }
            let mut can_move = false;
            match direction {
                Direction::North => {
                    if row > 0 {
                        can_move = grid[row - 1][col] == '.';
                    }
                    if is_rock & can_move {
                        grid[row - 1][col] = 'O';
                        grid[row][col] = '.';
                        grid_changed = true;
                    }
                }
                Direction::South => {
                    if row < grid.len() - 1 {
                        can_move = grid[row + 1][col] == '.';
                    }
                    if is_rock & can_move {
                        grid[row + 1][col] = 'O';
                        grid[row][col] = '.';
                        grid_changed = true;
                    }
                }
                Direction::East => {
                    if col < grid[0].len() - 1 {
                        can_move = grid[row][col + 1] == '.';
                    }
                    if is_rock & can_move {
                        grid[row][col + 1] = 'O';
                        grid[row][col] = '.';
                        grid_changed = true;
                    }
                }
                Direction::West => {
                    if col > 0 {
                        can_move = grid[row][col - 1] == '.';
                    }
                    if is_rock & can_move {
                        grid[row][col - 1] = 'O';
                        grid[row][col] = '.';
                        grid_changed = true;
                    }
                }
            }
        }
    }
    grid_changed
}

pub fn calculate_load(grid: &Grid) -> u32 {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(row_number, row)| (row_number + 1) * row.iter().filter(|&c| *c == 'O').count())
        .sum::<usize>() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Grid = input.lines().map(|l| l.chars().collect()).collect();
    while step(&mut grid, &Direction::North) {
        continue;
    }
    Some(calculate_load(&grid))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Grid = input.lines().map(|l| l.chars().collect()).collect();
    let mut load_cycle_hash_map: HashMap<Grid, (usize, u32)> = HashMap::new();
    let mut cycle_length: Option<usize> = None;
    let num_cycles = 1_000_000_000;

    for cycle in 0..num_cycles {
        while step(&mut grid, &Direction::North) {
            continue;
        }
        while step(&mut grid, &Direction::West) {
            continue;
        }
        while step(&mut grid, &Direction::South) {
            continue;
        }
        while step(&mut grid, &Direction::East) {
            continue;
        }

        if let Some(len) = cycle_length {
            let (c, l) = load_cycle_hash_map.get(&grid).unwrap();
            if (c % len) == ((num_cycles - 1) % len) {
                return Some(*l);
            }
        } else {
            let load = calculate_load(&grid);
            if let Some((c, _)) = load_cycle_hash_map.insert(grid.clone(), (cycle, load)) {
                cycle_length = Some(cycle - c);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
