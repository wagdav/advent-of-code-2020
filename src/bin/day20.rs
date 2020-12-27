use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Clone, Debug, PartialEq)]
struct Tile {
    id: usize,
    size: usize,
    pixels: Vec<Vec<char>>,
}

impl Tile {
    fn from_input(input: &str) -> Option<Self> {
        let id = input
            .lines()
            .next()?
            .trim()
            .strip_prefix("Tile ")?
            .strip_suffix(":")?
            .parse()
            .ok()?;

        let mut size = 0;
        let mut pixels = vec![];

        for row in input.lines().skip(1) {
            let row = row.trim();
            size = row.len();
            pixels.push(row.chars().collect());
        }

        Some(Self { id, size, pixels })
    }

    fn borders(&self) -> Vec<String> {
        let mut top: Vec<char> = vec![];
        let mut bottom = vec![];
        let mut left = vec![];
        let mut right = vec![];

        let last = self.size - 1;

        for i in 0..self.size {
            top.push(self.pixels[0][i]);
            bottom.push(self.pixels[last][i]);
            left.push(self.pixels[i][0]);
            right.push(self.pixels[i][last]);
        }

        [top, bottom, left, right]
            .iter()
            .map(|b| b.iter().collect())
            .collect()
    }

    fn top(&self) -> String {
        self.borders()[0].clone()
    }

    fn bottom(&self) -> String {
        self.borders()[1].clone()
    }

    fn left(&self) -> String {
        self.borders()[2].clone()
    }

    fn right(&self) -> String {
        self.borders()[3].clone()
    }

    /// Rotate the tile 90 degrees counter-clockwise
    #[allow(clippy::needless_range_loop)]
    fn rot90(&self) -> Self {
        let mut new_pixels = self.pixels.to_owned();
        for i in 0..self.size {
            for j in 0..self.size {
                new_pixels[i][j] = self.pixels[j][self.size - 1 - i];
            }
        }

        Self {
            pixels: new_pixels,
            ..*self
        }
    }

    fn flip(&self) -> Self {
        Self {
            pixels: self
                .pixels
                .iter()
                .map(|row| row.iter().rev().cloned().collect())
                .collect(),
            ..*self
        }
    }

    fn possible_borders(&self) -> Vec<String> {
        let mut res = vec![];
        res.extend_from_slice(&self.borders());
        res.extend_from_slice(
            &self
                .borders()
                .iter()
                .map(|x| x.chars().rev().collect())
                .collect::<Vec<String>>(),
        );

        res
    }
}

fn parse_input(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .filter_map(|t| Tile::from_input(t))
        .collect()
}

fn neigbours(tiles: &[Tile]) -> HashMap<usize, Vec<usize>> {
    let mut neigbours = HashMap::new();
    for i in 0..tiles.len() {
        for j in 0..i {
            if tiles[i]
                .possible_borders()
                .iter()
                .any(|border| tiles[j].borders().contains(border))
            {
                let i_id = tiles[i].id;
                let j_id = tiles[j].id;
                neigbours.entry(i_id).or_insert_with(Vec::new).push(j_id);
                neigbours.entry(j_id).or_insert_with(Vec::new).push(i_id);
            }
        }
    }
    neigbours
}

fn find_corners(tiles: &[Tile]) -> Vec<usize> {
    neigbours(&tiles)
        .iter()
        .filter_map(|(k, v)| if v.len() == 2 { Some(*k) } else { None })
        .collect()
}

