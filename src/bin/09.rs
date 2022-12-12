use itertools::Itertools;
use std::{cmp::Ordering, collections::HashSet};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct TailHead {
    tail: Tail,
    head: Head,
    memo: HashSet<(i32, i32)>,
}

enum Relation {
    Left,
    Right,
    Up,
    Down,
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
    Same,
}

impl TailHead {
    fn new(tail: Tail, head: Head) -> Self {
        Self {
            tail,
            head,
            memo: HashSet::new(),
        }
    }

    fn move_head(&mut self, op: Operation) {
        assert!(self.is_neighbor());

        let head = self.head.0;
        let head = match op {
            Operation::Right => head.move_right(),
            Operation::Left => head.move_left(),
            Operation::Up => head.move_up(),
            Operation::Down => head.move_down(),
        };
        self.head.0 = head;
    }

    fn insert_memo(&mut self) {
        self.memo.insert((self.tail.0.x, self.tail.0.y));
    }

    fn count(&self) -> usize {
        self.memo.len()
    }

    fn move_tail(&mut self) {
        if self.is_neighbor() {
            return;
        }
        let relation = self.relation();
        let tail = self.tail.0;
        let tail = match relation {
            Relation::Left => tail.move_left(),
            Relation::Right => tail.move_right(),
            Relation::Up => tail.move_up(),
            Relation::Down => tail.move_down(),
            Relation::UpperLeft => tail.move_upper_left(),
            Relation::UpperRight => tail.move_upper_right(),
            Relation::LowerLeft => tail.move_lower_left(),
            Relation::LowerRight => tail.move_lower_right(),
            Relation::Same => todo!(),
        };
        self.tail.0 = tail;
    }

    /// head locates `Relation` of tail
    /// H
    ///  T
    /// returns `Relation::UpperLeft`
    fn relation(&self) -> Relation {
        self.head.0.relation(self.tail.0)
    }

    fn is_neighbor(&self) -> bool {
        self.head.0.is_neighbor(self.tail.0) || self.head.0 == self.tail.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn move_up(self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn move_down(self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn move_left(self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn move_right(self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn move_upper_right(self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }

    fn move_upper_left(self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn move_lower_right(self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y - 1,
        }
    }

    fn move_lower_left(self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y - 1,
        }
    }

    fn relation(self, other: Self) -> Relation {
        match (self.x.cmp(&other.x), self.y.cmp(&other.y)) {
            (Ordering::Less, Ordering::Less) => Relation::LowerLeft,
            (Ordering::Less, Ordering::Equal) => Relation::Left,
            (Ordering::Less, Ordering::Greater) => Relation::UpperLeft,
            (Ordering::Equal, Ordering::Less) => Relation::Down,
            (Ordering::Equal, Ordering::Equal) => Relation::Same,
            (Ordering::Equal, Ordering::Greater) => Relation::Up,
            (Ordering::Greater, Ordering::Less) => Relation::LowerRight,
            (Ordering::Greater, Ordering::Equal) => Relation::Right,
            (Ordering::Greater, Ordering::Greater) => Relation::UpperRight,
        }
    }

    fn is_neighbor(self, other: Self) -> bool {
        if self.x == other.x {
            self.y == other.y + 1 || self.y == other.y - 1
        } else if self.y == other.y {
            self.x == other.x + 1 || self.x == other.x - 1
        } else if self.x + 1 == other.x || self.x - 1 == other.x {
            self.y + 1 == other.y || self.y - 1 == other.y
        } else {
            false
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
struct Tail(Pos);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
struct Head(Pos);

impl TryFrom<&str> for OpeWithDistance {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (ope, distance) = value.split_whitespace().collect_tuple().unwrap();
        let distance = distance.parse::<i32>().unwrap();
        let operation = match ope {
            "R" => Operation::Right,
            "L" => Operation::Left,
            "U" => Operation::Up,
            "D" => Operation::Down,
            _ => return Err(()),
        };
        Ok(Self {
            operation,
            distance,
        })
    }
}

struct OpeWithDistance {
    operation: Operation,
    distance: i32,
}

#[derive(Debug, Clone, Copy)]

enum Operation {
    Right,
    Left,
    Up,
    Down,
}

pub fn part_one(input: &str) -> Option<i32> {
    let ops = input
        .lines()
        .flat_map(OpeWithDistance::try_from)
        .collect_vec();

    let mut th = TailHead::default();
    th.insert_memo();
    for op in ops {
        for i in 0..op.distance {
            th.move_head(op.operation);
            th.move_tail();
            th.insert_memo();
        }
    }

    Some(th.count() as i32)
}

pub fn part_two(input: &str) -> Option<i32> {
    let ops = input
        .lines()
        .flat_map(OpeWithDistance::try_from)
        .collect_vec();

    let mut ths = (0..9).map(|_| TailHead::default()).collect_vec();
    ths.iter_mut().for_each(|v| v.insert_memo());

    for op in ops {
        for _ in 0..op.distance {
            ths[0].move_head(op.operation);
            ths[0].move_tail();
            ths[0].insert_memo();
            let mut start = 0;
            let mut end = 1;
            while end < ths.len() {
                let tail = ths[start].tail;
                ths[end].head.0 = tail.0;
                ths[end].move_tail();
                ths[end].insert_memo();

                start += 1;
                end += 1;
            }
        }
    }

    Some(ths.iter().last().unwrap().count() as i32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
