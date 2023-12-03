pub fn parse(input: &str) -> Vec<Vec<char>> {
    input.trim().lines().map(|line| line.chars().collect()).collect()
}

pub fn part1(input: &[Vec<char>]) -> u32 {
    let mut sum = 0u32;

    for (row_idx, row) in input.iter().enumerate() {
        let mut num_acc = 0u32;
        let mut len_acc = 0u32;
        for (col_idx, c) in row.iter().enumerate() {
            if c.is_ascii_digit() {
                let digit = c.to_digit(10).unwrap();
                num_acc = num_acc * 10 + digit;
                len_acc += 1;
            } else {
                if num_acc > 0 {
                    if touches_part(input, row_idx, col_idx, len_acc) {
                        sum += num_acc;
                    }

                    num_acc = 0;
                    len_acc = 0;
                }
            }
        }
        if num_acc > 0 {
            if touches_part(input, row_idx, row.len(), len_acc) {
                sum += num_acc;
            }
        }
    }

    sum
}


fn check_idx(input: &[Vec<char>], row: usize, col: usize) -> bool {
    let c = input.get(row).and_then(|row| row.get(col)).unwrap_or(&'.');
    !c.is_ascii_digit() && c != &'.'
}

fn touches_part(input: &[Vec<char>], row: usize, col: usize, num_len: u32) -> bool {
    let row = row as isize;
    let col = col as isize;
    let num_len = num_len as isize;
    let top_left = (row - 1, col - (num_len + 1));
    let bottom_right = (row + 1, col);

    // check top
    for col_idx in top_left.1 ..= bottom_right.1 {
        if check_idx(input, top_left.0 as usize, col_idx as usize) {
            return true
        }
    }

    // check left
    if check_idx(input, row as usize, top_left.1 as usize) {
        return true
    }

    // check right
    if check_idx(input, row as usize, col as usize) {
        return true
    }

    // check bottom
    for col_idx in top_left.1 ..= bottom_right.1 {
        if check_idx(input, bottom_right.0 as usize, col_idx as usize) {
            return true
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run03() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        //println!("{:?}", part2(parsed));
    }

    #[test]
    fn test_sample() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        //println!("{:?}", part2(parsed));
    }
}
