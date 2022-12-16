use std::{
    cmp::Ordering,
    str::{Chars, FromStr},
};

const SAMPLE_INPUT: &str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

#[derive(Debug)]
pub struct ParseInputError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Item {
    Number(u32),
    List(Vec<Item>),
}

impl FromStr for Item {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_item(&mut s.chars()))
    }
}

impl Item {
    pub fn is_number(&self) -> bool {
        if let Item::Number(_) = self {
            true
        } else {
            false
        }
    }

    pub fn as_number(&self) -> u32 {
        let Item::Number(n) = self else {
            panic!();
        };
        *n
    }

    pub fn is_list(&self) -> bool {
        if let Item::List(_) = self {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct Pair {
    pub left: Item,
    pub right: Item,
}

pub fn parse_item(chars: &mut Chars) -> Item {
    let mut result = vec![];

    let mut number: Option<u32> = None;
    while let Some(char) = chars.next() {
        match char {
            '[' => {
                let new_item = parse_item(chars);
                result.push(new_item);
            }
            ']' => {
                break;
            }
            '0'..='9' => {
                let current = number.unwrap_or(0);
                let next = char.to_digit(10).unwrap();
                number = Some(current * 10 + next);
            }
            ',' => {
                if let Some(n) = number {
                    result.push(Item::Number(n));
                    number = None;
                }
            }
            _ => {}
        }
    }
    if let Some(n) = number {
        result.push(Item::Number(n))
    }
    Item::List(result)
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.is_number() && other.is_number() {
            return self.as_number().partial_cmp(&other.as_number());
        }

        let mut left_holder: Vec<Item> = vec![];
        let left = match self {
            Item::List(v) => v,
            Item::Number(n) => {
                left_holder = vec![Item::Number(*n)];
                &left_holder
            }
        };

        let mut right_holder: Vec<Item> = vec![];
        let right = match other {
            Item::List(v) => v,
            Item::Number(n) => {
                right_holder = vec![Item::Number(*n)];
                &right_holder
            }
        };

        for (left, right) in left.iter().zip(right.iter()) {
            if let Some(ordering) = left.partial_cmp(right) {
                match ordering {
                    Ordering::Greater | Ordering::Less => return Some(ordering),
                    _ => {}
                }
            }
        }

        left.len().partial_cmp(&right.len())
    }
}

fn main() {
    // let input = SAMPLE_INPUT;
    let input = include_str!("day13.txt");
    let mut pairs = vec![];

    let mut lines = input.lines();
    while let Some(mut line) = lines.next() {
        let left = line.parse::<Item>().unwrap();
        line = lines.next().unwrap();
        let right = line.parse::<Item>().unwrap();
        pairs.push((left, right));

        lines.next(); // We don't really care about this result. It's either an empty line of EOF.
    }

    let mut sum = 0;
    for (i, (left, right)) in pairs.iter().enumerate() {
        if let Some(Ordering::Less) = left.partial_cmp(&right) {
            sum += i + 1;
        }
    }

    println!("Part 1: {}", sum);

    let divider_1 = "[[2]]".parse::<Item>().unwrap();
    let divider_2 = "[[6]]".parse::<Item>().unwrap();

    let mut all_packets = vec![divider_1.clone(), divider_2.clone()];

    for (left, right) in pairs {
        all_packets.push(left);
        all_packets.push(right)
    }

    all_packets.sort();

    let mut divider_1_index = 0;
    let mut divider_2_index = 0;

    for (i, item) in all_packets.iter().enumerate() {
        if *item == divider_1 {
            divider_1_index = i + 1;
        }

        if *item == divider_2 {
            divider_2_index = i + 1;
        }
    }

    println!(
        "Part2 => Divider 1 = {}, Divider 2 = {}, Result = {}",
        divider_1_index,
        divider_2_index,
        divider_1_index * divider_2_index
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_item() {
        let item = "[1,1,3,1,1]".parse::<Item>().unwrap();
        assert!(item.is_list());
    }

    #[test]
    fn tests_comparisons() {
        let left = "[1,1,3,1,1]".parse::<Item>().unwrap();
        let right = "[1,1,5,1,1]".parse::<Item>().unwrap();
        assert_eq!(left.partial_cmp(&right), Some(Ordering::Less));

        let left = "[[1],[2,3,4]]".parse::<Item>().unwrap();
        let right = "[[1],4]".parse::<Item>().unwrap();
        assert_eq!(left.partial_cmp(&right), Some(Ordering::Less));

        let left = "[9]".parse::<Item>().unwrap();
        let right = "[[8,7,6]]".parse::<Item>().unwrap();
        assert_eq!(left.partial_cmp(&right), Some(Ordering::Greater));

        let left = "[[4,4],4,4]".parse::<Item>().unwrap();
        let right = "[[4,4],4,4,4]".parse::<Item>().unwrap();
        assert_eq!(left.partial_cmp(&right), Some(Ordering::Less));

        let left = "[7,7,7,7]".parse::<Item>().unwrap();
        let right = "[7,7,7]".parse::<Item>().unwrap();
        assert_eq!(left.partial_cmp(&right), Some(Ordering::Greater));

        let left = "[]".parse::<Item>().unwrap();
        let right = "[3]".parse::<Item>().unwrap();
        assert_eq!(left.partial_cmp(&right), Some(Ordering::Less));

        let left = "[[[]]]".parse::<Item>().unwrap();
        let right = "[[]]".parse::<Item>().unwrap();
        assert_eq!(left.partial_cmp(&right), Some(Ordering::Greater));

        let left = "[1,[2,[3,[4,[5,6,7]]]],8,9]".parse::<Item>().unwrap();
        let right = "[1,[2,[3,[4,[5,6,0]]]],8,9]".parse::<Item>().unwrap();
        assert_eq!(left.partial_cmp(&right), Some(Ordering::Greater));
    }
}
