advent_of_code::solution!(11);

type Point = (i64, i64);

pub fn empty_rows_and_cols<'a>(input: &str) -> (Vec<i64>, Vec<i64>) {
    let grid = input.lines().collect::<Vec<_>>();
    let empty_rows = grid
        .iter()
        .enumerate()
        .filter(|(_, row)| row.chars().all(|c| c == '.'))
        .map(|elt| elt.0 as i64)
        .collect::<Vec<_>>();

    let mut empty_cols: Vec<i64> = Vec::new();
    for col in 0..grid[0].len() {
        if grid.iter().all(|row| row.chars().nth(col).unwrap() == '.') {
            empty_cols.push(col as i64);
        }
    }

    (empty_rows, empty_cols)
}

pub fn get_locations(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(j, _)| (i as i64, j as i64))
        })
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<i64> {
    let l1_dist = |(p, q): (&Point, &Point)| (p.0 - q.0).abs() + (p.1 - q.1).abs();
    let locations = get_locations(input);
    let (empty_rows, empty_cols) = empty_rows_and_cols(input);

    // compute original distances (this double-counts)
    let mut answer = locations
        .iter()
        .flat_map(|p: &Point| locations.iter().map(move |q| l1_dist((&p, &q))))
        .sum::<i64>()
        / 2;

    // add distance based on empty rows & columns.
    answer += empty_rows
        .iter()
        .map(|&i| {
            let nleft = locations.iter().filter(|p| p.0 < i).count() as i64;
            let nright = locations.iter().filter(|p| p.0 > i).count() as i64;
            nleft * nright
        })
        .sum::<i64>();
    answer += empty_cols
        .iter()
        .map(|&j| {
            let nleft = locations.iter().filter(|p| p.1 < j).count() as i64;
            let nright = locations.iter().filter(|p| p.1 > j).count() as i64;
            nleft * nright
        })
        .sum::<i64>();

    Some(answer)
}

pub fn part_two(input: &str) -> Option<i64> {
    let l1_dist = |(p, q): (&Point, &Point)| (p.0 - q.0).abs() + (p.1 - q.1).abs();
    let locations = get_locations(input);
    let (empty_rows, empty_cols) = empty_rows_and_cols(input);

    // compute original distances (this double-counts)
    let mut answer = locations
        .iter()
        .flat_map(|p: &Point| locations.iter().map(move |q| l1_dist((&p, &q))))
        .sum::<i64>()
        / 2;

    // add distance based on empty rows & columns.
    answer += empty_rows
        .iter()
        .map(|&i| {
            let nleft = locations.iter().filter(|p| p.0 < i).count() as i64;
            let nright = locations.iter().filter(|p| p.0 > i).count() as i64;
            nleft * nright * (1000000 - 1)
        })
        .sum::<i64>();
    answer += empty_cols
        .iter()
        .map(|&j| {
            let nleft = locations.iter().filter(|p| p.1 < j).count() as i64;
            let nright = locations.iter().filter(|p| p.1 > j).count() as i64;
            nleft * nright * (1000000 - 1)
        })
        .sum::<i64>();

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1030));
    }
}
