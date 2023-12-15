use itertools::Itertools;

advent_of_code::solution!(13);

type Grid = Vec<Vec<char>>;

pub fn to_string(grid: &Grid) -> String {
    grid.iter().map(|row| row.iter().join("")).join("\n") + "\n"
}

pub fn transpose(grid: &Grid) -> Grid {
    assert!(!grid.is_empty());
    (0..grid[0].len())
        .map(|i| grid.iter().map(|row| row[i]).collect::<Vec<char>>())
        .collect()
}

pub fn get_reflection_column(grid: &Grid, part_two: bool) -> Option<usize> {
    let ncols = grid[0].len();
    let mut part_one_col: Option<usize> = None;
    for col in 1..ncols {
        let length = usize::min(col, ncols - col);
        let error_count = grid
            .iter()
            .map(|row| {
                row[col - length..col]
                    .iter()
                    .zip(row[col..col + length].iter().rev())
                    .map(|(xi, yi)| (xi != yi) as u32)
                    .sum::<u32>()
            })
            .sum::<u32>();
        if error_count == 0 {
            part_one_col = Some(col);
        }
        if part_two & (error_count == 1) {
            println!("Found part two candidate: {}", col);
            if (Some(col) != part_one_col) & part_one_col.is_some() {
                println!("Found UNIQUE part two answer: {}", col);
                return Some(col);
            }
        }
    }
    return part_one_col;
}

pub fn get_note(grid: &Grid, part_two: bool) -> usize {
    let col_part_one = get_reflection_column(grid, false);
    let row_part_one = get_reflection_column(&transpose(grid), false);
    if !part_two {
        println!("PART ONE");
        return col_part_one.unwrap_or(100 * row_part_one.unwrap_or(0));
    }

    let col_part_two = get_reflection_column(grid, true);
    let row_part_two = get_reflection_column(&transpose(grid), true);
    return col_part_two.unwrap_or(100 * row_part_two.unwrap_or(0));
}

pub fn part_one(input: &str) -> Option<usize> {
    let answer = input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect::<Grid>()
        })
        .map(|grid| get_note(&grid, false))
        .sum();

    Some(answer)
}

pub fn part_two(input: &str) -> Option<usize> {
    let answer = input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect::<Grid>()
        })
        .map(|grid| get_note(&grid, true))
        .sum();

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
