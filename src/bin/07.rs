use std::ops::Deref;

use itertools::{Itertools, Position};

advent_of_code::solution!(7);

pub fn score(hand: &str) -> (HandType, (u32, u32, u32, u32, u32)) {
    use HandType::*;
    let counts = hand.chars().counts();
    let values = if let Some(joker_count) = counts.get(&'J') {
        if *joker_count == 5 {
            "5".to_string()
        } else {
            counts
                .iter()
                .filter_map(|(key, value)| (key != &'J').then_some(value))
                .sorted()
                .with_position()
                .map(|(position, value)| match position {
                    Position::Last | Position::Only => value + joker_count,
                    _ => *value,
                })
                .join("")
        }
    } else {
        counts.values().sorted().join("")
    };
    let hand_type = match values.deref() {
        "5" => FiveOfAKind,
        "14" => FourOfAKind,
        "23" => FullHouse,
        "113" => ThreeOfAKind,
        "122" => TwoPair,
        "1112" => OnePair,
        "11111" => HighCard,
        value => unreachable!(),
    };
    let card_scores = hand
        .chars()
        .map(|card| match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            value => value.to_digit(10).unwrap(),
        })
        .collect_tuple()
        .unwrap();
    (hand_type, card_scores)
}

#[derive(Debug, Clone, Copy)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

pub fn part_one(input: &str) -> Option<u32> {
    //     let input = String::from(
    //         "32T3K 765
    // T55J5 684
    // KK677 28
    // KTJJT 220
    // QQQJA 483",
    //     );
    let mut sets = parse(&input);
    Some(
        sets.into_iter()
            .sorted_by_key(|set| (score(&set.hand).0 as u8, score(&set.hand).1))
            .enumerate()
            .map(|(i, set)| (i as u32 + 1) * set.bid)
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug)]
pub struct Set {
    pub hand: String,
    pub bid: u32,
}

pub fn parse(input: &str) -> Vec<Set> {
    input
        .lines()
        .map(|l| {
            {
                l.split_once(' ')
                    .map(|(h, b)| Set {
                        hand: h.chars().collect(),
                        bid: b.parse().unwrap(),
                    })
                    .unwrap()
            }
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
