use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(|v| v as u8 - b'0').collect())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    let transposed = transpose(grid.clone());
    let mut memo = vec![vec![false; grid[0].len()]; grid.len()];

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, val) in row.iter().enumerate() {
            if is_edge(&grid, row_idx, col_idx) {
                memo[row_idx][col_idx] = true;
                continue;
            }

            let visible = row[0..col_idx].iter().all(|v| v < val)
                || row[col_idx + 1..].iter().all(|v| v < val);

            memo[row_idx][col_idx] |= visible;
        }
    }

    for (row_idx, row) in transposed.iter().enumerate() {
        for (col_idx, val) in row.iter().enumerate() {
            // if is_edge(&grid, row_idx, col_idx) {
            //     memo[row_idx][col_idx] = true;
            //     continue;
            // }

            let visible = row[0..col_idx].iter().all(|v| v < val)
                || row[col_idx + 1..].iter().all(|v| v < val);

            // here is only difference
            memo[col_idx][row_idx] |= visible;
        }
    }

    let result = memo
        .iter()
        .map(|v| v.iter().filter(|v| **v).count() as u32)
        .sum::<u32>();

    Some(result)
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn is_edge(grid: &Vec<Vec<u8>>, row_idx: usize, col_idx: usize) -> bool {
    row_idx == 0 || col_idx == 0 || row_idx == grid.len() - 1 || col_idx == grid[0].len() - 1
}

fn count_iter(iter: impl Iterator<Item = u8>, val: u8) -> u32 {
    let mut count = 0;
    for i in iter {
        count += 1;
        if i >= val {
            break;
        }
    }
    count
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse(input);
    let transposed = transpose(grid.clone());
    let mut memo = vec![vec![1u32; grid[0].len()]; grid.len()];

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, val) in row.iter().enumerate() {
            let left = count_iter(row[0..col_idx].iter().copied().rev(), *val);
            let right = count_iter(row[col_idx + 1..].iter().copied(), *val);

            memo[row_idx][col_idx] *= left * right;
        }
    }

    for (row_idx, row) in transposed.iter().enumerate() {
        for (col_idx, val) in row.iter().enumerate() {
            let left = count_iter(row[0..col_idx].iter().copied().rev(), *val);
            let right = count_iter(row[col_idx + 1..].iter().copied(), *val);

            memo[col_idx][row_idx] *= left * right;
        }
    }

    let result = memo
        .iter()
        .map(|v| *v.iter().max().unwrap() as u32)
        .max()
        .unwrap();

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
