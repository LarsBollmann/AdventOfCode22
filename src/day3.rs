mod input;

fn part1(inp: &str) -> u32 {
    inp.to_string().lines().fold(0, |score, line| {
        let (left, right) = line.split_at(line.len() / 2);
        for c in left.chars() {
            if right.contains(c) {
                return score
                    + match c.is_lowercase() {
                        true => c as u32 - 96,
                        false => c as u32 - 38,
                    };
            }
        }
        score
    })
}

fn part2(inp: &str) -> u32 {
    let mut score: u32 = 0;
    let inp_string = inp.to_string();
    let lines: Vec<&str> = inp_string.lines().collect();
    for i in 0..(lines.len() / 3) {
        let elve1 = lines[i * 3];
        let elve2 = lines[i * 3 + 1];
        let elve3 = lines[i * 3 + 2];

        for c in elve1.chars() {
            if elve2.contains(c) && elve3.contains(c) {
                score
                    += match c.is_lowercase() {
                        true => c as u32 - 96,
                        false => c as u32 - 38,
                    };
                break;
            }
        }
    }

    return score;
}

fn main() {
    let input = input::get_input(3);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = 
"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(INPUT), 157);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(INPUT), 70);
    }
}
