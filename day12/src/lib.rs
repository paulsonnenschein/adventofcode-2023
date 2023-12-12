#[derive(Debug)]
pub struct Row {
    fields: Vec<char>,
    desc: Vec<usize>,
}

pub fn parse(input: &str) -> Vec<Row> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (fields, desc) = line.split_once(' ').unwrap();

            Row {
                fields: fields.chars().collect(),
                desc: desc.split(',').map(|r| r.parse().unwrap()).collect(),
            }
        })
        .collect()
}

fn find_combinations(row: &Row) -> u64 {
    let mut dp = vec![vec![None; row.desc.len() + 1]; row.fields.len() + 1];
    place_groups(&row.fields, &row.desc, &mut dp)
}

fn place_groups(fields: &[char], desc: &[usize], dp: &mut Vec<Vec<Option<u64>>>) -> u64 {
    if desc.is_empty() {
        return if fields.iter().all(|c| c == &'?' || c == &'.') {
            1
        } else {
            0
        };
    }
    if let Some(x) = dp[fields.len()][desc.len()] {
        return x;
    }

    let min_required_space = desc.iter().copied().sum::<usize>() + desc.len() - 1; // space of each group + one between

    let mut total = 0;
    for i in 0..=(fields.len() - min_required_space) {
        if can_place_group(&fields[i..], desc[0]) {
            total += if fields.len() > i + desc[0] {
                place_groups(&fields[(i + desc[0] + 1)..], &desc[1..], dp)
            } else {
                1
            };
        }
        if fields[i] == '#' {
            dp[fields.len()][desc.len()] = Some(total);
            return total; // if we are at a #, we must be able to place current group
        }
    }

    dp[fields.len()][desc.len()] = Some(total);

    total
}

fn can_place_group(fields: &[char], group: usize) -> bool {
    let can_place =
        fields.len() >= group && fields[0..group].iter().all(|c| c == &'?' || c == &'#');
    if can_place {
        if fields.len() > group {
            fields[group] == '.' || fields[group] == '?'
        } else {
            true
        }
    } else {
        false
    }
}

pub fn part1(input: &[Row]) -> u64 {
    input.iter().map(find_combinations).sum()
}

pub fn part2(mut input: Vec<Row>, repeat: usize) -> u64 {
    // unfold
    for row in input.iter_mut() {
        let mut new_fields = row.fields.clone();
        for _ in 2..=repeat {
            new_fields.push('?');
            new_fields.append(&mut row.fields.clone());
        }
        row.fields = new_fields;
        row.desc = row.desc.repeat(repeat);
    }

    input.iter().map(find_combinations).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run12() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(parsed, 5));
    }
}
