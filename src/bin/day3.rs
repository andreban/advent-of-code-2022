use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug)]
pub struct InputParseError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Item(char);

impl Item {
    pub fn value(&self) -> u32 {
        match self {
            Item(c) if c >= &'a' && c <= &'z' => *c as u32 - 96,
            Item(c) if c >= &'A' && c <= &'Z' => *c as u32 - 38,
            _ => 0,
        }
    }
}

pub struct Backpack {
    pub items: Vec<Item>,
}

impl Backpack {
    pub fn find_common_item_between_pockets(&self) -> Option<Item> {
        let half_len = self.items.len() / 2;
        let left_pocket = &self.items[0..half_len];
        let right_pocket = &self.items[half_len..self.items.len()];

        // Using the `find_common_items` generic function.
        find_common_items(&[left_pocket, right_pocket])
            .iter()
            .map(|i| *i)
            .next()

        // // Using brute force :).
        // for left_item in left_pocket {
        //     for right_item in right_pocket {
        //         if left_item == right_item {
        //             return Some(*left_item);
        //         }
        //     }
        // }
        // None
    }
}

impl FromStr for Backpack {
    type Err = InputParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let items = input
            .trim()
            .split("")
            .filter(|s| !s.is_empty() && s.is_ascii())
            .map(|s| Item(s.chars().next().unwrap()))
            .collect::<Vec<Item>>();

        Ok(Self { items })
    }
}

pub fn compute_sum_of_badges_priorities(backpacks: &[Backpack]) -> u32 {
    let mut sum = 0;
    for i in (0..backpacks.len()).step_by(3) {
        let item_lists: Vec<&[Item]> = vec![
            &backpacks[i].items,
            &backpacks[i + 1].items,
            &backpacks[i + 2].items,
        ];

        // Using the `find_common_items` generic function.
        if let Some(common_item) = find_common_items(&item_lists).iter().next() {
            sum += common_item.value();
        }

        // // Using brute force via `find_common_item_between_backpacks`.
        // if let Some(common_item) =
        //     find_common_item_between_backpacks(&backpacks[i], &backpacks[i + 1], &backpacks[i + 2])
        // {
        //     sum += common_item.value();
        // }
    }
    sum
}

pub fn compute_sum_of_common_item_priorities(backpacks: &[Backpack]) -> u32 {
    let mut sum = 0;
    for backpack in backpacks {
        if let Some(item) = backpack.find_common_item_between_pockets() {
            sum += item.value();
        }
    }
    sum
}

// A more generic approach
pub fn find_common_items(item_lists: &[&[Item]]) -> Vec<Item> {
    let min_items = item_lists.len() as u32;
    let mut unique_items: Vec<HashSet<Item>> = Vec::with_capacity(item_lists.len());

    // Transform each list of item into unique lists.
    for item_list in item_lists {
        let items: HashSet<Item> = item_list.into_iter().map(|i| *i).collect();
        unique_items.push(items);
    }

    // Iterate over the unique lists, counting how many times an item shows across all lists,
    // then store in a Map.
    let mut item_counts = HashMap::<Item, u32>::new();
    for item in unique_items.iter().flat_map(|items| items.iter()) {
        let count = *item_counts.get(&item).unwrap_or(&0);
        item_counts.insert(*item, count + 1);
    }

    // Filter Map keeping only items that show across all lists and return the final list of items.
    let counts = item_counts
        .iter()
        .filter(|(_, count)| **count >= min_items)
        .map(|(item, _)| *item)
        .collect::<Vec<_>>();
    counts
}

pub fn find_common_item_between_backpacks(
    backpack1: &Backpack,
    backpack2: &Backpack,
    backpack3: &Backpack,
) -> Option<Item> {
    // Going for brute force again.
    for item1 in &backpack1.items {
        for item2 in &backpack2.items {
            if item1 == item2 {
                // Minor optimisation: we only care about item3 if 1 and 2 are the same.
                for item3 in &backpack3.items {
                    if item1 == item3 {
                        return Some(*item1);
                    }
                }
            }
        }
    }
    None
}

