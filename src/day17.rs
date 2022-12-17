use std::collections::{HashMap, HashSet, VecDeque};

mod input;

type Position = (i64, i64);

type RockShape = Vec<Position>;

struct Chamber {
    occupied_positions: HashSet<Position>,
    width: i64,
    highest_position: i64,
}

impl Chamber {
    fn does_collide(&self, rock_shape: &RockShape, global_position: Position) -> bool {
        for position in rock_shape {
            if self.occupied_positions.contains(&(
                position.0 + global_position.0,
                position.1 + global_position.1,
            )) || position.1 + global_position.1 >= self.width
                || position.1 + global_position.1 < 0
                || position.0 + global_position.0 < 0
            {
                return true;
            }
        }
        false
    }

    fn move_and_clamp(
        &self,
        rock_shape: &RockShape,
        position: Position,
        direction: char,
    ) -> Position {
        let position = position;
        match direction {
            '>' => {
                if self.does_collide(rock_shape, (position.0, position.1 + 1)) {
                    return position;
                }
                (position.0, position.1 + 1)
            }
            '<' => {
                if self.does_collide(rock_shape, (position.0, position.1 - 1)) {
                    return position;
                }
                (position.0, position.1 - 1)
            }
            'd' => {
                if self.does_collide(rock_shape, (position.0 - 1, position.1)) {
                    return position;
                }
                (position.0 - 1, position.1)
            }
            _ => panic!("Invalid direction: {}", direction),
        }
    }

    fn get_starting_position(&self) -> Position {
        (self.highest_position + 4, 2)
    }

    fn insert_rock_shape(&mut self, rock_shape: &RockShape, global_position: Position) {
        for position in rock_shape {
            let position = (
                position.0 + global_position.0,
                position.1 + global_position.1,
            );

            if position.0 > self.highest_position {
                self.highest_position = position.0;
            }

            if !self.occupied_positions.insert((position.0, position.1)) {
                panic!("Tried to insert a rock that already exists");
            }
        }
    }
}

fn main() {
    let inp = input::get_input(17);
    println!("Part 1: {}", part_1(&inp, 2022));
    println!("Part 2: {}", part_1(&inp, 1000000000000));
}

fn get_rock_shapes() -> Vec<RockShape> {
    vec![
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ]
}

fn part_1(inp: &str, n: usize) -> i64 {
    let rock_shapes = get_rock_shapes();
    let movements: Vec<char> = inp.chars().filter(|c| *c == '<' || *c == '>').collect();
    let num_movements = movements.len();
    let mut patterns = HashMap::new();
    let mut current_pattern = VecDeque::new();

    let mut total = 0;

    let mut chamber = Chamber {
        occupied_positions: HashSet::new(),
        width: 7,
        highest_position: -1,
    };

    let mut i = 0;
    let mut i_wind = 0;
    let mut round = 0;
    let mut height_old = -1;

    // Loop over every rock shape n times
    while round < n {
        let rock_shape = &rock_shapes[i];
        let mut position = chamber.get_starting_position();

        // Loop until cant move down anymore
        loop {
            position = chamber.move_and_clamp(rock_shape, position, movements[i_wind]);
            i_wind = (i_wind + 1) % num_movements;

            if chamber.does_collide(rock_shape, (position.0 - 1, position.1)) {
                current_pattern.push_back((i, chamber.highest_position - height_old, i_wind));
                height_old = chamber.highest_position;
                chamber.insert_rock_shape(rock_shape, position);
                break;
            }

            position = (position.0 - 1, position.1);
        }
        if current_pattern.len() == 10 {
            if let Some((r, height)) =
                patterns.insert(current_pattern.clone(), (round, chamber.highest_position))
            {
                let period = round - r;
                let diff = chamber.highest_position - height;
                let rounds_left = n - round;
                total += diff * (rounds_left / period) as i64;
                let rounds_skipped = rounds_left - (rounds_left % period);
                round += rounds_skipped;
            }
            current_pattern.pop_front();
        }

        i = (i + 1) % rock_shapes.len();
        round += 1;
    }
    total as i64 + chamber.highest_position + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT, 2022), 3068);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_1(INPUT, 1000000000000), 1514285714288);
    }
}
