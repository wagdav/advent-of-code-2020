use regex::Regex;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
struct Entry {
    min_occurs: usize,
    max_occurs: usize,
    letter: char,
    password: String,
}

impl Entry {
    fn valid_part1(&self) -> bool {
        let count = self.password.chars().filter(|x| *x == self.letter).count();
        self.min_occurs <= count && count <= self.max_occurs
    }

    fn valid_part2(&self) -> bool {
        // indexing is 1-based
        let pos1 = self.min_occurs - 1;
        let pos2 = self.max_occurs - 1;

        let c1 = self.password.chars().nth(pos1);
        let c2 = self.password.chars().nth(pos2);

        (c1 == Some(self.letter)) ^ (c2 == Some(self.letter))
    }

    fn parse(input: &str) -> Option<Self> {
        let re = Regex::new(
            r"(?x)
            ^(?P<min_occurs>\d+)
            -
            (?P<max_occurs>\d+)
            \s
            (?P<letter>[[:alpha:]])
            :
            \s
            (?P<password>[[:alpha:]]+)
            ",
        )
        .expect("Invalid regular expression");
        let captures = re.captures(input)?;

        let min_occurs = captures.name("min_occurs")?.as_str().parse().ok()?;
        let max_occurs = captures.name("max_occurs")?.as_str().parse().ok()?;
        let letter = captures.name("letter")?.as_str().chars().next()?;
        let password = captures.name("password")?.as_str().to_string();

        Some(Self {
            min_occurs,
            max_occurs,
            letter,
            password,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("inputs/day02.txt")?;

    println!(
        "Part 1 {:?}",
        input
            .lines()
            .filter_map(|line| Entry::parse(&line))
            .filter(|entry| entry.valid_part1())
            .count()
    );

    println!(
        "Part 2 {:?}",
        input
            .lines()
            .filter_map(|line| Entry::parse(&line))
            .filter(|entry| entry.valid_part2())
            .count()
    );

    Ok(())
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn example1() {
        let entry = Entry {
            min_occurs: 1,
            max_occurs: 3,
            letter: 'a',
            password: "abcde".to_string(),
        };
        assert_eq!(entry.valid_part1(), true);
        assert_eq!(entry.valid_part2(), true);
        assert_eq!(Some(entry), Entry::parse(&"1-3 a: abcde"))
    }

    #[test]
    fn example2() {
        let entry = Entry {
            min_occurs: 1,
            max_occurs: 3,
            letter: 'b',
            password: "cdefg".to_string(),
        };
        assert_eq!(entry.valid_part1(), false);
        assert_eq!(entry.valid_part2(), false);
        assert_eq!(Some(entry), Entry::parse("1-3 b: cdefg"))
    }

    #[test]
    fn example3() {
        let entry = Entry {
            min_occurs: 2,
            max_occurs: 9,
            letter: 'c',
            password: "ccccccccc".to_string(),
        };
        assert_eq!(entry.valid_part1(), true);
        assert_eq!(entry.valid_part2(), false);
        assert_eq!(Some(entry), Entry::parse("2-9 c: ccccccccc"))
    }
}
