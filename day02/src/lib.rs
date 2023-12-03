use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, u32 as parse_u32},
    combinator::{all_consuming, value},
    multi::separated_list0,
    sequence::{delimited, terminated},
    IResult,
};

#[derive(Debug)]
pub struct Game {
    id: u32,
    draws: Vec<Draw>,
}

#[derive(Debug)]
struct Draw {
    r: u32,
    g: u32,
    b: u32,
}

#[derive(Copy, Clone)]
enum DrawColor {
    Red,
    Green,
    Blue,
}

fn color(i: &str) -> IResult<&str, (u32, DrawColor)> {
    let (i, num) = terminated(parse_u32, tag(" "))(i)?;
    let (i, color) = alt((
        value(DrawColor::Red, tag("red")),
        value(DrawColor::Blue, tag("blue")),
        value(DrawColor::Green, tag("green")),
    ))(i)?;

    Ok((i, (num, color)))
}

fn draw(i: &str) -> IResult<&str, Draw> {
    let (i, colors) = separated_list0(tag(", "), color)(i)?;
    let draw = colors
        .iter()
        .fold(Draw { r: 0, g: 0, b: 0 }, |mut draw, (num, color)| {
            match color {
                DrawColor::Red => draw.r = *num,
                DrawColor::Green => draw.g = *num,
                DrawColor::Blue => draw.b = *num,
            }
            draw
        });

    Ok((i, draw))
}

fn game(i: &str) -> IResult<&str, Game> {
    let (i, id) = delimited(tag("Game "), parse_u32, tag(": "))(i)?;
    let (i, draws) = separated_list0(tag("; "), draw)(i)?;
    Ok((i, Game { id, draws }))
}

pub fn parse(input: &str) -> Vec<Game> {
    all_consuming(separated_list0(line_ending, game))(input.trim())
        .unwrap()
        .1
}

pub fn part1(input: &[Game]) -> u32 {
    let (max_r, max_g, max_b) = (12u32, 13u32, 14u32);

    input
        .iter()
        .filter(|game| {
            game.draws
                .iter()
                .all(|draw| draw.r <= max_r && draw.g <= max_g && draw.b <= max_b)
        })
        .map(|g| g.id)
        .sum()
}

pub fn part2(input: Vec<Game>) -> u32 {
    input
        .into_iter()
        .map(|game| {
            let res = game
                .draws
                .iter()
                .fold(Draw { r: 0, g: 0, b: 0 }, |mut acc, draw| {
                    acc.r = acc.r.max(draw.r);
                    acc.g = acc.g.max(draw.g);
                    acc.b = acc.b.max(draw.b);
                    acc
                });

            res.r * res.g * res.b
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run02() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(parsed));
    }
}