fn reconstruct_image(tiles: &[Tile]) -> Tile {
    let n = neigbours(&tiles);
    let corners = find_corners(&tiles);

    let tiles: HashMap<usize, Tile> = tiles.iter().map(|t| (t.id, t.clone())).collect();
    let adjacent: HashMap<usize, Vec<Tile>> = n
        .iter()
        .map(|(k, v)| (*k, v.iter().map(|id| tiles[id].clone()).collect()))
        .collect();

    let mut reconstruct: Vec<Tile> = vec![];

    // fix this as top left corner
    let mut current = tiles[&corners[0]].clone();
    current = make_top_left(&current, &adjacent[&current.id]).unwrap();
    let mut top_left = current.clone();
    let mut n_rows = 0;

    loop {
        while let Some(x) = right_neighbour(&current, &adjacent[&current.id]) {
            reconstruct.push(current.clone());
            current = x.clone();
        }
        reconstruct.push(current.clone());

        if n_rows == 0 {
            n_rows = reconstruct.len();
        }

        if let Some(next) = bottom_neighbour(&top_left, &adjacent[&top_left.id]) {
            top_left = next;
            current = top_left.clone();
        } else {
            break;
        }
    }

    let mut image = vec![];
    let tile_size = reconstruct[0].size;

    // assemble the image from the tiles without borders
    for k in 0..n_rows {
        for j in 1..(tile_size - 1) {
            let mut image_row = vec![];
            for i in 0..n_rows {
                image_row
                    .extend_from_slice(&reconstruct[i + k * n_rows].pixels[j][1..tile_size - 1]);
            }
            image.push(image_row);
        }
    }

    Tile {
        id: 0,
        size: n_rows * (tile_size - 2),
        pixels: image,
    }
}

fn solve_part1(tiles: &[Tile]) -> usize {
    find_corners(&tiles).iter().product()
}

fn solve_part2(tiles: &[Tile]) -> usize {
    let image = reconstruct_image(&tiles);

    let mut monster_count = 0;
    for v in variants(&image) {
        for i in 0..image.size - 3 {
            for j in 0..image.size - 20 {
                if match_monster(&v, i, j) {
                    monster_count += 1;
                }
            }
        }

        if monster_count > 0 {
            break;
        }
    }

    image
        .pixels
        .iter()
        .map(|row| row.iter().filter(|c| **c == '#').count())
        .sum::<usize>()
        - 15 * monster_count
}

#[allow(clippy::needless_range_loop)]
fn match_monster(image: &Tile, i: usize, j: usize) -> bool {
    let monster: Vec<Vec<char>> = vec![
        "                  # ".chars().collect(),
        "#    ##    ##    ###".chars().collect(),
        " #  #  #  #  #  #   ".chars().collect(),
    ];

    for mi in 0..3 {
        for mj in 0..20 {
            if monster[mi][mj] != '#' {
                continue;
            }

            if image.pixels[i + mi][j + mj] != '#' {
                return false;
            }
        }
    }

    true
}

fn variants(tile: &Tile) -> Vec<Tile> {
    vec![
        tile.clone(),
        tile.rot90(),
        tile.rot90().rot90(),
        tile.rot90().rot90().rot90(),
        tile.flip(),
        tile.rot90().flip(),
        tile.rot90().rot90().flip(),
        tile.rot90().rot90().rot90().flip(),
    ]
}

fn right_neighbour(current: &Tile, candidates: &[Tile]) -> Option<Tile> {
    for c in candidates {
        for v in variants(&c) {
            if current.right() == v.left() {
                return Some(v);
            }
        }
    }

    None
}

fn bottom_neighbour(current: &Tile, candidates: &[Tile]) -> Option<Tile> {
    for c in candidates {
        for v in variants(&c) {
            if current.bottom() == v.top() {
                return Some(v);
            }
        }
    }

    None
}

