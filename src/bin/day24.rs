use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::read_to_string;

type Coords = (i32, i32);

fn parse_input(input: &str) -> Vec<Coords> {
    input.lines().filter_map(|line| parse(&line)).collect()
}

fn parse(input: &str) -> Option<Coords> {
    let input = input.trim();
    let re = Regex::new("(?P<direction>se|e|w|nw|ne|sw)+?").expect("Invalid regular expression");

    let mut x = 0;
    let mut y = 0;
    for g in re.captures_iter(&input) {
        match &g["direction"] {
            "e" => x += 1,
            "se" => {
                x += 1;
                y -= 1;
            }
            "sw" => y -= 1,
            "w" => x -= 1,
            "nw" => {
                x -= 1;
                y += 1;
            }
            "ne" => {
                y += 1;
            }
            _ => panic!("Unexpected direction"),
        }
    }
    Some((x, y))
}

fn black_tiles(tiles: &[Coords]) -> HashSet<Coords> {
    let mut count: HashMap<Coords, usize> = HashMap::new();
    for tile in tiles {
        *count.entry(*tile).or_insert(0) += 1;
    }

    count
        .iter()
        .filter_map(|(coords, state)| if *state % 2 == 1 { Some(coords) } else { None })
        .cloned()
        .collect()
}

fn near(cell: &Coords) -> impl Iterator<Item = Coords> + '_ {
    [cell.0, cell.1]
        .iter()
        .map(|i| (i - 1)..=(i + 1)) // any differs += 1
        .multi_cartesian_product()
        .map(|x| (x[0], x[1]))
        // Filter combinations that are invalid on a hexagon
        .filter(move |x| *x != (cell.0 + 1, cell.1 + 1))
        .filter(move |x| *x != (cell.0 - 1, cell.1 - 1))
}

fn active_neighbours(cell: &Coords, grid: &HashSet<Coords>) -> usize {
    near(&cell)
        .filter(move |x| x != cell) // exclude the cell itself
        .filter(|x| grid.contains(x))
        .count()
}

fn solve_part1(tiles: &[Coords]) -> usize {
    black_tiles(&tiles).iter().count()
}

fn solve_part2(tiles: &[Coords], steps: usize) -> usize {
    let mut grid = black_tiles(&tiles);

    for step in 0..steps {
        grid = grid
            .iter()
            .flat_map(|c| {
                near(&c).filter(|cell| {
                    let num_active = active_neighbours(&cell, &grid);
                    if grid.contains(&cell) {
                        // active (black) cell
                        !(num_active == 0 || num_active > 2)
                    } else {
                        // inactive (white) cell
                        num_active == 2
                    }
                })
            })
            .collect();
        println!("Day {:>3}: {}", step + 1, grid.len());
    }

    grid.len()
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let file = read_to_string("inputs/day24.txt")?;

    let tiles = parse_input(&file);

    println!("Part 1 {:?}", solve_part1(&tiles));
    println!("Part 2 {:?}", solve_part2(&tiles, 100));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        sesenwnenenewseeswwswswwnenewsewsw
        neeenesenwnwwswnenewnwwsewnenwseswesw
        seswneswswsenwwnwse
        nwnwneseeswswnenewneswwnewseswneseene
        swweswneswnenwsewnwneneseenw
        eesenwseswswnenwswnwnwsewwnwsene
        sewnenenenesenwsewnenwwwse
        wenwwweseeeweswwwnwwe
        wsweesenenewnwwnwsenewsenwwsesesenwne
        neeswseenwwswnwswswnw
        nenwswwsewswnenenewsenwsenwnesesenew
        enewnwewneswsewnwswenweswnenwsenwsw
        sweneswneswneneenwnewenewwneswswnese
        swwesenesewenwneswnwwneseswwne
        enesenwswwswneneswsenwnewswseenwsese
        wnwnesenesenenwwnenwsewesewsesesew
        nenewswnwewswnenesenwnesewesw
        eneswnwswnwsenenwnwnwwseeswneewsenese
        neswnwewnwnwseenwseesewsenwsweewe
        wseweeenwnesenwwwswnew\
    ";

    #[test]
    fn parses_puzzle() {
        assert_eq!(parse("esew"), Some((1, -1))); // se neigbour of the reference tile
        assert_eq!(parse("nwwswee"), Some((0, 0)));
    }

    #[test]
    fn solves_part1() {
        let tiles = parse_input(&EXAMPLE);
        assert_eq!(solve_part1(&tiles), 10);
    }

    #[test]
    fn solves_part2() {
        let tiles = parse_input(&EXAMPLE);
        assert_eq!(solve_part2(&tiles, 1), 15);
        assert_eq!(solve_part2(&tiles, 10), 37);
    }

    #[test]
    fn count_neigbours() {
        assert_eq!(
            near(&(0, 0)).collect::<Vec<Coords>>(),
            vec![
                (-1, 0), // w
                (-1, 1), // nw
                (0, -1), // sw
                (0, 0),  // cell itself
                (0, 1),  // ne
                (1, -1), // se
                (1, 0)   // e
            ]
        );
    }
}
