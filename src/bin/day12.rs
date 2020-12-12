use regex::Regex;
use std::error::Error;
use std::fs::read_to_string;

fn parse(input: &str) -> Vec<Instruction> {
    let re = Regex::new(
        r"(?x)
        ^
        (?P<cmd>[[:alpha:]])
        (?P<arg>[[:digit:]]+)
        $
        ",
    )
    .expect("Invalid regular expression");

    let mut res = vec![];
    for line in input.lines() {
        let line = line.trim();
        let captures = re.captures(line).unwrap();
        let cmd = captures.name("cmd").unwrap().as_str().parse().unwrap();
        let arg = captures.name("arg").unwrap().as_str().parse().unwrap();

        res.push((cmd, arg));
    }

    res
}

type Instruction = (char, i32);

fn solve_part1(input: &[Instruction]) -> i32 {
    #[derive(Clone, Debug)]
    struct Ship {
        lon: i32,
        lat: i32,
        heading: i32,
    }

    let start = Ship {
        lon: 0,
        lat: 0,
        heading: 90,
    };

    let res = input.iter().fold(start, |cur, &(cmd, arg)| match cmd {
        'N' => Ship {
            lat: cur.lat + arg,
            ..cur
        },
        'S' => Ship {
            lat: cur.lat - arg,
            ..cur
        },
        'E' => Ship {
            lon: cur.lon - arg,
            ..cur
        },
        'W' => Ship {
            lon: cur.lon + arg,
            ..cur
        },
        'L' => Ship {
            heading: (cur.heading - arg + 360) % 360,
            ..cur
        },
        'R' => Ship {
            heading: (cur.heading + arg + 360) % 360,
            ..cur
        },
        'F' => match cur.heading {
            0 => Ship {
                lat: cur.lat + arg,
                ..cur
            },
            90 => Ship {
                lon: cur.lon + arg,
                ..cur
            },
            180 => Ship {
                lat: cur.lat - arg,
                ..cur
            },
            270 => Ship {
                lon: cur.lon - arg,
                ..cur
            },
            _ => panic!("Invalid heading {:?}", cur.heading),
        },
        _ => panic!("Invalid instruction"),
    });

    res.lon.abs() + res.lat.abs()
}

/// Rotate the vector with the given angle.  Positive angle means counter-clockwise.
fn rot((x, y): (i32, i32), angle: i32) -> (i32, i32) {
    let x = x as f32;
    let y = y as f32;
    let angle = (angle as f32).to_radians();

    let xp = angle.cos() * x - angle.sin() * y;
    let yp = angle.sin() * x + angle.cos() * y;

    (xp.round() as i32, yp.round() as i32)
}

fn solve_part2(input: &[Instruction]) -> i32 {
    struct Ship {
        lon: i32,
        lat: i32,
        dlon: i32,
        dlat: i32,
    }

    let start = Ship {
        lon: 0,
        lat: 0,
        dlon: 10,
        dlat: 1,
    };

    let res = input.iter().fold(start, |cur, &(cmd, arg)| match cmd {
        'N' => Ship {
            dlat: cur.dlat + arg,
            ..cur
        },
        'S' => Ship {
            dlat: cur.dlat - arg,
            ..cur
        },
        'E' => Ship {
            dlon: cur.dlon + arg,
            ..cur
        },
        'W' => Ship {
            dlon: cur.dlon - arg,
            ..cur
        },

        'L' => {
            let (x, y) = rot((cur.dlon, cur.dlat), arg);
            Ship {
                dlon: x,
                dlat: y,
                ..cur
            }
        }
        'R' => {
            let (x, y) = rot((cur.dlon, cur.dlat), -arg);
            Ship {
                dlon: x,
                dlat: y,
                ..cur
            }
        }
        'F' => Ship {
            lon: cur.lon + cur.dlon * arg,
            lat: cur.lat + cur.dlat * arg,
            ..cur
        },
        _ => panic!("Invalid instruction"),
    });

    res.lon.abs() + res.lat.abs()
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let file = read_to_string("inputs/day12.txt")?;

    let input = parse(&file);

    println!("Part 1 {:?}", solve_part1(&input));
    println!("Part 2 {:?}", solve_part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "\
        F10
        N3
        F7
        R90
        F11\
    ";

    #[test]
    fn parses_puzzle() {
        let input = parse(&EXAMPLE);
        assert_eq!(input.len(), 5);
        assert_eq!(input[3], ('R', 90));
    }

    #[test]
    fn solves_part1() {
        let input = parse(&EXAMPLE);
        assert_eq!(solve_part1(&input), 25);
    }

    #[test]
    fn solves_part2() {
        let input = parse(&EXAMPLE);
        assert_eq!(solve_part2(&input), 286);
    }
}
