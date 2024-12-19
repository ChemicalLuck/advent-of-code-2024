use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .map(|l| {
            let nums: Vec<u64> = l.split_whitespace().map(|c| c.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .unzip()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut left, mut right): (Vec<u64>, Vec<u64>) = parse_input(input);

    left.sort();
    right.sort();

    Some(
        left.into_iter()
            .zip(right)
            .into_iter()
            .map(|g| g.0.abs_diff(g.1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (left, right) = parse_input(input);

    let mut ids = HashMap::new();

    let sum = left
        .iter()
        .map(|id| {
            let count = ids
                .entry(id)
                .or_insert_with(|| right.iter().filter(|v| **v == *id).count() as u64);
            *id * *count
        })
        .sum();

    Some(sum)
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