fn make_top_left(current: &Tile, candidates: &[Tile]) -> Option<Tile> {
    variants(&current).into_iter().find(|v| {
        candidates[0].possible_borders().contains(&v.right())
            && candidates[1].possible_borders().contains(&v.bottom())
    })
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let file = read_to_string("inputs/day20.txt")?;

    let tiles = parse_input(&file);

    println!("Part 1 {:?}", solve_part1(&tiles));
    println!("Part 2 {:?}", solve_part2(&tiles));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_tile() {
        assert_eq!(
            Tile::from_input(&TILE).unwrap(),
            Tile {
                size: 10,
                id: 2311,
                pixels: vec![
                    vec!['.', '.', '#', '#', '.', '#', '.', '.', '#', '.'],
                    vec!['#', '#', '.', '.', '#', '.', '.', '.', '.', '.'],
                    vec!['#', '.', '.', '.', '#', '#', '.', '.', '#', '.'],
                    vec!['#', '#', '#', '#', '.', '#', '.', '.', '.', '#'],
                    vec!['#', '#', '.', '#', '#', '.', '#', '#', '#', '.'],
                    vec!['#', '#', '.', '.', '.', '#', '.', '#', '#', '#'],
                    vec!['.', '#', '.', '#', '.', '#', '.', '.', '#', '#'],
                    vec!['.', '.', '#', '.', '.', '.', '.', '#', '.', '.'],
                    vec!['#', '#', '#', '.', '.', '.', '#', '.', '#', '.'],
                    vec!['.', '.', '#', '#', '#', '.', '.', '#', '#', '#'],
                ]
            }
        );
    }

    #[test]
    fn borders() {
        assert_eq!(
            Tile::from_input(&TILE).unwrap().borders(),
            vec![
                "..##.#..#.", // top
                "..###..###", // bot
                ".#####..#.", // left
                "...#.##..#", // right
            ]
        );
    }

    #[test]
    fn rotate() {
        let tile = Tile::from_input(&TILE).unwrap();
        assert_eq!(
            tile.rot90().borders(),
            vec![
                "...#.##..#", // top (was right)
                ".#####..#.", // bot (was left)
                ".#..#.##..", // left (was top, reversed)
                "###..###..", // right, (was bot, reversed)
            ]
        );

        assert_eq!(tile, tile.rot90().rot90().rot90().rot90());
    }

    #[test]
    fn flip_horizontal() {
        let tile = Tile::from_input(&TILE).unwrap();

        assert_eq!(
            tile.flip().borders(),
            vec![
                ".#..#.##..", // top (reversed)
                "###..###..", // bot (reversed)
                "...#.##..#", // left (was right)
                ".#####..#.", // right (was left)
            ]
        );
    }

    #[test]
    fn solves_part1() {
        let tiles = parse_input(&EXAMPLE);
        assert_eq!(solve_part1(&tiles), 1951 * 3079 * 2971 * 1171);
    }

    #[test]
    fn solves_part2() {
        let tiles = parse_input(&EXAMPLE);
        assert_eq!(solve_part2(&tiles), 273);
    }

    const TILE: &str = "\
        Tile 2311:
        ..##.#..#.
        ##..#.....
        #...##..#.
        ####.#...#
        ##.##.###.
        ##...#.###
        .#.#.#..##
        ..#....#..
        ###...#.#.
        ..###..###\
    ";

    const EXAMPLE: &str = "\
        Tile 2311:
        ..##.#..#.
        ##..#.....
        #...##..#.
        ####.#...#
        ##.##.###.
        ##...#.###
        .#.#.#..##
        ..#....#..
        ###...#.#.
        ..###..###

        Tile 1951:
        #.##...##.
        #.####...#
        .....#..##
        #...######
        .##.#....#
        .###.#####
        ###.##.##.
        .###....#.
        ..#.#..#.#
        #...##.#..

        Tile 1171:
        ####...##.
        #..##.#..#
        ##.#..#.#.
        .###.####.
        ..###.####
        .##....##.
        .#...####.
        #.##.####.
        ####..#...
        .....##...

        Tile 1427:
        ###.##.#..
        .#..#.##..
        .#.##.#..#
        #.#.#.##.#
        ....#...##
        ...##..##.
        ...#.#####
        .#.####.#.
        ..#..###.#
        ..##.#..#.

        Tile 1489:
        ##.#.#....
        ..##...#..
        .##..##...
        ..#...#...
        #####...#.
        #..#.#.#.#
        ...#.#.#..
        ##.#...##.
        ..##.##.##
        ###.##.#..

        Tile 2473:
        #....####.
        #..#.##...
        #.##..#...
        ######.#.#
        .#...#.#.#
        .#########
        .###.#..#.
        ########.#
        ##...##.#.
        ..###.#.#.

        Tile 2971:
        ..#.#....#
        #...###...
        #.#.###...
        ##.##..#..
        .#####..##
        .#..####.#
        #..#.#..#.
        ..####.###
        ..#.#.###.
        ...#.#.#.#

        Tile 2729:
        ...#.#.#.#
        ####.#....
        ..#.#.....
        ....#..#.#
        .##..##.#.
        .#.####...
        ####.#.#..
        ##.####...
        ##..#.##..
        #.##...##.

        Tile 3079:
        #.#.#####.
        .#..######
        ..#.......
        ######....
        ####.#..#.
        .#...#.##.
        #.#####.##
        ..#.###...
        ..#.......
        ..#.###...\
    ";
}
