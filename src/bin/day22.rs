use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs::read_to_string;

type Deck = VecDeque<usize>;

fn parse_deck(input: &str) -> Deck {
    input
        .lines()
        .skip(1)
        .filter_map(|l| l.trim().parse().ok())
        .collect()
}

fn parse(file: &str) -> Option<(Deck, Deck)> {
    let mut parts = file.split("\n\n");
    Some((parse_deck(parts.next()?), parse_deck(parts.next()?)))
}

fn score(deck: &Deck) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (i + 1) * card)
        .sum()
}

fn solve_part1(p1: &Deck, p2: &Deck) -> usize {
    let mut p1 = p1.to_owned();
    let mut p2 = p2.to_owned();

    while !p1.is_empty() && !p2.is_empty() {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        if c1 > c2 {
            // Player 1 wins
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            // Player 2 wins
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }

    score(&p1) + score(&p2)
}

fn combat(p1: &Deck, p2: &Deck) -> (usize, usize) {
    let mut p1 = p1.to_owned();
    let mut p2 = p2.to_owned();

    let mut prev_states: HashSet<(Deck, Deck)> = HashSet::new();

    while !p1.is_empty() && !p2.is_empty() {
        if !prev_states.insert((p1.to_owned(), p2.to_owned())) {
            // Same configuration as before, player 1 wins.
            p2.clear();
            break;
        }

        // Players draw from the top of the deck
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        let winner = if c1 <= p1.len() && c2 <= p2.len() {
            // Both players have at least as many cards remaining in
            // their deck as the value of the card they just drew
            combat(
                &p1.iter().take(c1).cloned().collect(),
                &p2.iter().take(c2).cloned().collect(),
            )
            .0
        } else {
            // Otherwise the winner of the round is the player with the
            // higher-value card
            if c1 > c2 {
                1
            } else {
                2
            }
        };

        if winner == 1 {
            // Player 1 wins
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            // Player 2 wins
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }

    let winner = if p1.is_empty() { 2 } else { 1 };

    (winner, score(&p1) + score(&p2))
}

fn solve_part2(p1: &Deck, p2: &Deck) -> usize {
    combat(&p1, &p2).1
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let file = read_to_string("inputs/day22.txt")?;

    let (p1, p2) = parse(&file).unwrap();

    println!("Part 1 {:?}", solve_part1(&p1, &p2));
    println!("Part 2 {:?}", solve_part2(&p1, &p2));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "\
        Player 1:
        9
        2
        6
        3
        1

        Player 2:
        5
        8
        4
        7
        10\
    ";

    const EXAMPLE2: &str = "\
        Player 1:
        43
        19

        Player 2:
        2
        29
        14\
    ";

    #[test]
    fn solves_part1() {
        let (p1, p2) = parse(&EXAMPLE).unwrap();
        assert_eq!(solve_part1(&p1, &p2), 306);
    }

    #[test]
    fn avoids_infinite_loop() {
        let (p1, p2) = parse(&EXAMPLE2).unwrap();
        assert_eq!(solve_part2(&p1, &p2), 105);
    }

    #[test]
    fn solves_part2() {
        let (p1, p2) = parse(&EXAMPLE).unwrap();
        assert_eq!(solve_part2(&p1, &p2), 291);
    }
}
