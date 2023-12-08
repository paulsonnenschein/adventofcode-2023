use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{complete::alphanumeric1, complete::line_ending},
    combinator::{all_consuming, map, value},
    multi::{many1, separated_list0},
    sequence::tuple,
    IResult,
};

#[derive(Copy, Clone, Debug)]
pub enum LR {
    L,
    R,
}

#[derive(Debug)]
pub struct Input {
    dirs: Vec<LR>,
    mappings: HashMap<&'static str, (&'static str, &'static str)>,
}

fn parse_input(i: &'static str) -> IResult<&str, Input> {
    let (i, dirs) = many1(alt((value(LR::L, tag("L")), value(LR::R, tag("R")))))(i)?;
    let (i, _) = tag("\n\n")(i)?;
    let (i, mappings) = separated_list0(
        line_ending,
        map(
            tuple((
                alphanumeric1,
                tag(" = ("),
                alphanumeric1,
                tag(", "),
                alphanumeric1,
                tag(")"),
            )),
            |(l, _, m, _, r, _)| (l, (m, r)),
        ),
    )(i)?;

    Ok((
        i,
        Input {
            dirs,
            mappings: mappings.into_iter().collect(),
        },
    ))
}

pub fn parse(input: &'static str) -> Input {
    all_consuming(parse_input)(input.trim()).unwrap().1
}

pub fn part1(input: &Input) -> u64 {
    get_cycle_length(input, &"AAA")
}

pub fn part2(input: Input) -> u64 {
    input
        .mappings
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|start| get_cycle_length(&input, start))
        .fold(1u64, num::integer::lcm)
}

fn get_cycle_length(input: &Input, start: &&'static str) -> u64 {
    let mut dirs = input.dirs.iter().cycle();
    let mut current = start;
    let mut steps = 0u64;

    while !current.ends_with('Z') {
        let mappings = &input.mappings[current];
        current = match dirs.next().unwrap() {
            LR::L => &mappings.0,
            LR::R => &mappings.1,
        };
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run08() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(parsed));
    }
}
