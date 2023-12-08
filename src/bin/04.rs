advent_of_code::solution!(4);
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.split('\n') {
        let s = line.get(8..).unwrap_or("");
        if !s.is_empty() {
            let (a, b) = s.rsplit_once('|').unwrap_or(("", ""));
            let a: HashSet<&str> = a.trim().split(' ').filter(|s| !s.is_empty()).collect();
            let b: HashSet<&str> = b.trim().split(' ').filter(|s| !s.is_empty()).collect();
            let count = a.intersection(&b).count();
            if count > 0 {
                sum += (1 << count - 1);
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input: Vec<&str> = input.split('\n').collect();
    let mut multiplier = vec![1; input.len() - 1];
    for (index, line) in input.into_iter().enumerate() {
        let s = line.get(8..).unwrap_or("");
        if !s.is_empty() {
            let (a, b) = s.rsplit_once('|').unwrap_or(("", ""));
            let a: HashSet<&str> = a.trim().split(' ').filter(|s| !s.is_empty()).collect();
            let b: HashSet<&str> = b.trim().split(' ').filter(|s| !s.is_empty()).collect();
            let count = a.intersection(&b).count();
            for i in index + 1..index + 1 + count {
                multiplier[i] += multiplier[index];
            }
        }
    }
    Some(multiplier.iter().sum())
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
