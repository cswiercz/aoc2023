use std::collections::{HashMap, HashSet};

advent_of_code::solution!(3);

#[derive(PartialEq)]
pub enum Symbol {
    Digit(char),
    Empty,
    Symbol,
    Gear,
}

pub type Grid = Vec<Vec<Symbol>>;

pub fn neighbors(grid: &Grid, i: usize, j: usize) -> Vec<(usize, usize)> {
    let nrows = grid.len();
    let ncols = grid[0].len();

    let mut coords: Vec<(usize, usize)> = Vec::new();

    // up-down-left-right
    if i > 0 {coords.push((i-1, j))};
    if j > 0 {coords.push((i, j-1))};
    if i < nrows - 1 {coords.push((i+1, j))};
    if j < ncols - 1 {coords.push((i, j+1))};

    // diagonals
    if (i > 0) & (j > 0) {coords.push((i-1, j-1))};
    if (i > 0) & (j < ncols-1) {coords.push((i-1, j+1))};
    if (i < nrows-1) & (j > 0) {coords.push((i+1, j-1))};
    if (i < nrows-1) & (j < ncols-1) {coords.push((i+1, j+1))};

    coords
}

pub fn parse_grid(input: &str) -> Grid {
    let mut grid = Vec::new();
    input
        .split('\n')
        .for_each(|line| {
            let mut row = Vec::new();
            line.chars().for_each(|c|
                if c.is_numeric() {row.push(Symbol::Digit(c))}
                else if c == '.' {row.push(Symbol::Empty)}
                else if c == '*' {row.push(Symbol::Gear)}
                else {row.push(Symbol::Symbol)}
            );
            grid.push(row)
        });

    grid
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let nrows = grid.len();
    let ncols = grid[0].len();

    let mut current_number: String = String::new();
    let mut is_adjacent = false;

    let mut answer: u32 = 0;

    for i in 0..nrows {
        current_number.clear();
        is_adjacent = false;
        for j in 0..ncols {
            if let Symbol::Digit(c) = grid[i][j] {
                current_number.push(c);
                is_adjacent |= neighbors(&grid, i, j)
                    .iter()
                    .any(|(u, v)|
                        (grid[*u][*v] == Symbol::Symbol) | (grid[*u][*v] == Symbol::Gear)
                    );
            } else {
                if !current_number.is_empty() & is_adjacent {
                    answer += current_number.as_str().parse::<u32>().unwrap();
                }
                current_number.clear();
                is_adjacent = false;
                continue;
            }
        }
    }

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let nrows = grid.len();
    let ncols = grid[0].len();

    let mut gears: HashMap<(usize, usize), HashSet<u32>> = HashMap::new();
    let mut current_number: String = String::new();
    let mut current_adjacent_gears: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..nrows {
        for j in 0..ncols {
            if let Symbol::Digit(c) = grid[i][j] {
                current_number.push(c);
                neighbors(&grid, i, j)
                    .iter()
                    .filter(|(u, v)| (grid[*u][*v] == Symbol::Gear))
                    .for_each(|(u, v)| _ = current_adjacent_gears.insert((*u, *v)));
            } else {
                if !current_number.is_empty() {
                    let n = current_number.parse::<u32>().unwrap();
                    for (u, v) in current_adjacent_gears.iter() {
                        let key = &(*u, *v);
                        if gears.contains_key(key) {
                            gears.get_mut(key).unwrap().insert(n);
                        } else {
                            let mut s = HashSet::new();
                            s.insert(n);
                            gears.insert(*key, s);
                        }
                    }
                }
                current_number.clear();
                current_adjacent_gears.clear();
                continue;
            }
        }
    }

    // gears now contains coordinate -> [adjacent part numbers]
    let answer = gears
        .iter()
//        .inspect(|(coord, nums)| println!("{:?}: {:?}", coord, nums))
        .filter(|(_, nums)| nums.len() == 2)
        .inspect(|(coord, nums)| println!("{:?}:\t {:?}", coord, nums))
        .map(|(_, nums)| nums.iter().product::<u32>())
        .sum();

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
