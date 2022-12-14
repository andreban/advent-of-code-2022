use std::collections::HashMap;
use std::io::Write;
use std::thread;

const SAMPLE_INPUT: &str = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    height_map: Vec<u8>,
}

impl Map {
    fn pos_at(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    pub fn height_at_coord(&self, coordinate: &Coordinate) -> u8 {
        self.height_at(coordinate.y, coordinate.x)
    }

    pub fn height_at(&self, row: usize, col: usize) -> u8 {
        let pos = self.pos_at(row, col);
        self.height_map[pos]
    }

    pub fn can_move(&self, from: &Coordinate, to: &Coordinate) -> bool {
        self.height_at_coord(to) <= self.height_at_coord(from) + 1
    }

    pub fn coordinate_at(&self, from: &Coordinate, direction: Direction) -> Option<Coordinate> {
        match direction {
            Direction::Up => {
                if from.y <= 0 {
                    None
                } else {
                    Some(Coordinate::new(from.x, from.y - 1))
                }
            }

            Direction::Down => {
                if from.y >= self.height - 1 {
                    None
                } else {
                    Some(Coordinate::new(from.x, from.y - 1))
                }
            }

            Direction::Left => {
                if from.x <= 0 {
                    None
                } else {
                    Some(Coordinate::new(from.x - 1, from.y))
                }
            }

            Direction::Right => {
                if from.x >= self.width - 1 {
                    None
                } else {
                    Some(Coordinate::new(from.x + 1, from.y))
                }
            }
        }
    }
}

fn calculate_shortest_distance(
    map: &Map,
    current_position: Coordinate,
    end_position: Coordinate,
    current_distance: usize,
    visited: &mut HashMap<Coordinate, usize>,
) -> Option<usize> {
    if current_position == end_position {
        return Some(current_distance);
    }

    // This coordinate has already been visited and the distance is smaller than the current distance,
    // so we just return it.
    if let Some(d) = visited.get(&current_position) {
        if *d <= current_distance {
            return None;
        }
    }

    visited.insert(current_position, current_distance);

    let mut shortest_distance = None;
    // North
    if current_position.y > 0 {
        let c = Coordinate::new(current_position.x, current_position.y - 1);
        if map.can_move(&current_position, &c) {
            let new_distance =
                calculate_shortest_distance(map, c, end_position, current_distance + 1, visited);
            if let Some(d) = new_distance {
                if shortest_distance.is_none() || d < shortest_distance.unwrap() {
                    shortest_distance = new_distance;
                }
            }
        }
    }

    // South
    if current_position.y < map.height - 1 {
        let c = Coordinate::new(current_position.x, current_position.y + 1);
        if map.can_move(&current_position, &c) {
            let new_distance =
                calculate_shortest_distance(map, c, end_position, current_distance + 1, visited);
            if let Some(d) = new_distance {
                if shortest_distance.is_none() || d < shortest_distance.unwrap() {
                    shortest_distance = new_distance;
                }
            }
        }
    }

    // Left
    if current_position.x > 0 {
        let c = Coordinate::new(current_position.x - 1, current_position.y);
        if map.can_move(&current_position, &c) {
            let new_distance =
                calculate_shortest_distance(map, c, end_position, current_distance + 1, visited);
            if let Some(d) = new_distance {
                if shortest_distance.is_none() || d < shortest_distance.unwrap() {
                    shortest_distance = new_distance;
                }
            }
        }
    }

    // Right
    if current_position.x < map.width - 1 {
        let c = Coordinate::new(current_position.x + 1, current_position.y);
        if map.can_move(&current_position, &c) {
            let new_distance =
                calculate_shortest_distance(map, c, end_position, current_distance + 1, visited);
            if let Some(d) = new_distance {
                if shortest_distance.is_none() || d < shortest_distance.unwrap() {
                    shortest_distance = new_distance;
                }
            }
        }
    }

    shortest_distance
}

fn main() {
    let child = thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            let input = include_str!("day12.txt");
            // let input = SAMPLE_INPUT;
            let map_width = input.lines().next().unwrap().len();
            let mut map_height = 0;
            let mut heights = vec![];
            let mut initial_position = Coordinate::default();
            let mut destination = Coordinate::default();
            for line in input.lines() {
                for (position, char) in line.chars().enumerate() {
                    match char {
                        'a'..='z' => {
                            heights.push(char as u8);
                        }
                        'S' => {
                            initial_position = Coordinate::new(position, map_height);
                            heights.push('a' as u8);
                        }
                        'E' => {
                            destination = Coordinate::new(position, map_height);
                            heights.push('z' as u8);
                        }
                        _ => {}
                    }
                }
                map_height += 1;
            }
            println!("{}, {}", map_width, map_height);
            let map = Map {
                width: map_width,
                height: map_height,
                height_map: heights,
            };

            let mut visited = HashMap::new();
            let shortest_distance =
                calculate_shortest_distance(&map, initial_position, destination, 0, &mut visited);
            println!("Part 1: {:?}", shortest_distance);

            let mut best_distance = usize::MAX;
            for y in 0..map.height {
                for x in 0..map.width {
                    print!("\r{}:{}", y, x);
                    std::io::stdout().flush().unwrap();
                    if map.height_at(y, x) == 'a' as u8 {
                        visited.clear();
                        let shortest_distance = calculate_shortest_distance(
                            &map,
                            Coordinate::new(x, y),
                            destination,
                            0,
                            &mut visited,
                        );
                        if shortest_distance.is_some() && shortest_distance.unwrap() < best_distance
                        {
                            best_distance = shortest_distance.unwrap();
                            println!("Best: {}, {}, {}", y, x, best_distance);
                        }
                    }
                }
            }
            println!("Part 2: {:?}", best_distance);
        })
        .unwrap();
    child.join().unwrap();
}
