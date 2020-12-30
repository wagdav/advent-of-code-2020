use std::collections::HashMap;
use std::error::Error;

fn play(cups: &[usize], rounds: usize) -> Vec<usize> {
    let max = *cups.iter().max().unwrap();
    let mut cups = cups.to_owned();
    let mut current = cups[0];

    for _round in 0..rounds {
        let mut taken_cups: Vec<usize> = cups
            .iter()
            .cycle()
            .skip_while(|x| **x != current)
            .skip(1)
            .take(3)
            .cloned()
            .collect();

        // remove taken cups
        cups = cups
            .iter()
            .filter(|x| !taken_cups.contains(x))
            .cloned()
            .collect();

        // find destination cup and its position
        let mut destination = if current > 1 { current - 1 } else { max };

        loop {
            if let Some(pos) = cups.iter().cloned().position(|x| x == destination) {
                // insert taken cups after
                while let Some(taken_cup) = taken_cups.pop() {
                    cups.insert(pos + 1, taken_cup);
                }
                break;
            }

            destination = if destination > 1 {
                destination - 1
            } else {
                max
            };
        }

        current = *cups
            .iter()
            .cycle()
            .skip_while(|x| **x != current)
            .nth(1)
            .unwrap();
    }

    cups
}

fn solve_part1(cups: &[usize]) -> String {
    play(cups, 100)
        .iter()
        .cycle()
        .skip_while(|x| **x != 1)
        .skip(1)
        .take(cups.len() - 1)
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn play_v2(cups: &[usize], rounds: usize) -> Vec<usize> {
    let n = cups.len();
    let mut next: HashMap<usize, usize> = cups
        .windows(2)
        .map(|w| (w[0], w[1]))
        .collect();
    next.insert(cups[n - 1], cups[0]);

    let mut current = cups[0];

    for _round in 0..rounds {
        let mut pick: Vec<usize> = vec![next[&current]];
        for _ in 0..2 {
            pick.push(next[pick.iter().last().unwrap()]);
        }

        let mut destination = if current > 1 { current - 1 } else { n };

        while pick.contains(&destination) {
            destination = if destination > 1 { destination - 1 } else { n }
        }

        next.insert(current, next[&pick[2]]);
        next.insert(pick[2], next[&destination]);
        next.insert(destination, pick[0]);

        current = next[&current];
    }

    let mut res: Vec<usize> = vec![next[&1]];
    for _ in 0..1 {
        res.push(next[res.iter().last().unwrap()]);
    }

    res
}

fn solve_part2(cups: &[usize]) -> usize {
    let mut cups = cups.to_owned();
    for c in cups.len() + 1..=1_000_000 {
        cups.push(c);
    }

    assert_eq!(cups.len(), 1_000_000);
    assert_eq!(cups.iter().max(), Some(&1_000_000));

    play_v2(&cups, 10_000_000).iter().take(2).product()
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let cups = [3, 8, 9, 5, 4, 7, 6, 1, 2];

    println!("Part 1 {:?}", solve_part1(&cups));
    println!("Part 2 {:?}", solve_part2(&cups));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part1() {
        let cups = [3, 8, 9, 1, 2, 5, 4, 6, 7];
        assert_eq!(play(&cups, 10), vec![5, 8, 3, 7, 4, 1, 9, 2, 6]);
        assert_eq!(solve_part1(&cups), "67384529");
    }

    #[test]
    fn test_play_v2() {
        let cups = [3, 8, 9, 1, 2, 5, 4, 6, 7];
        assert_eq!(play_v2(&cups, 1), vec![5, 4]);
        assert_eq!(play_v2(&cups, 10), vec![9, 2]);
    }

    #[test]
    #[ignore]
    fn solves_part2() {
        let cups = [3, 8, 9, 1, 2, 5, 4, 6, 7];
        assert_eq!(solve_part2(&cups), 149245887792);
    }
}
