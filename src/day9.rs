use std::{fmt::{Display, Error, Formatter}, collections::HashMap};

mod input;

type Position = (i32, i32);

struct Rope {
    head: Position,
    tail: Position,
    visited_positions: HashMap<Position, bool>,
}

impl Rope {
    fn new() -> Self {
        Rope {
            head: (0, 0),
            tail: (0, 0),
            visited_positions: HashMap::new(),
        }
    }

    fn move_up(&mut self) {
        self.head.1 += 1;
        self.adjust_tail();
    }

    fn move_down(&mut self) {
        self.head.1 -= 1;
        self.adjust_tail();
    }

    fn move_left(&mut self) {
        self.head.0 -= 1;
        self.adjust_tail();
    }

    fn move_right(&mut self) {
        self.head.0 += 1;
        self.adjust_tail();
    }

    fn get_distance(&self) -> f64 {
        f64::sqrt(
            f64::powi(self.head.0 as f64 - self.tail.0 as f64, 2)
                + f64::powi(self.head.1 as f64 - self.tail.1 as f64, 2),
        )
    }

    fn is_tail_touching(&self) -> bool {
        self.get_distance() < 2.0
    }

    fn adjust_tail(&mut self) {
        if self.is_tail_touching() {
            return;
        }
        if self.get_distance() == 2.0 {
            self.tail.0 += (self.head.0-self.tail.0)/2;
            self.tail.1 += (self.head.1-self.tail.1)/2;
        }
        if self.get_distance() > 2.0 {
            if self.head.0 - 1 > self.tail.0 && self.head.1 > self.tail.1 {
                self.tail.0 += 1;
                self.tail.1 += 1;
            } else if self.head.0 - 1 > self.tail.0 && self.head.1 < self.tail.1 {
                self.tail.0 += 1;
                self.tail.1 -= 1;
            } else if self.head.0 + 1 < self.tail.0 && self.head.1 > self.tail.1 {
                self.tail.0 -= 1;
                self.tail.1 += 1;
            } else if self.head.0 + 1 < self.tail.0 && self.head.1 < self.tail.1 {
                self.tail.0 -= 1;
                self.tail.1 -= 1;
            } else if self.head.0 > self.tail.0 && self.head.1 - 1 > self.tail.1 {
                self.tail.0 += 1;
                self.tail.1 += 1;
            } else if self.head.0 > self.tail.0 && self.head.1 + 1 < self.tail.1 {
                self.tail.0 += 1;
                self.tail.1 -= 1;
            } else if self.head.0 < self.tail.0 && self.head.1 - 1 > self.tail.1 {
                self.tail.0 -= 1;
                self.tail.1 += 1;
            } else if self.head.0 < self.tail.0 && self.head.1 + 1 < self.tail.1 {
                self.tail.0 -= 1;
                self.tail.1 -= 1;
            }
        }
        self.visited_positions.insert(self.tail, true);
    }
}

impl Display for Rope {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "({}, {}) -> ({}, {})",
            self.head.0, self.head.1, self.tail.0, self.tail.1
        )
    }
}

fn main() {
    let inp = input::get_input(9);
    println!("Part 1: {}", part_1(&inp));
}

fn part_1(input: &str) -> usize {
    let mut rope = Rope::new();

    for instruction in input.lines() {
        let (direction, steps) = instruction.split_at(1);
        let steps = steps.trim().parse::<i32>().unwrap();
        match direction {
            "U" => {
                for _ in 0..steps {
                    rope.move_up();
                }
            }
            "D" => {
                for _ in 0..steps {
                    rope.move_down();
                }
            }
            "L" => {
                for _ in 0..steps {
                    rope.move_left();
                }
            }
            "R" => {
                for _ in 0..steps {
                    rope.move_right();
                }
            }
            _ => panic!("Unknown direction"),
        }
    }

    rope.visited_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(INPUT), 13);
    }
}
