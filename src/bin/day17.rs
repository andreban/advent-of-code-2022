use std::time::Duration;

use lazy_static::lazy_static;

const SAMPLE_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
const CHAMBER_WIDTH: usize = 7;
const NUM_ROCKS: usize = 2022;

lazy_static! {
    static ref ROCKS: Vec<RockShape> = vec![
        RockShape::new(4, 1, vec![
            true, true, true, true,  // ####
        ]),
        RockShape::new(3, 3, vec![
            false, true, false,      // .#.
            true, true, true,        // ###
            false, true, false,      // .#.
        ]),
        RockShape::new(3,3, vec![
            true, true, true,        // ###     In our chamber, 0 is the bottom, so our piece is
            false, false, true,      // ..#  => upside down here. Thi is the only asymmetrical
            false, false, true,      // ..#     piece.
        ]),
        RockShape::new(1, 4, vec![
            true,                    // #
            true,                    // #
            true,                    // #
            true,                    // #
        ]),
        RockShape::new(2, 2, vec![
            true, true,              // ##
            true, true,              // ##
        ]),
    ];
}

struct CircularIterator<'a, T> {
    container: &'a [T],
    current_index: usize,
}

impl<'a, T> CircularIterator<'a, T> {
    pub fn new(container: &'a [T]) -> Self {
        CircularIterator {
            container,
            current_index: 0,
        }
    }

    pub fn next(&mut self) -> &T {
        let result = &self.container[self.current_index];
        self.current_index = (self.current_index + 1) % self.container.len();
        result
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
    }
}

#[derive(Debug)]
struct RockShape {
    pub width: usize,
    pub height: usize,
    pattern: Vec<bool>,
}

impl RockShape {
    pub fn new(width: usize, height: usize, pattern: Vec<bool>) -> Self {
        assert_eq!(pattern.len(), width * height);
        Self {
            width,
            height,
            pattern,
        }
    }

    pub fn is_rock(&self, x: usize, y: usize) -> bool {
        assert!(x < self.width && y < self.height);
        self.pattern[y * self.width + x]
    }
}

#[derive(Debug)]
struct Rock<'a> {
    shape: &'a RockShape,
    x: isize,
    y: isize,
}

impl<'a> Rock<'a> {
    pub fn new(shape: &'a RockShape, x: isize, y: isize) -> Self {
        Rock { shape, x, y }
    }

    pub fn left(&self) -> isize {
        self.x
    }

    pub fn right(&self) -> isize {
        self.x + self.shape.width as isize
    }

    pub fn top(&self) -> isize {
        self.y + self.shape.height as isize
    }

    pub fn bottom(&self) -> isize {
        self.y
    }

    pub fn is_rock(&self, x: isize, y: isize) -> bool {
        // Translate global coordinates to the local coordinate for the shape.
        let shape_x = x - self.x;
        let shape_y = y - self.y;

        assert!(shape_x >= 0 && shape_x < self.shape.width as isize);
        assert!(shape_y >= 0 && shape_y < self.shape.height as isize);

        self.shape.is_rock(shape_x as usize, shape_y as usize)
    }
}

#[derive(Debug)]
struct Chamber {
    width: usize,
    height: usize,
    pattern: Vec<bool>,
    high_water_mark: usize,
}

impl Chamber {
    pub fn new() -> Self {
        Self {
            width: CHAMBER_WIDTH,
            height: 0,
            pattern: vec![],
            high_water_mark: 0,
        }
    }

    pub fn ensure_height(&mut self, new_height: usize) {
        let diff = new_height.saturating_sub(self.height);
        if diff == 0 {
            return;
        }

        self.pattern.extend(vec![false; diff * self.width]);
        self.height = new_height;
    }

    pub fn add_rock(&mut self, rock: &Rock) {
        for x in rock.left()..rock.right() {
            for y in rock.bottom()..rock.top() {
                self.set_rock(x, y, rock.is_rock(x, y));
            }
        }
        self.high_water_mark = self.high_water_mark.max(rock.top() as usize);
    }

    pub fn collides(&self, rock: &Rock) -> bool {
        // Check for collision against the walls.
        if rock.left() < 0 || rock.right() > self.width as isize || rock.bottom() < 0 {
            return true;
        }

        // Ceck for collision against other rocks.
        for x in rock.left()..rock.right() {
            for y in rock.bottom()..rock.top() {
                if rock.is_rock(x, y) && self.is_rock(x, y) {
                    return true;
                }
            }
        }
        false
    }

    pub fn set_rock(&mut self, x: isize, y: isize, is_rock: bool) {
        assert!(x >= 0 && x < self.width as isize);
        assert!(y >= 0 && y < self.height as isize);

        let x = x as usize;
        let y = y as usize;

        self.pattern[y * self.width + x] = is_rock
    }

    pub fn is_rock(&self, x: isize, y: isize) -> bool {
        assert!(x >= 0 && x < self.width as isize);
        assert!(y >= 0 && y < self.height as isize);

        let x = x as usize;
        let y = y as usize;

        self.pattern[y * self.width + x]
    }
}

fn print(chamber: &Chamber, rock: &Rock, movement: char) {
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    print!("{esc}[1;1H", esc = 27 as char);
    println!("Movement: {}", movement);
    for y in (chamber.height.saturating_sub(25) as isize..chamber.height as isize).rev() {
        print!("{:5} |", y);
        for x in 0..chamber.width as isize {
            if chamber.is_rock(x, y) {
                print!("#");
            } else if y >= rock.bottom()
                && y < rock.top()
                && x >= rock.left()
                && x < rock.right()
                && rock.is_rock(x, y)
            {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("      +-------+");
    std::thread::sleep(Duration::from_millis(200));
    // std::io::stdin().read_line(&mut String::new()).unwrap();
}

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let input = SAMPLE_INPUT;
    // let input = include_str!("day17.txt");

    let jets = input
        .chars()
        .filter(|c| *c == '<' || *c == '>')
        .collect::<Vec<_>>();

    let mut jet_sequence = CircularIterator::new(&jets);
    let mut rock_sequence = CircularIterator::new(&ROCKS);
    let mut chamber = Chamber::new();

    for i in 1..=NUM_ROCKS {
        // if i % 100000 == 0 {
        println!("{} -> {}", i, chamber.high_water_mark);
        // }
        let mut rock = Rock::new(
            rock_sequence.next(),
            2,
            chamber.high_water_mark as isize + 3,
        );
        chamber.ensure_height(rock.top() as usize);
        // print(&chamber, &rock, '-');
        let mut moved_down = true;
        while moved_down {
            let jet = jet_sequence.next();

            // Can the rock be pushed by the jet?
            let x = match jet {
                '>' => rock.x as isize + 1,
                '<' => rock.x as isize - 1,
                _ => panic!("Invalid movement"),
            };
            let next_rock = Rock::new(rock.shape, x, rock.y);

            // Check if rock is bumping into the chamber walls.
            if !chamber.collides(&next_rock) {
                rock = next_rock;
            }

            // let arrow = match jet {
            //     '>' => '→',
            //     '<' => '←',
            //     _ => panic!("Woooooooot!!??"),
            // };
            // print(&chamber, &rock, arrow);

            // Can the rock fall down?
            let y = rock.y as isize - 1;
            let next_rock = Rock::new(rock.shape, rock.x, y);
            if chamber.collides(&next_rock) {
                chamber.add_rock(&rock);
                moved_down = false;
                continue;
            }
            rock = next_rock;
            // print(&chamber, &rock, '↓');
        }
    }

    println!("{:?}", chamber.high_water_mark);
}
