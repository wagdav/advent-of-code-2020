use std::error::Error;
use std::fs::read_to_string;

fn solve_part1(input: &[i32]) -> Option<usize> {
    let mut x: Vec<i32> = input.to_vec();
    x.push(0);

    x.sort();

    let ones = x
        .iter()
        .zip(x.iter().skip(1))
        .map(|(a, b)| b - a)
        .filter(|&x| x == 1)
        .count();
    let three = x
        .iter()
        .zip(x.iter().skip(1))
        .map(|(a, b)| b - a)
        .filter(|&x| x == 3)
        .count();

    Some(ones * (three + 1))
}

fn solve_part2(input: &[i32]) -> Option<i32> {
    let mut x: Vec<i32> = input.to_vec();
    x.push(0);

    x.sort();
    let diff: Vec<i32> = x.iter().zip(x.iter().skip(1)).map(|(a, b)| b - a).collect();

    let mut res: i32 = 1;
    let mut acc: Vec<i32> = vec![];
    for d in diff {
        if d == 1 {
            acc.push(d);
        } else {
            if acc.len() > 1 {
                let base: i32 = 2;
                //let n_one_one = std::cmp::max(acc.len() as i32, 3);
                res *= base.pow(acc.len() as u32 - 1);
            }
            acc.clear();
        }
    }

    dbg!(res);

    Some(res)
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let file = read_to_string("inputs/day10.txt")?;
    let jolt: Vec<i32> = file.lines().filter_map(|line| line.parse().ok()).collect();

    println!("Part 1 {:?}", solve_part1(&jolt));
    println!("Part 2 {:?}", solve_part2(&jolt));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &[i32] = &[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    const EXAMPLE2: &[i32] = &[
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];

    #[test]
    fn example1() {
        assert_eq!(solve_part1(&EXAMPLE1), Some(7 * 5));
        //assert_eq!(solve_part2(&EXAMPLE1), Some(8));
    }

    #[test]
    fn example2() {
        //assert_eq!(solve_part2(&EXAMPLE2), Some(19208));
    }
}
