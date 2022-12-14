use std::str::FromStr;

type Item = u64;

#[derive(Debug)]
pub struct ParseInputError;

#[derive(Clone, Debug)]
pub enum Operand {
    Old,
    Value(Item),
}

#[derive(Copy, Clone, Debug)]
pub enum Operator {
    Sum,
    Mult,
}

impl Operator {
    pub fn apply(&self, o1: &Item, o2: &Operand) -> Item {
        let o2 = match o2 {
            Operand::Old => o1,
            Operand::Value(v) => v,
        };

        match self {
            Operator::Sum => o1 + o2,
            Operator::Mult => o1 * o2,
        }
    }
}

#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<Item>,
    pub operation: (Operator, Operand),
    pub divisible_by: Item,
    pub monkey1: usize,
    pub monkey2: usize,
}

impl FromStr for Monkey {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split("\r\n").collect();

        // Parse items starting items.
        let items = lines[1].split(':').next_back().unwrap();
        let items = items
            .split(',')
            .map(|f| f.trim().parse().unwrap())
            .collect::<Vec<_>>();

        // Parse Operation.
        let parts = lines[2]
            .split('=')
            .next_back()
            .unwrap()
            .split(' ')
            .collect::<Vec<_>>();
        let operator = match parts[2] {
            "*" => Operator::Mult,
            "+" => Operator::Sum,
            _ => return Err(ParseInputError),
        };

        // Parse Operand.
        let operand = if parts[3] == "old" {
            Operand::Old
        } else {
            Operand::Value(parts[3].parse().unwrap())
        };

        // Parse Divisible by
        let divisible_by = lines[3]
            .split(' ')
            .next_back()
            .ok_or(ParseInputError)?
            .parse()
            .map_err(|_| ParseInputError)?;

        let monkey1 = lines[4]
            .split(' ')
            .next_back()
            .ok_or(ParseInputError)?
            .parse::<usize>()
            .map_err(|_| ParseInputError)?;

        let monkey2 = lines[5]
            .split(' ')
            .next_back()
            .ok_or(ParseInputError)?
            .parse::<usize>()
            .map_err(|_| ParseInputError)?;

        Ok(Monkey {
            items,
            operation: (operator, operand),
            divisible_by,
            monkey1,
            monkey2,
        })
    }
}

fn main() {
    let input = include_str!("day11_sample.txt");
    // let input = include_str!("day11.txt");

    let mut monkeys: Vec<Monkey> = vec![];
    input.split("\r\n\r\n").for_each(|monkey| {
        let monkey: Monkey = monkey.parse().unwrap();
        println!("{:?}", monkey);
        monkeys.push(monkey);
    });

    let div = monkeys
        .iter()
        .map(|m| m.divisible_by)
        .fold(1_u64, |acc, f| acc * f);

    let mut inspections = vec![0_usize; monkeys.len()];

    let part2 = false;
    let iterations = if part2 { 10000 } else { 20 };

    for round in 1..=iterations {
        // Executes one round.
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop() {
                inspections[i] += 1;
                let (operator, operand) = &monkeys[i].operation;
                let item = if part2 {
                    operator.apply(&item, operand) % div
                } else {
                    operator.apply(&item, operand) / 3
                };
                let divisable = &item % monkeys[i].divisible_by == 0;

                let target_id = if divisable {
                    monkeys[i].monkey1
                } else {
                    monkeys[i].monkey2
                };
                monkeys[target_id].items.push(item);
            }
        }

        if round == 1 || round == 20 || round % 1000 == 0 {
            println!("Round {}", round);
            for (monkey, inspections) in inspections.iter().enumerate() {
                println!("\tMonkey: {}, Inspections: {}", monkey, inspections);
            }
            println!();
        }
    }
    inspections.sort();
    let top2 = inspections.iter().rev().take(2).collect::<Vec<_>>();
    println!("Top 2: {:?}", top2);
    println!("Monkey Business: {}", top2[0] * top2[1]);
}
