use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

fn solve_part1(input: &[i32]) -> usize {
    let mut x: Vec<i32> = input.to_vec();
    x.push(0);

    x.sort();

    let ones = x
        .windows(2)
        .map(|w| w[1] - w[0])
        .filter(|&x| x == 1)
        .count();
    let three = x
        .windows(2)
        .map(|w| w[1] - w[0])
        .filter(|&x| x == 3)
        .count();

    ones * (three + 1)
}

fn solve_part2(input: &[i32]) -> usize {
    let mut x = input.to_vec();
    let mut m: HashMap<i32, usize> = HashMap::new();

    x.sort();

    let last = x.pop().unwrap();
    x.push(last);
    x.push(last + 3);

    m.insert(0, 1);
    for i in &x {
        m.insert(
            *i,
            *m.get(&(i - 1)).unwrap_or(&0)
                + *m.get(&(i - 2)).unwrap_or(&0)
                + *m.get(&(i - 3)).unwrap_or(&0),
        );
    }

    let last = x.iter().last().unwrap();

    *m.get(last).unwrap()
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
        assert_eq!(solve_part1(&EXAMPLE1), 7 * 5);
        assert_eq!(solve_part2(&EXAMPLE1), 8);
    }

    #[test]
    fn example2() {
        assert_eq!(solve_part2(&EXAMPLE2), 19208);
    }
}
