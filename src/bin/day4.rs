// Solution for https://adventofcode.com/2022/day/4.

use std::{str::FromStr, str::Split};

#[derive(Debug)]
pub struct InputParseError;

#[derive(Debug)]
pub struct Range {
    start: u8,
    end: u8,
}

impl Range {
    pub fn new(start: u8, end: u8) -> Self {
        Range { start, end }
    }

    pub fn contains(&self, other: &Range) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        (self.start <= other.end && self.start >= other.start)
            || (other.start <= self.end && other.start >= self.start)
    }
}

impl FromStr for Range {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.trim().split("-");

        fn next_u8(input: &mut Split<&str>) -> Result<u8, InputParseError> {
            let Some(value) = input.next() else {
                return Err(InputParseError);  
            };
            Ok(value.parse::<u8>().map_err(|_err| InputParseError)?)
        }

        Ok(Self::new(next_u8(&mut input)?, next_u8(&mut input)?))
    }
}

fn main() {
    let input = include_str!("day4.txt");
    let pairs = parse_input(input).unwrap();
    println!("{}", count_contained_pairs(&pairs));
    println!("{}", count_overlapping_pairs(&pairs));
}

pub fn parse_input(input: &str) -> Result<Vec<(Range, Range)>, InputParseError> {
    let mut pairs = vec![];
    for line in input.lines() {
        let ranges = line.trim().split(",").collect::<Vec<&str>>();
        if ranges.len() < 2 {
            return Err(InputParseError);
        }

        let pair = (ranges[0].parse::<Range>()?, ranges[1].parse::<Range>()?);
        pairs.push(pair);
    }
    Ok(pairs)
}

pub fn count_contained_pairs(pairs: &Vec<(Range, Range)>) -> u32 {
    pairs
        .into_iter()
        .filter(|(a, b)| a.contains(&b) || b.contains(&a))
        .count() as u32
}

pub fn count_overlapping_pairs(pairs: &Vec<(Range, Range)>) -> u32 {
    pairs.into_iter().filter(|(a, b)| a.overlaps(&b)).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";

    #[test]
    fn part1_test() {
        let pair_list = parse_input(TEST_INPUT).unwrap();
        let result = count_contained_pairs(&pair_list);
        assert_eq!(result, 2);
    }

    #[test]
    fn counts_contained_pairs() {
        let pair_list = vec![
            (Range::new(2, 4), Range::new(6, 8)),
            (Range::new(2, 3), Range::new(4, 5)),
            (Range::new(5, 7), Range::new(7, 9)),
            (Range::new(2, 8), Range::new(3, 7)),
            (Range::new(6, 6), Range::new(4, 6)),
            (Range::new(2, 6), Range::new(4, 8)),
        ];
        let result = count_contained_pairs(&pair_list);
        assert_eq!(result, 2);
    }

    #[test]
    fn counts_overlapping_pairs() {
        let pair_list = vec![
            (Range::new(2, 4), Range::new(6, 8)),
            (Range::new(2, 3), Range::new(4, 5)),
            (Range::new(5, 7), Range::new(7, 9)),
            (Range::new(2, 8), Range::new(3, 7)),
            (Range::new(6, 6), Range::new(4, 6)),
            (Range::new(2, 6), Range::new(4, 8)),
        ];
        let result = count_overlapping_pairs(&pair_list);
        assert_eq!(result, 4);
    }

    #[test]
    fn range_contains() {
        assert!(Range::new(2, 3).contains(&Range::new(2, 2)));
    }

    #[test]
    fn range_overlaps() {
        assert!(Range::new(2, 3).contains(&Range::new(2, 2)));
    }
}
