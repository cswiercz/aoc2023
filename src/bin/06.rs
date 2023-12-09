use itertools::Itertools;

advent_of_code::solution!(6);

pub fn roots(t: u64, d: u64) -> (u64, u64) {
    let ft = t as f64;
    let fd = d as f64;
    let disc = (ft * ft - 4.0 * fd).sqrt();
    (
        f64::ceil((ft - disc) / 2.0 + 1e-8) as u64,
        f64::floor((ft + disc) / 2.0 - 1e-8) as u64,
    )
}

pub fn vectorize(s: &str, skip: usize) -> Vec<u64> {
    s.split_whitespace()
        .skip(skip)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

pub fn solve(time: &Vec<u64>, distance: &Vec<u64>) -> Option<u32> {
    let answer = time
        .iter()
        .zip(distance.iter())
        .map(|(t, d)| roots(*t, *d))
        .map(|(r1, r2)| r2 - r1 + 1)
        .product::<u64>();
    Some(answer as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (time, distance): (Vec<u64>, Vec<u64>) = input
        .split_once('\n')
        .map(|pair| (vectorize(pair.0, 1), vectorize(pair.1, 1)))
        .unwrap();

    solve(&time, &distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let get_num = |l: &str| {
        l.split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .join("")
            .parse::<u64>()
            .unwrap()
    };

    let answer = input
        .split_once('\n')
        .map(|(t_str, d_str)| (get_num(t_str), get_num(d_str)))
        .map(|(t, d)| roots(t, d))
        .map(|(r1, r2)| r2 - r1 + 1)
        .unwrap();
    Some(answer as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
