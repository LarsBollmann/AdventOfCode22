use std::collections::{BinaryHeap, HashSet};
use std::cmp::{Ord, PartialOrd, Ordering};
mod input;


type Heightmap = Vec<Vec<char>>;

#[derive(Debug,Hash,PartialEq,Eq,Clone,Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn neighbors(&self, rows: usize, columns: usize) -> Vec<Self> {
        let mut neighbors = Vec::new();
        if self.x > 0 {
            neighbors.push(Position { x: self.x - 1, y: self.y });
        }
        if self.x < columns - 1 {
            neighbors.push(Position { x: self.x + 1, y: self.y });
        }
        if self.y > 0 {
            neighbors.push(Position { x: self.x, y: self.y - 1 });
        }
        if self.y < rows - 1 {
            neighbors.push(Position { x: self.x, y: self.y + 1 });
        }
        neighbors
    }
}

#[derive(Debug,Eq,PartialEq)]
struct QueueItem {
    position: Position,
    cost: usize,
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> (Heightmap, Position, Position) {
    let mut heightmap: Heightmap = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    
    let mut y = heightmap.iter().position(|row| row.contains(&'S')).unwrap();
    let mut x = heightmap[y].iter().position(|&c| c == 'S').unwrap();
    let start_position = Position { x, y };

    y = heightmap.iter().position(|row| row.contains(&'E')).unwrap();
    x = heightmap[y].iter().position(|&c| c == 'E').unwrap();
    let end_position = Position { x, y };

    heightmap[start_position.y][start_position.x] = 'a';
    heightmap[end_position.y][end_position.x] = 'z';
    (heightmap, start_position, end_position)
}

fn get_shortest_steps(heightmap: &Heightmap, start_position: Position, end_position: Position) -> Option<usize> {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    let rows = heightmap.len();
    let columns = heightmap[0].len();

    queue.push(QueueItem { position: start_position, cost: 0 });

    while let Some(QueueItem { position, cost }) = queue.pop() {
        if position == end_position {
            return Some(cost);
        }

        let current_height = heightmap[position.y][position.x] as u8;

        let mut neighbors = position.neighbors(rows, columns);
        neighbors.retain(|neighbor| !visited.contains(neighbor) && heightmap[neighbor.y][neighbor.x] as u8 - 1 <= current_height);

        for neighbor in neighbors {
            visited.insert(neighbor);
            queue.push(QueueItem { position: neighbor, cost: cost + 1 });
        }
    }
    None
}

fn part_2(heightmap: &Heightmap, end_position: Position) -> usize {
    let mut possible_start_positions = Vec::new();

    for y in 0..heightmap.len() {
        for x in 0..heightmap[0].len() {
            if heightmap[y][x] == 'a' {
                possible_start_positions.push(Position { x, y });
            }
        }
    }

    possible_start_positions
        .iter()
        .filter_map(|&start_position|  get_shortest_steps(heightmap, start_position, end_position))
        .min()
        .unwrap()
}

fn main() {
    let inp = input::get_input(12);
    let (heightmap, start_position, end_position) = parse_input(&inp);
    println!("Part 1: {}", get_shortest_steps(&heightmap, start_position, end_position).unwrap());
    println!("Part 2: {}", part_2(&heightmap, end_position));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = 
"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part_1() {
        let (heightmap, start_position, end_position) = parse_input(INPUT);
        assert_eq!(get_shortest_steps(&heightmap, start_position, end_position).unwrap(), 31);
    }

    #[test]
    fn test_part_2() {
        let (heightmap, _, end_position) = parse_input(INPUT);
        assert_eq!(part_2(&heightmap, end_position), 29);
    }
}