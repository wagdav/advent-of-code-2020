use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug)]
struct Input {
    fields: Vec<(String, Vec<(usize, usize)>)>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

fn parse(input: &str) -> Option<Input> {
    let mut fields: Vec<(String, Vec<(usize, usize)>)> = vec![];

    let mut sections = input.split("\n\n");

    let fields_section = sections.next()?;

    let re_range = Regex::new(r"(?P<low>[[:digit:]]+)\-(?P<high>[[:digit:]]+)").unwrap();
    for line in fields_section.lines() {
        let mut parts = line.trim().split(':');

        let name = parts.next()?.to_string();
        let text = parts.next()?;
        let mut values = vec![];
        for r in re_range.captures_iter(text) {
            values.push((r["low"].parse().ok()?, r["high"].parse().ok()?));
        }
        fields.push((name, values));
    }

    let my_ticket: Vec<usize> = sections
        .next()?
        .lines()
        .nth(1)?
        .trim()
        .split(',')
        .filter_map(|x| x.parse().ok())
        .collect();

    let nearby_tickets = sections
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| {
            line.trim()
                .split(',')
                .filter_map(|x| x.parse().ok())
                .collect()
        })
        .collect();

    Some(Input {
        fields,
        my_ticket,
        nearby_tickets,
    })
}

fn invalid(ticket: &[usize], fields: &[(String, Vec<(usize, usize)>)]) -> Vec<usize> {
    let mut res = vec![];

    for t in ticket {
        let mut ok = false;
        't: for (_, f) in fields.iter() {
            for (lo, hi) in f {
                if t >= lo && t <= hi {
                    ok = true;
                    break 't;
                }
            }
        }

        if !ok {
            res.push(*t)
        }
    }

    res
}

fn solve_part1(input: &Input) -> usize {
    input
        .nearby_tickets
        .iter()
        .map(|ticket| invalid(&ticket, &input.fields).iter().sum::<usize>())
        .sum()
}

fn find_fields(input: &Input) -> Vec<(String, usize)> {
    let valid_tickets: Vec<_> = input
        .nearby_tickets
        .iter()
        .filter(|t| invalid(&t, &input.fields).is_empty())
        .collect();

    let mut indexes: HashSet<usize> = (0..input.my_ticket.len()).collect();
    let mut res: Vec<(String, usize)> = vec![];

    for _ in 0..input.my_ticket.len() {
        for (n, c) in &input.fields {
            let (lo1, hi1) = c[0];
            let (lo2, hi2) = c[1];

            let mut matching: Vec<usize> = vec![];
            for i in &indexes {
                if valid_tickets
                    .iter()
                    .all(|t| (lo1 <= t[*i] && t[*i] <= hi1) || (lo2 <= t[*i] && t[*i] <= hi2))
                {
                    matching.push(*i);
                }
            }

            if matching.len() == 1 {
                indexes.remove(&matching[0]);
                res.push((n.to_string(), input.my_ticket[matching[0]]))
            }
        }
    }

    res
}

fn solve_part2(input: &Input) -> usize {
    find_fields(&input)
        .iter()
        .filter_map(|(name, t)| {
            if name.starts_with("departure") {
                Some(t)
            } else {
                None
            }
        })
        .product()
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let file = read_to_string("inputs/day16.txt")?;

    let input = parse(&file).unwrap();

    println!("Part 1 {:?}", solve_part1(&input));
    println!("Part 2 {:?}", solve_part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE1: &str = "\
        class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50

        your ticket:
        7,1,14

        nearby tickets:
        7,3,47
        40,4,50
        55,2,20
        38,6,12\
    ";

    const EXAMPLE2: &str = "\
        class: 0-1 or 4-19
        row: 0-5 or 8-19
        seat: 0-13 or 16-19

        your ticket:
        11,12,13

        nearby tickets:
        3,9,18
        15,1,5
        5,14,9\
    ";

    #[test]
    fn parses_puzzle() {
        let input = parse(&EXAMPLE1).unwrap();
        assert_eq!(input.fields.len(), 3);
        assert_eq!(input.my_ticket.len(), 3);
        assert_eq!(input.nearby_tickets.len(), 4);
    }

    #[test]
    fn solves_part1() {
        let input = parse(&EXAMPLE1).unwrap();
        assert_eq!(solve_part1(&input), 71);
    }

    #[test]
    fn identifies_fields() {
        let input = parse(&EXAMPLE2).unwrap();

        assert_eq!(
            find_fields(&input),
            vec![
                ("seat".to_string(), 13),
                ("class".to_string(), 12),
                ("row".to_string(), 11),
            ]
        );
    }
}
