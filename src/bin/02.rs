#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn vs(&self, other: &RPS) -> RPSResult {
        match (self, other) {
            (RPS::Rock, RPS::Rock) | (RPS::Paper, RPS::Paper) | (RPS::Scissors, RPS::Scissors) => {
                RPSResult::Draw
            }
            (RPS::Rock, RPS::Paper) | (RPS::Paper, RPS::Scissors) | (RPS::Scissors, RPS::Rock) => {
                RPSResult::Lose
            }
            (RPS::Rock, RPS::Scissors) | (RPS::Scissors, RPS::Paper) | (RPS::Paper, RPS::Rock) => {
                RPSResult::Win
            }
        }
    }

    fn score(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn select_hand(&self, result: RPSResult) -> Self {
        match (self, result) {
            (RPS::Rock, RPSResult::Win) => RPS::Paper,
            (RPS::Rock, RPSResult::Lose) => RPS::Scissors,
            (RPS::Rock, RPSResult::Draw) => RPS::Rock,
            (RPS::Paper, RPSResult::Win) => RPS::Scissors,
            (RPS::Paper, RPSResult::Lose) => RPS::Rock,
            (RPS::Paper, RPSResult::Draw) => RPS::Paper,
            (RPS::Scissors, RPSResult::Win) => RPS::Rock,
            (RPS::Scissors, RPSResult::Lose) => RPS::Paper,
            (RPS::Scissors, RPSResult::Draw) => RPS::Scissors,
        }
    }
}

impl TryFrom<&str> for RPS {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(RPS::Rock),
            "B" | "Y" => Ok(RPS::Paper),
            "C" | "Z" => Ok(RPS::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RPSResult {
    Win,
    Lose,
    Draw,
}

impl RPSResult {
    fn score(&self) -> u32 {
        match self {
            RPSResult::Win => 6,
            RPSResult::Lose => 0,
            RPSResult::Draw => 3,
        }
    }
}

impl TryFrom<&str> for RPSResult {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Z" => Ok(RPSResult::Win),
            "X" => Ok(RPSResult::Lose),
            "Y" => Ok(RPSResult::Draw),
            _ => Err(()),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut v = line.split_whitespace().flat_map(RPS::try_from);
                let lhs = v.next().unwrap();
                let rhs = v.next().unwrap();
                rhs.vs(&lhs).score() + rhs.score()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut v = line.split_whitespace();
                let lhs = v.next().unwrap();
                let lhs = RPS::try_from(lhs).unwrap();

                let rhs = v.next().unwrap();
                let rhs = RPSResult::try_from(rhs).unwrap();
                let rhs = lhs.select_hand(rhs);
                rhs.vs(&lhs).score() + rhs.score()
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
