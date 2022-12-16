use std::{str::FromStr, collections::HashSet};

const SAMPLE_INPUT: &str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

#[derive(Debug)]
pub struct InputParseError;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl FromStr for Point {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next()
            .ok_or(InputParseError)?
            .parse::<u32>()
            .map_err(|_| InputParseError)?;

        let y = parts.next()
            .ok_or(InputParseError)?
            .parse::<u32>()
            .map_err(|_| InputParseError)?;
        Ok(Self { x, y })
    }   
}

fn main() {
    // let input = include_str!("day14.txt");
    let input = SAMPLE_INPUT;
    let mut walls = vec![];
    let (mut min_x, mut max_x) = (u32::MAX, u32::MIN);
    let (mut min_y, mut max_y) = (0, u32::MIN);
    
    for line in input.lines() {
        let mut wall = vec![];
        for split in line.split("->") {
            let point = split.trim().parse::<Point>().unwrap();
            min_x = u32::min(min_x, point.x);
            max_x = u32::max(max_x, point.x);
            min_y = u32::min(min_y, point.y);
            max_y = u32::max(max_y, point.y);
            wall.push(point);
        } 
        walls.push(wall);
    }

    let mut blocks = HashSet::new();
    for wall in walls {
        let mut it = wall.iter();
        let mut current = it.next().unwrap();
        while let Some(next) = it.next() {
            // This bit assumes that when movin between points, it moves either in the X or Y axis,
            // but never both. We'd need to prioritise which axis to move if that was the case. 
            let (min_x, max_x) = (u32::min(current.x, next.x), u32::max(current.x, next.x));
            for x in min_x..=max_x {
                blocks.insert(Point {x, y: current.y});
            }

            let (min_y, max_y) = (u32::min(current.y, next.y), u32::max(current.y, next.y));
            for y in min_y..=max_y {
                blocks.insert(Point {x: current.x, y});
            }

            current = next;
        }
    }

    let mut grain_count = 0;
    loop {
        let mut grain = Point {x: 500, y: 0};

        let mut moved = true;
        while moved {
            moved = true;

            if grain.y == max_y + 1 {
                blocks.insert(grain);
                break;     
            }

            // Can move directly down?
            let down = Point { x: grain.x, y: grain.y + 1};
            if !blocks.contains(&down) {
                grain = down;
                continue;
            }

            let down_left = Point {x: grain.x - 1, y: grain.y + 1};
            if !blocks.contains(&down_left) {
                grain = down_left;
                continue;
            }

            let down_right = Point {x: grain.x + 1, y: grain.y + 1};
            if !blocks.contains(&down_right) {
                grain = down_right;
                continue;
            }

            moved = false;
            blocks.insert(grain);
        }

        // if grain.y >= max_y {
        //     break;
        // } else {
        //     grain_count += 1;
        // }

        grain_count += 1;
        if blocks.contains(&Point {x: 500, y: 0}) {
            break;
        } 
    }
    println!("{:?}", grain_count);
}

