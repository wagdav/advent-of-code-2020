use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

fn parse_line(line: &str) -> (String, Vec<(i32, String)>) {
    let line = line.trim();
    let mut parts = line.split(" bags contain ");

    let bag = parts.next().unwrap();
    let content = parts.next().unwrap();

    let mut res = vec![];

    if content == "no other bags." {
        return (bag.to_string(), res);
    }

    for content in content.split(", ") {
        let mut parts = content.split(' ');
        let qty = parts.next().unwrap().parse().unwrap();
        res.push((
            qty,
            format!("{} {}", parts.next().unwrap(), parts.next().unwrap()),
        ));
    }

    (bag.to_string(), res)
}

type Rules = HashMap<String, Vec<(i32, String)>>;

fn rules(lines: &[&str]) -> Rules {
    let mut rules = Rules::new();
    for line in lines {
        let (bag_color, contents) = parse_line(line);
        assert_eq!(rules.contains_key(&bag_color), false);
        rules.insert(bag_color, contents);
    }

    rules
}

fn count_shiny_gold(rules: &Rules, color: &str) -> i32 {
    let target = "shiny gold";
    let mut sum = 0;

    for (_qty, value) in &rules[color] {
        if value == target {
            sum += 1;
        } else {
            sum += count_shiny_gold(rules, &value);
        }
    }

    sum
}

fn count_bags(rules: &Rules, color: &str) -> i32 {
    rules[color]
        .iter()
        .map(|(num_bags, color)| num_bags + num_bags * count_bags(&rules, &color))
        .sum()
}

fn solve_part1(rules: &Rules) -> usize {
    rules
        .iter()
        .map(|(color, _)| count_shiny_gold(&rules, color))
        .filter(|&x| x > 0)
        .count() as usize
}

fn solve_part2(rules: &Rules) -> i32 {
    count_bags(&rules, "shiny gold")
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = read_to_string("inputs/day07.txt")?;
    let lines: Vec<_> = file.lines().collect();

    let rules = rules(&lines);

    println!("Part 1 {:?}", solve_part1(&rules));
    println!("Part 2 {:?}", solve_part2(&rules));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_common_line() {
        assert_eq!(
            parse_line("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags."),
            (
                "shiny gold".to_string(),
                vec!(
                    (1, "dark olive".to_string()),
                    (2, "vibrant plum".to_string())
                )
            )
        );
    }

    #[test]
    fn can_parse_no_other_bags_line() {
        assert_eq!(
            parse_line("silver blue bags contain no other bags."),
            ("silver blue".to_string(), vec![],)
        );
    }

    #[test]
    fn example1() {
        let text: Vec<_> = "\
                light red bags contain 1 bright white bag, 2 muted yellow bags.
                dark orange bags contain 3 bright white bags, 4 muted yellow bags.
                bright white bags contain 1 shiny gold bag.
                muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
                shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
                dark olive bags contain 3 faded blue bags, 4 dotted black bags.
                vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
                faded blue bags contain no other bags.
                dotted black bags contain no other bags.\
            "
        .lines()
        .collect();

        let rules = rules(&text);

        assert_eq!(solve_part1(&rules), 4);

        assert_eq!(count_bags(&rules, "faded blue"), 0);
        assert_eq!(count_bags(&rules, "vibrant plum"), 11);
        assert_eq!(count_bags(&rules, "shiny gold"), 32);
    }
}
