mod input;

struct Cpu {
    register: i32,
    cycle: i32,
    history: Vec<i32>,
    output: String,
    width: usize,
}

enum Instruction {
    Addx(i32),
    Noop
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            register: 1,
            cycle: 0,
            history: Vec::new(),
            output: String::new(),
            width: 40,
        }
    }

    fn advance_cycle(&mut self) {
        let x = self.cycle % self.width as i32;
        if x == 0 && self.cycle != 0 {
            self.output.push('\n');
        }

        if (self.register - x).abs() <= 1 {
            self.output.push('#');
        } else {
            self.output.push('.');
        }

        self.history.push(self.register);
        self.cycle += 1;
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Addx(x) => {
                self.advance_cycle();
                self.advance_cycle();
                self.register += x
            },
            Instruction::Noop => self.advance_cycle(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        match parts[0] {
            "addx" => Instruction::Addx(parts[1].parse::<i32>().unwrap()),
            "noop" => Instruction::Noop,
            _ => panic!("Unknown instruction"),
        }
    }).collect()
}
fn main() {
    let inp = input::get_input(10);
    println!("Part 1: {}", part_1(parse_input(&inp)));
    println!("Part 2:\n{}", part_2(parse_input(&inp)));
}

fn part_1(instructions: Vec<Instruction>) -> i32 {
    let mut cpu = Cpu::new();
    for instruction in instructions {
        cpu.execute(instruction);
    }
    cpu.history.iter().skip(19).step_by(40).enumerate().map(|(i, x)| {
        (i*40+20) as i32 * x
    }).sum()
}

fn part_2(instructions: Vec<Instruction>) -> String {
    let mut cpu = Cpu::new();

    for instruction in instructions {
        cpu.execute(instruction);
    }
    cpu.output
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../test_input/day10.txt");

    #[test]
    fn test_part_1() {
        let inp = parse_input(INPUT);
        assert_eq!(part_1(inp), 13140);
    }

    #[test]
    fn test_part_2() {
        let inp = parse_input(INPUT);
        assert_eq!(part_2(inp), 
"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....");
    }
}