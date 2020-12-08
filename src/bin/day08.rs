use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = s.trim().split(' ');
        let name = parts
            .next()
            .ok_or_else(|| "no instruction name".to_string())?;
        let arg = parts
            .next()
            .ok_or_else(|| "no instruction argument".to_string())?
            .parse()
            .map_err(|_| "invalid instruction argument".to_string())?;

        match name {
            "nop" => Ok(Op::Nop(arg)),
            "acc" => Ok(Op::Acc(arg)),
            "jmp" => Ok(Op::Jmp(arg)),
            _ => Err("unexpected instruction".to_string()),
        }
    }
}

type Program = Vec<Op>;

type Result = std::result::Result<i32, (i32, Vec<usize>)>;

fn program(lines: &[&str]) -> Program {
    lines.iter().filter_map(|line| line.parse().ok()).collect()
}

fn run(p: &[Op]) -> Result {
    let mut pc = 0; // current program counter
    let mut acc = 0; // accumulator

    let mut visited = vec![];

    while pc != p.len() {
        if visited.contains(&pc) {
            return Err((acc, visited));
        }

        visited.push(pc);
        let inst = &p[pc];

        match inst {
            Op::Nop(_) => {
                pc += 1;
            }
            Op::Acc(arg) => {
                acc += arg;
                pc += 1;
            }
            Op::Jmp(pos) => {
                pc = (pc as i32 + pos) as usize;
            }
        }
    }

    Ok(acc)
}

fn replace_instruction(p: &[Op], i: usize) -> Program {
    let mut p = p.to_owned();
    match p[i] {
        Op::Jmp(x) => p[i] = Op::Nop(x),
        Op::Nop(x) => p[i] = Op::Jmp(x),
        _ => {}
    }

    p
}

fn solve_part1(p: &[Op]) -> i32 {
    let (acc, _) = run(&p).unwrap_err();

    acc
}

fn solve_part2(p: &[Op]) -> i32 {
    let (acc, visited) = run(&p).unwrap_err();

    for pc in visited {
        let new_prog = replace_instruction(p, pc);

        if let Ok(acc) = run(&new_prog) {
            return acc;
        }
    }

    acc
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let file = read_to_string("inputs/day08.txt")?;
    let lines: Vec<&str> = file.lines().collect();

    let p = program(&lines);

    println!("Part 1 {:?}", solve_part1(&p));
    println!("Part 2 {:?}", solve_part2(&p));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_instructions() {
        assert_eq!("nop +0".parse(), Ok(Op::Nop(0)));
        assert_eq!("acc +4".parse(), Ok(Op::Acc(4)));
        assert_eq!("jmp -3".parse(), Ok(Op::Jmp(-3)));
        assert_eq!(
            "some other string".parse::<Op>(),
            Err("invalid instruction argument".to_string())
        );
    }

    #[test]
    fn example1() {
        let text: Vec<_> = "
            nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6
        "
        .lines()
        .collect();

        let p = program(&text);
        assert_eq!(solve_part1(&p), 5);
        assert_eq!(solve_part2(&p), 8);
    }
}
