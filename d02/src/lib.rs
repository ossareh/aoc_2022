use std::{
    convert::From,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Weapon {
    Rock,
    Paper,
    Scissors,
}

impl From<Weapon> for u64 {
    fn from(value: Weapon) -> Self {
        match value {
            Weapon::Rock => 1,
            Weapon::Paper => 2,
            Weapon::Scissors => 3,
        }
    }
}

impl From<u8> for Weapon {
    fn from(value: u8) -> Self {
        match value {
            0x41 => Weapon::Rock,
            0x42 => Weapon::Paper,
            0x43 => Weapon::Scissors,
            _ => unreachable!("invalid weapon code"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Outcome {
    Win,
    Tie,
    Loss,
}

impl From<Outcome> for u64 {
    fn from(value: Outcome) -> Self {
        match value {
            Outcome::Win => 6,
            Outcome::Tie => 3,
            Outcome::Loss => 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Round {
    opp: Weapon,
    you: Weapon,
}

impl Round {
    /// Played from the point of view of you
    ///
    /// Rules:
    ///  * Rock beats Scissors
    ///  * Scissors beats Paper
    ///  * Paper beats Rock
    fn play(&self) -> Outcome {
        match (self.you, self.opp) {
            (Weapon::Rock, Weapon::Rock)
            | (Weapon::Paper, Weapon::Paper)
            | (Weapon::Scissors, Weapon::Scissors) => Outcome::Tie,

            (Weapon::Rock, Weapon::Paper) => Outcome::Loss,
            (Weapon::Paper, Weapon::Rock) => Outcome::Win,

            (Weapon::Rock, Weapon::Scissors) => Outcome::Win,
            (Weapon::Scissors, Weapon::Rock) => Outcome::Loss,

            (Weapon::Scissors, Weapon::Paper) => Outcome::Win,
            (Weapon::Paper, Weapon::Scissors) => Outcome::Loss,
        }
    }
}

/// implements the first strategy
///
/// Here we assume The second column is one of "X" (Rock), "Y" (Paper), "Z" (Scissors)
pub fn strategy_1(row: Vec<u8>) -> Round {
    Round {
        opp: row[0].into(),
        you: match row.last() {
            Some(0x58) => Weapon::Rock,
            Some(0x59) => Weapon::Paper,
            Some(0x5A) => Weapon::Scissors,
            _ => unreachable!("invalid table code"),
        },
    }
}

/// implements the second, correct, strategy
///
/// It turns out we were wrong in our assumption, and that the second column is:
///
///  - X = we need to lose this round
///  - Y = we need to tie this round
///  - Z = we need to win this round
///
///
pub fn strategy_2(row: Vec<u8>) -> Round {
    let opp: Weapon = row[0].into();
    Round {
        opp,
        you: match row.last() {
            // need to lose
            Some(0x58) => match opp {
                Weapon::Rock => Weapon::Scissors,
                Weapon::Paper => Weapon::Rock,
                Weapon::Scissors => Weapon::Paper,
            },
            // need to tie
            Some(0x59) => row[0].into(),
            // need to win
            Some(0x5A) => match opp {
                Weapon::Rock => Weapon::Paper,
                Weapon::Paper => Weapon::Scissors,
                Weapon::Scissors => Weapon::Rock,
            },
            _ => unreachable!("invalid table code"),
        },
    }
}

pub type RoundParser = fn(row: Vec<u8>) -> Round;

/// parses the input as a table
///
/// Each round is up to 4 bytes: `[opponent][space][you][LF (optional)]`
///
/// The record marker of `\n` is only optional on the file entry in the list.
fn parse_rounds(buf: &[u8], round_parser: RoundParser) -> Vec<Round> {
    BufReader::new(buf)
        .split(0xA)
        .map(|b| b.unwrap())
        .map(round_parser)
        .collect()
}

fn score_round(r: &Round) -> u64 {
    u64::from(r.you) + u64::from(r.play())
}

/// compute score from table
///
/// You're given a strategy guide for how to win the tent closest to the
/// food in the elf camp. The provider of the guide describes the following:
///
/// The table contains a list of rounds of Rock, Paper, Scissors
/// The table has two columns, seperated by a space
/// Each row is seperated by a \n
///
/// The first column is one of "A" (Rock), "B" (Paper), "C" (Scissors)
///
/// We don't know what the second column is, but we have a guess. See strategy_1
///
/// Rules:
///  - Rock beats Scissors
///  - Scissors beats Paper
///  - Paper beats Rock
///
/// Scoring:
///  - If you win a round you get 6 points
///  - If you draw a round you get 3 points
///  - If you lose a round you get 0 points
///
/// You are also given points based on your choice of weapon:
///
///  - Rock == 1
///  - Paper == 2
///  - Scissors == 3
///
/// ... Or so you think. See strategy_2 for more information.
pub fn compute(buf: &[u8], parser: RoundParser) -> u64 {
    let rounds = parse_rounds(buf, parser);
    rounds.iter().map(score_round).sum()
}

#[cfg(test)]
mod test {

    use super::*;

    fn test_data() -> Vec<u8> {
        Vec::from(
            &b"A Y
B X
C Z"[..],
        )
    }

    #[test]
    fn test_read_round() {
        assert_eq!(
            strategy_1(b"A X".to_vec()),
            Round {
                opp: Weapon::Rock,
                you: Weapon::Rock,
            }
        );

        assert_eq!(
            strategy_1(b"B Y".to_vec()),
            Round {
                opp: Weapon::Paper,
                you: Weapon::Paper,
            }
        );

        assert_eq!(
            strategy_1(b"C Z".to_vec()),
            Round {
                opp: Weapon::Scissors,
                you: Weapon::Scissors,
            }
        );
    }

    #[test]
    fn test_play_round() {
        assert_eq!(strategy_1(b"A X".to_vec()).play(), Outcome::Tie);
        assert_eq!(strategy_1(b"A Y".to_vec()).play(), Outcome::Win);
        assert_eq!(strategy_1(b"A Z".to_vec()).play(), Outcome::Loss);
    }

    #[test]
    fn test_outcome_score() {
        assert_eq!(u64::from(Outcome::Loss), 0);
        assert_eq!(u64::from(Outcome::Tie), 3);
        assert_eq!(u64::from(Outcome::Win), 6);
    }

    #[test]
    fn test_weapon_score() {
        assert_eq!(u64::from(Weapon::Rock), 1);
        assert_eq!(u64::from(Weapon::Paper), 2);
        assert_eq!(u64::from(Weapon::Scissors), 3);
    }

    #[test]
    fn test_parse_rounds_read_round_strategy_1() {
        let have = parse_rounds(&test_data(), strategy_1);
        assert_eq!(have.len(), 3);
    }

    #[test]
    fn test_parse_rounds_read_round_strategy_2() {
        let have = parse_rounds(&test_data(), strategy_2);
        assert_eq!(have.len(), 3);
    }

    #[test]
    fn test_score_round() {
        // Tie = 3 + 1 (Rock)
        assert_eq!(
            score_round(&Round {
                opp: Weapon::Rock,
                you: Weapon::Rock,
            }),
            4
        );

        // Win = 6 + 2 (Paper)
        assert_eq!(
            score_round(&Round {
                opp: Weapon::Rock,
                you: Weapon::Paper,
            }),
            8
        );

        // Loss = 0 + 3 (Scissors)
        assert_eq!(
            score_round(&Round {
                opp: Weapon::Rock,
                you: Weapon::Scissors,
            }),
            3
        )
    }

    #[test]
    fn test_compute() {
        assert_eq!(compute(&test_data(), strategy_1), 15);
        assert_eq!(compute(&test_data(), strategy_2), 12);
    }
}
