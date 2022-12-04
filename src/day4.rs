mod input;

fn parse_input(input: &str) -> Vec<(usize, usize, usize, usize)> {
    input.lines().map(|line| {
        let parts = line.split(",").map(|side| {
            side.split("-").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>()
        }).flatten().collect::<Vec<usize>>();
        assert!(parts.len() == 4, "Invalid input");
        return (parts[0], parts[1], parts[2], parts[3]);
    }).collect()
}

fn is_contained(start1: &usize, end1: &usize, start2: &usize, end2: &usize) -> bool {
    if start1 >= start2 && end1 <= end2 {
        return true;
    }
    if start2 >= start1 && end2 <= end1 {
        return true;
    }
    false
}

fn does_overlap(start1: &usize, end1: &usize, start2: &usize, end2: &usize) -> bool {
    if start1 >= start2 && start1 <= end2 {
        return true;
    }
    if start2 >= start1 && start2 <= end1 {
        return true;
    }
    false
}

fn part1(inp: &Vec<(usize, usize, usize, usize)>) -> u32 {
    inp.iter().fold(0, |acc, (start1, end1, start2, end2)| {
        if is_contained(start1, end1, start2, end2) {
            return acc + 1;
        }
        acc
    })
}

fn part2(inp: &Vec<(usize, usize, usize, usize)>) -> u32 {
    inp.iter().fold(0, |acc, (start1, end1, start2, end2)| {
        if does_overlap(start1, end1, start2, end2) {
            return acc + 1;
        }
        acc
    })
}

fn main() {
    let input = input::get_input(4);
    let parsed_input = parse_input(&input);
    println!("Part 1: {}", part1(&parsed_input));
    println!("Part 2: {}", part2(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str =
"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part_1() {
        let parsed_input = parse_input(INPUT);
        let result = part1(parsed_input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_2() {
        let parsed_input = parse_input(INPUT);
        let result = part2(parsed_input);
        assert_eq!(result, 4);
    }
}