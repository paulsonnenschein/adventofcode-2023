use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1, u32 as parse_u32},
    combinator::all_consuming,
    multi::separated_list0,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
pub struct Card {
    _id: u32,
    winning: Vec<u32>,
    yours: Vec<u32>,
}

fn number_list(i: &str) -> IResult<&str, Vec<u32>> {
    separated_list0(space1, parse_u32)(i.trim())
}

fn card(i: &str) -> IResult<&str, Card> {
    let (i, (_, _, id, _, _)) = tuple((tag("Card"), space1, parse_u32, tag(":"), space1))(i)?;
    let (i, (winning, yours)) = separated_pair(number_list, tag(" | "), number_list)(i)?;

    Ok((
        i,
        Card {
            _id: id,
            winning,
            yours,
        },
    ))
}

pub fn parse(input: &str) -> Vec<Card> {
    all_consuming(separated_list0(line_ending, card))(input.trim())
        .unwrap()
        .1
}

pub fn part1(input: &[Card]) -> u32 {
    input
        .iter()
        .map(|card| {
            card.yours
                .iter()
                .filter(|my_number| card.winning.contains(my_number))
                .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
        })
        .sum()
}

pub fn part2(input: Vec<Card>) -> u32 {
    let mut num_cards = vec![1u32; input.len()];

    for (idx, card) in input.into_iter().enumerate() {
        let matches = card
            .yours
            .iter()
            .filter(|my_number| card.winning.contains(my_number))
            .count();

        let inc_amount = num_cards[idx];
        for item in num_cards.iter_mut().skip(idx + 1).take(matches) {
            *item += inc_amount;
        }
    }

    num_cards.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run04() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(parsed));
    }
}
