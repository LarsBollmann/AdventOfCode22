mod input;
use std::{collections::{HashMap, HashSet}};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn get_distance(&self, other: &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

fn flood_cubes(
    cubes: &mut HashMap<Position, usize>,
    flooded_cubes: &mut HashSet<Position>,
    pos: Position,
    bbox: &(Position, Position),
) -> HashSet<Position> {
    flooded_cubes.insert(pos);
    if pos.x > bbox.0.x && !cubes.contains_key(&Position { x: pos.x - 1, y: pos.y, z: pos.z }) && !flooded_cubes.contains(&Position { x: pos.x - 1, y: pos.y, z: pos.z }) {
        flooded_cubes.insert(Position { x: pos.x - 1, y: pos.y, z: pos.z });
        flood_cubes(cubes, flooded_cubes, Position { x: pos.x - 1, y: pos.y, z: pos.z }, bbox);
    }
    if pos.x < bbox.1.x && !cubes.contains_key(&Position { x: pos.x + 1, y: pos.y, z: pos.z }) && !flooded_cubes.contains(&Position { x: pos.x + 1, y: pos.y, z: pos.z }) {
        flooded_cubes.insert(Position { x: pos.x + 1, y: pos.y, z: pos.z });
        flood_cubes(cubes, flooded_cubes, Position { x: pos.x + 1, y: pos.y, z: pos.z }, bbox);
    }
    if pos.y > bbox.0.y && !cubes.contains_key(&Position { x: pos.x, y: pos.y - 1, z: pos.z }) && !flooded_cubes.contains(&Position { x: pos.x, y: pos.y - 1, z: pos.z }) {
        flooded_cubes.insert(Position { x: pos.x, y: pos.y - 1, z: pos.z });
        flood_cubes(cubes, flooded_cubes, Position { x: pos.x, y: pos.y - 1, z: pos.z }, bbox);
    }
    if pos.y < bbox.1.y && !cubes.contains_key(&Position { x: pos.x, y: pos.y + 1, z: pos.z }) && !flooded_cubes.contains(&Position { x: pos.x, y: pos.y + 1, z: pos.z }) {
        flooded_cubes.insert(Position { x: pos.x, y: pos.y + 1, z: pos.z });
       flood_cubes(cubes, flooded_cubes, Position { x: pos.x, y: pos.y + 1, z: pos.z }, bbox);
    }
    if pos.z > bbox.0.z && !cubes.contains_key(&Position { x: pos.x, y: pos.y, z: pos.z - 1 }) && !flooded_cubes.contains(&Position { x: pos.x, y: pos.y, z: pos.z - 1 }) {
        flooded_cubes.insert(Position { x: pos.x, y: pos.y, z: pos.z - 1 });
       flood_cubes(cubes, flooded_cubes, Position { x: pos.x, y: pos.y, z: pos.z - 1 }, bbox);
    }
    if pos.z < bbox.1.z && !cubes.contains_key(&Position { x: pos.x, y: pos.y, z: pos.z + 1 }) && !flooded_cubes.contains(&Position { x: pos.x, y: pos.y, z: pos.z + 1 }) {
        flooded_cubes.insert(Position { x: pos.x, y: pos.y, z: pos.z + 1 });
        flood_cubes(cubes, flooded_cubes, Position { x: pos.x, y: pos.y, z: pos.z + 1 }, bbox);
    }

    flooded_cubes.clone()
}

fn part_1(inp: &str) -> (usize, usize) {
    let mut cubes = HashMap::new();
    for line in inp.lines() {
        let mut coords = line.split(',').map(|x| x.parse::<i32>().unwrap());
        let new_position = Position {
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
            z: coords.next().unwrap(),
        };
        let mut new_obstructed_sides = 0;
        for (pos, obstructed_sides) in cubes.iter_mut() {
            if new_position.get_distance(pos) == 1 {
                *obstructed_sides += 1;
                new_obstructed_sides += 1;
            }
        }
        cubes.insert(new_position, new_obstructed_sides);
    }

    let bbox_min = Position { x: 0, y: 0, z: 0 };
    let bbox_max = Position {
        x: cubes.keys().max_by_key(|c| c.x).unwrap().x+1,
        y: cubes.keys().max_by_key(|c| c.y).unwrap().y+1,
        z: cubes.keys().max_by_key(|c| c.z).unwrap().z+1,
    };

    let flooded_cubes = flood_cubes(&mut cubes, &mut HashSet::new(), Position { x: 0, y: 0, z: 0 }, &(bbox_min, bbox_max));

    let mut air_pocket_surface = 0;
    for x in bbox_min.x..bbox_max.x {
        for y in bbox_min.y..bbox_max.y {
            for z in bbox_min.z..bbox_max.z{
                let pos = Position { x, y, z };
                if !cubes.contains_key(&pos) && !flooded_cubes.contains(&pos) {
                    if cubes.contains_key(&Position { x: pos.x - 1, y: pos.y, z: pos.z }) {
                        air_pocket_surface += 1;
                    }
                    if cubes.contains_key(&Position { x: pos.x + 1, y: pos.y, z: pos.z }) {
                        air_pocket_surface += 1;
                    }
                    if cubes.contains_key(&Position { x: pos.x, y: pos.y - 1, z: pos.z }) {
                        air_pocket_surface += 1;
                    }
                    if cubes.contains_key(&Position { x: pos.x, y: pos.y + 1, z: pos.z }) {
                        air_pocket_surface += 1;
                    }
                    if cubes.contains_key(&Position { x: pos.x, y: pos.y, z: pos.z - 1 }) {
                        air_pocket_surface += 1;
                    }
                    if cubes.contains_key(&Position { x: pos.x, y: pos.y, z: pos.z + 1 }) {
                        air_pocket_surface += 1;
                    }
                }
            }
        }
    }

    let not_blocked_cube_sides = cubes.len() * 6 - cubes.values().sum::<usize>();
    let surface_area = not_blocked_cube_sides - air_pocket_surface;

    (not_blocked_cube_sides, surface_area)
}

fn main() {
    let inp = input::get_input(18);
    let (part_1, part_2) = part_1(&inp);
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_part1() {
        let (part_1, _) = part_1(INPUT);
        assert_eq!(part_1, 64);
    }

    #[test]
    fn test_part2() {
        let (_, part_2) = part_1(INPUT);
        assert_eq!(part_2, 58);
    }
}
