use std::collections::{HashMap, HashSet};

advent_of_code::solution!(3);

const NEIGHBORS: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, -1),
    (1, 1),
    (1, -1),
];

pub type Grid = Vec<Vec<char>>;

pub fn neighbors<'a>(
    nrows: &'a usize,
    ncols: &'a usize,
    i: &'a usize,
    j: &'a usize,
) -> impl Iterator<Item = (usize, usize)> + 'a {
    NEIGHBORS
        .iter()
        .map(|&(u, v)| (u + *i as i32, v + *j as i32))
        .filter(|&(u, v)| (0 <= u) & (u < *nrows as i32) & (0 <= v) & (v < *ncols as i32))
        .map(|(u, v)| (u as usize, v as usize))
}

pub fn make_grid(input: &str) -> Grid {
    let mut grid = Grid::new();
    input
        .split('\n')
        .for_each(|line| grid.push(line.chars().collect()));
    grid
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = make_grid(input);
    let (nrows, ncols) = (grid.len(), grid[0].len());

    let mut current_number = String::new();
    let mut is_adjacent = false;
    let mut answer: u32 = 0;

    for i in 0..nrows {
        for j in 0..ncols {
            if grid[i][j].is_numeric() {
                current_number.push(grid[i][j]);
                is_adjacent |= neighbors(&nrows, &ncols, &i, &j)
                    .any(|(u, v)| (grid[u][v] != '.') & !grid[u][v].is_numeric())
            }

            if !grid[i][j].is_numeric() | (j == ncols - 1) {
                if !current_number.is_empty() & is_adjacent {
                    answer += current_number.as_str().parse::<u32>().unwrap();
                }
                current_number.clear();
                is_adjacent = false;
            }
        }
    }
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = make_grid(input);
    let (nrows, ncols) = (grid.len(), grid[0].len());

    let mut current_number = String::new();
    let mut current_adjacent_gears: HashSet<(usize, usize)> = HashSet::new();
    let mut gear_parts: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for i in 0..nrows {
        for j in 0..ncols {
            if grid[i][j].is_numeric() {
                current_number.push(grid[i][j]);
                neighbors(&nrows, &ncols, &i, &j)
                    .filter(|(u, v)| grid[*u][*v] == '*')
                    .for_each(|(u, v)| _ = current_adjacent_gears.insert((u, v)));
            }

            if !grid[i][j].is_numeric() | (j == ncols - 1) {
                if !current_number.is_empty() {
                    let current_number = current_number.as_str().parse::<u32>().unwrap();
                    if current_adjacent_gears.len() == 1 {
                        let (u, v) = current_adjacent_gears.iter().next().unwrap();
                        gear_parts
                            .entry((*u, *v))
                            .or_insert(Vec::new())
                            .push(current_number);
                    }
                }
                current_number.clear();
                current_adjacent_gears.clear();
            }
        }
    }

    let answer = gear_parts
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v.iter().product::<u32>())
        .sum::<u32>();

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
