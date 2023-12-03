use std::ops::RangeInclusive;

pub struct Input {
    numbers: Vec<Vec<Number>>,
    symbols: Vec<Vec<Symbol>>,
}

struct Number {
    cols: RangeInclusive<isize>, // including 1 col before and after
    value: u32,
}

struct Symbol {
    col: isize,
    symbol: char,
}

pub fn parse(input: &str) -> Input {
    let mut numbers = vec![];
    let mut symbols = vec![];
    for line in input.trim().lines() {
        let mut line_numbers = vec![];
        let mut line_symbols = vec![];

        let mut num_acc = 0u32;
        let mut len_acc = 0isize;

        for (col, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                let digit = c.to_digit(10).unwrap();
                num_acc = num_acc * 10 + digit;
                len_acc += 1;
            } else {
                if num_acc > 0 {
                    line_numbers.push(Number {
                        cols: (col as isize - (len_acc + 1))..=col as isize,
                        value: num_acc,
                    });

                    num_acc = 0;
                    len_acc = 0;
                }
                if c != '.' {
                    line_symbols.push(Symbol {
                        col: col as isize,
                        symbol: c,
                    })
                }
            }
        }

        if num_acc > 0 {
            let col = line.len();
            line_numbers.push(Number {
                cols: (col as isize - (len_acc + 1))..=col as isize,
                value: num_acc,
            });
        }

        numbers.push(line_numbers);
        symbols.push(line_symbols);
    }

    Input { numbers, symbols }
}

pub fn part1(input: &Input) -> u32 {
    let mut sum = 0u32;

    for (row_idx, row) in input.numbers.iter().enumerate() {
        let empty = vec![];
        let symbols_above = if row_idx > 0 {
            &input.symbols[row_idx - 1]
        } else {
            &empty
        };
        let symbols = &input.symbols[row_idx];
        let symbols_below = input.symbols.get(row_idx + 1).unwrap_or(&empty);

        for num in row {
            if symbols_above.iter().any(|s| num.cols.contains(&s.col))
                || symbols.iter().any(|s| num.cols.contains(&s.col))
                || symbols_below.iter().any(|s| num.cols.contains(&s.col))
            {
                sum += num.value;
            }
        }
    }

    sum
}

pub fn part2(input: &Input) -> u32 {
    let mut sum = 0u32;

    for (row_idx, row) in input.symbols.iter().enumerate() {
        for symbol in row.iter().filter(|sym| sym.symbol == '*') {
            let empty = vec![];
            let numbers_above = if row_idx > 0 {
                &input.numbers[row_idx - 1]
            } else {
                &empty
            };
            let numbers = &input.numbers[row_idx];
            let numbers_below = input.numbers.get(row_idx + 1).unwrap_or(&empty);

            let matching_numbers = numbers_above
                .iter()
                .filter(|num| num.cols.contains(&symbol.col))
                .chain(numbers.iter().filter(|num| num.cols.contains(&symbol.col)))
                .chain(
                    numbers_below
                        .iter()
                        .filter(|num| num.cols.contains(&symbol.col)),
                )
                .collect::<Vec<_>>();

            if matching_numbers.len() == 2 {
                sum += matching_numbers[0].value * matching_numbers[1].value;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run03() {
        let input = include_str!("./input.txt");
        let parsed = parse(input);
        println!("{:?}", part1(&parsed));
        println!("{:?}", part2(&parsed));
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
