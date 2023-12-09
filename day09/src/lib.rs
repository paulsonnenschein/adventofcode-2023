pub fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|part| part.parse().unwrap())
                .collect()
        })
        .collect()
}

fn calc_next(row: &[i32]) -> i32 {
    let mut pyramid = Vec::<Vec<i32>>::new();
    pyramid.push(row.to_vec());

    while pyramid.last().unwrap().iter().any(|el| el != &0) {
        let next_row = pyramid
            .last()
            .unwrap()
            .windows(2)
            .map(|win| win[1] - win[0])
            .collect();
        pyramid.push(next_row);
    }

    for i in (0..pyramid.len()).rev() {
        if i == (pyramid.len() - 1) {
            pyramid[i].push(0);
        } else {
            let sum = pyramid[i].last().unwrap() + pyramid[i + 1].last().unwrap();
            pyramid[i].push(sum);
        }
    }

    *pyramid[0].last().unwrap()
}

pub fn part1(input: &[Vec<i32>]) -> i32 {
    input.iter().map(|row| calc_next(row)).sum()
}

fn calc_prev(row: &[i32]) -> i32 {
    let mut pyramid = Vec::<Vec<i32>>::new();
    pyramid.push(row.to_vec());

    while pyramid.last().unwrap().iter().any(|el| el != &0) {
        let next_row = pyramid
            .last()
            .unwrap()
            .windows(2)
            .map(|win| win[1] - win[0])
            .collect();
        pyramid.push(next_row);
    }

    for i in (0..pyramid.len()).rev() {
        if i == (pyramid.len() - 1) {
            pyramid[i].insert(0, 0);
        } else {
            let sum = pyramid[i][0] - pyramid[i + 1][0];
            pyramid[i].insert(0, sum);
        }
    }

    pyramid[0][0]
}
pub fn part2(input: &[Vec<i32>]) -> i32 {
    input.iter().map(|row| calc_prev(row)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run09() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
