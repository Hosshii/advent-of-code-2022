use std::ops::RangeInclusive;

use itertools::{self, Itertools};

trait RangeInclusiveExt {
    fn contains_range(&self, other: &Self) -> bool;
    fn is_overlap(&self, other: &Self) -> bool;
}

impl<T> RangeInclusiveExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn is_overlap(&self, other: &Self) -> bool {
        self.start() <= other.end() && self.end() >= other.start()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|s| {
                s.split(',')
                    .map(|v| {
                        let (l, r) = v
                            .split('-')
                            .map(|v| v.parse::<u32>().unwrap())
                            .collect_tuple::<(_, _)>()
                            .expect("has 2 items");
                        l..=r
                    })
                    .collect_tuple::<(_, _)>()
                    .expect("has 2 items")
            })
            .filter(|(l, r)| l.contains_range(r) || r.contains_range(l))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|s| {
                s.split(',')
                    .map(|v| {
                        let (l, r) = v
                            .split('-')
                            .map(|v| v.parse::<u32>().unwrap())
                            .collect_tuple::<(_, _)>()
                            .expect("has 2 items");
                        l..=r
                    })
                    .collect_tuple::<(_, _)>()
                    .expect("has 2 items")
            })
            .filter(|(l, r)| l.is_overlap(r))
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
