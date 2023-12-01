use std::cmp::min;

advent_of_code::solution!(1);

const DIGITS_TO_N: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn get_num_at_pos(input: &str, pos: usize, parse: bool) -> Option<u32> {
    if let Some(d) = input.chars().nth(pos).unwrap_or(' ').to_digit(10) {
        return Some(d)
    }

    if !parse {
        return None
    }

    for (haystack, digit) in DIGITS_TO_N {
        let end = min(pos + haystack.len(), input.len());
        if &input[pos..end] == haystack {
            return Some(digit)
        }
    }

    return None
}

fn get_sum(input: &str, parse: bool) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = None;
        for idx in 0..line.len() {
            if let Some(d) =  get_num_at_pos(line, idx, parse) {
                if first_digit.is_none() {
                    first_digit = Some(d);
                }
                last_digit = Some(d);
            }
        };

        match (first_digit, last_digit) {
            (Some(l), Some(r)) => sum += l * 10 + r,
            (Some(l), None) => sum += l,
            (None, Some(_)) => (),
            (None, None) => ()
        }
    }
    return sum;
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(get_sum(input, false))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(get_sum(input, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
