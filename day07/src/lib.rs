use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Card(char);

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(other) {
            Ordering::Equal
        } else if self.0 == 'J' {
            Ordering::Less
        } else if other.0 == 'J' {
            Ordering::Greater
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
                ('F', _) => Ordering::Greater,
                (_, 'F') => Ordering::Less,
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

        let num_j = vec.iter().take_while(|c| c.0 == 'J').count();

        if Draw::is_five_of_a_kind(num_j, &vec) {
            HandType::FiveOfAKind
        } else if Draw::is_four_of_a_kind(num_j, &vec) {
            HandType::FourOfAKind
        } else if Draw::is_full_house(num_j, &vec) {
            HandType::FullHouse
        } else if Draw::is_three_of_a_kind(num_j, &vec) {
            HandType::ThreeOfAKind
        } else {
            let pairs = Draw::count_pairs(num_j, &vec);
            match pairs {
                2 => HandType::TwoPair,
                1 => HandType::OnePair,
                _ => HandType::HighCard,
            }
        }
    }

    fn is_five_of_a_kind(num_j: usize, cards: &[Card]) -> bool {
        for card in &cards[num_j..] {
            if card != &cards[4] {
                return false;
            }
        }

        true
    }

    fn is_four_of_a_kind(num_j: usize, cards: &[Card]) -> bool {
        debug_assert!(num_j <= 3);

        let window_size = 4 - num_j;
        cards[num_j..]
            .windows(window_size)
            .any(|win| win.iter().all(|c| c == &win[window_size - 1]))
    }

    fn is_full_house(num_j: usize, cards: &[Card]) -> bool {
        debug_assert!(num_j <= 2);

        if num_j == 2 {
            // in every situation where we can make a full house, we can also make a four of a kind
            false
        } else if num_j == 1 {
            // we need two pairs
            cards[1] == cards[2] && cards[3] == cards[4]
        } else {
            // 11222 or 22233
            (cards[0] == cards[2] && cards[3] == cards[4])
                || (cards[0] == cards[1] && cards[2] == cards[4])
        }
    }

    fn is_three_of_a_kind(num_j: usize, cards: &[Card]) -> bool {
        debug_assert!(num_j <= 2);

        if num_j == 2 {
            // we can always make a three of a kind
            true
        } else {
            let window_size = 3 - num_j;
            cards[num_j..]
                .windows(window_size)
                .any(|win| win.iter().all(|c| c == &win[window_size - 1]))
        }
    }

    fn count_pairs(num_j: usize, cards: &[Card]) -> usize {
        debug_assert!(num_j <= 1);

        // we can always make another pair with a J
        cards.windows(2).filter(|win| win[0] == win[1]).count() + num_j
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
    // replace all Jokers with a fake card that takes the role of card J in part1
    input.iter_mut().for_each(|draw| {
        draw.0.iter_mut().for_each(|card| {
            if card.0 == 'J' {
                card.0 = 'F'
            }
        })
    });
    input.sort();
    input
        .iter()
        .enumerate()
        .map(|(i, draw)| (i + 1) as u32 * draw.1)
        .sum()
}

pub fn part2(mut input: Vec<Draw>) -> u32 {
    input.sort();
    //dbg!(&input);
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
        println!("{:?}", part2(parsed));
    }
}
