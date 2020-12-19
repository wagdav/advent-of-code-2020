use std::error::Error;
use std::fs::read_to_string;

#[derive(Clone, Debug, PartialEq)]
enum Exp {
    Add(Box<Exp>, Box<Exp>),
    Mul(Box<Exp>, Box<Exp>),
    Number(u64),
}

struct Parser {
    stack: Vec<Exp>,
    current_op: Vec<Option<char>>,
}

impl Parser {
    fn new() -> Self {
        Self {
            stack: vec![],
            current_op: vec![],
        }
    }

    fn parse(&mut self, input: &str) -> Exp {
        for token in input.chars().filter(|c| *c != ' ') {
            match token {
                '0'..='9' => {
                    let n = token.to_digit(10).expect("Invalid number");
                    self.stack.push(Exp::Number(n as u64))
                }
                '(' => {
                    self.current_op.push(None);
                }
                ')' => {
                    self.emit();
                }
                _ => {
                    self.emit();
                    self.current_op.push(Some(token));
                }
            }
        }

        self.emit();

        self.stack.pop().unwrap()
    }

    fn emit(&mut self) -> Option<()> {
        let op = self.current_op.pop()??;
        let o1 = self.stack.pop()?;
        let o2 = self.stack.pop()?;
        match op {
            '+' => self.stack.push(Exp::Add(Box::new(o1), Box::new(o2))),
            '*' => self.stack.push(Exp::Mul(Box::new(o1), Box::new(o2))),
            _ => unimplemented!("Unexpected operation {}", op),
        }
        Some(())
    }
}

impl Exp {
    fn new(input: &str) -> Self {
        let mut p = Parser::new();
        p.parse(&input)
    }

    fn eval(&self) -> u64 {
        match self {
            Exp::Number(n) => *n,
            Exp::Add(a, b) => a.eval() + b.eval(),
            Exp::Mul(a, b) => a.eval() * b.eval(),
        }
    }
}

fn solve_part1(input: &str) -> u64 {
    input.lines().map(|l| Exp::new(l).eval()).sum()
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let file = read_to_string("inputs/day18.txt")?;

    println!("Part 1 {:?}", solve_part1(&file));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let ast = Exp::new("1 + 2 * 3 + 4 * 5 + 6");
        assert_eq!(ast.eval(), 71);
    }

    #[test]
    fn test_example2() {
        let ast = Exp::new("1 + (2 * 3) + (4 * (5 + 6))");
        assert_eq!(ast.eval(), 51);
    }

    #[test]
    fn test_example3() {
        let ast = Exp::new("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        assert_eq!(ast.eval(), 13632);
    }
}
