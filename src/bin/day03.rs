use std::error::Error;
use std::fs::read_to_string;

fn trees(input: &[&str], right: usize, down: usize) -> usize {
    input
        .iter()
        .step_by(down)
        .enumerate()
        .map(|(i, items)| items.chars().cycle().nth(i * right))
        .filter(|&c| c == Some('#'))
        .count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("inputs/day03.txt")?;
    let lines: Vec<_> = input.lines().collect();

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

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ];

        assert_eq!(trees(&example, 1, 1), 2);
        assert_eq!(trees(&example, 3, 1), 7);
        assert_eq!(trees(&example, 5, 1), 3);
        assert_eq!(trees(&example, 7, 1), 4);
        assert_eq!(trees(&example, 1, 2), 2);
    }
}
