pub fn part1(input: &str) -> u32 {
    let mut lines = input.trim().lines();
    let times = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|time| time.parse::<u32>().unwrap());
    let distances = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|dist| dist.parse::<u32>().unwrap());

    times
        .zip(distances)
        .map(|(time, best_dist)| {
            let mut faster = 0u32;
            for time_step in 0..time {
                let travel_distance = time_step * (time - time_step);
                if travel_distance > best_dist {
                    faster += 1;
                }
            }
            faster
        })
        .product()
}

pub fn part2(input: &str) -> u64 {
    let mut lines = input.trim().lines();
    let time = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();
    let best_distance = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();

    let mut faster = 0u64;
    for time_step in 0..time {
        let travel_distance = time_step * (time - time_step);
        if travel_distance > best_distance {
            faster += 1;
        }
    }
    faster
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run06() {
        let input = include_str!("./input.txt");
        println!("{:?}", part1(input));
        println!("{:?}", part2(input));
    }
}
