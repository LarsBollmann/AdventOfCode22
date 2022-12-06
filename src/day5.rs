mod input;

type Stacks = Vec<Vec<char>>;

#[derive(Debug)]
struct Instruction {
    from: u8,
    to: u8,
    times: u8
}

fn parse_input(inp: &str) -> (Stacks, Vec<Instruction>) {
    let inp_string = inp.to_string();
    let mut parts = inp_string.split("\n\n").collect::<Vec<&str>>();
    let instruction_str = parts.pop().unwrap();
    let configuration_str = parts.pop().unwrap();
    
    let stack_number = (configuration_str.split_once('\n').unwrap().0.len()+1)/4;
    let mut stacks: Stacks = Vec::with_capacity(stack_number);
    for _ in 0..stack_number {
        stacks.push(Vec::new());
    }

    println!("configuration_str:\n{}", configuration_str);
    println!("number of stacks: {}", stack_number);

    for line in configuration_str.lines().rev() {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let letter = line.chars().nth(i*4+1).unwrap();
            if !letter.is_whitespace() && !letter.is_ascii_digit() {
                stack.push(letter);
            }
        }
    }

    let instructions = instruction_str.lines().map(|line|{
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        Instruction {
            from: parts[3].parse().unwrap(),
            to: parts[5].parse().unwrap(),
            times: parts[1].parse().unwrap()
        }
    }).collect::<Vec<Instruction>>();

    (stacks, instructions)
}

fn get_keyword(stacks: &Stacks) -> String {
    let mut word = String::new();
    for stack in stacks {
        word.push(*stack.last().unwrap());
    }
    word
}

fn part_1(stacks: &Stacks, instructions: &Vec<Instruction>) -> String {
    let mut stacks = stacks.clone();

    for instruction in instructions {
        for _ in 0..instruction.times {
            let letter = stacks[(instruction.from-1) as usize].pop().unwrap();
            stacks[(instruction.to-1) as usize].push(letter);
        }
    };

    get_keyword(&stacks)
}

fn part_2(stacks: &Stacks, instructions: &Vec<Instruction>) -> String {
    let mut stacks = stacks.clone();
    for instruction in instructions {
        let mut buffer = Vec::new();
        for _ in 0..instruction.times {
            let letter = stacks[(instruction.from-1) as usize].pop().unwrap();
            buffer.push(letter);
        }

        buffer.reverse();

        for letter in buffer {
            stacks[(instruction.to-1) as usize].push(letter);
        }
    };

    get_keyword(&stacks)
}

fn main() {
    let input = input::get_input(5);
    let (crates, instructions) = parse_input(&input);
    println!("Part 1: {}", part_1(&crates, &instructions));
    println!("Part 2: {}", part_2(&crates, &instructions));

}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part_one() {
        let (crates, instructions) = parse_input(INPUT);
        assert_eq!(part_1(&crates, &instructions), "CMZ");
    }

    #[test]
    fn part_two() {
        let (crates, instructions) = parse_input(INPUT);
        assert_eq!(part_2(&crates, &instructions), "MCD");
    }
}