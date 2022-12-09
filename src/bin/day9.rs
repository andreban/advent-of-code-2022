use std::collections::HashSet;

const SAMPLE_INPUT: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

#[derive(Debug, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
struct Position {x: isize, y: isize}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Position {
    pub fn move_amount(&mut self, x: isize, y: isize) {
        self.x += x;
        self.y += y;
    }

    pub fn calculate_movement(&self, other: &Position) -> (isize, isize) {
        let diff_x = self.x - other.x;
        let diff_y = self.y - other.y;

        // println!("diffs ({diff_x}, {diff_y})");
        
        if diff_x.abs() > 1 || diff_y.abs() > 1 {
            let move_x = if diff_x != 0 {
                diff_x / diff_x.abs()
            } else {
                0
            };

            let move_y = if diff_y != 0 {
                diff_y / diff_y.abs()
            } else {
                0
            };
            (move_x, move_y)
        } else {
            (0, 0)
        }
    }
}

fn main () {
    let mut head = Position::default();
    let mut tail = Position::default();
    let mut tail_positions = HashSet::<Position>::new();

    println!("----==== Part 1 ====----");
    // for line in include_str!("day9.txt").lines() {
    for line in SAMPLE_INPUT.lines() {
        println!("{line}");
        let mut split = line.split(" ");
        let direction: (isize, isize) = match split.next().unwrap() {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => panic!("Invalid input"),
        };
        let amount = split.next().unwrap().parse().unwrap();
        for _ in 0..amount {
            head.move_amount(direction.0, direction.1);
            let (x, y) = head.calculate_movement(&tail);
            println!("move_tail: ({x}, {y})");
            tail.move_amount(x, y);
            println!("head: {:?}, tail: {:?}", head, tail);
            tail_positions.insert(tail);
        } 
    }
    println!("Number tail positions: {}", tail_positions.len());

    println!("----==== Part 2 ====----");
    let mut rope = vec![Position::default(); 10];
    let mut tail_positions = HashSet::<Position>::new();
    // for line in include_str!("day9.txt").lines() {
    for line in SAMPLE_INPUT.lines() {
        let mut split = line.split(" ");
        let (x, y) = match split.next().unwrap() {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => panic!("Invalid input"),
        };
        let amount = split.next().unwrap().parse().unwrap();
        for _ in 0..amount {
            rope[0].move_amount(x, y);
            for i in 1..rope.len() {
                let (x, y) = rope[i - 1].calculate_movement(&rope[i]);
                rope[i].move_amount(x, y);
            }
            tail_positions.insert(rope[rope.len() - 1]);
        }
    }
    println!("Number tail positions: {}", tail_positions.len());
}
