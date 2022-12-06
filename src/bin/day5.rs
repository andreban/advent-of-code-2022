#[derive(Debug)]
pub struct InputParseError;

fn main() {
    let input = include_str!("day5.txt");
    let (mut stacks, moves) = parse_input(input).unwrap();
    moves.iter().for_each(|m| stacks.apply_move_9001(m));
    for i in 0..stacks.0.len() {
        print!("{}", stacks.top(i + 1).unwrap_or(&' '));
    }
    println!();
}


#[derive(Debug)]
pub struct Stacks(Vec<Vec<char>>);

impl Stacks {
    pub fn apply_move(&mut self, mv: &Move) {
        for _ in 0..mv.quantity {
            let element = self.0[mv.from as usize - 1].pop().unwrap();
            self.0[mv.to as usize - 1].push(element);
        }
    }

    pub fn apply_move_9001(&mut self, mv: &Move) {
        let mut temp = vec![];
        for _ in 0..mv.quantity {
            let e = self.0[mv.from as usize - 1].pop().unwrap();
            temp.push(e);
        }

        while !temp.is_empty() {
            let e = temp.pop().unwrap();
            self.0[mv.to as usize - 1].push(e);
        }
    }

    pub fn top(&self, stack: usize) -> Option<&char> {
        self.0[stack - 1].last()
    }
}

#[derive(Debug)]
pub struct Move {
    pub from: u32,
    pub to: u32,
    pub quantity: u32,
}

pub fn parse_input(input: &str) -> Result<(Stacks, Vec<Move>), InputParseError> {
    let mut stacks: Stacks = Stacks(vec![]);

    let mut lines = input.lines();
    // Parse initial stack.
    loop {
        let Some(line) = lines.next() else {
            break;
        };

        // Empty line marks the end of the stacks section.
        if line.trim().is_empty() {
            break;
        }

        for (position, item) in line.chars().skip(1).step_by(4).enumerate() {
            if !item.is_alphabetic() {
                continue;
            }

            while stacks.0.len() <= position {
                stacks.0.push(vec![]);
            }

            stacks.0[position].insert(0, item);
        }
    }

    // Parse movements.
    let mut moves: Vec<Move> = vec![];
    for line in lines {
        let parts = line.split(" ").collect::<Vec<_>>();
        moves.push(Move {
            from: parts[3].parse().unwrap(),
            to: parts[5].parse().unwrap(),
            quantity: parts[1].parse().unwrap(),
        });
    }

    Ok((stacks, moves))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";


    #[test]
    fn part1_test() {
        let (mut stacks, moves) = parse_input(TEST_INPUT).unwrap();
        moves.iter().for_each(|m| stacks.apply_move(m));
        println!("{:?}", stacks);
        assert_eq!(stacks.top(1), Some(&'C'));
        assert_eq!(stacks.top(2), Some(&'M'));
        assert_eq!(stacks.top(3), Some(&'Z'));
    }
}
