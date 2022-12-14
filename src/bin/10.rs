#[derive(Debug, Clone, Copy)]
enum Op {
    Noop,
    Addx(i32),
}

impl Op {
    fn cycle(&self) -> u32 {
        match self {
            Op::Noop => 1,
            Op::Addx(_) => 2,
        }
    }
}

// impl<U> TryFrom<U> for Op
// where U: Into<Op> {}

impl TryFrom<&str> for Op {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split_whitespace();
        let op = parts.next().ok_or("No op")?;
        match op {
            "noop" => Ok(Op::Noop),
            "addx" => Ok(Op::Addx(
                parts
                    .next()
                    .ok_or("No arg")?
                    .parse()
                    .map_err(|_| "Invalid")?,
            )),
            _ => Err("Invalid op"),
        }
    }
}

struct Cpu {
    // part1
    register_x: i32,
    cycle: u32,
    state: Vec<i32>,

    // part2
    display: [[bool; 40]; 6],
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            register_x: 1,
            cycle: 0,
            state: Vec::new(),

            display: [[false; 40]; 6],
        }
    }

    fn process(&mut self, op: Op) {
        self.cycle += op.cycle();
        for _ in 0..op.cycle() {
            self.state.push(self.register_x);
        }
        match op {
            Op::Noop => {}
            Op::Addx(x) => self.register_x += x,
        }
    }

    fn calc_score(&self, cycle: u32) -> i32 {
        cycle as i32 * self.state[cycle as usize - 1]
    }

    fn process2(&mut self, op: Op) {
        let cycle = op.cycle();
        for _ in 0..cycle {
            self.draw();
            self.cycle += 1;
        }
        match op {
            Op::Noop => {}
            Op::Addx(x) => self.register_x += x,
        }
    }

    fn draw(&mut self) {
        let (x, y) = self.pos();
        if self.register_x - 1 <= x as i32 && x as i32 <= self.register_x + 1 {
            self.display[y][x] = true;
        }
    }

    fn pos(&self) -> (usize, usize) {
        ((self.cycle % 40) as usize, (self.cycle / 40) as usize)
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut cpu = Cpu::new();
    let programs = input.lines().flat_map(Op::try_from);
    programs.for_each(|p| cpu.process(p));

    let start_cycle = 20;
    Some(
        (start_cycle..)
            .step_by(40)
            .take(6)
            .map(|i| cpu.calc_score(i))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cpu = Cpu::new();
    let programs = input.lines().flat_map(Op::try_from);
    programs.for_each(|p| cpu.process2(p));

    for row in cpu.display {
        for col in row {
            print!("{}", if col { '#' } else { '.' });
        }
        println!();
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