pub fn parse_backpacks(input: &str) -> Result<Vec<Backpack>, InputParseError> {
    let mut backpacks = vec![];
    for line in input.lines() {
        let backpack = line.parse::<Backpack>()?;
        backpacks.push(backpack);
    }
    Ok(backpacks)
}

fn main() {
    let input = include_str!("day3.txt");
    let backpacks = parse_backpacks(input).unwrap();

    // Part 1.
    println!(
        "Part 1: {}",
        compute_sum_of_common_item_priorities(&backpacks)
    );

    // Part 2:
    println!("Part 2: {}", compute_sum_of_badges_priorities(&backpacks));
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw";
    use super::*;

    #[test]
    fn computes_sum_of_badges_priorities() {
        let backpacks = parse_backpacks(SAMPLE_INPUT).unwrap();
        assert_eq!(compute_sum_of_badges_priorities(&backpacks), 70);
    }

    #[test]
    fn computes_sum_of_common_item_priorities() {
        let backpacks = parse_backpacks(SAMPLE_INPUT).unwrap();
        assert_eq!(compute_sum_of_common_item_priorities(&backpacks), 157);
    }

    #[test]
    pub fn parses_backpack() {
        let backpack = "vJrwpWtwJgWrhcsFMMfFFhFp".parse::<Backpack>().unwrap();
        assert_eq!(backpack.items.len(), 24);
        assert_eq!(backpack.items[0].0, 'v');
        assert_eq!(backpack.items.last().unwrap().0, 'p');
    }

    #[test]
    pub fn computes_lowercase_item_value() {
        let item = Item('p');
        assert_eq!(item.value(), 16);

        let item = Item('v');
        assert_eq!(item.value(), 22);

        let item = Item('t');
        assert_eq!(item.value(), 20);

        let item = Item('s');
        assert_eq!(item.value(), 19);
    }

    #[test]
    pub fn computes_uppercase_item_value() {
        let item = Item('L');
        assert_eq!(item.value(), 38);

        let item = Item('P');
        assert_eq!(item.value(), 42);
    }

    #[test]
    pub fn finds_common_item_between_pockets() {
        let backpack = "vJrwpWtwJgWrhcsFMMfFFhFp".parse::<Backpack>().unwrap();
        assert_eq!(
            backpack.find_common_item_between_pockets().unwrap(),
            Item('p')
        );

        let backpack = "    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"
            .parse::<Backpack>()
            .unwrap();
        assert_eq!(
            backpack.find_common_item_between_pockets().unwrap(),
            Item('L')
        );

        let backpack = "PmmdzqPrVvPwwTWBwg   ".parse::<Backpack>().unwrap();
        assert_eq!(
            backpack.find_common_item_between_pockets().unwrap(),
            Item('P')
        );

        let backpack = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"
            .parse::<Backpack>()
            .unwrap();
        assert_eq!(
            backpack.find_common_item_between_pockets().unwrap(),
            Item('v')
        );

        let backpack = "ttgJtRGJQctTZtZT".parse::<Backpack>().unwrap();
        assert_eq!(
            backpack.find_common_item_between_pockets().unwrap(),
            Item('t')
        );

        let backpack = "CrZsJsPPZsGzwwsLwLmpwMDw".parse::<Backpack>().unwrap();
        assert_eq!(
            backpack.find_common_item_between_pockets().unwrap(),
            Item('s')
        );
    }

    #[test]
    fn finds_common_item_between_backpacks() {
        let backpacks = parse_backpacks(SAMPLE_INPUT).unwrap();
        let common_item =
            find_common_item_between_backpacks(&backpacks[0], &backpacks[1], &backpacks[2])
                .unwrap();
        assert_eq!(common_item, Item('r'));

        let common_item =
            find_common_item_between_backpacks(&backpacks[3], &backpacks[4], &backpacks[5])
                .unwrap();
        assert_eq!(common_item, Item('Z'));
    }

    #[test]
    fn finds_common_item_shared() {
        let backpacks = parse_backpacks(SAMPLE_INPUT).unwrap();
        let items = &[
            &backpacks[0].items as &[Item],
            &backpacks[1].items as &[Item],
            &backpacks[2].items as &[Item],
        ];

        let common_item = find_common_items(items).iter().next().unwrap().clone();
        assert_eq!(common_item, Item('r'));
    }
}
