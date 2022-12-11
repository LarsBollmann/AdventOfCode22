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
        let distance = self.get_distance();
        if distance == 2.0 {
            self.tail.1 += 1;
        } else if distance > 2.0 {
            if self.tail.0 < self.head.0 {
                self.tail.0 += 1;
                self.tail.1 += 1;
            } else {
                self.tail.0 -= 1;
                self.tail.1 += 1;
            }
        }
        self.visited_positions.insert(self.tail, true);
    }

    fn move_down(&mut self) {
        self.head.1 -= 1;
        let distance = self.get_distance();
        if distance == 2.0 {
            self.tail.1 -= 1;
        } else if distance > 2.0 {
            if self.tail.0 < self.head.0 {
                self.tail.0 += 1;
                self.tail.1 -= 1;
            } else {
                self.tail.0 -= 1;
                self.tail.1 -= 1;
            }
        }
        self.visited_positions.insert(self.tail, true);
    }

    fn move_left(&mut self) {
        self.head.0 -= 1;
        let distance = self.get_distance();
        if distance == 2.0 {
            self.tail.0 -= 1;
        } else if distance > 2.0 {
            if self.tail.1 < self.head.1 {
                self.tail.0 -= 1;
                self.tail.1 += 1;
            } else {
                self.tail.0 -= 1;
                self.tail.1 -= 1;
            }
        }
        self.visited_positions.insert(self.tail, true);
    }

    fn move_right(&mut self) {
        self.head.0 += 1;
        let distance = self.get_distance();
        if distance == 2.0 {
            self.tail.0 += 1;
        } else if distance > 2.0 {
            if self.tail.1 < self.head.1 {
                self.tail.0 += 1;
                self.tail.1 += 1;
            } else {
                self.tail.0 += 1;
                self.tail.1 -= 1;
            }
        }
        self.visited_positions.insert(self.tail, true);
    }

    fn get_distance(&self) -> f64 {
        f64::sqrt(
            f64::powi(self.head.0 as f64 - self.tail.0 as f64, 2)
                + f64::powi(self.head.1 as f64 - self.tail.1 as f64, 2),
        )
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
