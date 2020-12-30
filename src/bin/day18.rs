use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug)]
enum Exp {
    Add(Box<Exp>, Box<Exp>),
    Mul(Box<Exp>, Box<Exp>),
    Number(u64),
}

impl Exp {
    fn v1(input: &str) -> Self {
        let rpn = rpn(&input, 1);
        ast(&rpn)
    }

    fn v2(input: &str) -> Self {
        let rpn = rpn(&input, 2);
        ast(&rpn)
    }

    fn eval(&self) -> u64 {
        match self {
            Exp::Number(n) => *n,
            Exp::Add(a, b) => a.eval() + b.eval(),
            Exp::Mul(a, b) => a.eval() * b.eval(),
        }
    }
}

/// Convert the infix format into RPN using Dijkstra's shunting-yard algorithm
fn rpn(input: &str, part: u8) -> Vec<char> {
    let mut output: Vec<char> = vec![];
    let mut stack: Vec<char> = vec![];

    let precedence_rules = if part == 1 {
        "+*" // equal precedence, left-associative
    } else {
        "+" // '+' has highest precedence, left-associative
    };

    for token in input.chars().filter(|&c| c != ' ') {
        match token {
            '0'..='9' => output.push(token),
            '(' => stack.push(token),
            ')' => {
                while let Some(op) = stack.pop() {
                    if op == '(' {
                        break;
                    } else {
                        output.push(op);
                    }
                }
            }
            '+' | '*' => {
                if let Some(op) = stack.pop() {
                    if precedence_rules.contains(op) {
                        output.push(op);
                    } else {
                        stack.push(op); // put it back
                    }
                }
                stack.push(token);
            }
            _ => panic!("Unexpected token {}", token),
        }
    }

    while let Some(op) = stack.pop() {
        output.push(op);
    }

    output
}

fn ast(tokens: &[char]) -> Exp {
    let mut stack: Vec<Exp> = vec![];

    for token in tokens {
        match token {
            '0'..='9' => {
                let n = token.to_digit(10).expect("Invalid number");
                stack.push(Exp::Number(n as u64))
            }
            '+' => {
                let o1 = stack.pop().unwrap();
                let o2 = stack.pop().unwrap();
                stack.push(Exp::Add(Box::new(o1), Box::new(o2)));
            }
            '*' => {
                let o1 = stack.pop().unwrap();
                let o2 = stack.pop().unwrap();
                stack.push(Exp::Mul(Box::new(o1), Box::new(o2)));
            }
            _ => panic!("Unexpected token {}", token),
        }
    }

    stack.pop().unwrap()
}

fn solve_part1(input: &str) -> u64 {
    input.lines().map(|line| Exp::v1(line).eval()).sum()
}

fn solve_part2(input: &str) -> u64 {
    input.lines().map(|line| Exp::v2(line).eval()).sum()
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let file = read_to_string("inputs/day18.txt")?;

    println!("Part 1 {:?}", solve_part1(&file));
    println!("Part 2 {:?}", solve_part2(&file));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let exp = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(Exp::v1(&exp).eval(), 71);
        assert_eq!(Exp::v2(&exp).eval(), 231);
    }

    #[test]
    fn test_example2() {
        let exp = "1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(Exp::v1(&exp).eval(), 51);
        assert_eq!(Exp::v2(&exp).eval(), 51);
    }

    #[test]
    fn test_example3() {
        let exp = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(Exp::v1(&exp).eval(), 13632);
        assert_eq!(Exp::v2(&exp).eval(), 23340);
    }
}
