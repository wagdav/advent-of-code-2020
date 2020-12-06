use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

fn solve_part1(lines: &[&str]) -> usize {
    let mut sum = 0;

    for group in lines.split(|&line| line == "") {
        let mut questions: HashSet<char> = HashSet::new();

        for person in group {
            for answer in person.chars() {
                questions.insert(answer);
            }
        }

        sum += questions.len();
    }

    sum
}

fn common_answers(a: &str, b: &str) -> String {
    let a: HashSet<char> = a.chars().collect();
    let b: HashSet<char> = b.chars().collect();

    a.intersection(&b).collect()
}

fn solve_part2(lines: &[&str]) -> usize {
    let mut sum = 0;

    for group in lines.split(|&line| line == "") {
        sum += group
            .iter()
            .fold(group.first().unwrap().to_string(), |acc, x| {
                common_answers(&acc, &x)
            })
            .len();
    }

    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = read_to_string("inputs/day06.txt")?;
    let lines: Vec<&str> = file.lines().collect();

    println!("Part 1 {:?}", solve_part1(lines.as_slice()));
    println!("Part 2 {:?}", solve_part2(lines.as_slice()));

    Ok(())
}
