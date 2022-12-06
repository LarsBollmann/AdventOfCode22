mod input;

fn has_unique_chars(word: &str) -> bool {
    let mut chars = word.chars().collect::<Vec<char>>();
    chars.sort();
    chars.dedup();
    chars.len() == word.len()
}

fn get_position_of_n_unique_chars(inp: &str, length: usize) -> usize {
    assert!(length > 0 && length < inp.len(), "length must be between 1 and the length of the input");
    for i in 0..(inp.len()-length) {
        let sliced = &inp[i..i+length];
        if has_unique_chars(sliced) {
            return i + length;
        };
    }
    panic!("No unique char sequence of lenght {} found", length);
}

fn main() {
    let input = input::get_input(6);
    println!("Part 1: {}", get_position_of_n_unique_chars(&input, 4));
    println!("Part 2: {}", get_position_of_n_unique_chars(&input, 14));
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_part_1() {
        
        assert_eq!(get_position_of_n_unique_chars(INPUT, 4), 7);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(get_position_of_n_unique_chars(INPUT, 14), 19);   
    }
}
