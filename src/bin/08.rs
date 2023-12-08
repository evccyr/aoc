use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let input = String::from(
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
    );
    let map = parse(&input);
    dbg!(map);
    todo!()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse(input: &str) -> HashMap<String, (String, String)> {
    let instructions = input.lines().nth(0).unwrap();
    dbg!(instructions);
    input
        .lines()
        .skip(2)
        .map(|l| {
            l.split_once('=')
                .map(|(src, dest)| {
                    (
                        src.trim().to_string(),
                        dest.split_once(',')
                            .map(|(left, right)| {
                                (
                                    left.chars().filter(|c| c.is_ascii_alphabetic()).collect(),
                                    right.chars().filter(|c| c.is_ascii_alphabetic()).collect(),
                                )
                            })
                            .unwrap(),
                    )
                })
                .unwrap()
        })
        .collect()
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
