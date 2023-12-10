use std::collections::HashSet;

pub fn parse(input: &str) -> Vec<Vec<char>> {
    input.trim().lines().map(|l| l.chars().collect()).collect()
}

#[derive(Copy, Clone)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn apply(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Dir::Up => (pos.0 - 1, pos.1),
            Dir::Right => (pos.0, pos.1 + 1),
            Dir::Down => (pos.0 + 1, pos.1),
            Dir::Left => (pos.0, pos.1 - 1),
        }
    }
}

fn find_start(input: &[Vec<char>]) -> (usize, usize) {
    input
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .position(|char| char == &'S')
                .map(|col| (row_idx, col))
        })
        .next()
        .unwrap()
}

pub fn part1(input: &[Vec<char>], start_dir: Dir) -> u32 {
    // find S
    let start_pos = find_start(input);

    let mut current = (start_dir.apply(start_pos), start_dir);
    let mut steps = 1u32;

    while input[current.0 .0][current.0 .1] != 'S' {
        current = next_pos(input, current.0, current.1);
        steps += 1;
    }

    steps / 2
}

fn next_pos(
    input: &[Vec<char>],
    current_pos: (usize, usize),
    last_direction: Dir,
) -> ((usize, usize), Dir) {
    let dir = match input[current_pos.0][current_pos.1] {
        '|' => match last_direction {
            Dir::Up => Dir::Up,
            Dir::Down => Dir::Down,
            _ => unreachable!(),
        },
        '-' => match last_direction {
            Dir::Left => Dir::Left,
            Dir::Right => Dir::Right,
            _ => unreachable!(),
        },
        'L' => match last_direction {
            Dir::Left => Dir::Up,
            Dir::Down => Dir::Right,
            _ => unreachable!(),
        },
        'J' => match last_direction {
            Dir::Down => Dir::Left,
            Dir::Right => Dir::Up,
            _ => unreachable!(),
        },
        '7' => match last_direction {
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Down,
            _ => unreachable!(),
        },
        'F' => match last_direction {
            Dir::Left => Dir::Down,
            Dir::Up => Dir::Right,
            _ => unreachable!(),
        },
        '.' => unreachable!("ground"),
        'S' => unreachable!("start"),
        _ => unreachable!("invalid tile"),
    };
    (dir.apply(current_pos), dir)
}

pub fn part2(input: &[Vec<char>], start_dir: Dir, start_char: char) -> u32 {
    // find S
    let start_pos = find_start(input);

    // find all tiles belonging to the path
    let mut current = (start_dir.apply(start_pos), start_dir);
    let mut path = HashSet::new();
    path.insert(start_pos);
    path.insert(current.0);

    while input[current.0 .0][current.0 .1] != 'S' {
        current = next_pos(input, current.0, current.1);
        path.insert(current.0);
    }

    // find all tiles inside in path
    let mut inside_tiles = 0;
    for (row_idx, row) in input.iter().enumerate() {
        let mut inside = false; // on horizontal line: the side below
        let mut inside_start_col = 0;

        for (col_idx, c) in row.iter().enumerate() {
            let c = if c == &'S' { start_char } else { *c };
            let on_path = path.contains(&(row_idx, col_idx));
            if on_path {
                match c {
                    '|' => {
                        if inside {
                            inside_tiles += col_idx - inside_start_col;
                        }
                        inside = !inside;
                        if inside {
                            inside_start_col = col_idx + 1;
                        }
                    }
                    '-' => {}
                    'F' => {
                        if inside {
                            inside_tiles += col_idx - inside_start_col;
                        }
                        inside = !inside;
                    }
                    'L' => {
                        if inside {
                            inside_tiles += col_idx - inside_start_col;
                        }
                    }
                    '7' => {
                        inside = !inside;
                        if inside {
                            inside_start_col = col_idx + 1;
                        }
                    }
                    'J' => {
                        if inside {
                            inside_start_col = col_idx + 1;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    inside_tiles as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run010() {
        let input = include_str!("./input.txt");
        let dir = Dir::Up;
        let start_char = 'L';
        let parsed = parse(input);
        println!("{:?}", part1(&parsed, dir));
        println!("{:?}", part2(&parsed, dir, start_char));
    }
}
