enum DigitChecker {
    Digit(char, u32),
    Word(&'static str, u32),
}

impl DigitChecker {
    fn leftmost_idx(&self, haystack: &str) -> i32 {
        match self {
            DigitChecker::Digit(c, _) => haystack.find(*c).map(|i| i as i32).unwrap_or(i32::MAX),
            DigitChecker::Word(word, _) => {
                haystack.find(word).map(|i| i as i32).unwrap_or(i32::MAX)
            }
        }
    }

    fn rightmost_index(&self, haystack: &str) -> i32 {
        match self {
            DigitChecker::Digit(c, _) => haystack.rfind(*c).map(|i| i as i32).unwrap_or(i32::MIN),
            DigitChecker::Word(word, _) => {
                haystack.rfind(word).map(|i| i as i32).unwrap_or(i32::MIN)
            }
        }
    }

    fn digit(&self) -> u32 {
        match self {
            DigitChecker::Digit(_, d) => *d,
            DigitChecker::Word(_, d) => *d,
        }
    }
}

fn parse_line(line: &str, checker: &[DigitChecker]) -> u32 {
    let x = checker
        .iter()
        .min_by_key(|c| c.leftmost_idx(line))
        .unwrap()
        .digit();
    let y = checker
        .iter()
        .max_by_key(|c| c.rightmost_index(line))
        .unwrap()
        .digit();

    x * 10 + y
}

pub fn part1(input: &str) -> u32 {
    let checker = vec![
        DigitChecker::Digit('1', 1),
        DigitChecker::Digit('2', 2),
        DigitChecker::Digit('3', 3),
        DigitChecker::Digit('4', 4),
        DigitChecker::Digit('5', 5),
        DigitChecker::Digit('6', 6),
        DigitChecker::Digit('7', 7),
        DigitChecker::Digit('8', 8),
        DigitChecker::Digit('9', 9),
    ];
    input
        .trim()
        .lines()
        .map(|line| parse_line(line, &checker))
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let checker = part2_checker();
    input
        .trim()
        .lines()
        .map(|line| parse_line(line, &checker))
        .sum()
}

fn part2_checker() -> Vec<DigitChecker> {
    let checker = vec![
        DigitChecker::Digit('1', 1),
        DigitChecker::Digit('2', 2),
        DigitChecker::Digit('3', 3),
        DigitChecker::Digit('4', 4),
        DigitChecker::Digit('5', 5),
        DigitChecker::Digit('6', 6),
        DigitChecker::Digit('7', 7),
        DigitChecker::Digit('8', 8),
        DigitChecker::Digit('9', 9),
        DigitChecker::Word("one", 1),
        DigitChecker::Word("two", 2),
        DigitChecker::Word("three", 3),
        DigitChecker::Word("four", 4),
        DigitChecker::Word("five", 5),
        DigitChecker::Word("six", 6),
        DigitChecker::Word("seven", 7),
        DigitChecker::Word("eight", 8),
        DigitChecker::Word("nine", 9),
    ];
    checker
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run01() {
        let input = include_str!("./input.txt");
        println!("part1: {:?}", part1(input));
        println!("part2: {:?}", part2(input));
    }

    #[test]
    fn test1() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        println!("part2 sample: {:?}", part2(input));
    }

    #[test]
    fn test2() {
        let checker = part2_checker();
        assert_eq!(11, parse_line("one", &checker))
    }
}
