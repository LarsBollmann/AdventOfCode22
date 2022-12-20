use std::{str::FromStr, collections::HashMap, io::{stdin, Read}};

#[derive(Debug,Clone,Copy,Hash,PartialEq,Eq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl FromStr for Resource {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ore" => Ok(Resource::Ore),
            "clay" => Ok(Resource::Clay),
            "obsidian" => Ok(Resource::Obsidian),
            "geode" => Ok(Resource::Geode),
            _ => panic!("Unknown resource"),
        }
    }
}

#[derive(Debug)]
struct Robot {
    produces: Resource,
    required_resources: Vec<(Resource, u32)>,
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let produced = Resource::from_str(parts[1]);
        let costs = &parts[4..];
        let mut required_resources = Vec::new();
        for i in (0..costs.len()).step_by(3) {
            let resource = Resource::from_str(costs[i + 1]);
            let amount = costs[i].parse::<u32>().unwrap();
            required_resources.push((resource.unwrap(), amount));
        }

        Ok(Robot {
            produces: produced.unwrap(),
            required_resources,
        })
    }
}

fn traverse(robot_config: &Vec<Robot>, mut robots: Vec<Resource>, qeued_robot: Option<Resource>, mut resources: HashMap<Resource, u32>, mut remaining_minutes: u32) -> u32 {


    for robot in &robots {
        resources.entry(*robot).and_modify(|e| *e += 1).or_insert(1);
    }

    if remaining_minutes == 0 {
        return *resources.get(&Resource::Geode).unwrap_or(&0);
    }

    if let Some(robot) = qeued_robot {
        robots.push(robot);
    }


    //println!("Minute {}: {:?} {:?}", 24 - remaining_minutes, resources, robots);

    let mut max = 0;
    let mut max_action: Option<Resource> = None;

    for possible_robot in robot_config {
        if robots.iter().filter(|r| **r == possible_robot.produces).count() > 5 {
            continue;
        }
        let mut wait_time = 0;
        for (resource, cost) in &possible_robot.required_resources {
            let mut missing_resources = 0;
            if *resources.get(resource).unwrap_or(&0) < *cost {  
                missing_resources = cost - *resources.get(resource).unwrap_or(&0);
            }
            
            let producing_robots = robots.iter().filter(|r| **r == *resource).count() as u32;

            if producing_robots == 0 {
                wait_time = u32::MAX-1;
                break;
            }
            let mut steps = missing_resources / producing_robots;
            if missing_resources % producing_robots != 0 {
                steps += 1;
            }
            if steps > wait_time {
                wait_time = steps;
            }
        }
        //println!("Wait time: {}, remaining: {}", wait_time, remaining_minutes);
        if remaining_minutes >= wait_time + 1 {
            let mut new_robots = robots.clone();

            let mut new_resources = resources.clone();
            for robot in &robots {
                new_resources.entry(*robot).and_modify(|e| *e += wait_time);
            }
            for (resource, cost) in &possible_robot.required_resources {
                new_resources.entry(*resource).and_modify(|e| *e -= cost);
            }

            let result = traverse(robot_config, new_robots, Some(possible_robot.produces), new_resources, remaining_minutes - wait_time-1);
            if result > max {
                max = result;
                max_action = Some(possible_robot.produces);
            }

        }
    }
    if max == 0 {
        for robot in &robots {
            resources.entry(*robot).and_modify(|e| *e += remaining_minutes);
        }
        *resources.get(&Resource::Geode).unwrap_or(&0)
    } else {
        //println!("The best option at minute {} is to produce a {:?} robot", 24 - remaining_minutes, max_action.unwrap());
        max
    }
}

fn part_1 (input: &str) -> u32 {
    for line in input.lines() {
        let mut robots = Vec::new();
        let robots_str = line.split(':').nth(1).unwrap();
        for robot_str in robots_str.split_terminator('.') {
            let robot = Robot::from_str(robot_str).unwrap();
            robots.push(robot);
        }
        robots.reverse();
        println!("{}", traverse(&robots, vec![Resource::Ore], None, HashMap::new(), 23));
    }
    0
}

fn main() {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = 
"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_part1() {
        let part_1 = part_1(INPUT);
        assert_eq!(part_1, 12);
    }
}