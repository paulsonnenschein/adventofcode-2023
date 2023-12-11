use itertools::Itertools;

pub fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_galaxies(input: &[Vec<char>]) -> Vec<(isize, isize)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| c == &&'#')
                .map(move |(col_idx, _)| (row_idx as isize, col_idx as isize))
        })
        .collect()
}

fn expand(galaxies: &mut [(isize, isize)], factor: isize) {
    // expand rows
    galaxies.sort_by_key(|g| g.0); // sort by col
    let mut last_galaxy_row = -1;
    let mut current_expansion = 0;
    for gal in galaxies.iter_mut() {
        if gal.0 > last_galaxy_row + 1 {
            current_expansion += gal.0 - (last_galaxy_row + 1);
        }
        last_galaxy_row = gal.0; // store val before expansion
        gal.0 += current_expansion * (factor - 1);
    }

    // expand cols
    galaxies.sort_by_key(|g| g.1); // sort by col
    let mut last_galaxy_col = -1;
    let mut current_expansion = 0;
    for gal in galaxies.iter_mut() {
        if gal.1 > last_galaxy_col + 1 {
            current_expansion += gal.1 - (last_galaxy_col + 1);
        }
        last_galaxy_col = gal.1; // store val before expansion
        gal.1 += current_expansion * (factor - 1);
    }
}

pub fn part1(input: &[Vec<char>]) -> usize {
    let mut galaxies = find_galaxies(input);

    expand(&mut galaxies, 2);

    galaxies
        .iter()
        .tuple_combinations()
        .map(|(g1, g2)| g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1))
        .sum()
}

pub fn part2(input: &[Vec<char>]) -> usize {
    let mut galaxies = find_galaxies(input);

    expand(&mut galaxies, 1000000);

    galaxies
        .iter()
        .tuple_combinations()
        .map(|(g1, g2)| g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run11() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
    }
}
