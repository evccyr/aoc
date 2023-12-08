advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let records = parse(&input);
    let mut res = 1;
    for record in records {
        res *= ways(record);
    }
    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut t = String::from("");
    let mut d = String::from("");
    parse(&input).into_iter().for_each(|r| {
        t = format!("{}{}", t, r.time);
        d = format!("{}{}", d, r.distance);
    });
    Some(ways(Record {
        time: t.parse().unwrap(),
        distance: d.parse().unwrap(),
    }))
}

pub fn ways(record: Record) -> u32 {
    let mut ways = 0;
    let t = record.time;
    for s in 1..=t {
        if (s * (t - s)) > record.distance {
            ways += 1;
        }
    }
    ways
}

pub fn parse(input: &str) -> Records {
    input
        .lines()
        .map(|(time, distance)| {
            time.split_whitespace()
                .zip(distance.split_whitespace())
                .skip(1)
                .map(|(t, d)| Record {
                    time: t.parse::<usize>().unwrap(),
                    distance: d.parse::<usize>().unwrap(),
                })
        })
        .unwrap()
        .collect()
}

pub type Records = Vec<Record>;

#[derive(Debug)]
pub struct Record {
    pub time: usize,
    pub distance: usize,
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
