use std::{
    collections::{HashMap, HashSet},
    ops::Range,
    str::FromStr
};

mod input;

fn main() {
    let inp = input::get_input(15);
    let sensors = parse_input(&inp);
    println!("{}", part_1(&sensors, 2000000));
    println!("{}", part_2(&sensors, 4000000));
}

type Position = (isize, isize);

#[derive(Debug)]
struct Sensor {
    position: Position,
    closest_beacon: Position,
}

impl Sensor {
    fn distance(&self) -> isize {
        let (x1, y1) = self.position;
        let (x2, y2) = self.closest_beacon;
        (x1 - x2).abs() + (y1 - y2).abs()
    }

    fn get_empty_positions_x(&self, row: isize) -> Option<Range<isize>> {
        let distance = self.distance();

        let x_range = distance - (self.position.1 - row).abs();
        if x_range <= 0 {
            return None;
        }

        Some(self.position.0 - x_range..self.position.0 + x_range + 1)
    }
}

impl FromStr for Sensor {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let x_sensor = parts[2][2..].trim_end_matches(',').parse::<isize>()?;
        let y_sensor = parts[3][2..].trim_end_matches(':').parse::<isize>()?;
        let x_beacon = parts[8][2..].trim_end_matches(',').parse::<isize>()?;
        let y_beacon = parts[9][2..].parse::<usize>().unwrap();

        Ok(Sensor {
            position: (x_sensor, y_sensor),
            closest_beacon: (x_beacon, y_beacon as isize),
        })
    }
}

fn parse_input(inp: &str) -> Vec<Sensor> {
    let mut sensors = Vec::new();
    for line in inp.lines() {
        sensors.push(Sensor::from_str(line).unwrap());
    }
    sensors
}

fn part_1(sensors: &[Sensor], row: isize) -> usize {
    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;

    for sensor in sensors {
        let range = sensor.get_empty_positions_x(row);
        if let Some(range) = range {
            if range.start < min_x {
                min_x = range.start;
            }
            if range.end > max_x {
                max_x = range.end;
            }
        }
    }

    (max_x - min_x - 1) as usize
}

fn part_2(sensors: &[Sensor], max: isize) -> isize {
    for y in 0..max {
        let mut ranges = Vec::new();

        for sensor in sensors {
            let range = sensor.get_empty_positions_x(y);
            if let Some(range) = range {
                ranges.push(range);
            }
        }
        ranges.sort_by_key(|range| range.start);

        if ranges.first().unwrap().start > 0 {
            return y;
        }

        let mut last_end = ranges[0].end;

        for range in ranges {
            if range.start > last_end {
                return last_end * 4000000 + y;
            }
            if range.end > last_end {
                last_end = range.end;
            }
        }
    }

    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part1() {
        let sensors = parse_input(INPUT);
        assert_eq!(part_1(&sensors, 10), 26);
    }

    #[test]
    fn test_part2() {
        let sensors = parse_input(INPUT);
        assert_eq!(part_2(&sensors, 20), 56000011);
    }
}
