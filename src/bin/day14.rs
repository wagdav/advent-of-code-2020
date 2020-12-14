use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::default::Default;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Clone, Debug, PartialEq)]
enum Op {
    Mask(Mask),
    Memset(u64, u64),
}

#[derive(Default, Copy, Clone, Debug, PartialEq)]
struct Mask {
    enabled: u64,
    mask: u64,
}

impl Mask {
    fn new(op_arg: &str) -> Self {
        let mask_value = op_arg
            .chars()
            .enumerate()
            .map(|(i, v)| match v {
                '1' => 2_u64.pow(35 - (i as u32)),
                '0' => 0,
                'X' => 0,
                _ => 0,
            })
            .sum::<u64>();

        let mask_range = op_arg
            .chars()
            .enumerate()
            .map(|(i, v)| {
                if v != 'X' {
                    2_u64.pow(35 - (i as u32))
                } else {
                    0
                }
            })
            .sum::<u64>();

        Self {
            enabled: mask_range,
            mask: mask_value,
        }
    }

    fn apply(&self, input: u64) -> u64 {
        let mut res = input;

        for i in 0..64 {
            let flag = 1 << i;
            if (flag & self.enabled) > 0 {
                if ((res & flag) > 0) && (self.mask & flag == 0) {
                    res &= !flag; // force to 0
                } else {
                    res |= self.mask & flag; // force to 1
                }
            }
        }

        res
    }
}

fn parse(input: &str) -> Vec<Op> {
    let re_mem_cmd = Regex::new(r"^mem\[(?P<index>[[:digit:]]+)\]$").unwrap();

    let mut res = vec![];
    for line in input.lines() {
        let mut parts = line.trim().split(" = ");

        let op_name = parts.next().unwrap();
        let op_arg = parts.next().unwrap();

        if op_name == "mask" {
            res.push(Op::Mask(Mask::new(&op_arg)));
        } else {
            let captures = re_mem_cmd.captures(op_name).unwrap();
            let index = captures.name("index").unwrap().as_str().parse().unwrap();
            let val = op_arg.parse().unwrap();
            res.push(Op::Memset(index, val));
        }
    }

    res
}

fn solve_part1(input: &[Op]) -> u64 {
    let mut mask: Mask = Default::default();
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for op in input {
        match op {
            Op::Memset(index, value) => {
                memory.insert(*index, mask.apply(*value));
            }
            Op::Mask(new_mask) => mask = *new_mask,
        }
    }

    memory.iter().map(|(_, v)| *v).sum()
}

fn decode(address: u64, mask: &Mask) -> HashSet<u64> {
    let mut address = address;
    let mut res: HashSet<u64> = HashSet::new();

    // If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
    for i in 0..64 {
        address |= mask.mask & (1 << i);
    }

    let it = (0..36)
        .map(|i| 1 << i)
        .filter(|flag| mask.enabled & flag == 0)
        .map(|flag| vec![(false, flag), (true, flag)])
        .multi_cartesian_product();

    for variant in it {
        let mut x = address;
        for (b, flag) in variant {
            match b {
                true => x |= flag,   // force to 1
                false => x &= !flag, // force to 0
            }
            res.insert(x);
        }
    }

    res
}

fn solve_part2(input: &[Op]) -> u64 {
    let mut mask: Mask = Default::default();
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for op in input {
        match op {
            Op::Memset(address, value) => {
                for a in decode(*address, &mask) {
                    memory.insert(a, *value);
                }
            }
            Op::Mask(new_mask) => mask = *new_mask,
        }
    }

    memory.iter().map(|(_, v)| *v).sum()
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let file = read_to_string("inputs/day14.txt")?;

    let input = parse(&file);

    println!("Part 1 {:?}", solve_part1(&input));
    println!("Part 2 {:?}", solve_part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "\
        mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        mem[8] = 11
        mem[7] = 101
        mem[8] = 0\
    ";

    #[test]
    fn parses_puzzle() {
        let input = parse(&EXAMPLE);
        assert_eq!(
            input,
            vec![
                Op::Mask(Mask::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")),
                Op::Memset(8, 11),
                Op::Memset(7, 101),
                Op::Memset(8, 0)
            ]
        );
    }

    #[test]
    fn test_apply() {
        let mask = Mask::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(mask.apply(11), 73);
        assert_eq!(mask.apply(101), 101);
        assert_eq!(mask.apply(0), 64);
    }

    #[test]
    fn solves_part1() {
        let input = parse(&EXAMPLE);
        assert_eq!(solve_part1(&input), 165);
    }

    #[test]
    fn test_decoder() {
        let mask = Mask::new("000000000000000000000000000000X1001X");
        assert_eq!(
            decode(42, &mask),
            *&[26, 27, 58, 59].iter().cloned().collect::<HashSet<u64>>()
        );
    }

    #[test]
    fn solves_part2() {
        let p: &str = "\
            mask = 000000000000000000000000000000X1001X
            mem[42] = 100
            mask = 00000000000000000000000000000000X0XX
            mem[26] = 1\
        ";

        let input = parse(&p);
        assert_eq!(solve_part2(&input), 208);
    }
}
