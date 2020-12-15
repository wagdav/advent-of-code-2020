use std::collections::HashMap;
use std::error::Error;

fn last_number_v1(starting: &[usize], end: usize) -> usize {
    let mut numbers: Vec<usize> = vec![];

    numbers.extend_from_slice(starting);
    numbers.reserve(end);

    while numbers.len() < end {
        let last = *numbers.last().unwrap();

        let say = match numbers.iter().rev().skip(1).position(|&x| x == last) {
            Some(diff) => diff + 1,
            None => 0,
        };

        numbers.push(say);
    }

    *numbers.last().unwrap()
}

fn last_number_v2(starting: &[usize], end: usize) -> usize {
    let mut mem: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut turn = 0;
    let mut prev = 0;
    for n in starting.iter() {
        prev = *n;
        mem.insert(*n, vec![turn]);
        turn += 1;
    }

    while turn < end {
        let say = mem
            .get(&prev)
            .map(|v| if v.len() == 1 { 0 } else { v[0] - v[1] })
            .unwrap();

        match mem.get_mut(&say) {
            Some(v) => {
                v.insert(0, turn);
                v.truncate(2);
            }
            None => {
                mem.insert(say, vec![turn]);
            }
        }

        prev = say;
        turn += 1;
    }

    prev
}

fn solve_part1(starting: &[usize]) -> usize {
    last_number_v1(&starting, 2020)
}

fn solve_part2(starting: &[usize]) -> usize {
    last_number_v2(&starting, 30000000)
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let input = &[8, 0, 17, 4, 1, 12];

    println!("Part 1 {:?}", solve_part1(input));
    println!("Part 2 {:?}", solve_part2(input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part1() {
        assert_eq!(solve_part1(&[0, 3, 6]), 436);
        assert_eq!(solve_part1(&[1, 3, 2]), 1);
        assert_eq!(solve_part1(&[3, 1, 2]), 1836);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(last_number_v2(&[0, 3, 6], 2020), 436);
        assert_eq!(last_number_v2(&[1, 3, 2], 2020), 1);
        assert_eq!(last_number_v2(&[3, 1, 2], 2020), 1836);
    }
}
