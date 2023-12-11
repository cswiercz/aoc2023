advent_of_code::solution!(11);

type Point = (i32, i32);

pub fn expand_galaxy<'a>(input: &str) -> (Vec<i32>, Vec<i32>) {
    let grid = input.lines().collect::<Vec<_>>();
    let empty_rows = grid
        .iter()
        .enumerate()
        .filter(|(_, row)| row.chars().all(|c| c == '.'))
        .map(|elt| elt.0 as i32)
        .collect::<Vec<_>>();

    let mut empty_cols: Vec<i32> = Vec::new();
    for col in 0..grid[0].len() {
        if grid.iter().all(|row| row.chars().nth(col).unwrap() == '.') {
            empty_cols.push(col as i32);
        }
    }

    (empty_rows, empty_cols)
}

pub fn get_locations(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(j, _)| (i as i32, j as i32))
        })
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<i32> {
    let l1_dist = |(p, q): (&Point, &Point)| (p.0 - q.0).abs() + (p.1 - q.1).abs();
    let locations = get_locations(input);
    let mut answer = 0;
    let mut num_pairs = 0;
    for i in 0..locations.len() {
        for j in i + 1..locations.len() {
            let p = locations[i];
            let q = locations[j];
            let dist = l1_dist((&p, &q));
            answer += dist;
            num_pairs += 1;
        }
    }

    println!("num_pairs: {}", num_pairs);
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
