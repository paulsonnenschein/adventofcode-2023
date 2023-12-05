use std::ops::Range;

use nom::{
    bytes::complete::{tag, take_until1},
    character::complete::{i64 as parse_i64, line_ending},
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
pub struct Input {
    seeds: Vec<i64>,
    layers: Vec<Layer>,
}

#[derive(Debug)]
pub struct Layer {
    descriptions: Vec<(i64, i64, i64)>,
    // based on input
    mappings: Vec<(Range<i64>, i64)>, // filled in later
}

fn parse_layer(i: &str) -> IResult<&str, Layer> {
    let (i, _) = tuple((take_until1(" "), tag(" map:\n")))(i)?;

    let (i, descriptions) = separated_list0(
        line_ending,
        map(
            tuple((parse_i64, tag(" "), parse_i64, tag(" "), parse_i64)),
            |(l, _, m, _, r)| (l, m, r),
        ),
    )(i)?;

    Ok((
        i,
        Layer {
            descriptions,
            mappings: Vec::new(),
        },
    ))
}

fn parse_input(i: &str) -> IResult<&str, Input> {
    let (i, seeds) = preceded(tag("seeds: "), separated_list0(tag(" "), parse_i64))(i)?;
    let (i, layers) = separated_list0(tag("\n\n"), parse_layer)(i.trim())?;

    Ok((i, Input { seeds, layers }))
}

pub fn parse(input: &str) -> Input {
    all_consuming(parse_input)(input.trim()).unwrap().1
}

pub fn build_mappings(input: &mut Input) {
    for layer in input.layers.iter_mut() {
        for (dst_start, src_start, src_len) in &layer.descriptions {
            layer
                .mappings
                .push((*src_start..(src_start + src_len), *dst_start - *src_start))
        }
        layer.mappings.sort_by_key(|l| l.0.start)
    }
}
pub fn part1(input: &Input) -> i64 {
    input
        .seeds
        .iter()
        .map(|seed| find_location_with_range(input, *seed).0)
        .min()
        .unwrap()
}

fn find_location_with_range(input: &Input, seed_nr: i64) -> (i64, i64) {
    let mut current = seed_nr;
    let mut end_of_valid_range = i64::MAX;

    'layers: for layer in &input.layers {
        let mut last_range_end = 0i64;
        for (range, op) in &layer.mappings {
            if (last_range_end..range.start).contains(&current) {
                // current stays unchanged
                end_of_valid_range = end_of_valid_range.min(seed_nr + (range.start - current));
                continue 'layers;
            }
            if range.contains(&current) {
                end_of_valid_range = end_of_valid_range.min(seed_nr + (range.end - current));
                current += op;
                continue 'layers;
            }

            last_range_end = range.end;
        }

        // current and end_of_valid_range stay unchanged
    }

    (current, end_of_valid_range)
}

pub fn part2(input: Input) -> i64 {
    let mut min_position = i64::MAX;
    for seed_instruction in input.seeds.chunks_exact(2) {
        let mut seed = seed_instruction[0];
        let length = seed_instruction[1];
        let end = seed + length; // exclusive

        while seed < end {
            let (loc, end_valid) = find_location_with_range(&input, seed);
            min_position = min_position.min(loc);
            seed = end_valid;
        }
    }

    min_position
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run05() {
        let input = include_str!("./input.txt");
        let mut parsed = parse(input);
        build_mappings(&mut parsed);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(parsed));
    }
}
