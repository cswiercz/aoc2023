advent_of_code::solution!(9);

pub fn get_layers(sequence: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut diffs: Vec<Vec<i64>> = Vec::new();
    diffs.push(sequence.clone());

    while !diffs.last().unwrap().iter().all(|&x| x == 0) {
        let diff: Vec<i64> = diffs
            .last()
            .unwrap()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect();
        diffs.push(diff);
    }
    diffs
}

pub fn forecast(sequence: &Vec<i64>) -> i64 {
    let diffs = get_layers(sequence);
    let mut diff = 0;
    let mut next = 0;
    for layer in diffs.iter().rev() {
        next = *layer.last().unwrap() + diff;
        diff = next;
    }
    next
}

pub fn backcast(sequence: &Vec<i64>) -> i64 {
    let layers = get_layers(sequence);

    0
}

pub fn part_one(input: &str) -> Option<i64> {
    let sequences: Vec<Vec<i64>> = input
        .lines()
        .map(|line| line.split(' ').map(|s| s.parse::<i64>().unwrap()).collect())
        .collect();

    let answer = sequences.iter().map(forecast).sum();
    Some(answer)
}

pub fn part_two(input: &str) -> Option<i64> {
    let sequences: Vec<Vec<i64>> = input
        .lines()
        .map(|line| line.split(' ').map(|s| s.parse::<i64>().unwrap()).collect())
        .collect();

    let answer = sequences.iter().map(backcast).sum();

    Some(answer)
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
