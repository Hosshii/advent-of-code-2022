use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    input
        .as_bytes()
        .windows(4)
        .enumerate()
        .find(|(_, w)| w.iter().collect::<HashSet<_>>().len() == 4)
        .map(|(i, _)| i as u32 + 1 + 3)
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .as_bytes()
        .windows(14)
        .enumerate()
        .find(|(_, w)| w.iter().collect::<HashSet<_>>().len() == 14)
        .map(|(i, _)| i as u32 + 1 + 13)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
