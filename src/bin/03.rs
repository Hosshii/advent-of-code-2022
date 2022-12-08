use std::collections::HashSet;

fn score(ipt: char) -> u32 {
    if ipt.is_ascii_lowercase() {
        (ipt as u32) - b'a' as u32 + 1
    } else if ipt.is_ascii_uppercase() {
        (ipt as u32) - b'A' as u32 + 27
    } else {
        panic!("Invalid character")
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|v| {
                let (lhs, rhs) = v.split_at(v.len() / 2);
                let [lhs, rhs] = [lhs, rhs].map(|v| v.chars().collect::<HashSet<_>>());
                let chars = lhs.intersection(&rhs).copied().collect::<HashSet<_>>();
                chars.into_iter().map(score).sum::<u32>()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines().peekable();
    let mut result = 0;
    loop {
        if lines.peek().is_none() {
            break;
        }
        let l1 = lines.next().unwrap();
        let l2 = lines.next().unwrap();
        let l3 = lines.next().unwrap();
        let [l1, l2, l3] = [l1, l2, l3].map(|v| v.chars().collect::<HashSet<_>>());
        let l = l1
            .intersection(&l2)
            .copied()
            .collect::<HashSet<_>>()
            .intersection(&l3)
            .copied()
            .collect::<HashSet<_>>();
        result += l.into_iter().map(score).sum::<u32>();
    }
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
