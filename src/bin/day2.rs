use std::str::FromStr;

#[derive(Debug)]
struct ParseMatchError;

enum RoundResult {
    Loss,
    Win,
    Draw,
}

impl FromStr for RoundResult {
    type Err = ParseMatchError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RoundResult::Loss),
            "Y" => Ok(RoundResult::Draw),
            "Z" => Ok(RoundResult::Win),
            _ => Err(ParseMatchError),
        }
    }
}

#[derive(Clone, Copy)]
enum HandShape {
    Rock,
    Paper,
    Scissor,
}

impl HandShape {
    pub fn points(&self) -> u32 {
        match self {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissor => 3,
        }
    }
}

impl FromStr for HandShape {
    type Err = ParseMatchError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // "X" | "A" => Ok(HandShape::Rock),
            // "Y" | "B" => Ok(HandShape::Paper),
            // "Z" | "C" => Ok(HandShape::Scissor),
            "A" => Ok(HandShape::Rock),
            "B" => Ok(HandShape::Paper),
            "C" => Ok(HandShape::Scissor),
            _ => Err(ParseMatchError),
        }
    }
}

struct Round {
    player1_hand: HandShape,
    player2_hand: HandShape,
}

impl Round {
    pub fn round_result(&self) -> RoundResult {
        match self.player1_hand {
            HandShape::Rock => match self.player2_hand {
                HandShape::Rock => RoundResult::Draw,
                HandShape::Paper => RoundResult::Win,
                HandShape::Scissor => RoundResult::Loss,
            },
            HandShape::Paper => match self.player2_hand {
                HandShape::Rock => RoundResult::Loss,
                HandShape::Paper => RoundResult::Draw,
                HandShape::Scissor => RoundResult::Win,
            },
            HandShape::Scissor => match self.player2_hand {
                HandShape::Rock => RoundResult::Win,
                HandShape::Paper => RoundResult::Loss,
                HandShape::Scissor => RoundResult::Draw,
            },
        }
    }

    pub fn score(&self) -> u32 {
        let mut score = 0;
        score += self.player2_hand.points();

        score += match self.round_result() {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Loss => 0,
        };
        score
    }
}

impl FromStr for Round {
    type Err = ParseMatchError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        let player1_hand = parts[0].parse::<HandShape>()?;
        // let player2_hand = parts[1].parse::<HandShape>()?;

        let target_result = parts[1].parse::<RoundResult>()?;
        let player2_hand = match target_result {
            RoundResult::Win => match player1_hand {
                HandShape::Rock => HandShape::Paper,
                HandShape::Paper => HandShape::Scissor,
                HandShape::Scissor => HandShape::Rock,
            },
            RoundResult::Draw => player1_hand,
            RoundResult::Loss => match player1_hand {
                HandShape::Rock => HandShape::Scissor,
                HandShape::Paper => HandShape::Rock,
                HandShape::Scissor => HandShape::Paper,
            },
        };

        Ok(Self {
            player1_hand,
            player2_hand,
        })
    }
}

fn main() {
    let input = include_str!("day2.txt");
    let mut total_score = 0;
    for game in input.lines() {
        let game = game.parse::<Round>().unwrap();
        total_score += game.score();
    }
    println!("{total_score}");
}
