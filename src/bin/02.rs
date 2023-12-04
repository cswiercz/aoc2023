advent_of_code::solution!(2);

use std::cmp::max;

#[derive(Debug)]
struct Draw(u32, u32, u32);

#[derive(Debug)]
struct Game(u32, Vec<Draw>);

fn parse_draw(str_draw: &str) -> Draw {
    let mut draw = Draw(0, 0, 0);
    str_draw
        .trim()
        .split(", ")
        .for_each(|str_each| {
            let (str_count, str_color) = str_each.split_once(' ').unwrap();
            let count = str_count.parse::<u32>().unwrap();
            match str_color {
                "red" => {draw.0 = count},
                "green" => {draw.1 = count},
                "blue" => {draw.2 = count},
                _ => unimplemented!("color {} unknown", str_color),
            }
        });
    draw
}

fn parse_line(line: &str) -> Game {
    let mut game = Game(0, Vec::new());
    let (str_game, str_draws) = line.split_once(": ").unwrap();

    game.0 = str_game.split_once(' ').unwrap().1.parse::<u32>().unwrap();
    str_draws
        .split(';')   // each draw in a game
        .for_each(|str_draw| game.1.push(parse_draw(str_draw)));
    game
}

pub fn part_one(input: &str) -> Option<u32> {
    const MAX_COUNTS: (u32, u32, u32) = (12, 13, 14);

    let answer = input
        .split('\n')
        .map(parse_line)
        .filter_map(|game| {
            for draw in game.1 {
                if (draw.0 > MAX_COUNTS.0) | (draw.1 > MAX_COUNTS.1) | (draw.2 > MAX_COUNTS.2) {
                    return None;
                }
            }
            return Some(game.0);
        })
        .sum();

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let answer = input
        .split('\n')
        .map(parse_line)
        .map(|game| {
            let mut minima = (0, 0, 0);
            game.1.iter().for_each(|draw| {
                minima.0 = max(draw.0, minima.0);
                minima.1 = max(draw.1, minima.1);
                minima.2 = max(draw.2, minima.2);

            });
            minima.0 * minima.1 * minima.2
        })
        .sum();

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
