mod input;

use core::fmt;
use std::{cmp, collections::HashSet};

#[derive(Hash, PartialEq, Eq, Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Grid {
    rocks: HashSet<Position>,
    sand: HashSet<Position>,
    max: Position,
    min: Position,
    floor: Option<usize>,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            rocks: HashSet::new(),
            sand: HashSet::new(),
            max: Position {
                x: usize::MIN,
                y: usize::MIN,
            },
            min: Position {
                x: usize::MAX,
                y: usize::MAX,
            },
            floor: None,
        }
    }
    fn insert_rock(&mut self, pos: Position) {
        if pos.x > self.max.x {
            self.max.x = pos.x;
        }
        if pos.y > self.max.y {
            self.max.y = pos.y;
        }
        if pos.x < self.min.x {
            self.min.x = pos.x;
        }
        if pos.y < self.min.y {
            self.min.y = pos.y;
        }
        self.rocks.insert(pos);
    }

    fn contains(&self, pos: &Position) -> bool {
        self.rocks.contains(pos) || self.sand.contains(pos)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\x1B[2J\x1B[1;1H")?;

        for y in 0..(if self.floor.is_some() {self.floor.unwrap()} else {self.max.y}) + 1 {
            for x in self.min.x..self.max.x + 1 {
                let pos = Position { x, y };
                if self.rocks.contains(&pos) {
                    write!(f, "#")?;
                } else if self.sand.contains(&pos) {
                    write!(f, "+")?;
                } else if self.floor.is_some() && y == self.floor.unwrap() {
                    write!(f, "~")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_input(inp: &str) -> Grid {
    let mut grid = Grid::new();

    for line in inp.lines() {
        let parts = line.split(" -> ").collect::<Vec<&str>>();
        for i in 0..parts.len() - 1 {
            let start = parts[i].split(',').collect::<Vec<&str>>();
            let end = parts[i + 1].split(',').collect::<Vec<&str>>();
            let start_x = start[0].parse::<usize>().unwrap();
            let start_y = start[1].parse::<usize>().unwrap();
            let end_x = end[0].parse::<usize>().unwrap();
            let end_y = end[1].parse::<usize>().unwrap();
            for x in cmp::min(start_x, end_x)..cmp::max(start_x, end_x) + 1 {
                for y in cmp::min(start_y, end_y)..cmp::max(start_y, end_y) + 1 {
                    grid.insert_rock(Position { x, y });
                }
            }
        }
    }
    grid
}

fn simulate_sand(grid: &mut Grid) -> usize {
    for i in 0..usize::MAX {
        let mut x = 500;
        let mut y = 0;
        loop {
            if grid.contains(&Position { x, y }) {
                return i;
            }
            if grid.floor.is_none() && y > grid.max.y {
                return i;
            } else if grid.floor.is_some() && y + 1 == grid.floor.unwrap() {
                grid.sand.insert(Position { x, y });
                break;
            } else if !grid.contains(&Position { x, y: y + 1 }) {
                y += 1;
            } else if !grid.contains(&Position { x: x - 1, y: y + 1 }) {
                y += 1;
                x -= 1;
            } else if !grid.contains(&Position { x: x + 1, y: y + 1 }) {
                y += 1;
                x += 1;
            }
            else {
                grid.sand.insert(Position { x, y });
                break;
            }
        }
    }
    10
}

fn main() {
    let inp = input::get_input(14);
    let mut grid = parse_input(&inp);
    let solution_1 = simulate_sand(&mut grid);
    println!("{}", grid);
    grid.floor = Some(grid.max.y + 2);
    grid.sand = HashSet::new();
    let solution_2 = simulate_sand(&mut grid);
    println!("{}", grid);
    println!("Part 1: {}", solution_1);
    println!("Part 2: {}", solution_2);
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part1() {
        println!("Input: {}", INPUT);
        let mut grid = parse_input(INPUT);
        println!("Positions: {}", grid);
        assert_eq!(simulate_sand(&mut grid), 24);
    }

    #[test]
    fn test_part2() {
        println!("Input: {}", INPUT);
        let mut grid = parse_input(INPUT);
        grid.floor = Some(grid.max.y + 2);
        println!("Positions: {}", grid);
        assert_eq!(simulate_sand(&mut grid), 93);
    }
}
