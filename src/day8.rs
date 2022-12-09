mod input;

fn main() {
    let inp = input::get_input(8);
    let parsed_input = parse_input(&inp);
    println!("Part 1: {}", part_1(&parsed_input));
    println!("Part 2: {}", part_2(&parsed_input));
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Could not convert char into digit") as u8)
                .collect::<Vec<u8>>()
        })
        .collect()
}

fn get_coordinates_by_direction(
    y: usize,
    x: usize,
    size: usize,
    direction: &str,
) -> (usize, usize) {
    match direction {
        "right" => (y, x),
        "left" => (y, size - x - 1),
        "down" => (x, y),
        "up" => (size - x - 1, y),
        _ => panic!("Invalid direction"),
    }
}

fn get_visible_trees(inp: &Vec<Vec<u8>>, visibility_map: &mut [Vec<u8>], direction: &str) {
    let size = inp.len();

    for y in 1..size - 1 {
        let first_element_cords = get_coordinates_by_direction(y, 0, size, direction);
        let mut max_height: u8 = inp[first_element_cords.0][first_element_cords.1];
        for x in 1..size - 1 {
            let coords = get_coordinates_by_direction(y, x, size, direction);
            if inp[coords.0][coords.1] > max_height {
                max_height = inp[coords.0][coords.1];
                visibility_map[coords.0][coords.1] = 1;
            }
        }
    }
}

fn part_1(inp: &Vec<Vec<u8>>) -> usize {
    let mut visibility_map = vec![vec![0; inp.len()]; inp.len()];

    get_visible_trees(inp, &mut visibility_map, "right");
    get_visible_trees(inp, &mut visibility_map, "left");
    get_visible_trees(inp, &mut visibility_map, "down");
    get_visible_trees(inp, &mut visibility_map, "up");

    visibility_map.iter().flatten().filter(|&&v| v == 1).count() + 4 * inp.len() - 4
}

fn get_scenic_score(i: usize, j: usize, inp: &Vec<Vec<u8>>) -> usize {
    let mut viewing_distance = 0;
    if j < inp.len() - 1 {
        for k in (j + 1)..inp.len() {
            viewing_distance += 1;
            if inp[i][k] >= inp[i][j] {
                break;
            }
        }
    }
    let mut scenic_score = viewing_distance;

    viewing_distance = 0;

    if j > 0 {
        for k in (0..j).rev() {
            viewing_distance += 1;
            if inp[i][k] >= inp[i][j] {
                break;
            }
        }
    }

    scenic_score *= viewing_distance;
    viewing_distance = 0;
    if i < inp.len() - 1 {
        for k in (i + 1)..inp.len() {
            viewing_distance += 1;
            if inp[k][j] >= inp[i][j] {
                break;
            }
        }
    }

    scenic_score *= viewing_distance;
    viewing_distance = 0;
    if i > 0 {
        for k in (0..i).rev() {
            viewing_distance += 1;
            if inp[k][j] >= inp[i][j] {
                break;
            }
        }
    }

    scenic_score * viewing_distance
}

fn part_2(inp: &Vec<Vec<u8>>) -> usize {
    let mut distances: Vec<usize> = Vec::new();
    for i in 0..inp.len() {
        for j in 0..inp.len() {
            distances.push(get_scenic_score(i, j, inp));
        }
    }

    *distances.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part_1() {
        let parsed_input = parse_input(INPUT);
        assert_eq!(part_1(&parsed_input), 21);
    }

    #[test]
    fn test_part_2() {
        let parsed_input = parse_input(INPUT);
        assert_eq!(part_2(&parsed_input), 8);
    }
}
