mod input;

use std::{
    cmp::{Ord, Ordering},
    str::Chars,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Entry {
    List(Vec<Entry>),
    Number(usize),
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Entry::List(packet_left), Entry::List(packet_right)) => {
                for (entry_left, entry_right) in packet_left.iter().zip(packet_right.iter()) {
                    match entry_left.cmp(entry_right) {
                        Ordering::Equal => continue,
                        order => return order,
                    }
                }
                packet_left.len().cmp(&packet_right.len())
            }
            (Entry::Number(n1), Entry::List(_)) => Self::List(vec![Self::Number(*n1)]).cmp(other),
            (Entry::List(_), Entry::Number(n2)) => self.cmp(&Self::List(vec![Self::Number(*n2)])),
            (Entry::Number(n1), Entry::Number(n2)) => n1.cmp(n2),
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
        packet_vec.push((
            get_packet_vector(&mut packet1.chars()),
            get_packet_vector(&mut packet2.chars()),
        ));
    }

    packet_vec
}

fn parse_all_packets(input: &str) -> Vec<Entry> {
    let lines = input.lines();
    lines
        .filter(|line| !line.is_empty())
        .map(|line| get_packet_vector(&mut line.chars()))
        .collect()
}

fn part1(packet_pairs: &[(Entry, Entry)]) -> usize {
    let orders: Vec<Ordering> = packet_pairs
        .iter()
        .map(|(packet_left, packet_right)| packet_left.cmp(packet_right))
        .collect();

    orders
        .iter()
        .enumerate()
        .fold(0, |acc, (i, order)| match order {
            Ordering::Less => acc + i + 1,
            Ordering::Equal => panic!(),
            _ => acc,
        })
}

fn part2(packets: &mut Vec<Entry>) -> usize {
    packets.push(Entry::List(vec![Entry::Number(6)]));
    packets.push(Entry::List(vec![Entry::Number(2)]));

    packets.sort();

    let i1 = packets
        .iter()
        .position(|packet| *packet == Entry::List(vec![Entry::Number(2)]))
        .unwrap()
        + 1;

    let i2 = packets
        .iter()
        .position(|packet| *packet == Entry::List(vec![Entry::Number(6)]))
        .unwrap()
        + 1;

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
