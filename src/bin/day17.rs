use itertools::Itertools;
use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

type Coords = Vec<i32>;

#[derive(Clone, Debug, PartialEq)]
struct Cube {
    grid: HashSet<Coords>,
}

impl Cube {
    fn new(input: &str, dim: usize) -> Self {
        let mut grid: HashSet<Coords> = HashSet::new();

        for (row, line) in input.lines().enumerate() {
            let line = line.trim();

            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    let mut elem = vec![0; dim];
                    elem[0] = col as i32;
                    elem[1] = row as i32;
                    grid.insert(elem);
                }
            }
        }

        Self { grid }
    }

    fn occupied(&self) -> usize {
        self.grid.len()
    }

    fn step1(&mut self) {
        self.grid = self
            .grid
            .iter()
            .flat_map(|c| {
                c.iter()
                    .map(|i| (i - 1)..=(i + 1))
                    .multi_cartesian_product()
                    .map(|cube| {
                        let num_active_neigbours = neighbours(&cube)
                            .iter()
                            .filter(|&x| self.grid.contains(x))
                            .count();
                        if self.grid.contains(&cube) {
                            // active cell
                            if [2, 3].contains(&num_active_neigbours) {
                                Some(cube)
                            } else {
                                None
                            }
                        } else {
                            // inactive cell
                            if num_active_neigbours == 3 {
                                Some(cube)
                            } else {
                                None
                            }
                        }
                    })
            })
            .filter_map(|x| x)
            .collect();
    }
}

/// Return the neighbours' coordinates
fn neighbours(cell: &[i32]) -> Vec<Coords> {
    cell.iter()
        .map(|i| (i - 1)..=(i + 1)) // any differs += 1
        .multi_cartesian_product()
        .filter(|x| x.iter().ne(cell.iter())) // exclude the cell itself
        .collect()
}

fn solve(input: &Cube) -> usize {
    let mut input = input.to_owned();

    for i in 0..6 {
        println!("Step {}", &i);
        input.step1();
    }

    input.occupied()
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let file = read_to_string("inputs/day17.txt")?;

    let cube3 = Cube::new(&file, 3);
    let cube4 = Cube::new(&file, 4);
    println!("Part 1 {:?}", solve(&cube3));
    println!("Part 2 {:?}", solve(&cube4));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "\
        .#.
        ..#
        ###\
        ";

    #[test]
    fn parses_puzzle() {
        let plan = Cube::new(&EXAMPLE, 3);
        assert_eq!(plan.occupied(), 5);
    }

    #[test]
    fn number_of_neigbours() {
        assert_eq!(neighbours(&vec![0, 0, 0]).len(), 26);
        assert_eq!(neighbours(&vec![0, 0, 0, 0]).len(), 80);
    }

    #[test]
    fn solves_part1() {
        let plan = Cube::new(&EXAMPLE, 3);
        assert_eq!(solve(&plan), 112);
    }

    #[test]
    #[ignore]
    fn solves_part2() {
        let plan = Cube::new(&EXAMPLE, 4);
        assert_eq!(solve(&plan), 848);
    }
}
