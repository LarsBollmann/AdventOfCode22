mod input;

#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn get_score(&self, other: &Hand) -> u32 {
        match self {
            Hand::Rock => match other {
                Hand::Rock => 4,
                Hand::Paper => 1,
                Hand::Scissors => 7,
            },
            Hand::Paper => match other {
                Hand::Rock => 8,
                Hand::Paper => 5,
                Hand::Scissors => 2,
            },
            Hand::Scissors => match other {
                Hand::Rock => 3,
                Hand::Paper => 9,
                Hand::Scissors => 6,
            },
        }
    }

    fn get_winning_hand(hand: &Hand) -> Self{
        match hand {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    fn get_losing_hand(hand: &Hand) -> Self{
        match hand {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        match s {
            "X" | "A" => Hand::Rock,
            "Y" | "B" => Hand::Paper,
            "Z" | "C" => Hand::Scissors,
            _ => panic!("Invalid hand"),
        }
    }
}

fn main() {
    let input_string = input::get_input(2);

    let score: u32 = input_string.lines().fold(0, |score, line| {
        let hands = line.split_whitespace().map(Hand::from).collect::<Vec<Hand>>();
        assert!(hands.len() == 2, "Invalid match string");
        
        score + hands[1].get_score(&hands[0])
    });

    println!("Score: {}", score);

    // Part 2
    let score2 = input_string.lines().fold(0, |score, line| {
        let chars = line.split_whitespace().collect::<Vec<&str>>();
        assert!(chars.len() == 2, "Invalid match string");

        let hand1 = Hand::from(chars[0]);
        let hand2 = match chars[1] {
            "X" => Hand::get_losing_hand(&hand1),
            "Y" => hand1,
            "Z" => Hand::get_winning_hand(&hand1),
            _ => panic!("Invalid hand"),
        };

        score + hand2.get_score(&hand1)
    });

    println!("Score2: {}", score2);
}