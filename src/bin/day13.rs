use std::error::Error;
use std::fs::read_to_string;

/// Code "borrowed" from Rosetta-code
/// https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn parse(input: &str) -> (i32, Vec<Option<i32>>) {
    let mut lines = input.lines();
    let departure_time = lines.next().unwrap().trim().parse().unwrap();
    let buses: Vec<_> = lines
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse().ok())
        .collect();
    (departure_time, buses)
}

fn solve_part1((departure, buses): &(i32, Vec<Option<i32>>)) -> i32 {
    for t in *departure.. {
        for b in buses.iter().filter_map(|x| *x) {
            if t % b == 0 {
                return (t - departure) * b;
            }
        }
    }

    0
}

fn solve_part2(buses: &[Option<i32>]) -> i64 {
    let (residues, modulii): (Vec<_>, Vec<_>) = buses
        .iter()
        .enumerate()
        .filter_map(|(a, b)| b.map(|n| (a as i64, n as i64)))
        .unzip();

    modulii.iter().product::<i64>() - chinese_remainder(&residues, &modulii).unwrap()
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let file = read_to_string("inputs/day13.txt")?;

    let input = parse(&file);

    println!("Part 1 {:?}", solve_part1(&input));
    println!("Part 2 {:?}", solve_part2(&input.1));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "\
        939
        7,13,x,x,59,x,31,19\
    ";

    #[test]
    fn parses_puzzle() {
        let input = parse(&EXAMPLE);
        assert_eq!(
            input,
            (
                939,
                vec![
                    Some(7),
                    Some(13),
                    None,
                    None,
                    Some(59),
                    None,
                    Some(31),
                    Some(19)
                ]
            )
        );
    }

    #[test]
    fn solves_part1() {
        let input = parse(&EXAMPLE);
        assert_eq!(solve_part1(&input), 295);
    }

    #[test]
    fn solves_part2() {
        let (_, schedule) = parse(&EXAMPLE);
        assert_eq!(solve_part2(&schedule), 1068781);
        assert_eq!(solve_part2(&[Some(17), None, Some(13), Some(19)]), 3417);
    }
}
