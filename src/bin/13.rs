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

pub fn get_reflection_column(grid: &Grid) -> Option<usize> {
    let ncols = grid[0].len();
    for col in 1..ncols {
        let length = usize::min(col, ncols - col);
        let valid_reflection = grid.iter().all(|row| {
            row[col - length..col]
                .iter()
                .zip(row[col..col + length].iter().rev())
                .all(|(xi, yi)| xi == yi)
        });
        if valid_reflection {
            return Some(col);
        }
    }
    None
}

pub fn get_note(grid: &Grid) -> usize {
    if let Some(note) = get_reflection_column(grid) {
        return note;
    } else if let Some(note) = get_reflection_column(&transpose(grid)) {
        return 100 * note;
    }
    unreachable!("expected reflection column or row\n{}", to_string(&grid));
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
        .map(|grid| get_note(&grid))
        .sum();

    Some(answer)
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
