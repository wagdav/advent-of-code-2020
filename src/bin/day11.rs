use std::error::Error;
use std::fmt;
use std::fs::read_to_string;

#[derive(Clone, Debug, PartialEq)]
struct SeatPlan {
    seats: Vec<char>,
    rows: i32,
    cols: i32,
}

const DIRECTIONS: &[(i32, i32)] = &[
    (0, -1),
    (0, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (1, -1),
    (-1, 1),
    (1, 1),
];

impl SeatPlan {
    fn new(input: &str) -> Self {
        let mut seats = vec![];
        let mut cols = 0;
        let mut rows = 0;

        for line in input.lines() {
            let line = line.trim();
            seats.extend(line.chars());
            rows += 1;
            cols = line.len();
        }

        let cols = cols as i32;
        let rows = rows as i32;

        Self { seats, rows, cols }
    }

    fn at(&self, col: i32, row: i32) -> Option<char> {
        if 0 <= col && col < self.cols && 0 <= row && row < self.rows {
            let col = col as usize;
            let row = row as usize;
            let cols = self.cols as usize;
            Some(self.seats[col + cols * row])
        } else {
            None
        }
    }

    fn neighbours(&self, i: usize) -> usize {
        let mut res = vec![];
        let (col, row) = self.to_col_row(&i);

        for (dc, dr) in DIRECTIONS {
            if let Some(c) = self.at(col + dc, row + dr) {
                res.push(c)
            };
        }

        res.into_iter().filter(|c| *c == '#').count()
    }

    fn first_seat(&self, col: &i32, row: &i32, dc: &i32, dr: &i32) -> Option<char> {
        let mut r = *row;
        let mut c = *col;
        loop {
            r += dr;
            c += dc;

            match self.at(c, r) {
                Some(x) => {
                    if x != '.' {
                        return Some(x);
                    } else {
                        continue;
                    }
                }
                None => return None,
            }
        }
    }

    fn visible(&self, i: usize) -> usize {
        let mut res = vec![];
        let (col, row) = self.to_col_row(&i);

        for (dc, dr) in DIRECTIONS {
            if let Some(c) = self.first_seat(&col, &row, dc, dr) {
                res.push(c)
            };
        }

        res.into_iter().filter(|c| *c == '#').count()
    }

    fn occupied(&self) -> i32 {
        self.seats.iter().filter(|&c| *c == '#').count() as i32
    }

    fn to_col_row(&self, i: &usize) -> (i32, i32) {
        let cols = self.cols as usize;

        let col = i % cols;
        let row = (i - col) / cols;

        (col as i32, row as i32)
    }

    fn new_seat(&self, n: usize, seat: char, limit: usize) -> char {
        if n == 0 && seat == 'L' {
            return '#';
        }

        if n >= limit && seat == '#' {
            return 'L';
        }

        seat
    }

    fn step1(&mut self) {
        self.seats = self
            .seats
            .iter()
            .enumerate()
            .map(|(i, seat)| (self.neighbours(i), seat))
            .map(|(n, seat)| self.new_seat(n, *seat, 4))
            .collect();
    }

    fn step2(&mut self) {
        self.seats = self
            .seats
            .iter()
            .enumerate()
            .map(|(i, seat)| (self.visible(i), seat))
            .map(|(n_visible, seat)| self.new_seat(n_visible, *seat, 5))
            .collect();
    }
}

impl fmt::Display for SeatPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{}", self.at(j, i).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn solve_part1(input: &SeatPlan) -> i32 {
    let mut input = input.to_owned();
    let mut n = 0;
    loop {
        input.step1();
        if input.occupied() == n {
            break;
        } else {
            n = input.occupied()
        }
    }

    n
}

fn solve_part2(input: &SeatPlan) -> i32 {
    let mut input = input.to_owned();
    let mut n = 0;
    loop {
        input.step2();
        if input.occupied() == n {
            break;
        } else {
            n = input.occupied()
        }
    }

    n
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let file = read_to_string("inputs/day11.txt")?;

    let plan = SeatPlan::new(&file);

    println!("{}", &plan);

    println!("Part 1 {:?}", solve_part1(&plan));
    println!("Part 2 {:?}", solve_part2(&plan));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "\
        L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";

    #[test]
    fn parses_puzzle() {
        let plan = SeatPlan::new(&EXAMPLE);
        assert_eq!(plan.rows, 10);
        assert_eq!(plan.cols, 10);
        assert_eq!(plan.occupied(), 0);
    }

    #[test]
    fn solves_part1() {
        let plan = SeatPlan::new(&EXAMPLE);
        assert_eq!(plan.at(9, 8), Some('L'));
        assert_eq!(solve_part1(&plan), 37);
    }

    #[test]
    fn solves_part2() {
        let plan = SeatPlan::new(&EXAMPLE);
        assert_eq!(solve_part2(&plan), 26);
    }
}
