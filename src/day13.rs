mod input;

use core::panic;
use std::{str::Chars, cmp::{Ordering, PartialOrd, Ord}};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Entry {
    List(Vec<Entry>),
    Number(usize),
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        compare(self, other)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare(self, other))
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entry::List(list) => {
                let mut s = String::new();
                for entry in list {
                    s.push_str(&format!("{},", entry));
                }
                write!(f, "[{}]", s)
            },
            Entry::Number(n) => write!(f, "{}", n),
        }
    }
}

fn get_packet_vector(packet_chars: &mut Chars) -> Entry {
    let mut packet_vector = Vec::new();

    while let Some(mut char) = packet_chars.next() {
        let mut number = String::new();
        while char.is_ascii_digit() {
            number.push(char);
            if let Some(c) = packet_chars.next() {
                char = c;
            } else {
                break;
            }
        }
        if !number.is_empty() {
            packet_vector.push(Entry::Number(number.parse().unwrap()));
        }
        let entry = match char {
            '[' => get_packet_vector(packet_chars),
            ']' => return Entry::List(packet_vector),
            ',' => continue,
            _ => panic!(),
        };
        packet_vector.push(entry);
    }
    Entry::List(packet_vector)
}

fn parse_packet_pairs(input: &str) -> Vec<(Entry, Entry)> {
    let packet_pairs = input.split("\n\n");
    let mut packet_vec = Vec::new();

    for packet_pair in packet_pairs {
        let mut packets = packet_pair.split('\n');
        let packet1 = packets.next().unwrap();
        let packet2 = packets.next().unwrap();
        packet_vec.push((get_packet_vector(&mut packet1.chars()), get_packet_vector(&mut packet2.chars())));
    }

    packet_vec
}

fn parse_all_packets(input: &str) -> Vec<Entry> {
    let lines = input.lines();
    lines.filter(|line| !line.is_empty()).map(|line| get_packet_vector(&mut line.chars())).collect()
}

fn compare(packet_left: &Entry, packet_right: &Entry) -> Ordering {
    if let (Entry::List(packet_left), Entry::List(packet_right)) = (packet_left, packet_right) {
        for (entry_left, entry_right) in packet_left.iter().zip(packet_right.iter()) {
            match (entry_left, entry_right) {
                (Entry::Number(n1), Entry::Number(n2)) => {
                    match n1.cmp(n2) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                },
                (v1, v2) if matches!(v1, Entry::List(_)) && matches!(v2, Entry::List(_)) => match compare(v1, v2) {
                    Ordering::Equal => continue,
                    other => return other,
                },
                (v1, Entry::Number(n2)) if matches!(v1, Entry::List(_)) => match compare(v1, &Entry::List(vec![Entry::Number(*n2)])) {
                    Ordering::Equal => continue,
                    other => return other,
                },
                (Entry::Number(n1), v2) if matches!(v2, Entry::List(_)) => match compare(&Entry::List(vec![Entry::Number(*n1)]), v2) {
                    Ordering::Equal => continue,
                    other => return other,
                },
                _ => panic!(),
            }
        }
        match packet_left.len().cmp(&packet_right.len()) {
            Ordering::Equal => return Ordering::Equal,
            other => return other,
        }
    }
    panic!();
}

fn part1(packet_pairs: &[(Entry, Entry)]) -> usize {
    let orders: Vec<Ordering> = packet_pairs.iter().map(|(packet_left, packet_right)| {
        compare(packet_left, packet_right)
    }).collect();

    orders.iter().enumerate().fold(0, |acc, (i, order)| {
        match order {
            Ordering::Less => acc + i+1,
            Ordering::Equal => panic!(),
            _ => acc,
        }
    })
}

fn part2(packets: &mut Vec<Entry>) -> usize {
    packets.push(Entry::List(vec![Entry::Number(6)]));
    packets.push(Entry::List(vec![Entry::Number(2)]));

    packets.sort();

    let i1 = packets.iter().position(|packet| {
        *packet == Entry::List(vec![Entry::Number(2)])
    }).unwrap() + 1;

    let i2 = packets.iter().position(|packet| {
        *packet == Entry::List(vec![Entry::Number(6)])
    }).unwrap() + 1;

    i1 * i2
}

fn main() {
    let inp = input::get_input(13);
    let packets = parse_packet_pairs(&inp);
    println!("Part 1: {}", part1(&packets));
    let mut packets = parse_all_packets(&inp);
    println!("Part 2: {}", part2(&mut packets));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../test_input/day13.txt");

    #[test]
    fn test_part1() {
        let packets = parse_packet_pairs(INPUT);
        assert_eq!(part1(&packets), 13);
    }

    #[test]
    fn test_part2() {
        let mut packets = parse_all_packets(INPUT);
        assert_eq!(part2(&mut packets), 140);
    }
}
