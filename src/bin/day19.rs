use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
enum Rule {
    SingleChar(char),
    Seq(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
}

impl Rule {
    fn new(input: &str) -> Self {
        if input.contains('"') {
            return Rule::SingleChar(input.chars().nth(1).unwrap());
        }

        let parts: Vec<_> = input.split('|').collect();

        if parts.len() == 1 {
            Rule::Seq(parts[0].split(' ').map(|x| x.parse().unwrap()).collect())
        } else {
            Rule::Or(
                parts[0]
                    .trim()
                    .split(' ')
                    .map(|x| x.parse().unwrap())
                    .collect(),
                parts[1]
                    .trim()
                    .split(' ')
                    .map(|x| x.parse().unwrap())
                    .collect(),
            )
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    rules: HashMap<usize, Rule>,
    lines: Vec<String>,
}

fn parse(file: &str) -> Input {
    let mut rules: HashMap<usize, Rule> = HashMap::new();
    let mut lines: Vec<String> = vec![];

    let mut parts = file.split("\n\n");

    let rule_lines = parts.next().unwrap();
    let msg_lines = parts.next().unwrap();

    for line in rule_lines.lines() {
        let line = line.trim();
        let mut parts = line.split(':');

        let id: usize = parts.next().unwrap().parse().unwrap();
        let ctx = parts.next().unwrap().trim();
        rules.insert(id, Rule::new(&ctx));
    }

    for line in msg_lines.lines() {
        let line = line.trim();
        lines.push(line.to_string());
    }

    Input { rules, lines }
}

impl Input {
    fn to_re(&self, i: usize, limit: usize) -> String {
        if limit == 0 {
            return "".to_string();
        }

        match &self.rules[&i] {
            Rule::SingleChar(c) => c.to_string(),
            Rule::Seq(seq) => {
                let mut res = "".to_string();
                for i in seq {
                    res += &self.to_re(*i, limit - 1);
                }
                res
            }

            Rule::Or(a, b) => {
                let mut res_a = "".to_string();
                for i in a {
                    res_a += &self.to_re(*i, limit - 1);
                }

                let mut res_b = "".to_string();
                for i in b {
                    res_b += &self.to_re(*i, limit - 1);
                }

                "(".to_owned() + &res_a + "|" + &res_b + ")"
            }
        }
    }
}

fn solve_part1(input: &Input) -> usize {
    let re = Regex::new(&("^".to_owned() + &input.to_re(0, 1000) + "$")).unwrap();

    input.lines.iter().filter(|x| re.is_match(x)).count()
}

fn solve_part2(input: &Input) -> usize {
    let mut input = input.to_owned();

    input.rules.insert(8, Rule::Or(vec![42], vec![42, 8]));
    input
        .rules
        .insert(11, Rule::Or(vec![42, 31], vec![42, 11, 31]));

    let re = Regex::new(&("^".to_owned() + &input.to_re(0, 20) + "$")).unwrap();

    input.lines.iter().filter(|x| re.is_match(x)).count()
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let file = read_to_string("inputs/day19.txt")?;

    let input = parse(&file);

    println!("Part 1 {:?}", solve_part1(&input));
    println!("Part 2 {:?}", solve_part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "\
        0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: \"a\"
        5: \"b\"

        ababbb
        bababa
        abbbab
        aaabbb
        aaaabbb\
        ";

    const EXAMPLE2: &str = "\
        42: 9 14 | 10 1
        9: 14 27 | 1 26
        10: 23 14 | 28 1
        1: \"a\"
        11: 42 31
        5: 1 14 | 15 1
        19: 14 1 | 14 14
        12: 24 14 | 19 1
        16: 15 1 | 14 14
        31: 14 17 | 1 13
        6: 14 14 | 1 14
        2: 1 24 | 14 4
        0: 8 11
        13: 14 3 | 1 12
        15: 1 | 14
        17: 14 2 | 1 7
        23: 25 1 | 22 14
        28: 16 1
        4: 1 1
        20: 14 14 | 1 15
        3: 5 14 | 16 1
        27: 1 6 | 14 18
        14: \"b\"
        21: 14 1 | 1 14
        25: 1 1 | 1 14
        22: 14 14
        8: 42
        26: 14 22 | 1 20
        18: 15 15
        7: 14 5 | 1 21
        24: 14 1

        abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
        bbabbbbaabaabba
        babbbbaabbbbbabbbbbbaabaaabaaa
        aaabbbbbbaaaabaababaabababbabaaabbababababaaa
        bbbbbbbaaaabbbbaaabbabaaa
        bbbababbbbaaaaaaaabbababaaababaabab
        ababaaaaaabaaab
        ababaaaaabbbaba
        baabbaaaabbaaaababbaababb
        abbbbabbbbaaaababbbbbbaaaababb
        aaaaabbaabaaaaababaa
        aaaabbaaaabbaaa
        aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
        babaaabbbaaabaababbaabababaaab
        aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba\
        ";

    #[test]
    fn parses_puzzle() {}

    #[test]
    fn solves_part1() {
        let input = parse(&EXAMPLE);
        assert_eq!(solve_part1(&input), 2);
    }

    #[test]
    fn solves_part2() {
        let input = parse(&EXAMPLE2);
        assert_eq!(solve_part2(&input), 12);
    }
}
