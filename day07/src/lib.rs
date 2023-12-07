use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Card(char);

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(other) {
            Ordering::Equal
        } else if self.0.is_ascii_digit() {
            if other.0.is_ascii_digit() {
                self.0.cmp(&other.0)
            } else {
                Ordering::Less
            }
        } else if other.0.is_ascii_digit() {
            Ordering::Greater
        } else {
            match (self.0, other.0) {
                ('A', _) => Ordering::Greater,
                (_, 'A') => Ordering::Less,
                ('K', _) => Ordering::Greater,
                (_, 'K') => Ordering::Less,
                ('Q', _) => Ordering::Greater,
                (_, 'Q') => Ordering::Less,
                ('J', _) => Ordering::Greater,
                (_, 'J') => Ordering::Less,
                _ => unreachable!("cant get here? {} {}", self.0, other.0),
            }
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Draw([Card; 5], u32);

impl Draw {
    fn hand_type(&self) -> HandType {
        let mut vec = self.0.to_vec();
        vec.sort();

        if vec[0] == vec[4] {
            HandType::FiveOfAKind
        } else if vec[0] == vec[3] || vec[1] == vec[4] {
            HandType::FourOfAKind
        } else if (vec[0] == vec[2] && vec[3] == vec[4]) || (vec[0] == vec[1] && vec[2] == vec[4]) {
            HandType::FullHouse
        } else if vec
            .windows(3)
            .any(|win| win[0] == win[1] && win[0] == win[2])
        {
            HandType::ThreeOfAKind
        } else {
            let pairs = vec.windows(2).filter(|win| win[0] == win[1]).count();
            match pairs {
                2 => HandType::TwoPair,
                1 => HandType::OnePair,
                _ => HandType::HighCard,
            }
        }
    }
}

impl Ord for Draw {
    fn cmp(&self, other: &Self) -> Ordering {
        if other.eq(self) {
            Ordering::Equal
        } else {
            if self.0 == other.0 {
                unreachable!("two draws with same cards?")
            }
            let self_hand = self.hand_type();
            let other_hand = other.hand_type();

            match self_hand.cmp(&other_hand) {
                ord @ (Ordering::Less | Ordering::Greater) => ord,
                Ordering::Equal => self
                    .0
                    .iter()
                    .zip(other.0.iter())
                    .map(|(card_self, card_other)| card_self.cmp(card_other))
                    .find(|o| o != &Ordering::Equal)
                    .expect("Hands cant be equal!"),
            }
        }
    }
}

impl PartialOrd for Draw {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Debug for Draw {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}{:?}{:?}{:?}{:?}: {} ({:?})",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.1,
            self.hand_type()
        )
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn parse(input: &str) -> Vec<Draw> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (draw_str, bid_str) = line.split_once(' ').unwrap();

            let mut c = draw_str.chars();

            let card = [
                Card(c.next().unwrap()),
                Card(c.next().unwrap()),
                Card(c.next().unwrap()),
                Card(c.next().unwrap()),
                Card(c.next().unwrap()),
            ];

            Draw(card, bid_str.parse().unwrap())
        })
        .collect()
}

pub fn part1(input: &[Draw]) -> u32 {
    let mut input = input.to_vec();
    input.sort();
    input
        .iter()
        .enumerate()
        .map(|(i, draw)| (i + 1) as u32 * draw.1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run07() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        //println!("{:?}", part2(parsed));
    }
}
