use std::fs::File;
use std::io::{self, BufRead};

fn trees(input: &[String], right: usize, down: usize) -> usize {
    input
        .iter()
        .step_by(down)
        .enumerate()
        .map(|(i, items)| items.chars().cycle().nth(i * right))
        .filter(|&c| c == Some('#'))
        .count()
}

fn main() {
    let file = File::open("inputs/day03.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let lines: Vec<String> = lines.map(|line| line.unwrap()).collect();

    println!("Part 1 {:?}", trees(&lines, 3, 1));

    let prod: usize = [
        trees(&lines, 1, 1),
        trees(&lines, 3, 1),
        trees(&lines, 5, 1),
        trees(&lines, 7, 1),
        trees(&lines, 1, 2),
    ]
    .iter()
    .product();
    println!("Part 2 {:?}", prod);
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn example1() {
        let example = vec![
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ];

        assert_eq!(trees(&example, 1, 1), 2);
        assert_eq!(trees(&example, 3, 1), 7);
        assert_eq!(trees(&example, 5, 1), 3);
        assert_eq!(trees(&example, 7, 1), 4);
        assert_eq!(trees(&example, 1, 2), 2);
    }
}
