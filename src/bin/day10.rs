use std::str::FromStr;

#[derive(Debug)]
pub struct ParseInputError(String);

#[derive(Debug)]
pub enum Command {
    Addx(isize),
    Noop,
}

impl FromStr for Command {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s == "noop" {
            return Ok(Command::Noop);
        }

        let mut split = s.split(' ');
        if split.next().unwrap() == "addx" {
            let value = split
                .next()
                .ok_or_else(|| ParseInputError("addx command missing value".to_string()))?
                .parse::<isize>()
                .map_err(|e| ParseInputError(e.to_string()))?;
            return Ok(Command::Addx(value));
        }

        Err(ParseInputError(format!("Unknown command: {}", s)))
    }
}

#[derive(Debug)]
pub struct CRT {
    pub pixels: Vec<char>,
    pub width: usize,
    pub height: usize,
}

impl CRT {
    fn char_pos(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn set_char_at(&mut self, x: usize, y: usize, c: char) {
        let pos = self.char_pos(x, y);
        self.pixels[pos] = c;
    }

    pub fn char_at(&self, x: usize, y: usize) -> char {
        let pos = self.char_pos(x, y);
        self.pixels[pos]
    }
}

impl ToString for CRT {
    fn to_string(&self) -> String {
        let mut content = String::with_capacity((self.width + 1) * self.height + 2);
        content.push('\n');
        for y in 0..self.height {
            for x in 0..self.width {
                let char = self.char_at(x, y);
                content.push(char);
            }
            content.push('\n')
        }
        content.push('\n');
        content
    }
}

impl Default for CRT {
    fn default() -> Self {
        let width = 40;
        let height = 6;
        Self {
            pixels: vec!['.'; width * height],
            width,
            height,
        }
    }
}

fn main() {
    let mut crt = CRT::default();
    let mut cycle = 0;
    let mut register_x: isize = 1;
    let mut sum_frequencies = 0;

    for line in include_str!("day10.txt").lines() {
        // for line in include_str!("day10_sample.txt").lines() {
        let command = line.parse::<Command>().unwrap();
        let (cycles, value) = match command {
            Command::Noop => (1, 0),
            Command::Addx(value) => (2, value),
        };
        for _ in 0..cycles {
            println!("{} => {}", cycle, register_x);
            let row = cycle / 40;
            let col = cycle % 40;
            if col >= register_x - 1 && col <= register_x + 1 {
                crt.set_char_at(col as usize, row as usize, '#');
            }

            cycle += 1;
            if cycle == 20
                || cycle == 60
                || cycle == 100
                || cycle == 140
                || cycle == 180
                || cycle == 220
            {
                let frequency = cycle as isize * register_x;
                sum_frequencies += frequency;
                println!(
                    "Cycle: {} / X:{} => Frequency: {}",
                    cycle, register_x, frequency
                );
            }
        }
        register_x += value;
    }
    println!("Sum Frequencies: {sum_frequencies}");
    println!("{}", crt.to_string());
}
