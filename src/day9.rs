use std::{
    collections::HashMap,
    fmt::{Display, Error, Formatter},
};

mod input;

type Position = [i32; 2];

struct Rope {
    knots: Vec<Position>,
    visited_positions: HashMap<Position, bool>,
}

impl Rope {
    fn with_lenght(length: usize) -> Self {
        Rope {
            knots: vec![[0, 0]; length],
            visited_positions: HashMap::new(),
        }
    }

    fn adjust_tail(&mut self, head_index: usize, tail_index: usize) {
        let distance = self.get_distance(head_index, tail_index);
        let head = self.knots[head_index];
        let tail = &mut self.knots[tail_index];
        if distance == 2.0 {
            tail[0] += (head[0] - tail[0]) / 2;
            tail[1] += (head[1] - tail[1]) / 2;
        }
        if distance > 2.0 {
            if head[0] - 1 > tail[0] && head[1] > tail[1] {
                tail[0] += 1;
                tail[1] += 1;
            } else if head[0] - 1 > tail[0] && head[1] < tail[1] {
                tail[0] += 1;
                tail[1] -= 1;
            } else if head[0] + 1 < tail[0] && head[1] > tail[1] {
                tail[0] -= 1;
                tail[1] += 1;
            } else if head[0] + 1 < tail[0] && head[1] < tail[1] {
                tail[0] -= 1;
                tail[1] -= 1;
            } else if head[0] > tail[0] && head[1] - 1 > tail[1] {
                tail[0] += 1;
                tail[1] += 1;
            } else if head[0] > tail[0] && head[1] + 1 < tail[1] {
                tail[0] += 1;
                tail[1] -= 1;
            } else if head[0] < tail[0] && head[1] - 1 > tail[1] {
                tail[0] -= 1;
                tail[1] += 1;
            } else if head[0] < tail[0] && head[1] + 1 < tail[1] {
                tail[0] -= 1;
                tail[1] -= 1;
            }
        }
    }

    fn move_direction(&mut self, direction: &str) {
        match direction {
            "U" => {
                self.move_up();
            }
            "D" => {
                self.move_down();
            }
            "L" => {
                self.move_left();
            }
            "R" => {
                self.move_right();
            }
            _ => panic!("Unknown direction"),
        }
    }

    fn move_up(&mut self) {
        self.knots[0][1] += 1;
        for i in 1..self.knots.len() {
            self.adjust_tail(i-1, i);
        }
        self.visited_positions
            .insert(self.knots.last().unwrap().to_owned(), true);
    }

    fn move_down(&mut self) {
        self.knots[0][1] -= 1;
        for i in 1..self.knots.len() {
            self.adjust_tail(i-1, i);
        }
        self.visited_positions
            .insert(self.knots.last().unwrap().to_owned(), true);
    }

    fn move_left(&mut self) {
        self.knots[0][0] -= 1;
        for i in 1..self.knots.len() {
            self.adjust_tail(i-1, i);
        }
        self.visited_positions
            .insert(self.knots.last().unwrap().to_owned(), true);
    }

    fn move_right(&mut self) {
        self.knots[0][0] += 1;
        for i in 1..self.knots.len() {
            self.adjust_tail(i-1, i);
        }
        self.visited_positions
            .insert(self.knots.last().unwrap().to_owned(), true);
    }

    fn get_distance(&self, head: usize, tail: usize) -> f64 {
        f64::sqrt(
            ((self.knots[head][0] - self.knots[tail][0]).pow(2)
                + (self.knots[head][1] - self.knots[tail][1]).pow(2)) as f64,
        )
    }
}

impl Display for Rope {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for knot in &self.knots {
            write!(f, "({}, {})", knot[0], knot[1])?;
        }
        Ok(())
    }
}

fn main() {
    let inp = input::get_input(9);
    println!("Part 1: {}", part_1(&inp));
    println!("Part 2: {}", part_2(&inp));
}

fn part_1(input: &str) -> usize {
    let mut rope = Rope::with_lenght(2);

    for instruction in input.lines() {
        let (direction, steps) = instruction.split_at(1);
        let steps = steps.trim().parse::<i32>().unwrap();
        for _ in 0..steps {
            rope.move_direction(direction);
        }
    }

    rope.visited_positions.len()
}

fn part_2(input: &str) -> usize {
    let mut rope = Rope::with_lenght(10);

    for instruction in input.lines() {
        let (direction, steps) = instruction.split_at(1);
        let steps = steps.trim().parse::<i32>().unwrap();
        for _ in 0..steps {
            rope.move_direction(direction);
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

    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(INPUT), 1);
        assert_eq!(part_2(INPUT2), 36);
    }
}
