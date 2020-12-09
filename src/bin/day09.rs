use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs::read_to_string;

// Solution from day 01
fn offending(target: i64, input: &[i64]) -> Option<i64> {
    let input_set: HashSet<_> = input.iter().cloned().collect();
    let complement: HashSet<_> = input.iter().map(|x| target - x).collect();

    let res: HashSet<_> = input_set.intersection(&complement).collect();

    for i in res.iter() {
        for j in res.iter() {
            if input.contains(i) && input.contains(j) {
                return None;
            }
        }
    }

    Some(target)
}

fn solve_part1(input: &[i64], preamble: usize) -> Option<i64> {
    input
        .iter()
        .skip(preamble)
        .zip(input.windows(preamble))
        .find_map(|(&target, elems)| offending(target, elems))
}

fn solve_part2(input: &[i64], target: i64) -> Option<i64> {
    let mut buf: VecDeque<i64> = VecDeque::new();

    for elem in input.iter() {
        while buf.iter().sum::<i64>() + elem > target {
            buf.pop_front();
        }

        buf.push_back(*elem);

        if buf.iter().sum::<i64>() == target {
            let min = buf.iter().min()?;
            let max = buf.iter().max()?;
            return Some(min + max);
        }
    }

    None
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let file = read_to_string("inputs/day09.txt")?;
    let xmas: Vec<i64> = file.lines().filter_map(|line| line.parse().ok()).collect();

    let invalid_number = solve_part1(&xmas, 25).expect("Couldn't find the offending number");
    println!("Part 1 {:?}", invalid_number);
    println!("Part 2 {:?}", solve_part2(&xmas, invalid_number));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[i64] = &[
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    #[test]
    fn example1() {
        assert_eq!(solve_part1(&EXAMPLE, 5), Some(127));
        assert_eq!(solve_part2(&EXAMPLE, 127), Some(15 + 47)); // min + max between 15, 25, 47, 40
    }
}
