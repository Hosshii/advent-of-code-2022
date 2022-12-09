use itertools::Itertools;
use nom::{
    branch::{alt, permutation},
    bytes::complete::tag,
    character::complete::{anychar, digit1, newline},
    combinator::{map, map_res},
    multi::{many0, separated_list0, separated_list1},
    sequence::delimited,
    IResult,
};

#[derive(Debug, Clone, Copy)]
struct Crate(char);

impl Crate {
    pub fn new(c: char) -> Self {
        Self(c)
    }
}

fn parse_space(s: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(s)
}

fn parse_crate(s: &str) -> IResult<&str, Crate> {
    map(delimited(tag("["), anychar, tag("]")), Crate::new)(s)
}

fn parse_crate_or_space(s: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_space, |_| None)))(s)
}

fn parse_crate_line(s: &str) -> IResult<&str, Vec<Option<Crate>>> {
    separated_list1(tag(" "), parse_crate_or_space)(s)
}

#[derive(Debug)]
struct Stacks(Vec<Vec<Crate>>);

impl Stacks {
    fn process1(&mut self, op: Op) {
        let from_idx = op.from - 1;
        let to_idx = op.to - 1;
        for _ in 0..op.num {
            let a = self.0[from_idx].pop().expect("pop");
            self.0[to_idx].push(a);
        }
    }

    fn process2(&mut self, op: Op) {
        let from_idx = op.from - 1;
        let to_idx = op.to - 1;
        let len = self.0[from_idx].len() - op.num;
        let mut v2 = self.0[from_idx].split_off(len);
        self.0[to_idx].append(&mut v2);
    }

    fn tops(&self) -> String {
        self.0.iter().fold(String::new(), |mut acc, cur| {
            if let Some(x) = cur.last() {
                acc.push(x.0);
            }
            acc
        })
    }
}

impl From<Vec<Vec<Crate>>> for Stacks {
    fn from(v: Vec<Vec<Crate>>) -> Self {
        Self(v)
    }
}

fn parse_crate_lines(s: &str) -> IResult<&str, Stacks> {
    map(many0(permutation((parse_crate_line, newline))), |v| {
        let stack_num = v[0].0.len();
        v.into_iter()
            .rev()
            .fold(vec![vec![]; stack_num], |mut acc, w| {
                let w = w.0;
                for (i, crate_) in w.into_iter().enumerate() {
                    if let Some(c) = crate_ {
                        acc[i].push(c);
                    }
                }
                acc
            })
            .into()
    })(s)
}

#[derive(Debug, Clone, Copy)]
struct Op {
    num: usize,
    from: usize,
    to: usize,
}

impl Op {
    pub fn new(num: usize, from: usize, to: usize) -> Self {
        Self { num, from, to }
    }
}

fn parse_procedure(s: &str) -> IResult<&str, Op> {
    map(
        permutation((
            tag("move "),
            map_res(digit1, str::parse),
            tag(" from "),
            map_res(digit1, str::parse),
            tag(" to "),
            map_res(digit1, str::parse),
        )),
        |(_, num, _, from, _, to)| Op::new(num, from, to),
    )(s)
}

fn parse_procedures(s: &str) -> IResult<&str, Vec<Op>> {
    separated_list0(newline, parse_procedure)(s)
}

pub fn part_one(input: &str) -> Option<String> {
    let (crates, procedure) = input
        .split("\n\n")
        .collect_tuple()
        .expect("have crate and procedure");
    let (_, mut stacks) = parse_crate_lines(crates).expect("parse stack error");
    let (_, procedures) = parse_procedures(procedure).expect("parse procedure error");

    for p in procedures {
        stacks.process1(p);
    }

    Some(stacks.tops())
}

pub fn part_two(input: &str) -> Option<String> {
    let (crates, procedure) = input
        .split("\n\n")
        .collect_tuple()
        .expect("have crate and procedure");
    let (_, mut stacks) = parse_crate_lines(crates).expect("parse stack error");
    let (_, procedures) = parse_procedures(procedure).expect("parse procedure error");

    for p in procedures {
        stacks.process2(p);
    }

    Some(stacks.tops())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
