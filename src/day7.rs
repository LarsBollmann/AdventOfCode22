use core::panic;
use std::collections::HashMap;

mod input;

fn get_folders_with_sizes(input: &str) -> HashMap<String, usize> {
    let mut folder_sizes = HashMap::new();
    let mut current_path = Vec::new();

    let lines = input.lines();

    for line in lines {
        if line.contains("$ cd") {
            match line.split_whitespace().nth(2) {
                Some("..") => {
                    current_path.pop();
                }
                Some(dir) => {
                    current_path.push(dir);
                }
                None => {
                    panic!("Invalid cd command");
                }
            }
        }
        if let Ok(size) = line.split_whitespace().next().unwrap().parse::<usize>() {
            let mut path = String::from("");
            for dir in current_path.iter() {
                path.push_str(dir);
                if folder_sizes.contains_key(&path) {
                    let current_size = folder_sizes.get(&path).unwrap();
                    folder_sizes.insert(path.clone(), current_size + size);
                } else {
                    folder_sizes.insert(path.clone(), size);
                }
            }
        }
    }

    folder_sizes
}

fn part_1(input: &str) -> usize {
    let folders = get_folders_with_sizes(input);
    let small_folders = folders.iter().filter_map(|(_k, v)| {
        if *v <= 100000 {
            Some(*v)
        } else {
            None
        }
    });
    small_folders.sum()
}

fn part_2(input: &str) -> usize {
    let folders = get_folders_with_sizes(input);
    let needed_space = 30000000 - (70000000 - folders.get("/").unwrap());
    let mut sizes = folders.iter().map(|(_k, v)| *v).collect::<Vec<usize>>();
    sizes.sort();
    *sizes.iter().find(|&&v| v >= needed_space).unwrap()

}

fn main() {
    let input = input::get_input(7);
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = 
"    $ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    dir e
    29116 f
    2557 g
    62596 h.lst
    $ cd e
    $ ls
    584 i
    $ cd ..
    $ cd ..
    $ cd d
    $ ls
    4060174 j
    8033020 d.log
    5626152 d.ext
    7214296 k";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(INPUT), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(INPUT), 24933642);
    }
}