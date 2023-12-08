use std::collections::HashMap;
use std::usize::MAX;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let input = String::from(
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
    );
    let mut lines = input
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();
    let seeds = lines.remove(0).split(' ').collect::<Vec<&str>>();
    let mut map = HashMap::new();
    for chunk in lines
        .split(|s| s.as_bytes()[0].is_ascii_alphabetic())
        .filter(|c| !c.is_empty())
    {
        for line in chunk {
            let line = line.split(' ').collect::<Vec<&str>>();
            let (d, s, r): (usize, usize, usize) = (
                line[0].parse().unwrap(),
                line[1].parse().unwrap(),
                line[2].parse().unwrap(),
            );
            for i in 0..r {
                if let Some(_) = map.get(&s) {
                    rename_key(&mut map, s, d);
                } else {
                    map.insert(d + i, s + i);
                }
            }
        }
    }
    dbg!(map);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn rename_key(map: &mut HashMap<usize, usize>, old_key: usize, new_key: usize) {
    if let Some(v) = map.remove(&old_key) {
        map.insert(new_key, v);
    }
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
