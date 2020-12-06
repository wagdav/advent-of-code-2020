use std::error::Error;
use std::fs::read_to_string;

fn decode(text: &str, base: i32, _lower: &char, upper: &char) -> i32 {
    let mut base = base;
    let mut offset = 0;

    for c in text.chars() {
        base /= 2;
        if c == *upper {
            offset += base
        }
    }

    offset
}

fn decode_row(text: &str) -> i32 {
    decode(&text, 128, &'F', &'B')
}

fn decode_column(text: &str) -> i32 {
    decode(&text, 8, &'L', &'R')
}

fn seat_ids(lines: &[&str]) -> Vec<i32> {
    lines
        .iter()
        .map(|line| (&line[0..7], &line[7..10]))
        .map(|(row, col)| (decode_row(&row), decode_column(&col)))
        .map(|(row, col)| 8 * row + col)
        .collect()
}

fn solve_part1(lines: &[&str]) -> i32 {
    *seat_ids(lines).iter().max().unwrap()
}

fn solve_part2(lines: &[&str]) -> i32 {
    let mut r = seat_ids(lines);

    r.sort();

    let mut x = r
        .iter()
        .zip(r.iter().skip(1))
        .map(|(x, y)| (x, y - x))
        .filter(|(_, diff)| *diff == 2);

    let (seat_id, _) = x.next().unwrap();

    seat_id + 1
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = read_to_string("inputs/day05.txt")?;
    let lines : Vec<&str> = file.lines().collect();

    println!("Part 1 {:?}", solve_part1(lines.as_slice()));
    println!("Part 2 {:?}", solve_part2(lines.as_slice()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_rows() {
        assert_eq!(decode_row("BFFFBBF"), 70);
        assert_eq!(decode_row("FFFBBBF"), 14);
        assert_eq!(decode_row("BBFFBBF"), 102);
    }
}
