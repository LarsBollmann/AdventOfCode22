mod input;

use std::{collections::VecDeque, fmt::Display};

#[derive(Debug, Clone, Copy)]
enum Operand {
    Old,
    Number(usize),
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(Operand),
    Multiply(Operand),
}

#[derive(Debug)]
enum Test {
    DivisibleBy(usize),
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test: Test,
    true_monkey: usize,
    false_monkey: usize,
    inspections: usize,
    divide_by_three: bool,
}

impl Monkey {
    fn process_item(&mut self) -> (usize, usize) {
        let mut worry_level = self.items.pop_front().unwrap();

        self.inspections += 1;
        worry_level = match self.operation {
            Operation::Add(o) => match o {
                Operand::Old => worry_level + worry_level,
                Operand::Number(n) => worry_level + n,
            },
            Operation::Multiply(o) => match o {
                Operand::Old => worry_level * worry_level,
                Operand::Number(n) => worry_level * n,
            },
        };

        if self.divide_by_three {
            worry_level /= 3;
        }

        let test_success = match self.test {
            Test::DivisibleBy(n) => worry_level % n == 0,
        };

        if test_success {
            (self.true_monkey, worry_level)
        } else {
            (self.false_monkey, worry_level)
        }
    }
}

struct Monkeys {
    monkeys: Vec<Monkey>,
    round: usize,
    regularization: Option<usize>,
}

impl Monkeys {
    fn new() -> Monkeys {
        Monkeys {
            monkeys: Vec::new(),
            round: 0,
            regularization: None,
        }
    }

    fn process_round(&mut self) {
        for i in 0..self.monkeys.len() {
            while !self.monkeys[i].items.is_empty() {
                let (next_monkey, mut worry_level) = self.monkeys[i].process_item();
                if self.regularization.is_some() {
                    worry_level %= self.regularization.unwrap();
                }
                self.monkeys[next_monkey].items.push_back(worry_level);
            }
        }
        self.round += 1;
    }
}

impl Display for Monkeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "After round {}, the monkeys are holding items with these worry levels:",
            self.round
        )?;
        for (i, monkey) in self.monkeys.iter().enumerate() {
            write!(f, "Monkey {}: ", i)?;
            for item in &monkey.items {
                write!(f, "{}, ", item)?;
            }
            writeln!(f)?;
        }

        for (i, monkey) in self.monkeys.iter().enumerate() {
            writeln!(f, "Monkey {} has inspected {} items", i, monkey.inspections)?;
        }

        writeln!(f)?;
        Ok(())
    }
}

fn parse_input(input: &str, divide_by_three: bool) -> Monkeys {
    let mut monkeys = Monkeys::new();
    let lines = input.lines().collect::<Vec<&str>>();

    for i in 0..(lines.len() + 1) / 7 {
        let items: VecDeque<usize> = lines[i * 7 + 1].split(':').nth(1).unwrap().split(',').fold(
            VecDeque::new(),
            |mut acc, x| {
                acc.push_back(x.trim().parse().unwrap());
                acc
            },
        );
        let operation_str = lines[i * 7 + 2].split_whitespace().nth(4).unwrap();
        let operand = match lines[i * 7 + 2].split_whitespace().nth(5).unwrap() {
            "old" => Operand::Old,
            i => Operand::Number(i.parse().unwrap()),
        };

        let operation = match operation_str {
            "*" => Operation::Multiply(operand),
            "+" => Operation::Add(operand),
            _ => panic!("Unknown operation"),
        };

        let test_string = lines[i * 7 + 3].split_whitespace().nth(1).unwrap();
        let test_number: usize = lines[i * 7 + 3]
            .split_whitespace()
            .nth(3)
            .unwrap()
            .parse()
            .unwrap();

        let test = match test_string {
            "divisible" => Test::DivisibleBy(test_number),
            _ => panic!("Unknown test"),
        };

        let monkey = Monkey {
            items,
            operation,
            test,
            true_monkey: lines[i * 7 + 4]
                .split_whitespace()
                .nth(5)
                .unwrap()
                .trim()
                .parse()
                .unwrap(),
            false_monkey: lines[i * 7 + 5]
                .split_whitespace()
                .nth(5)
                .unwrap()
                .trim()
                .parse()
                .unwrap(),
            inspections: 0,
            divide_by_three,
        };

        monkeys.monkeys.push(monkey);
    }

    monkeys
}

fn process_and_get_solution(mut monkeys: Monkeys, rounds: usize) -> usize {
    for _ in 0..rounds {
        monkeys.process_round();
    }
    monkeys.monkeys.sort_by_key(|x| x.inspections);
    monkeys.monkeys.reverse();
    monkeys.monkeys[0].inspections * monkeys.monkeys[1].inspections
}

fn get_common_multiple(monkeys: &Monkeys) -> usize {
    monkeys
            .monkeys
            .iter()
            .map(|x| match x.test {
                Test::DivisibleBy(n) => n
            })
            .fold(1, |acc, x| if acc % x != 0 { acc * x } else { acc })
}

fn main() {
    let inp = input::get_input(11);
    let monkeys = parse_input(&inp, true);
    println!("{}", process_and_get_solution(monkeys, 20));
    let mut monkeys = parse_input(&inp, false);
    let common_multiple = get_common_multiple(&monkeys);
    monkeys.regularization = Some(common_multiple);
    println!("{}", process_and_get_solution(monkeys, 10000));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../test_input/day11.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(
            process_and_get_solution(parse_input(INPUT, true), 20),
            10605
        );
    }

    #[test]
    fn test_part_2() {
        let mut monkeys = parse_input(INPUT, false);
        let common_multiple = get_common_multiple(&monkeys);
        monkeys.regularization = Some(common_multiple);
        assert_eq!(process_and_get_solution(monkeys, 10000), 2713310158);
    }
}
